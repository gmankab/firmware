#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-assemble Hexagon V66 codec selector branches, bounded table check and return.

The [0x12764,0x127c8) source includes the original mpyi-based controller-table
lookup and immediate-string references. It emits no opaque reference opcode
words and does not embed the external codec handler table.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
T=Path(__file__).resolve().parent
START=0x12764
END=0x127c8
SIZE=END-START
PHYSICAL=0x8be0f764
SOURCE=T/'rwe-next-controller-format-selector-cases-b.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('adsp_emitter',T/'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing Hexagon ELF assembler')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,generated:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware reference revision')
    if len(generated)!=SIZE or generated!=reference[START:END]:
        raise ValueError('ADSP format-selector branch/lookup output differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP format-selector additional cases: {SIZE} byte-identical V66 bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
