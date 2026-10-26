#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check worker callbacks by reference, Rizin V66, and LLVM6 assembly.

All 27 words decode under Rizin. LLVM6 verifies the ordinary/compound
instructions; extended short-range CALLs retain reference/Rizin evidence,
not a claim of independent LLVM word reconstruction.
"""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import re
import subprocess
import tempfile

START=0x4f64
END=0x4fd0
PHYS=0x96c01f64
HASH='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
WORDS=[
 'p0=cmp.eq(r1,#0x1);if(p0.new)jump:nt0x96c01fd0',
 'call0x96c029b0',
 'immext(##0xfffff380)',
 'call0x96c01318',
 'immext(##0xa080)',
 'call0x96c0c004',
 'r0=##0x3',
 'immext(##0xfe024d00)',
 'memw(r1=##0xfe024d00)=r0.new',
 'dccleana(r1)',
 'call0x96c02184',
 'call0x96c02268',
 'call0x96c02418',
 'immext(##0x0)',
 'r0=##0x1e',
 'immext(##0xfe000040)',
 'memw(r1=##0xfe000044)=r0.new',
 'dccleana(r1)',
 'immext(##0xfffff380)',
 'call0x96c01364',
 'immext(##0x480)',
 'call0x96c02458',
 'immext(##0xadc0)',
 'call0x96c0cdb0',
 'immext(##0xa040)',
 'call0x96c0c004',
 'call0x96c02044',
]
assert len(WORDS)==27

def assemble(mc:Path,asm:str)->bytes:
 with tempfile.TemporaryDirectory(prefix='cdsp36-') as d:
  root=Path(d)
  (root/'sample.s').write_text('.text\n.global entry\nentry:\n'+asm+'\n')
  p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60',
   '-filetype=obj','-o',str(root/'sample.o'),str(root/'sample.s')],
   capture_output=True,text=True)
  if p.returncode:raise ValueError(f'LLVM6 assembly failed: {p.stderr}')
  module_path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
  spec=importlib.util.spec_from_file_location('cdsp_finalcalls_elf',module_path)
  mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
  return mod.parse_elf32_text((root/'sample.o').read_bytes())

def verify(args):
 ref=args.reference.read_bytes();actual=args.rust_output.read_bytes()
 if hashlib.sha256(ref).hexdigest()!=HASH:raise ValueError('CDSP firmware pinned SHA256 mismatch')
 if len(actual)!=END-START or actual!=ref[START:END]:
  raise ValueError('source-generated V66 callbacks differ from pinned reference')
 p=subprocess.run([str(args.rz_asm),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],
  capture_output=True,text=True,check=True)
 rows=[]
 for line in p.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:rows.append((int(m[1],16),''.join(m[2].lower().split())))
 if len(rows)!=len(WORDS):raise ValueError(f'Rizin got {len(rows)} of {len(WORDS)} words')
 for i,((phys,found),want) in enumerate(zip(rows,WORDS)):
  if phys!=PHYS+i*4 or want not in found:
   raise ValueError(f'V66 decoder differs at CDSP {START+i*4:#x}: {found!r} wanted {want!r}')
 confirmed=0
 group_checks=[
  (0,'{ p0=cmp.eq(r1,#1); if(p0.new) jump:nt label }\n.space 104\nlabel:\n{ jumpr lr }',1),
  (6,'{ r0=#3; memw(r1=##0xfe024d00)=r0.new }',3),
  (9,'{ dccleana(r1) }',1),
  (13,'{ r0=##30; memw(r1=##0xfe000044)=r0.new }',4),
  (17,'{ dccleana(r1) }',1),
 ]
 for index,asm,length in group_checks:
  compiled=assemble(args.llvm_mc,asm)[:4*length]
  if compiled!=actual[index*4:(index+length)*4]:
   raise ValueError(f'independent LLVM6 group differs at file {START+index*4:#x}')
  confirmed+=length
 # Independently assemble each short CALL to a local symbol positioned at
 # the same displacement. This also verifies the PC base and packet markers.
 for source,target in [(0x96c01f68,0x96c029b0),
                       (0x96c01f8c,0x96c02184),
                       (0x96c01f90,0x96c02268),
                       (0x96c01f94,0x96c02418),
                       (0x96c01fcc,0x96c02044)]:
  distance=target-source
  if distance<=4 or distance%4:raise ValueError('unexpected unaligned or backward call')
  sample='{ call target }\n.space '+str(distance-4)+'\ntarget:\n{ jumpr lr }'
  encoded=assemble(args.llvm_mc,sample)[:4]
  off=source-PHYS
  if encoded!=actual[off:off+4]:
   raise ValueError(f'LLVM6 packet-relative CALL differs at file {START+off:#x}')
  confirmed+=1
 print(f'CDSP worker final callbacks: {len(actual)} exact bytes; {len(WORDS)} V66 Rizin-decoded words; {confirmed} exact LLVM6 assembled words; extended CALLs Rizin/reference-verified only')

if __name__=='__main__':
 parser=argparse.ArgumentParser(description=__doc__)
 for name in ('reference','rust-output','rz-asm','llvm-mc'):
  parser.add_argument('--'+name,required=True,type=Path)
 verify(parser.parse_args())
