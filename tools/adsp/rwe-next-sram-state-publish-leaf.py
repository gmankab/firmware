#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon SRAM status leaf [file 0x11898,0x118b8) from typed source.

LLVM6 assembles the seven ordinary instruction words. The eighth word,
R10=SGP0, is patched only at an exact checked NOP placeholder. Subsequent
eight zero bytes are structural alignment fill, not executable source code.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import read_system_register,SystemRegister
START=0x11898;END=0x118b8;SIZE=END-START;PHYSICAL=0x8be0e898
FOLLOWING_FILL=(0x118b8,0x118c0)
REF_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=T/'rwe-next-sram-state-publish-leaf.S'
SGP0_AT=0x118ac
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing Hexagon assembler')
    a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
    data=bytearray(a.assemble(llvm_mc,SOURCE,SIZE));i=SGP0_AT-START
    if data[i:i+4]!=NOP:raise a.AdspError('Missing V66 SGP0 source placeholder')
    data[i:i+4]=struct.pack('<I',read_system_register(10,SystemRegister.SGP0))
    return bytes(data)

def verify(ref:bytes,built:bytes)->None:
    if hashlib.sha256(ref).hexdigest()!=REF_SHA:raise ValueError('Wrong reference SHA')
    if ref[FOLLOWING_FILL[0]:FOLLOWING_FILL[1]]!=bytes(8):raise ValueError('Following fill unexpectedly nonzero')
    if len(built)!=SIZE or built!=ref[START:END]:raise ValueError('SRAM publication leaf differs')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP SRAM publish helper: {SIZE} bytes file [0x{START:x},0x{END:x}) sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
