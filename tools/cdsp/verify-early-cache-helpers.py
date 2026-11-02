#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check CDSP cache-helper group: 228 reference-exact bytes, 57 Rizin V66
semantics and independent LLVM6 ordinary word assembly. Privileged CFGBASE,
physical-word access and LLVM-shortened extended GP loads are *not* asserted
as LLVM6 exact matches.
"""
import argparse,hashlib,importlib.util,re,subprocess,tempfile
from pathlib import Path
START=0x53a4;END=0x5488;PC=0x96c0_23a4
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=['R2 = lsr(R2,#0x2)', 'R6 = R2 ; R5 = R1', 'loop0(0x96c023b4,R2)', 'R2 = ##0x0', 'R4 = memw(R0++#0x4)', 'nop', 'memw(R1++#0x4) = R4', 'R1 = R5 ; R0 = R6', 'R0 = lsr(R0,#0x5)', 'R0 = add(R0,##0x1)', 'loop0(0x96c023d0,R0)', 'R1 = add(R1,##0x20)', 'dccleana(R1)', 'jumpr LR', 'R0 = CFGBASE', 'R1 = ##0x0', 'R0 = insert(R1,#0xc,#0x14)', 'R0 = asl(R0,#0x5)', 'R2 = ##0x8', 'R0 = memw_phys(R2,R0)', 'R0 = insert(R1,#0xc,#0x14)', 'R0 = asl(R0,#0x5)', 'R2 = ##0x44', 'R0 = add(R0,##-0x100)', 'R0 = memw_phys(R2,R0)', 'immext(##0xfe016e00)', 'memw(R1=##0xfe016e1c) = R0', 'dccleana(R1)', 'jumpr LR', 'allocframe(SP,#0x0):raw', 'R2 = ##0x80', 'immext(##0xfe016f40)', 'R1 = ##-0x1fe90a0', 'immext(##0xfe026dc0)', 'R0 = memw(GP+##0xfe026ddc)', 'immext(##0xffffff40)', 'call 0x96c023a4', 'R2 = ##0x80', 'immext(##0xfe016fc0)', 'R1 = ##-0x1fe9020', 'immext(##0xfe026dc0)', 'R0 = memw(GP+##0xfe026de8)', 'immext(##0xffffff40)', 'call 0x96c023a4', 'LR:FP = dealloc_return(FP):raw', 'immext(##0xfe016d40)', 'R0 = memw(GP+##0xfe016d40)', 'R2 = lsr(R0,#0xb)', 'immext(##0xfe017040)', 'R1 = ##-0x1fe8fa0', 'loop0(0x96c02478,#0x4)', 'immext(##0x7c0)', 'R0 = and(R0,##0x7ff)', 'R3 = memw_phys(R0,R2)', 'R0 = add(R0,##0x4)', 'memw(R1++#0x4) = R3', 'jumpr LR']
SAMPLE_CASES=[('{r2=lsr(r2,#2)}', [0]), ('{r6=r2;r5=r1}', [1]), ('{loop0(target,r2)}\n.space 4\ntarget:\n{nop}', [2]), ('{r2=#0}', [3]), ('{r4=memw(r0++#4)}', [4]), ('{nop}', [5]), ('{memw(r1++#4)=r4}', [6]), ('{r1=r5;r0=r6}', [7]), ('{r0=lsr(r0,#5)}', [8]), ('{r0=add(r0,#1)}', [9]), ('{loop0(target,r0)}\ntarget:\n{nop}', [10]), ('{r1=add(r1,#32)\n dccleana(r1)}', [11, 12]), ('{jumpr lr}', [13]), ('{r1=#0}', [15]), ('{r0=insert(r1,#12,#20)}', [16]), ('{r0=asl(r0,#5)}', [17]), ('{r2=#8}', [18]), ('{r0=insert(r1,#12,#20)}', [20]), ('{r0=asl(r0,#5)}', [21]), ('{r2=#68}', [22]), ('{r0=add(r0,#-256)}', [23]), ('{memw(r1=##0xfe016e1c)=r0}', [25, 26]), ('{dccleana(r1)}', [27]), ('{jumpr lr}', [28]), ('{allocframe(#0)}', [29]), ('{r2=#128}', [30]), ('{r1=##0xfe016f60}', [31, 32]), ('{call ##-140}', [35, 36]), ('{r2=#128}', [37]), ('{r1=##0xfe016fe0}', [38, 39]), ('{call ##-168}', [42, 43]), ('{dealloc_return}', [44]), ('{r2=lsr(r0,#11)}', [47]), ('{r1=##0xfe017060}', [48, 49]), ('{loop0(target,#4)}\n.space 8\ntarget:\n{nop}', [50]), ('{r0=and(r0,##0x7ff)}', [51, 52]), ('{r0=add(r0,#4)\nmemw(r1++#4)=r3}', [54, 55]), ('{jumpr lr}', [56])]

def assemble(mc:Path,source:str)->bytes:
 with tempfile.TemporaryDirectory() as td:
  d=Path(td);(d/'code.s').write_text('.text\n.global entry\nentry:\n'+source+'\n')
  p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
    '-o',str(d/'code.o'),str(d/'code.s')],capture_output=True,text=True)
  if p.returncode:raise ValueError('LLVM6 assembly rejected independent source '+repr(source)+': '+p.stderr)
  spec=importlib.util.spec_from_file_location('elf',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
  if not spec or not spec.loader:raise ValueError('Missing ELF .text reader')
  elf=importlib.util.module_from_spec(spec);spec.loader.exec_module(elf)
  return elf.parse_elf32_text((d/'code.o').read_bytes())

def verify(generated:Path,reference:Path,rz:Path,mc:Path):
 ref=reference.read_bytes(); ours=generated.read_bytes()
 if hashlib.sha256(ref).hexdigest()!=SHA:raise ValueError('Incorrect CDSP firmware reference')
 if len(ours)!=END-START or ours!=ref[START:END]:raise ValueError('Source-built helper code differs from pinned reference')
 q=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',ours.hex()],
                  capture_output=True,text=True,check=True)
 rows=[]
 for line in q.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:rows.append((int(m[1],16),''.join(m[2].lower().split()).split('<endloop')[0]))
 if len(rows)!=57:raise ValueError('Rizin decoded '+str(len(rows))+' rather than 57 instructions')
 for i,((pc,semantic),wanted) in enumerate(zip(rows,EXPECTED)):
  if pc!=PC+i*4 or ''.join(wanted.lower().split()) not in semantic:
   raise ValueError(f'Rizin mismatch word {i}: {semantic}, expected {wanted}')
 confirmed=set()
 for source,positions in SAMPLE_CASES:
  b=assemble(mc,source)
  if len(b)<4*len(positions):raise ValueError('LLVM6 produced too little code')
  for j,i in enumerate(positions):
   opcode=int.from_bytes(b[4*j:4*j+4],'little')&~0xc000
   reference_opcode=int.from_bytes(ours[4*i:4*i+4],'little')&~0xc000
   if opcode!=reference_opcode:
    raise ValueError(f'LLVM6 mismatch at instruction {i}: {opcode:#x} vs {reference_opcode:#x}')
   confirmed.add(i)
 if len(confirmed)<40:raise ValueError(f'Expected >=40 independent LLVM6 matches, found {len(confirmed)}')
 print(f'CDSP cache helper group: {len(ours)} exact bytes, {len(rows)} Rizin decoded words, {len(confirmed)} LLVM6-tested ordinary words; privileged/GP extenders not LLVM-exact')

if __name__=='__main__':
 p=argparse.ArgumentParser()
 for a in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+a,required=True,type=Path)
 q=p.parse_args();verify(q.rust_output,q.reference,q.rz_asm,q.llvm_mc)
