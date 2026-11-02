#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently compile controller-global record prefix [0x118c0,0x11928).

LLVM6 emits 22 ordinary words; four V66-only register reads are checked
against the assembler's own NOP slots, then re-encoded from architectural
fields. Source emits no unknown OEM code or alignment padding.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,ControlRegister,read_system_register,read_control_register
START=0x118c0;END=0x11928;SIZE=END-START;PHYSICAL=0x8be0e8c0
SOURCE=ASSEMBLY_DIR / 'rwe-next-global-record-prefix.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
PRIVILEGED={
    0x118c4:read_system_register(2,SystemRegister.SGP0),
    0x118d0:read_system_register(1,SystemRegister.REV),
    0x118d4:read_control_register(2,ControlRegister.GP),
    0x11914:read_system_register(4,SystemRegister.PCYCLELO),
}
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp',T / 'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing LLVM assembler')
    a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
    out=bytearray(a.assemble(llvm_mc,SOURCE,SIZE))
    for addr,code in PRIVILEGED.items():
        at=addr-START
        if out[at:at+4]!=NOP:raise a.AdspError(f'Expected privileged NOP at ADSP 0x{addr:x}')
        out[at:at+4]=struct.pack('<I',code)
    if any(out[i:i+4]==NOP for i in range(0,SIZE,4)):
        raise a.AdspError('Unexpected unresolved privileged placeholder')
    return bytes(out)

def verify(ref:bytes,source:bytes)->None:
    if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:raise ValueError('Incorrect ADSP reference SHA')
    if len(source)!=SIZE or source!=ref[START:END]:raise ValueError('Global record prefix differs from original')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP global record: {len(b)} source bytes at file [0x{START:x},0x{END:x}), '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
