#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent CDSP D-cache tag routine validation.

For a pinned external CDSP MBN: verify 180 source-built bytes, decode all 45
V66 instruction words with Rizin, and independently assemble 37 verified
ordinary word/packet encodings via LLVM6. The two GP-relative loads may be
optimized by LLVM into compact forms; four privileged LC/DCTAGR/PHYS words
have only Rizin/reference corroboration and are not counted as LLVM-exact.
"""
import argparse,hashlib,importlib.util,re,subprocess,tempfile
from pathlib import Path
START=0x51b4;END=0x5268;PC=0x96c0_21b4
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'immext(##0xfe039200)', 'R10=memw(GP+##0xfe039204)',
 'R8=mpyi(R10,R12)', 'immext(##0xfe026d00)',
 'R14=memw(GP+##0xfe026d38)', 'R8=asl(R8,#0x5)',
 'P0=cmp.eq(R14,##0x0)', 'if(P0.new)jump:nt0x96c02260',
 'loop1(0x96c021d8,R10)', 'R10=LC1', 'loop0(0x96c021e4,#0x8)',
 'R12=add(R10,##-0x1)', 'R10=LC0', 'R7=##0x28',
 'R10=add(R10,##-0x1)','R8=sub(R8,R7)',
 'R10=insert(R12,#0x7,#0x5)','R5=R8',
 'immext(##0x1ffffc0)','R4=##0x1ffffff','R1=dctagr(R10)',
 'P0=tstbit(R1,#0x1d)','if(!P0.new)jump:nt0x96c02244',
 'memw(R14+R5<<#0x0)=R1','R2=asl(R12,#0x5)',
 'R3=and(R1,R4)','R4=##0x0','R7=R2',
 'R7=insert(R3,#0x15,#0xb)','R5=add(R5,##0x4)',
 'memw(R14+R5<<#0x0)=R7','R6=memw_phys(R2,R3)',
 'R5=add(R5,##0x4);R4=add(R4,#1)',
 'R2=add(R2,##0x4)','memw(R14+R5<<#0x0)=R6',
 'P0=cmp.eq(R4,#0x8);if(!P0.new)jump:t0x96c02230',
 'nop','nop','nop','R1=##0x2','immext(##0xfe016e00)',
 'memw(R0=##0xfe016e24)=R1','dccleana(R0)',
 'LR:FP=deallocframe(FP):raw','jumprLR',
]
assert len(EXPECTED)==45
# LLVM6 sample has the *source* of thirty ordinary forms; the first sample
# includes the full exact packet-encoded duplex and the new INSERT offset bit.
SCALAR_INDICES=[2,5,6,11,13,14,15,16,17,18,19,21,23,24,25,26,27,28,
                29,30,32,33,34,36,39,40,41,42,43,44]
# Branch packet and hardware loop tests from independently authored labels.
FLOW_INDICES=[(3,7),(40,8),(41,10),(45,22),(64,35),(59,37),(60,38)]
PRIVILEGED_WORDS=[9,12,20,31]
GP_EXTENDED_WORDS=[0,1,3,4]  # LLVM chooses compact addressing, not the OEM long form.

SCALAR_SOURCE='.text\n.global entry\nentry:\n{ r8=mpyi(r10,r12) }\n{ r8=asl(r8,#5) }\n{ p0=cmp.eq(r14,#0) }\n{ r12=add(r10,#-1) }\n{ r7=#40 }\n{ r10=add(r10,#-1) }\n{ r8=sub(r8,r7) }\n{ r10=insert(r12,#7,#5) }\n{ r5=r8 }\n{ r4=##0x1ffffff }\n{ p0=tstbit(r1,#29) }\n{ memw(r14+r5<<#0)=r1 }\n{ r2=asl(r12,#5) }\n{ r3=and(r1,r4) }\n{ r4=#0 }\n{ r7=r2 }\n{ r7=insert(r3,#21,#11) }\n{ r5=add(r5,#4) }\n{ memw(r14+r5<<#0)=r7 }\n{ r5=add(r5,#4); r4=add(r4,#1) }\n{ r2=add(r2,#4) }\n{ memw(r14+r5<<#0)=r6 }\n{ nop }\n{ r1=#2 }\n{ memw(r0=##0xfe016e24)=r1 }\n{ dccleana(r0) }\n{ deallocframe }\n{ jumpr lr }\n'
FLOW_SOURCE='.text\n.global entry\nentry:\n{ r10=memw(gp+##0xfe039204) }\n{ r14=memw(gp+##0xfe026d38) }\n{ r8=asl(r8,#5)\n  p0=cmp.eq(r14,#0)\n  if (p0.new) jump:nt skip }\n.space (0x98-12)\nskip:\n{ loop1(next,r10) }\nnext:\n{ loop0(l0,#8) }\n.space 4\nl0:\n{ nop }\n{ p0=tstbit(r1,#29); if(!p0.new) jump:nt exit; memw(r14+r5<<#0)=r1 }\n.space (0x3c-12)\nexit:\n{ nop }\nback:\n{ nop }\n.space 12\n{ p0=cmp.eq(r4,#8); if(!p0.new) jump:t back }\n'

def assemble(mc:Path,source:str)->bytes:
 with tempfile.TemporaryDirectory(prefix='cdsp-tag-llvm-') as d:
  d=Path(d);(d/'code.s').write_text(source)
  p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60',
    '-filetype=obj','-o',str(d/'code.o'),str(d/'code.s')],capture_output=True,text=True)
  if p.returncode:raise ValueError('Independent LLVM6 source rejected: '+p.stderr)
  spec=importlib.util.spec_from_file_location('hexagon_elf',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
  assert spec and spec.loader
  mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
  return mod.parse_elf32_text((d/'code.o').read_bytes())

def verify(ours:Path,reference:Path,rz:Path,mc:Path):
 orig=reference.read_bytes();data=ours.read_bytes()
 if hashlib.sha256(orig).hexdigest()!=SHA:raise ValueError('Wrong CDSP revision')
 if len(data)!=END-START or orig[START:END]!=data:raise ValueError('CDSP D-cache tag bytes differ from pinned reference')
 decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',data.hex()],
                        capture_output=True,text=True,check=True)
 lines=[]
 for line in decoded.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:lines.append((int(m[1],16),''.join(m[2].lower().split())))
 if len(lines)!=len(EXPECTED):raise ValueError(f'Expected {len(EXPECTED)} decoder words, found {len(lines)}')
 for i,((pc,meaning),expected) in enumerate(zip(lines,EXPECTED)):
  if pc!=PC+i*4 or ''.join(expected.lower().split()) not in meaning:
   raise ValueError(f'Rizin mismatch at file {START+i*4:#x}: got {meaning}, expected {expected}')
 confirmed=set()
 for source,index_map in [(SCALAR_SOURCE,enumerate(SCALAR_INDICES)),
                          (FLOW_SOURCE,FLOW_INDICES)]:
  independent=assemble(mc,source)
  for j,i in index_map:
   if (j+1)*4>len(independent):raise ValueError('LLVM independently assembled fewer words than expected')
   a=int.from_bytes(independent[j*4:j*4+4],'little')&~0xc000
   b=int.from_bytes(data[i*4:i*4+4],'little')&~0xc000
   if a!=b:raise ValueError(f'LLVM {i} opcode fields differ: {a:#x} != {b:#x}')
   confirmed.add(i)
 assert confirmed==set(range(45))-set(PRIVILEGED_WORDS)-set(GP_EXTENDED_WORDS)
 print(f'CDSP D-cache tag recovery: {len(data)} exact bytes; {len(lines)} Rizin V66 words; {len(confirmed)} LLVM6-confirmed ordinary words; 4 GP extenders and 4 privileged words not LLVM-exact')

if __name__=='__main__':
 p=argparse.ArgumentParser()
 for name in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+name,required=True,type=Path)
 a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
