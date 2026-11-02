#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-assemble indexed controller record query normal return path.

The normal return at [0x130d0,0x131e0) is separate from its subsequent
recovery/list-manipulation continuation at 0x131e0 (not included here).
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register
START=0x130d0
END=0x131e0
SIZE=END-START
PHYSICAL=0x8be100d0
SGP0=0x130e0
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=T/'rwe-next-controller-record-query.S'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('hexagon_assembler',T/'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing Hexagon reassembler')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    b=bytearray(m.assemble(llvm_mc,SOURCE,SIZE));idx=SGP0-START
    if b[idx:idx+4]!=NOP:
        raise ValueError('Expected exactly one checked SGP0 NOP placeholder')
    struct.pack_into('<I',b,idx,read_system_register(2,SystemRegister.SGP0))
    return bytes(b)

def verify(reference:bytes,generated:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP reference revision')
    if len(generated)!=SIZE or generated!=reference[START:END]:
        raise ValueError('ADSP indexed record query differs from reference')
    if reference[START-12:START]!=bytes(12):
        raise ValueError('Expected preceding 12-byte non-executable alignment')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();out=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),out)
    args.output.write_bytes(out)
    print(f'ADSP controller indexed record query: {SIZE} code bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'1 checked SGP0; sha256={hashlib.sha256(out).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
