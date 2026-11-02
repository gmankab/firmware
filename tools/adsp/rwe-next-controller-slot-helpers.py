#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Compile four adjacent ADSP controller-slot accessors from LLVM6 source.

File interval [0x11d68,0x11e6c): query allocated slot, clear/set the
controller bit, and return the sentinel for an unsupported slot class.
Every instruction and packet boundary is natively produced by LLVM6.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
START=0x11d68;END=0x11e6c;SIZE=END-START;PHYSICAL=0x8be0ed68
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-slot-helpers.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
INTERVALS=((0x11d68,0x11d90,'lookup'),(0x11d90,0x11df8,'clear'),
           (0x11df8,0x11e64,'set'),(0x11e64,0x11e6c,'sentinel'))

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('a',T.parent / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('LLVM6 Hexagon assembler is required')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong ADSP firmware revision')
    if len(built)!=SIZE or built!=reference[START:END]:
        raise ValueError('ADSP slot helper bytes differ from reference')

def main()->int:
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--llvm-mc',required=True)
    ap.add_argument('--reference',type=Path)
    ap.add_argument('--output',type=Path,required=True)
    args=ap.parse_args();data=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),data)
    args.output.write_bytes(data)
    print(f'ADSP slot accessors: {SIZE} source-built bytes '
          f'[0x{START:x},0x{END:x}) physical [0x{PHYSICAL:x},'
          f'0x{PHYSICAL+SIZE:x}) sha256={hashlib.sha256(data).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
