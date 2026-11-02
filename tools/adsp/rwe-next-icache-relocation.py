#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently source-build the complete ADSP V66 I-cache relocation routine.

The function runs from file 0x11170 to 0x11394. Immediately following it is a
non-executable diagnostic string pool. Only mnemonic-assembled executable
bytes are emitted; private reference input is read solely for exact checking.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
FILE_START=0x11170
FILE_END=0x11394
SIZE=FILE_END-FILE_START
DATA_START=FILE_END
DATA_END=0x113f0
PHYSICAL_START=0x8be0e170
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
PARTS=((0x111dc,0x1121c,'rwe-next-icache-relocation-tail-a.S'),
       (0x1121c,0x112d4,'rwe-next-icache-relocation-tail-b.S'),
       (0x112d4,0x11394,'rwe-next-icache-relocation-tail-c.S'))

def earlier():
    spec=importlib.util.spec_from_file_location('adsp_cache_prefix',T/'rwe-next-icache-relocation-prefix.py')
    if not spec or not spec.loader:raise RuntimeError('Missing source-built cache prefix')
    p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)
    return p

def render(llvm_mc:str)->bytes:
    prefix=earlier()
    o=bytearray(prefix.render(llvm_mc))
    if len(o)!=0x111dc-FILE_START:raise ValueError('Cache relocation prefix size changed')
    mc=prefix.base()
    for start,end,name in PARTS:
        if len(o)!=start-FILE_START:
            raise ValueError(f'Non-contiguous source-built relocation instructions before 0x{start:x}')
        b=mc.assemble(llvm_mc,T/name,end-start)
        o.extend(b)
    if len(o)!=SIZE:
        raise ValueError('Complete cache relocation source changed in size')
    return bytes(o)

def verify(reference:bytes,rebuilt:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong pinned ADSP reference revision')
    if len(rebuilt)!=SIZE or rebuilt!=reference[FILE_START:FILE_END]:
        raise ValueError('Source-built ADSP cache relocation bytes differ')
    if not reference[DATA_START:DATA_START+len(b'trampoline index out of range!')].startswith(b'trampoline index out of range'):
        raise ValueError('Expected non-executable diagnostic text table absent')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP I-cache relocation full function {len(b)} source-built bytes '
          f'file [0x{FILE_START:x},0x{FILE_END:x}) physical '
          f'[0x{PHYSICAL_START:x},0x{PHYSICAL_START+SIZE:x}) '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':sys.exit(main())
