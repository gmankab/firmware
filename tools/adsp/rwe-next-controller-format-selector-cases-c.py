#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build additional V66 codec-format branches and their shared call/return path.

Produces [0x127c8, 0x1289c) from independently authored packetized Hexagon
assembly. The controller's external jump table is not included as opcodes.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
START=0x127c8
END=0x1289c
SIZE=END-START
PHYSICAL=0x8be0f7c8
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-format-selector-cases-c.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('hexagon_emit',T.parent / 'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing ELF .text builder')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,result:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware revision')
    if len(result)!=SIZE or result!=reference[START:END]:
        raise ValueError('ADSP selector branch/control-frame bytes differ from pinned reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP V66 selector conditional paths: {SIZE} executable bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
