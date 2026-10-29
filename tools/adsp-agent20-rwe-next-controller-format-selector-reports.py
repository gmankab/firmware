#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently rebuild 18 Hexagon controller format diagnostics/report cases.

Only the executable stubs at [0x1289c,0x129bc) are emitted. The 37
immediately following 32-bit handler pointer entries belong to a separate
non-executable memory table, deliberately excluded from this generator.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
T=Path(__file__).resolve().parent
START=0x1289c
END=0x129bc
SIZE=END-START
PHYSICAL=0x8be0f89c
SOURCE=T/'adsp-agent20-rwe-next-controller-format-selector-reports.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
# These are diagnostic format string pointers in controller virtual address space,
# NOT vendor machine words or executable bytes.
FORMAT_MESSAGES=(
    0xf000fb9d,0xf000fbbd,0xf000fbda,0xf000fbf9,
    0xf000fc1e,0xf000fc3d,0xf000fc4b,0xf000fc59,
    0xf000fc6f,0xf000fc90,0xf000fcbc,0xf000fcdf,
    0xf000fc9f,0xf000fcae,0xf000fd45,0xf000fcf1,
    0xf000fd03,0xf000fd1b,
)

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('adsp_elf_emit',T/'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing Hexagon assembler')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,generated:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware reference revision')
    if len(generated)!=SIZE or generated!=reference[START:END]:
        raise ValueError('ADSP diagnostic case stubs differ from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();b=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),b)
    args.output.write_bytes(b)
    print(f'ADSP diagnostic case stubs: {SIZE} executable bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
