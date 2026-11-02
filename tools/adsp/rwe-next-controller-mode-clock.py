#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-assemble ADSP mode clock snapshot/error trap [0x13a2c,0x13a7c)."""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import read_system_register_pair
START=0x13a2c; END=0x13a7c; SIZE=END-START;PHYSICAL=0x8be10a2c
SOURCE=T/'rwe-next-controller-mode-clock.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)
def render(llvm_mc:str)->bytes:
 spec=importlib.util.spec_from_file_location('hex_llvm6',T/'emit-adsp-entry.py')
 if not spec or not spec.loader:raise RuntimeError('Hexagon LLVM6 source assembler missing')
 emitter=importlib.util.module_from_spec(spec);spec.loader.exec_module(emitter)
 out=bytearray(emitter.assemble(llvm_mc,SOURCE,SIZE))
 if out[:4]!=NOP:raise emitter.AdspError('Expected first PCYCLE paired read placeholder')
 struct.pack_into('<I',out,0,read_system_register_pair(0,30))
 if any(out[i:i+4]==NOP for i in range(0,SIZE,4)):
  raise emitter.AdspError('Unexpected leftover privileged placeholder')
 return bytes(out)
def verify(ref:bytes,out:bytes)->None:
 if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:raise ValueError('ADSP reference revision mismatch')
 if len(out)!=SIZE or ref[START:END]!=out:raise ValueError('ADSP controller mode clock snapshot differs from reference')
def main()->int:
 ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--llvm-mc',required=True);ap.add_argument('--reference',type=Path);ap.add_argument('--output',type=Path,required=True)
 a=ap.parse_args();out=render(a.llvm_mc)
 if a.reference:verify(a.reference.read_bytes(),out)
 a.output.write_bytes(out)
 print(f'ADSP controller mode clock {SIZE} source-built bytes at [0x{START:x},0x{END:x}), sha256={hashlib.sha256(out).hexdigest()}')
 return 0
if __name__=='__main__':raise SystemExit(main())
