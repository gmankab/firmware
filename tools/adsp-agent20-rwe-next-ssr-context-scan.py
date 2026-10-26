#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently source-assemble SSR-gated controller table scan, 120 bytes.

Hexagon's endloop0 packet has a different four-scalar-to-duplex schedule
than LLVM6's default optimizer. Compile the four source scalars separately
with -mno-pairing, retain the two original-order source words, and substitute
an independently source-assembled Hexagon duplex for the final two. Parse
markers and packet boundaries are checked, not taken from proprietary code.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,subprocess,sys,tempfile
from pathlib import Path
TOOLS=Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
from adsp_hexagon_v66 import SystemRegister,read_system_register
SOURCE=TOOLS/'adsp-agent20-rwe-next-ssr-context-scan.S'
NO_PAIR=TOOLS/'adsp-agent20-rwe-next-ssr-advance-nopair.S'
LOOP_DUPLEX=TOOLS/'adsp-agent20-rwe-next-ssr-duplex.S'
FILE_START=0x11090
FILE_END=0x11108
SIZE=FILE_END-FILE_START
PHYSICAL_START=0x8be0e090
FILL_START=0x11088
FILL_END=FILE_START
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def base():
    spec=importlib.util.spec_from_file_location('adsp_asm',TOOLS/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing ADSP source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m

def independent_nopair(llvm_mc:str,previous)->bytes:
    with tempfile.TemporaryDirectory(prefix='adsp-loop-scalar-') as work:
        obj=Path(work)/'advance.o'
        cmd=[llvm_mc,'-triple=hexagon','-mcpu=hexagonv65',
             '-mno-pairing','-filetype=obj',str(NO_PAIR),'-o',str(obj)]
        proc=subprocess.run(cmd,text=True,capture_output=True)
        if proc.returncode:
            raise previous.AdspError('LLVM6 no-pairing scalar source failed: '+proc.stderr)
        return previous.read_text_section(obj.read_bytes(),20)

def render(llvm_mc:str)->bytes:
    m=base()
    data=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    if data[:4]!=struct.pack('<I',0x7f00c000):
        raise m.AdspError('SSR privileged slot changed')
    data[:4]=struct.pack('<I',read_system_register(2,SystemRegister.SSR))
    offset=0x110f4-FILE_START
    # Guard LLVM's *own* generated packet schedule, not bytes from the OEM.
    surrogate=tuple(struct.unpack_from('<I',data,offset+4*i)[0] for i in range(3))
    if surrogate!=(0xb003a803,0xb0004080,0x20443122):
        raise m.AdspError('LLVM6 SSR loop packet scheduler changed')
    ordered=independent_nopair(llvm_mc,m)
    scalar_a=ordered[4:8]
    scalar_b=ordered[8:12]
    duplex=m.assemble(llvm_mc,LOOP_DUPLEX,12)[-4:]
    # Only the exact original source operands and hardware-loop packet
    # marker are allowed in this 3-word replacement.
    if (struct.unpack('<I',scalar_a)[0]>>14)&3 != 2:
        raise m.AdspError('Source-first loop instruction lost loop marker')
    if (struct.unpack('<I',scalar_b)[0]>>14)&3 != 1:
        raise m.AdspError('Source-second loop instruction lost continuation')
    if (struct.unpack('<I',duplex)[0]>>14)&3 != 0:
        raise m.AdspError('Source-assembled duplex lacks :endloop0 marker')
    data[offset:offset+12]=scalar_a+scalar_b+duplex
    if len(data)!=SIZE:raise m.AdspError('Changed SSR scanner layout')
    return bytes(data)

def verify(reference:bytes,generated:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong ADSP reference revision')
    if len(generated)!=SIZE or generated!=reference[FILE_START:FILE_END]:
        raise ValueError('SSR controller scan differs from pinned ADSP firmware')
    if reference[FILL_START:FILL_END]!=b'\0'*(FILL_END-FILL_START):
        raise ValueError('Expected eight linker fill bytes before new function')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP SSR scan {len(b)} source-built bytes '
          f'file [0x{FILE_START:x},0x{FILE_END:x}) physical '
          f'[0x{PHYSICAL_START:x},0x{PHYSICAL_START+SIZE:x}) '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':sys.exit(main())
