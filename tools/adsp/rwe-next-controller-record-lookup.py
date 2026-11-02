#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild a 272-byte ADSP controller-record lookup/allocator in Hexagon V66.

This is disjoint executable code after a separate diagnostic-string data pool.
LLVM6 assembles 67 ordinary words, and the single unsupported SGP0 read is
reconstructed by a checked named system-register field encoder.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register
START=0x12e40
END=0x12f50
SIZE=END-START
PHYSICAL=0x8be0fe40
SGP0_FILE_OFFSET=0x12e50
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-record-lookup.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
LLVM6_NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('adsp_assembler',T / 'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing Hexagon ELF assembler')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    data=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    i=SGP0_FILE_OFFSET-START
    if data[i:i+4]!=LLVM6_NOP:
        raise ValueError('Wrong checked privileged SGP0 placeholder position')
    struct.pack_into('<I',data,i,read_system_register(2,SystemRegister.SGP0))
    return bytes(data)

def verify(reference:bytes,result:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware reference revision')
    if len(result)!=SIZE or result!=reference[START:END]:
        raise ValueError('ADSP controller record lookup differs from reference')
    # Original error diagnostic strings and their alignment bytes are not code.
    if reference[0x12e35:START]!=bytes(START-0x12e35):
        raise ValueError('Expected linker zero fill before record lookup')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP controller record lookup: {SIZE} code bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'1 checked SGP0 read; sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
