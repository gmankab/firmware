#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-rebuild disjoint Hexagon ADSP thread registration helper.

The first four bytes (file 0x10dc4) are an original standalone return stub,
followed by a controller thread-registration function at 0x10dc8. LLVM6
assembles all standard instructions; four unsupported V66 privilege opcodes
are built by narrowly checked Hexagon ISA encoders, never copied from OEM.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
TOOLS=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import (SystemRegister,read_system_register,
    read_system_register_pair,KernelZeroLock)
SOURCE=ASSEMBLY_DIR / 'rwe-next-thread-record.S'
FILE_START=0x10dc4
FILE_END=0x10eb4
SIZE=FILE_END-FILE_START
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
FILL_START=FILE_END
FILL_END=0x10ec0

def base():
    s=importlib.util.spec_from_file_location('adsp_validated_llvm',TOOLS.parent/'emit-adsp-entry.py')
    if not s or not s.loader:raise RuntimeError('Missing ADSP assembler')
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
    return m

def render(mc:str)->bytes:
    m=base();out=bytearray(m.assemble(mc,SOURCE,SIZE))
    marker=struct.pack('<I',0x7f00c000)
    replacements={
        0x10:(read_system_register(18,SystemRegister.HTID)),
        0x14:(read_system_register_pair(20,56)), # R21:20=S57:56
        0x90:int(KernelZeroLock.LOCK),
        0xa8:int(KernelZeroLock.UNLOCK),
    }
    for at,encoded in replacements.items():
        if out[at:at+4]!=marker:
            raise m.AdspError(f'Thread helper checked V66 slot moved +0x{at:x}')
        out[at:at+4]=struct.pack('<I',encoded)
    return bytes(out)

def verify(reference:bytes,generated:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong ADSP reference firmware revision')
    if len(generated)!=SIZE or generated!=reference[FILE_START:FILE_END]:
        raise ValueError('Thread helper executable differs from reference')
    if any(reference[FILL_START:FILL_END]):
        raise ValueError('Expected 12 linker-fill bytes after thread helper')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',required=True,type=Path)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print('ADSP V66 thread helper',len(b),'byte-identical bytes file',
          hex(FILE_START),'..',hex(FILE_END),'sha256',hashlib.sha256(b).hexdigest())
    return 0
if __name__=='__main__':sys.exit(main())
