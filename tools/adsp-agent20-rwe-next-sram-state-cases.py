#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild the V66 controller SRAM state switch [0x11754,0x11798).

Seven complete packets (17 Hexagon instruction words) assemble independently
from source, including dependent `.new` conditional stores and a two-operation
duplex. The original ADSP image is a read-only verification oracle.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path

TOOLS=Path(__file__).resolve().parent
SOURCE=TOOLS/'adsp-agent20-rwe-next-sram-state-cases.S'
START=0x11754
END=0x11798
SIZE=END-START
PHYSICAL=0x8be0e754
EXPECTED_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp',TOOLS/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing ADSP source assembler')
    tool=importlib.util.module_from_spec(spec);spec.loader.exec_module(tool)
    return tool.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,assembled:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=EXPECTED_SHA:
        raise ValueError('Incorrect ADSP pinned revision')
    if len(assembled)!=SIZE or assembled!=reference[START:END]:
        raise ValueError('Rebuilt SRAM state switch differs from original')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',required=True,type=Path)
    a=p.parse_args()
    code=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),code)
    a.output.write_bytes(code)
    print(f'ADSP source-assembled SRAM switch: {len(code)} bytes at '
          f'[0x{START:x},0x{END:x}), sha256={hashlib.sha256(code).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
