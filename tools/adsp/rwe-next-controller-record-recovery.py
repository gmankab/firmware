#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build a Hexagon V66 controller error/recovery continuation.

This 64-byte executable interval follows the normal-return record-selection
function. A further 12 bytes of alignment are explicitly excluded as fill.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import KernelZeroLock
START=0x13084
END=0x130c4
SIZE=END-START
PHYSICAL=0x8be10084
K0_LOCK=0x1308c
SOURCE=T/'rwe-next-controller-record-recovery.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('hexagon_assembler',T/'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing Hexagon ELF reassembler')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    data=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    pos=K0_LOCK-START
    if data[pos:pos+4]!=NOP:
        raise ValueError('Expected one checked privileged K0-lock NOP placeholder')
    struct.pack_into('<I',data,pos,int(KernelZeroLock.LOCK))
    return bytes(data)

def verify(reference:bytes,result:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware reference revision')
    if len(result)!=SIZE or result!=reference[START:END]:
        raise ValueError('ADSP controller-record recovery differs from reference')
    if reference[END:END+12]!=bytes(12):
        raise ValueError('Expected 12 bytes of linker fill after recovery code')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();b=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),b)
    args.output.write_bytes(b)
    print(f'ADSP controller recovery: {SIZE} exact executable bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'1 checked K0 lock; sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
