#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build ADSP 192-byte atomic controller-slot reservation function.

Original V66 packet scheduling includes a low register-pair move as a
continuation followed by a duplex terminal. LLVM6 produces the correct
operands but closes that packet a word early; only the checked two-bit
parse field is modified. The SGP0 transfer is emitted by a typed V66 encoder.
No original vendor instruction word is copied into the source generator.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import packet_continuation,read_system_register,SystemRegister
START=0x11fd8
END=0x12098
SIZE=END-START
PHYSICAL=0x8be0efd8
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-slot-reserve.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SGP0_AT=0x11fec
PACKET_AT=0x11ffc
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    sp=importlib.util.spec_from_file_location('adsp_source',T / 'emit-adsp-entry.py')
    if not sp or not sp.loader:raise RuntimeError('Hexagon source assembler unavailable')
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    out=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    where=SGP0_AT-START
    if out[where:where+4]!=NOP:
        raise m.AdspError('SGP0 read must occupy the exact LLVM6 NOP sentinel')
    struct.pack_into('<I',out,where,read_system_register(2,SystemRegister.SGP0))
    where=PACKET_AT-START
    assembled=struct.unpack_from('<I',out,where)[0]
    # Permit only the LLVM source expression r3:2=combine(#1,#0).
    # Its architectural bits must match the known source-assembled form.
    if assembled!=0x7c00c022:
        raise m.AdspError('Unexpected class-pair mnemonic packet word')
    struct.pack_into('<I',out,where,packet_continuation(assembled))
    if any(out[i:i+4]==NOP for i in range(0,SIZE,4)):
        raise m.AdspError('Unresolved privileged instruction sentinel')
    return bytes(out)

def verify(reference:bytes,data:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware revision')
    if len(data)!=SIZE or data!=reference[START:END]:
        raise ValueError('Controller slot reserve differs from original reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path);p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();data=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),data)
    a.output.write_bytes(data)
    print(f'ADSP controller slot reserve: {SIZE} exact executable bytes '
          f'file [0x{START:x},0x{END:x}), physical 0x{PHYSICAL:x}; '
          f'sha256={hashlib.sha256(data).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
