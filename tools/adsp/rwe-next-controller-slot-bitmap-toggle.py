#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build 128-byte Hexagon slot bitmap publication/toggle routine.

The source is LLVM6-assembled Hexagon V66 mnemonics, including explicit packet
boundaries, `.new` predicates and the GP-independent slot index bit logic.
"""
from __future__ import annotations
import argparse, hashlib, importlib.util, sys
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
START=0x11f58
END=0x11fd8
SIZE=END-START
PHYSICAL=0x8be0ef58
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-slot-bitmap-toggle.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_assembler',T / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Hexagon source assembler unavailable')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('ADSP firmware revision differs from pinned reference')
    if len(built)!=SIZE or built!=reference[START:END]:
        raise ValueError('Controller slot-class bitmap toggle differs from original')

def main()->int:
    a=argparse.ArgumentParser(description=__doc__)
    a.add_argument('--llvm-mc',required=True)
    a.add_argument('--reference',type=Path)
    a.add_argument('--output',type=Path,required=True)
    x=a.parse_args();data=render(x.llvm_mc)
    if x.reference:verify(x.reference.read_bytes(),data)
    x.output.write_bytes(data)
    print(f'ADSP slot-class toggle {len(data)} exact source bytes '
          f'file [0x{START:x},0x{END:x}) physical 0x{PHYSICAL:x} '
          f'sha256={hashlib.sha256(data).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
