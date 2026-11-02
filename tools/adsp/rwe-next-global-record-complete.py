#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Finish ADSP global-controller record allocation [0x11928,0x119c0).

LLVM6 builds 37 ordinary words (including original packet boundaries and
source-defined error trap). One K0 unlock is encoded by a checked V66 enum
at an exact source NOP placeholder. The original firmware is used only
when --reference explicitly requests strict verification.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import KernelZeroLock
START=0x11928;END=0x119c0;SIZE=END-START;PHYSICAL=0x8be0e928
SOURCE=T/'rwe-next-global-record-complete.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
UNLOCK_AT=0x1198c
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing LLVM source assembler')
    a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
    out=bytearray(a.assemble(llvm_mc,SOURCE,SIZE));p=UNLOCK_AT-START
    if out[p:p+4]!=NOP:raise a.AdspError('Source NOP placeholder missing for V66 k0unlock')
    out[p:p+4]=struct.pack('<I',int(KernelZeroLock.UNLOCK))
    if any(out[i:i+4]==NOP for i in range(0,SIZE,4)):
        raise a.AdspError('Unresolved privileged slot in completion')
    return bytes(out)

def verify(ref:bytes,built:bytes)->None:
    if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong original ADSP firmware revision')
    if len(built)!=SIZE or built!=ref[START:END]:
        raise ValueError('Global record completion differs from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP record completion {SIZE} bytes file [0x{START:x},0x{END:x}) '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
