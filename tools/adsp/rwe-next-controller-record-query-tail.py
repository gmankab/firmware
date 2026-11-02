#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-rebuild Hexagon V66 controller linked-list insertion/error tail.

Reconstruct [0x131e0,0x13250) from packetized assembler source; one missing
privileged K0 lock is encoded by an existing named checked V66 constructor.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import KernelZeroLock
START=0x131e0
END=0x13250
SIZE=END-START
PHYSICAL=0x8be101e0
K0_LOCK=0x13214
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-record-query-tail.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('adsp_emit',T.parent / 'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing Hexagon assembler')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    data=bytearray(m.assemble(llvm_mc,SOURCE,SIZE));p=K0_LOCK-START
    if data[p:p+4]!=NOP:
        raise ValueError('Expected exactly one checked K0 lock placeholder')
    struct.pack_into('<I',data,p,int(KernelZeroLock.LOCK))
    return bytes(data)

def verify(original:bytes,generated:bytes)->None:
    if hashlib.sha256(original).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware reference revision')
    if len(generated)!=SIZE or generated!=original[START:END]:
        raise ValueError('ADSP linked-list controller recovery differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP record-query recovery: {SIZE} source-built executable bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'1 checked K0 lock, sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
