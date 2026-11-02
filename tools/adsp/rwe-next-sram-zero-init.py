#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build ADSP SRAM initialization prefix preserving mem_noshuf.

Source-assembled Hexagon V66 bytes [file 0x116a0,0x116d8) with 128-iteration
hardware loop, byte-identical to pinned ADSP without source opcode copying.
The preceding eight bytes are verified structural fill, not executable code.
This function is incomplete after the prefix; no implied return follows.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
START=0x116a0
END=0x116d8
SIZE=END-START
PHYSICAL=0x8be0e6a0
FILL=(0x11698,0x116a0)
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=ASSEMBLY_DIR / 'rwe-next-sram-zero-init.S'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('No source assembler')
    a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
    return a.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP image')
    a,b=FILL
    if reference[a:b]!=bytes(b-a):raise ValueError('Preceding linker fill is not zero')
    if len(built)!=SIZE or built!=reference[START:END]:
        raise ValueError('SRAM initialization bytes differ from pinned ADSP')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    x=p.parse_args();built=render(x.llvm_mc)
    if x.reference:verify(x.reference.read_bytes(),built)
    x.output.write_bytes(built)
    print(f'ADSP SRAM loop prefix: {len(built)} source-built bytes '
          f'file [0x{START:x},0x{END:x}), sha256={hashlib.sha256(built).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
