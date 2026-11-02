#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Emit source-authored Hexagon ADSP slot deactivation `[0x12098,0x12114)`.

Performs checked slot-state validation, per-controller reset, bitmask
publication and optional callback dispatch. All 31 instruction words are
LLVM6-assembled, with no external proprietary opcode substitutions.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
START=0x12098
END=0x12114
SIZE=END-START
PHYSICAL=0x8be0f098
SOURCE=T/'rwe-next-controller-slot-deactivate.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('adsp_assembler',T/'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing Hexagon assembler')
    mod=importlib.util.module_from_spec(sp);sp.loader.exec_module(mod)
    return mod.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,data:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Incorrect ADSP firmware source revision')
    if len(data)!=SIZE or data!=reference[START:END]:
        raise ValueError('Controller slot deactivation differs from reference')
    if reference[END:END+4]!=bytes(4):
        raise ValueError('Expected separate four-byte linker fill after function')

def main()->int:
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--llvm-mc',required=True);ap.add_argument('--reference',type=Path)
    ap.add_argument('--output',type=Path,required=True)
    a=ap.parse_args();data=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),data)
    a.output.write_bytes(data)
    print(f'ADSP source-built slot deactivate: {SIZE} bytes '
          f'[0x{START:x},0x{END:x}), physical 0x{PHYSICAL:x} '
          f'sha256={hashlib.sha256(data).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
