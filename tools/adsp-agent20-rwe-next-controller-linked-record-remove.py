#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild V66 ADSP linked-record removal function [0x13750,0x13828).

All 54 instructions are independently source-assembled by LLVM6. Its early
external call continues the following packet in the original scheduler, so
one checked packet parse bit group is changed without altering operands.
The preceding twelve file-backed zero bytes are never counted as code.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
from adsp_hexagon_v66 import packet_continuation
START=0x13750;END=0x13828;SIZE=END-START;PHYSICAL=0x8be10750
SOURCE=T/'adsp-agent20-rwe-next-controller-linked-record-remove.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
CALL_PACKET=0x13760

def render(llvm_mc:str)->bytes:
 sp=importlib.util.spec_from_file_location('adsp_llvm',T/'emit-adsp-entry.py')
 if not sp or not sp.loader:raise RuntimeError('Missing LLVM6 emitter')
 m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 code=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
 off=CALL_PACKET-START; word=struct.unpack_from('<I',code,off)[0]
 changed=packet_continuation(word)
 if word & ~0xc000 != changed & ~0xc000:
  raise m.AdspError('Packet parse update modified operands')
 struct.pack_into('<I',code,off,changed)
 return bytes(code)

def verify(ref:bytes,rebuilt:bytes)->None:
 if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:
  raise ValueError('Wrong ADSP reference revision')
 if ref[START-12:START]!=bytes(12):
  raise ValueError('Expected 12 bytes of file-backed linker zeros')
 if len(rebuilt)!=SIZE or rebuilt!=ref[START:END]:
  raise ValueError('Controller linked record removal differs from reference')

def main()->int:
 p=argparse.ArgumentParser(description=__doc__)
 p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
 p.add_argument('--output',type=Path,required=True)
 a=p.parse_args();output=render(a.llvm_mc)
 if a.reference:verify(a.reference.read_bytes(),output)
 a.output.write_bytes(output)
 print(f'ADSP linked record remove: {SIZE} exact instruction bytes, file '
       f'[0x{START:x},0x{END:x}), physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}), '
       f'54 LLVM6 words, one checked packet marker, sha256={hashlib.sha256(output).hexdigest()}')
 return 0
if __name__=='__main__':raise SystemExit(main())
