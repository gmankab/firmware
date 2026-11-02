#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build ADSP global controller MMIO publication and conditional loop.

160 bytes of source-authored Hexagon packets [file 0x114a8,0x11548), directly
adjacent to the independently generated 88-byte startup fragment. This is
still an incomplete function: downstream initialization starts at 0x11548.
No OEM words, zero-fill or externally supplied instruction data are emitted.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
FILE_START=0x114a8
FILE_END=0x11548
SIZE=FILE_END-FILE_START
PHYSICAL_START=0x8be0e4a8
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-global-memory.S'

def render(llvm_mc:str)->bytes:
    p=T / 'emit-adsp-entry.py';spec=importlib.util.spec_from_file_location('adsp_generator',p)
    if not spec or not spec.loader:raise RuntimeError('Missing ADSP assembler module')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(original:bytes,built:bytes)->None:
    if hashlib.sha256(original).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong pinned ADSP firmware reference')
    if len(built)!=SIZE or built!=original[FILE_START:FILE_END]:
        raise ValueError('Generated controller global-memory Hexagon packets differ')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();built=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),built)
    a.output.write_bytes(built)
    print(f'ADSP source-built {SIZE} global-state bytes file '
          f'[0x{FILE_START:x},0x{FILE_END:x}) '
          f'physical [0x{PHYSICAL_START:x},0x{PHYSICAL_START+SIZE:x}) '
          f'sha256={hashlib.sha256(built).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
