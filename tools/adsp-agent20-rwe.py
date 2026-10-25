#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-built Hexagon V66 ADSP RWE entry routines; NEVER copy OEM bytes.

The first RWE PT_LOAD file begins at 0x6000 but its initial 0x30 bytes
are boot metadata; entry code begins at file 0x6030. This initial fragment
is a partial function and its future control-flow successors are not included.
"""
from __future__ import annotations
import argparse
import hashlib
import importlib.util
from pathlib import Path
import struct
import subprocess
import sys
import tempfile

TOOLS=Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:
    sys.path.insert(0,str(TOOLS))
from adsp_hexagon_v66 import SystemRegister,read_system_register,write_system_register

FILE_START=0x6030
FILE_END=0x6148
SIZE=FILE_END-FILE_START
PADDR=0x8BE03030
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=TOOLS/'adsp-agent20-rwe-startup.S'
DISPATCH_SOURCE=TOOLS/'adsp-agent20-rwe-dispatch.S'
STARTUP_SIZE=0x5c
DISPATCH_SIZE=0x48
FINALIZE_SOURCE=TOOLS/'adsp-agent20-rwe-finalize.S'
DUPLEX_SOURCE=TOOLS/'adsp-agent20-rwe-duplex.S'
FINALIZE_NOPAIR_SIZE=0x78
FINALIZE_SIZE=0x74


def baseline():
    spec=importlib.util.spec_from_file_location('adsp_previous_emitter',TOOLS/'emit-adsp-entry.py')
    if spec is None or spec.loader is None:
        raise RuntimeError('cannot load checked section assembler')
    mod=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def finalize(llvm_mc:str, previous)->bytes:
    """Build the privilege-state tail with its genuine two-op Hexagon duplex.

    LLVM6's automatic packet pairing moves an unrelated scalar operation into
    the duplex, changing instruction order. Disable pairing for the source
    assembly, then insert one *independently assembled source-defined duplex*
    in the verified two-word reserved location. This isn't a copied OEM word.
    """
    with tempfile.TemporaryDirectory(prefix='adsp-v66-state-') as dirname:
        obj=Path(dirname)/'state.o'
        result=subprocess.run([llvm_mc,'-triple=hexagon','-mcpu=hexagonv65',
            '-mno-pairing','-filetype=obj',str(FINALIZE_SOURCE),'-o',str(obj)],
            text=True,capture_output=True)
        if result.returncode:
            raise previous.AdspError('Hexagon no-pairing compilation failed: '+result.stderr)
        out=bytearray(previous.read_text_section(obj.read_bytes(),FINALIZE_NOPAIR_SIZE))
    # The LLVM6 non-duplex source must emit scalar r2=-1 and r3=-1 as two
    # consecutive still-in-packet words followed by r0=24 and r1=-1.
    # Their operand-encoded forms are independently produced by LLVM, not
    # substituted with vendor memory; exact slot boundaries are checked.
    src_words=tuple(struct.unpack_from('<I',out,at)[0] for at in (0x60,0x64,0x68,0x6c))
    if src_words!=(0x78df7fe2,0x78df7fe3,0x78004300,0x78dfffe1):
        raise previous.AdspError('unexpected scalar-pair instruction arrangement')
    duet=previous.assemble(llvm_mc,DUPLEX_SOURCE,4)
    if len(duet)!=4 or duet==struct.pack('<I',0x7f00c000):
        raise previous.AdspError('missing source-encoded Hexagon duplex')
    out=out[:0x68]+duet+out[0x70:]
    if len(out)!=FINALIZE_SIZE:
        raise previous.AdspError('size mismatch after Hexagon duplex scheduling')
    # Four privileged system-register transfers are specific V66 encodings.
    sentinel=struct.pack('<I',0x7f00c000)
    replacements={
        0x20:read_system_register(0,SystemRegister.SYSCFG),
        0x2c:write_system_register(SystemRegister.SYSCFG,0),
        0x30:read_system_register(0,SystemRegister.SSR),
        0x38:write_system_register(SystemRegister.SSR,0),
    }
    for at,word in replacements.items():
        if out[at:at+4]!=sentinel:
            raise previous.AdspError(f'privileged transfer sentinel moved at +0x{at:x}')
        out[at:at+4]=struct.pack('<I',word)
    if sentinel in (out[i:i+4] for i in range(0,len(out),4)):
        raise previous.AdspError('unresolved V66 sentinel in state-machine output')
    return bytes(out)


def render(llvm_mc:str)->bytes:
    previous=baseline()
    output=bytearray(previous.assemble(llvm_mc,SOURCE,STARTUP_SIZE))
    sentinel=struct.pack('<I',0x7F00C000)
    replacements={
        0x14:read_system_register(1,SystemRegister.SYSCFG),
        0x1c:write_system_register(SystemRegister.SYSCFG,1),
        0x20:read_system_register(0,SystemRegister.SGP0),
    }
    for at,word in replacements.items():
        if output[at:at+4]!=sentinel:
            raise previous.AdspError(f'unsupported register sentinel moved 0x{at:x}')
        output[at:at+4]=struct.pack('<I',word)
    if sentinel in (output[x:x+4] for x in range(0,len(output),4)):
        raise previous.AdspError('unresolved privileged sentinel')
    extra=previous.assemble(llvm_mc,DISPATCH_SOURCE,DISPATCH_SIZE)
    output.extend(extra)
    output.extend(finalize(llvm_mc,previous))
    if len(output)!=SIZE:
        raise previous.AdspError("RWE source fragments are not contiguous")
    return bytes(output)


def verify(reference:bytes,fragment:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('ADSP reference revision SHA-256 mismatch')
    hdr=baseline().elf32_header(reference)
    if hdr['type']!=2 or hdr['machine']!=164 or hdr['flags']!=0x66:
        raise ValueError('wrong ADSP ET_EXEC/V66 header')
    if hdr['phentsize']<32:
        raise ValueError('invalid program header stride')
    found=False
    for i in range(hdr['phnum']):
        o=hdr['phoff']+i*hdr['phentsize']
        if o+32>len(reference):raise ValueError('ADSP header exceeds image size')
        kind,start,vaddr,paddr,filesz,memsz,flags,align=struct.unpack_from('<8I',reference,o)
        if (kind,start,vaddr,paddr,filesz,memsz,flags & 7)==(1,0x6000,0xF0003000,0x8BE03000,0x32FE8,0x33000,7):
            found=True
    if not found:raise ValueError('expected second executable PT_LOAD was not found')
    if len(fragment)!=SIZE or reference[FILE_START:FILE_END]!=fragment:
        raise ValueError('independently reconstructed RWE bytes differ from source')


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args()
    b=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),b)
    args.output.write_bytes(b)
    print('ADSP RWE fragment: %d exact bytes, MBN [0x%x,0x%x), physical [0x%x,0x%x), SHA256=%s; function continues' %
          (len(b),FILE_START,FILE_END,PADDR,PADDR+len(b),hashlib.sha256(b).hexdigest()))
    return 0

if __name__=='__main__':sys.exit(main())
