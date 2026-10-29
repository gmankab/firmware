#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild ADSP controller-slot scanner's descriptor, atomic and tag stages.

All 60 Hexagon V66 instruction words are assembled by LLVM6 from readable
instructions. Tag-word updates and platform calls preserve their original
Hexagon packets, new-value dependencies and hardware addressing. This output
is executable code, not linker padding or firmware auth material.
"""
from __future__ import annotations
import argparse
import hashlib
import importlib.util
from pathlib import Path

T=Path(__file__).resolve().parent
START=0x12220
END=0x12310
SIZE=END-START
PHYSICAL=0x8be0f220
PARTS=(
 ('adsp-agent20-rwe-next-controller-slot-scan-setup.S',0x34),
 ('adsp-agent20-rwe-next-controller-slot-scan-atomic.S',0x40),
 ('adsp-agent20-rwe-next-controller-slot-scan-tag.S',0x7c),
)
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'


def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_entry',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing Hexagon source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    built=b''.join(m.assemble(llvm_mc,T/name,length) for name,length in PARTS)
    if len(built)!=SIZE:raise m.AdspError('Wrong scanner staged layout')
    return built


def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP image revision')
    if len(built)!=SIZE or built!=reference[START:END]:
        raise ValueError('ADSP controller-slot scanner differs from reference')


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args()
    result=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),result)
    a.output.write_bytes(result)
    print(f'ADSP scanner stages: {SIZE} bytes [{START:#x},{END:#x}), '
          f'physical 0x{PHYSICAL:x}, sha256={hashlib.sha256(result).hexdigest()}')
    return 0

if __name__=='__main__':raise SystemExit(main())
