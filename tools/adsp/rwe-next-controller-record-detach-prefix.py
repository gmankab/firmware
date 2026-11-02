#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build V66 controller record-detach prefix [0x13480,0x13514).

A partial function: executable error handling and final return extend beyond
this interval. Original byte input is used ONLY for explicit verification.
"""
from __future__ import annotations
import argparse, hashlib, importlib.util, struct, sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register
START=0x13480;END=0x13514;SIZE=END-START
PHYSICAL=0x8be10480
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-record-detach.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc: str) -> bytes:
    sp=importlib.util.spec_from_file_location('adsp_emitter',T.parent / 'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing source-assembly helper')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    b=bytearray(m.assemble(llvm_mc,SOURCE,SIZE));offset=0x1348c-START
    if b[offset:offset+4]!=NOP:raise m.AdspError('Expected checked SGP0 placeholder at 0x1348c')
    struct.pack_into('<I',b,offset,read_system_register(2,SystemRegister.SGP0))
    return bytes(b)

def verify(reference:bytes,output:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('ADSP pinned revision mismatch')
    if reference[START-8:START]!=bytes(8):
        raise ValueError('Expected eight-byte original linker fill before prefix')
    if len(output)!=SIZE or output!=reference[START:END]:
        raise ValueError('ADSP controller record detach prefix differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();code=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),code)
    a.output.write_bytes(code)
    print(f'ADSP controller record detach prefix: {SIZE} independent executable bytes '
          f'file [0x{START:x},0x{END:x}), physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}), '
          f'36 LLVM6 and 1 checked V66 read; sha256={hashlib.sha256(code).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
