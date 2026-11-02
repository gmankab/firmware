#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Reconstruct ADSP atomic slot registration [0x11ba0,0x11c88).

LLVM6 assembles the original 57 ordinary packet words and Rizin V66
decoding resolves its extra SGP0 read. That word is produced through the
strictly checked system-register encoder at one validated NOP placeholder.
The pinned ADSP reference is used only for verification, not output assembly.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
TOOLS=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register
START=0x11ba0;END=0x11c88;SIZE=END-START;PHYSICAL=0x8be0eba0
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-slot-register.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
READ_SGP0_AT=0x11bb0
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_llvm',TOOLS/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing LLVM6 source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    out=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    at=READ_SGP0_AT-START
    if out[at:at+4]!=NOP:
        raise m.AdspError('SGP0 placeholder must be the original full one-word packet NOP')
    out[at:at+4]=struct.pack('<I',read_system_register(2,SystemRegister.SGP0))
    if any(out[i:i+4]==NOP for i in range(0,SIZE,4)):
        raise m.AdspError('Unexpected unresolved privileged placeholder')
    return bytes(out)

def verify(reference:bytes,generated:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Unexpected original ADSP firmware revision')
    if len(generated)!=SIZE or generated!=reference[START:END]:
        raise ValueError('ADSP controller slot registration differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP controller slot register {SIZE} bytes [file 0x{START:x},'
          f'0x{END:x}) physical 0x{PHYSICAL:x},'
          f' sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
