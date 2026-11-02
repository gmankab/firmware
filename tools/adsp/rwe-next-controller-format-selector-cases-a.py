#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-reassemble initial table-dispatched ADSP codec mode cases.

Five switch cases and an indirect nested handler lookup. All 34 Hexagon V66
instruction words, including their extenders and original packets, are
reconstructed entirely by LLVM6; the external mode table is not emitted.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
START=0x126dc
END=0x12764
SIZE=END-START
PHYSICAL=0x8be0f6dc
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-format-selector-cases-a.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('emitter',T / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing LLVM6 source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,result:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware revision')
    if len(result)!=SIZE or result!=reference[START:END]:
        raise ValueError('ADSP codec format mode cases differ from original')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP indexed format cases: {SIZE} byte-identical executable bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
