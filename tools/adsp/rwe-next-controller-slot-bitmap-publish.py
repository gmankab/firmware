#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build ADSP bitmap publication helper file [0x11ed0,0x11f18) from LLVM6.

Emits source-authored V66 Hexagon packet instructions, with the original
p0.new conditional GP table read grouped in its producer's packet.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
START=0x11ed0;END=0x11f18;SIZE=END-START;PHYSICAL=0x8be0eed0
SOURCE=T/'rwe-next-controller-slot-bitmap-publish.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('x',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Source assembler unavailable')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.assemble(llvm_mc,SOURCE,SIZE)

def verify(reference:bytes,result:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('ADSP original firmware revision mismatch')
    if len(result)!=SIZE or result!=reference[START:END]:
        raise ValueError('ADSP slot bitmap publication differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP slot bitmap publication {len(b)} source-built bytes '
          f'[0x{START:x},0x{END:x}) physical [0x{PHYSICAL:x},'
          f'0x{PHYSICAL+SIZE:x}) sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
