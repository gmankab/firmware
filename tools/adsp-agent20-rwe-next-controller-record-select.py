#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild 308 bytes of indexed Hexagon V66 ADSP controller selection.

The ordinary return is covered; a separate error/recovery continuation starts
at file offset 0x13084 and is deliberately not emitted by this fragment.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
T=Path(__file__).resolve().parent
sys.path.insert(0,str(T))
from adsp_hexagon_v66 import SystemRegister,read_system_register
START=0x12f50
END=0x13084
SIZE=END-START
PHYSICAL=0x8be0ff50
SGP0=0x12f68
SOURCE=T/'adsp-agent20-rwe-next-controller-record-select.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('hexagon_assembler',T/'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Missing Hexagon ELF reassembler')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    data=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    off=SGP0-START
    if data[off:off+4]!=NOP:
        raise ValueError('Unexpected privileged SGP0 placeholder (require checked encoder)')
    struct.pack_into('<I',data,off,read_system_register(2,SystemRegister.SGP0))
    return bytes(data)

def verify(reference:bytes,generated:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware revision')
    if len(generated)!=SIZE or generated!=reference[START:END]:
        raise ValueError('ADSP indexed-controller selection differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP controller indexed-record selection: {SIZE} verified executable bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'1 checked SGP0; sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
