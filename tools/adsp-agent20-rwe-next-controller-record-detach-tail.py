#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 controller-record detach completion [0x13514,0x13610).

Build 62 ordinary packet words with LLVM6 and one checked V66 K0 lock;
join with the prior 148-byte prefix for the complete 400-byte function.
One parse-field adjustment groups the jump with its subsequent memory packet.
OEM bytes are only consulted for optional revision-pinned verification.
"""
from __future__ import annotations
import argparse, hashlib, importlib.util, struct, sys
from pathlib import Path
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
from adsp_hexagon_v66 import KernelZeroLock,packet_continuation
START=0x13514;END=0x13610;SIZE=END-START;PHYSICAL=0x8be10514
SOURCE=T/'adsp-agent20-rwe-next-controller-record-detach-tail.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
 sp=importlib.util.spec_from_file_location('adsp_llvm',T/'emit-adsp-entry.py')
 if not sp or not sp.loader:raise RuntimeError('Missing LLVM6 source assembler')
 m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 b=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
 a=0x13590-START
 if b[a:a+4]!=NOP:raise m.AdspError('Expected exact K0 lock NOP sentinel')
 struct.pack_into('<I',b,a,int(KernelZeroLock.LOCK))
 a=0x13538-START
 orig=struct.unpack_from('<I',b,a)[0]
 patched=packet_continuation(orig)
 if orig & ~0xc000 != patched & ~0xc000:
  raise m.AdspError('Packet adjustment may only change parse bits')
 struct.pack_into('<I',b,a,patched)
 if any(b[i:i+4]==NOP for i in range(0,SIZE,4)):
  raise m.AdspError('Unexpected NOP placeholder')
 return bytes(b)

def verify(ref:bytes,built:bytes)->None:
 if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:
  raise ValueError('Wrong ADSP pinned reference revision')
 if len(built)!=SIZE or built!=ref[START:END]:
  raise ValueError('ADSP controller detach tail differs from reference')

def main()->int:
 p=argparse.ArgumentParser(description=__doc__)
 p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
 p.add_argument('--output',type=Path,required=True)
 a=p.parse_args();b=render(a.llvm_mc)
 if a.reference:verify(a.reference.read_bytes(),b)
 a.output.write_bytes(b)
 print(f'ADSP detach tail: {SIZE} exact code bytes [0x{START:x},0x{END:x}), '
       f'62 LLVM6 words plus one checked V66 K0 lock and one packet-parse field, '
       f'SHA-256={hashlib.sha256(b).hexdigest()}')
 return 0
if __name__=='__main__':raise SystemExit(main())
