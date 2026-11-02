#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-reassemble four adjacent ADSP controller slot bitmap functions.

Emits 40 LLVM6 instruction words and one typed V66 SGP0 register transfer,
with original `:mem_noshuf` packet ordering in the flag17 helper. The
preceding four linker-fill bytes are excluded from executable counts.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register
START=0x12118;END=0x121bc;SIZE=END-START;PHYSICAL=0x8be0f118
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-slot-leaves.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SGP0_AT=0x12120
NOP=struct.pack('<I',0x7f00c000)
FUNCTIONS=((0x12118,0x12140),(0x12140,0x12168),(0x12168,0x121a0),(0x121a0,0x121bc))

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('assembly',T.parent / 'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Hexagon LLVM6 unavailable')
    module=importlib.util.module_from_spec(sp);sp.loader.exec_module(module)
    b=bytearray(module.assemble(llvm_mc,SOURCE,SIZE))
    where=SGP0_AT-START
    if b[where:where+4]!=NOP:
        raise module.AdspError('SGP0 one-packet LLVM6 NOP sentinel missing')
    struct.pack_into('<I',b,where,read_system_register(3,SystemRegister.SGP0))
    if any(b[k:k+4]==NOP for k in range(0,SIZE,4)):
        raise module.AdspError('Unexpected unresolved privileged source word')
    return bytes(b)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP image revision')
    if len(built)!=SIZE or built!=reference[START:END]:
        raise ValueError('Controller slot leaf code differs from reference')
    if reference[START-4:START]!=bytes(4):
        raise ValueError('Expected four-byte linker fill before these functions')

def main()->int:
    a=argparse.ArgumentParser(description=__doc__);a.add_argument('--llvm-mc',required=True)
    a.add_argument('--reference',type=Path);a.add_argument('--output',type=Path,required=True)
    x=a.parse_args();data=render(x.llvm_mc)
    if x.reference:verify(x.reference.read_bytes(),data)
    x.output.write_bytes(data)
    print(f'ADSP slot leaf quartet: {SIZE} source-built executable bytes '
          f'[0x{START:x},0x{END:x}), physical 0x{PHYSICAL:x} '
          f'sha256={hashlib.sha256(data).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
