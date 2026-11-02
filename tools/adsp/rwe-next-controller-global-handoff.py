#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild ADSP physical-controller handoff using 22 source-assembled words.

File [0x11624,0x1167c), physical [0x8be0e624,0x8be0e67c). It includes a
branch to external controller code, plus a branch to the next function
portion at 0x1167c. The overall global startup routine remains incomplete.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
TOOLS=Path(__file__).resolve().parent
FILE_START=0x11624
FILE_END=0x1167c
SIZE=FILE_END-FILE_START
PHYSICAL_START=0x8be0e624
REF_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=TOOLS/'rwe-next-controller-global-handoff.S'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp',TOOLS.parent/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing ADSP assembler')
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,build:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REF_SHA:
        raise ValueError('Wrong pinned ADSP reference')
    if len(build)!=SIZE or build!=reference[FILE_START:FILE_END]:
        raise ValueError('ADSP physical controller handoff differs')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();data=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),data)
    a.output.write_bytes(data)
    print(f'ADSP controller handoff: {SIZE} source-assembled bytes '
          f'[0x{FILE_START:x},0x{FILE_END:x}); sha256={hashlib.sha256(data).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
