#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build ADSP controller-slot bitmap status query [0x11e6c,0x11ed0).

All 25 instruction words are produced by LLVM6 from original mnemonic
logic, including the GP-indexed bitmap loads and packeted return. Original
firmware is an optional verification input; no OEM instructions are spliced.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
START=0x11e6c;END=0x11ed0;SIZE=END-START;PHYSICAL=0x8be0ee6c
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-slot-status.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('assembler',T / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing checked Hexagon assembler')
    asm=importlib.util.module_from_spec(spec);spec.loader.exec_module(asm)
    return asm.assemble(llvm_mc,SOURCE,SIZE)

def verify(original:bytes,out:bytes)->None:
    if hashlib.sha256(original).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong reference ADSP firmware revision')
    if len(out)!=SIZE or out!=original[START:END]:
        raise ValueError('Controller slot status differs from original reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP controller status {SIZE} exact source-built bytes '
          f'file [0x{START:x},0x{END:x}) physical [0x{PHYSICAL:x},'
          f'0x{PHYSICAL+SIZE:x}) sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
