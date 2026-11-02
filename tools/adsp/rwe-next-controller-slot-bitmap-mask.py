#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build ADSP slot-class bitmap mask accessor [0x11f18,0x11f58).

The original 16 Hexagon words, including a signed class-index mask and
predicated 0x10000 return, come entirely from LLVM6-assembled mnemonic source.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
START=0x11f18;END=0x11f58;SIZE=END-START;PHYSICAL=0x8be0ef18
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-slot-bitmap-mask.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('asm',T / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Hexagon LLVM assembler missing')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware revision')
    if len(built)!=SIZE or built!=reference[START:END]:
        raise ValueError('Controller bitmap mask source differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP bitmap mask {SIZE} source-built bytes [0x{START:x},0x{END:x}) '
          f'physical 0x{PHYSICAL:x} sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
