#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build ADSP SRAM/controller continuation [0x116d8,0x11754).

Hexagon v66 source packets are assembled by LLVM6; exactly two legacy V66
system-register reads are encoded from the architectural operand fields at
checked NOP placeholders. OEM bytes are read only as a verification oracle.
The surrounding controller routine continues past this file range.
"""
from __future__ import annotations
import argparse
import hashlib
import importlib.util
import struct
import sys
from pathlib import Path

T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister, read_system_register
START=0x116d8
END=0x11754
SIZE=END-START
PHYSICAL=0x8be0e6d8
EXPECTED_REF='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=ASSEMBLY_DIR / 'rwe-next-sram-controller-status.S'
SYSTEM_READS={0x116f0:read_system_register(2,SystemRegister.SYSCFG),
              0x11714:read_system_register(10,SystemRegister.SGP0)}
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_entry',T / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing source assembler')
    a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
    data=bytearray(a.assemble(llvm_mc,SOURCE,SIZE))
    for offset,word in SYSTEM_READS.items():
        i=offset-START
        if data[i:i+4] != NOP:
            raise a.AdspError(f'Expected independent LLVM6 NOP at 0x{offset:x}')
        data[i:i+4]=struct.pack('<I',word)
    if any(data[i:i+4]==NOP for i in range(0,SIZE,4)):
        raise a.AdspError('Unresolved privileged placeholder in controller status')
    return bytes(data)

def verify(original:bytes,rebuild:bytes)->None:
    if hashlib.sha256(original).hexdigest()!=EXPECTED_REF:
        raise ValueError('ADSP firmware reference SHA mismatch')
    if len(rebuild)!=SIZE or rebuild!=original[START:END]:
        at=next((i for i,(a,b) in enumerate(zip(rebuild,original[START:END])) if a!=b),None)
        raise ValueError(f'Controller continuation differs at relative byte {at}')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args()
    result=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),result)
    args.output.write_bytes(result)
    print(f'ADSP controller state: {SIZE} bytes, file [0x{START:x},0x{END:x}), '
          f'physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}), '
          f'sha256={hashlib.sha256(result).hexdigest()}')
    return 0

if __name__=='__main__':raise SystemExit(main())
