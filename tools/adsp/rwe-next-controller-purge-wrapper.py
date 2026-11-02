#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble ADSP V66 record-purge wrapper [0x13828,0x13844).

6 LLVM6 source words plus one checked SGP0 read. The `extractu` and call
share the packet, preserving the original byte-relative call target.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register
START=0x13828;END=0x13844;SIZE=END-START;PHYSICAL=0x8be10828
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-purge-wrapper.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
 spec=importlib.util.spec_from_file_location('adsp_llvm',T / 'emit-adsp-entry.py')
 if not spec or not spec.loader:raise RuntimeError('Missing LLVM6 emitter')
 m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
 code=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
 if code[4:8]!=NOP:raise m.AdspError('Expected checked R2=SGP0 placeholder')
 struct.pack_into('<I',code,4,read_system_register(2,SystemRegister.SGP0))
 return bytes(code)

def verify(ref:bytes,output:bytes)->None:
 if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:raise ValueError('Wrong pinned ADSP revision')
 if len(output)!=SIZE or output!=ref[START:END]:
  raise ValueError('ADSP purge wrapper differs from reference')
 if ref[END:END+12]!=bytes(12):
  raise ValueError('Expected following 12-byte original linker fill')

def main()->int:
 p=argparse.ArgumentParser(description=__doc__)
 p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
 p.add_argument('--output',type=Path,required=True)
 a=p.parse_args();code=render(a.llvm_mc)
 if a.reference:verify(a.reference.read_bytes(),code)
 a.output.write_bytes(code)
 print(f'ADSP purge wrapper: {SIZE} exact code bytes; physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}), '
       f'6 LLVM6 words plus checked SGP0; SHA-256={hashlib.sha256(code).hexdigest()}')
 return 0
if __name__=='__main__':raise SystemExit(main())
