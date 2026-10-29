#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild Hexagon V66 cycle-counter helper [0x139c8,0x139dc).

Four LLVM6 source-assembled words plus one checked paired PCYCLE read.
The jump target is a source-authored physical controller function address.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
from adsp_hexagon_v66 import read_system_register_pair
START=0x139c8;END=0x139dc;SIZE=END-START;PHYSICAL=0x8be109c8
SOURCE=T/'adsp-agent20-rwe-next-controller-cycle-helper.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
 sp=importlib.util.spec_from_file_location('hex_llvm',T/'emit-adsp-entry.py')
 if not sp or not sp.loader:raise RuntimeError('LLVM6 source assembler missing')
 m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 out=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
 if out[:4]!=NOP:raise m.AdspError('Expected paired PCYCLE NOP placeholder')
 struct.pack_into('<I',out,0,read_system_register_pair(0,30))
 return bytes(out)

def verify(ref:bytes,generated:bytes)->None:
 if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:
  raise ValueError('ADSP reference revision mismatch')
 if len(generated)!=SIZE or generated!=ref[START:END]:
  raise ValueError('ADSP controller cycle helper differs from reference')
 if ref[END:END+4]!=bytes(4):
  raise ValueError('Expected 4-byte linker alignment after cycle helper')

def main()->int:
 p=argparse.ArgumentParser(description=__doc__)
 p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
 p.add_argument('--output',type=Path,required=True)
 a=p.parse_args();out=render(a.llvm_mc)
 if a.reference:verify(a.reference.read_bytes(),out)
 a.output.write_bytes(out)
 print(f'ADSP cycle helper: {SIZE} byte-identical executable bytes file [0x{START:x},0x{END:x}) '
       f'physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}), 4 LLVM6 plus typed V66 PCYCLE; '
       f'SHA-256={hashlib.sha256(out).hexdigest()}')
 return 0
if __name__=='__main__':raise SystemExit(main())
