#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build the ADSP indexed codec format selector prefix.

The controller reads an index from a GP-backed record and dispatches into an
external non-inline 14-entry table. This is a partial routine; the case bodies
follow beyond 0x126dc and are not represented as executable instructions here.
All generated words come from ordinary LLVM6 Hexagon assembly.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
START=0x126a8
END=0x126dc
SIZE=END-START
PHYSICAL=0x8be0f6a8
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-format-selector-prefix.S'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_emit',T.parent / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing Hexagon LLVM6 assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,data:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware reference revision')
    if len(data)!=SIZE or data!=reference[START:END]:
        raise ValueError('Codec format-selector prefix differs from reference')
    if reference[START-4:START]!=bytes(4):
        raise ValueError('Expected separate linker fill before format selector')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP format-selector prefix: {SIZE} byte-identical executable bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
