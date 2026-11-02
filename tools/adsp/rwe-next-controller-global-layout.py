#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently build two contiguous global controller MMIO layout sections.

These 220 bytes [0x11548,0x11624) are two Hexagon source files assembled by
LLVM6 without privileged or OEM opcode substitutions. The larger controller
startup function is still incomplete after 0x11624; no fill is implied.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
T=Path(__file__).resolve().parent
FILE_START=0x11548
FILE_END=0x11624
SIZE=FILE_END-FILE_START
PHYSICAL_START=0x8be0e548
PARTS=(('global-pointers',0x11548,0x11594,'rwe-next-controller-global-pointers.S'),
       ('global-layout',0x11594,0x11624,'rwe-next-controller-global-layout.S'))
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_mcode',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing source assembler')
    core=importlib.util.module_from_spec(spec);spec.loader.exec_module(core)
    output=bytearray()
    for name,a,b,source in PARTS:
        if a != FILE_START+len(output):raise ValueError(f'Nonadjacent ADSP region {name}')
        output+=core.assemble(llvm_mc,T/source,b-a)
    if len(output)!=SIZE:raise ValueError('Final source interval changed')
    return bytes(output)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong pinned ADSP reference revision')
    if len(built)!=SIZE or built!=reference[FILE_START:FILE_END]:
        raise ValueError('ADSP controller global pointer/layout bytes differ')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'Source-built ADSP controller layout: {SIZE} bytes from {len(PARTS)} '
          f'ARM-independent Hexagon assembly sources at '
          f'[0x{FILE_START:x},0x{FILE_END:x}), sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
