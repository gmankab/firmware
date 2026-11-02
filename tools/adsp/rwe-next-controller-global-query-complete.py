#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild the complete ADSP global controller query and table-search helper.

LLVM6 produces 64 ordinary Hexagon V66 instruction words. The sole V66-only
privileged `R2=SGP0` is generated from a checked register-field encoder after
verifying the LLVM NOP placeholder; firmware references are never copied into
emitted executable bytes. This source is not an OEM-signed MBN image.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import read_system_register,SystemRegister
START=0x125a0
END=0x126a4
SIZE=END-START
PHYSICAL=0x8be0f5a0
SGP0_AT=0x125ac
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
PARTS=(
    ('rwe-next-controller-global-query-prefix.S',0x54),
    ('rwe-next-controller-global-query-search.S',0x70),
    ('rwe-next-controller-global-query-tail.S',0x40),
)
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_emit',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing Hexagon assembler source')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    data=bytearray(b''.join(m.assemble(llvm_mc,T/name,length) for name,length in PARTS))
    if len(data)!=SIZE:raise m.AdspError('Wrong controller-global query length')
    at=SGP0_AT-START
    if data[at:at+4]!=NOP:raise m.AdspError('Expected LLVM6 NOP at reserved V66-only system-register read')
    data[at:at+4]=struct.pack('<I',read_system_register(2,SystemRegister.SGP0))
    # The loop's terminating architectural NOP at 0x12654 is genuine code.
    # No other NOP sentinel may remain after the SGP0 substitution.
    remaining=[START+i for i in range(0,SIZE,4) if data[i:i+4]==NOP]
    if remaining != [0x12654]:
        raise m.AdspError(f'Unexpected NOP/privileged placeholder layout: {remaining}')
    return bytes(data)

def verify(reference:bytes,result:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Unexpected pinned ADSP reference revision')
    if len(result)!=SIZE or result!=reference[START:END]:
        raise ValueError('Controller-global query differs from original reference')
    if reference[START-4:START]!=bytes(4):
        raise ValueError('Expected four-byte linker fill before query')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP controller-global query: {SIZE} source-built bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
