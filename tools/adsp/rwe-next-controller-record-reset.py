#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build controller record-reset and linked-descriptor cleanup at [0x13250,0x13390).

Source code is packetized Hexagon V66 assembly. LLVM6 assembles every
nonprivileged instruction; two typed checked register operations replace
explicit NOP sentinels. This is not OEM code-signing or behavioral proof.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import KernelZeroLock,SystemRegister,read_system_register
START=0x13250
END=0x13390
SIZE=END-START
PHYSICAL=0x8be10250
SGP0=0x1325c
K0_LOCK=0x132f4
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-record-reset.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp_emit',T / 'emit-adsp-entry.py')
    if spec is None or spec.loader is None:
        raise RuntimeError('Missing Hexagon source-assembler helper')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    data=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    for address,encoder in ((SGP0,read_system_register(2,SystemRegister.SGP0)),
                            (K0_LOCK,int(KernelZeroLock.LOCK))):
        i=address-START
        if data[i:i+4]!=NOP:
            raise ValueError(f'Expected named V66 NOP sentinel at {address:#x}')
        struct.pack_into('<I',data,i,encoder)
    return bytes(data)

def verify(reference:bytes,reconstructed:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware reference revision')
    if len(reconstructed)!=SIZE or reconstructed!=reference[START:END]:
        raise ValueError('ADSP controller record reset differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();blob=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),blob)
    args.output.write_bytes(blob)
    print(f'ADSP record reset: {SIZE} independent executable bytes, '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'78 LLVM6-assembled plus 2 checked V66 words; sha256={hashlib.sha256(blob).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
