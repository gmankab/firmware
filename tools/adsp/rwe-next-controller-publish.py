#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-assembled ADSP controller SRAM publication [0x11798,0x1180c).

All 29 V66 instruction words are built by LLVM6 from mnemonic source.
The source groups call+store packets and the original `.new` GP write;
the pinned firmware is read only during explicit verification.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path

T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-publish.S'
START=0x11798
END=0x1180c
SIZE=END-START
PHYSICAL=0x8be0e798
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_entry',T / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing source assembler')
    a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
    return a.assemble(llvm_mc,SOURCE,SIZE)

def verify(ref:bytes,built:bytes)->None:
    if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong ADSP reference revision')
    if len(built)!=SIZE or built!=ref[START:END]:
        raise ValueError('Controller publication mismatch')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args()
    b=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),b)
    args.output.write_bytes(b)
    print(f'ADSP SRAM publication {len(b)} independent source bytes '
          f'[0x{START:x},0x{END:x}), sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
