#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""CDSP V66 classifier guard: compare 176 source-built bytes, all Rizin
semantic words and independent LLVM6 assembly for every instruction field.

Hexagon packet ordering can reorder compound predicate and branch operations:
LLVM6 sample groups explicitly map their assembled positions to the original
semantic positions; parse bits are checked through the full Rust/reference
byte comparison, not ignored by reference comparison.
"""
from pathlib import Path
import argparse, hashlib, importlib.util, re, subprocess, tempfile

START=0x5488;END=0x5538;PC=0x96c0_2488
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'allocframe(SP,#0x10):raw','R9 = ##0x0','R11 = R0','R11 = lsr(R11,#0xe)',
 'R11 = and(R11,##0x3)','R11 = asl(R11,#0x1e)','P0 = cmp.eq(R11,##0x0)',
 'if (P0) R9 = ##0x1','P0 = cmp.eq(R9,##0x1)','if (P0.new) jump:t 0x96c02680',
 'R9 = ##0x0','R11 = R0','R11 = lsr(R11,#0x1b)','R11 = and(R11,##0x1f)',
 'R11 = asl(R11,#0x1b)','immext(##0x10000000)','P0 = cmp.eq(R11,##0x10000000)',
 'if (P0) R9 = ##0x1','P0 = cmp.eq(R9,##0x1)','if (!P0.new) jump:nt 0x96c02504',
 'R9 = ##0x0','R11 = R0','R11 = lsr(R11,#0x19)','R11 = and(R11,##0x3)',
 'R11 = asl(R11,#0x1e)','immext(##0xc0000000)','P0 = cmp.eq(R11,##-0x40000000)',
 'if (P0) R9 = ##0x1','P0 = cmp.eq(R9,##0x1)','if (P0.new) jump:nt 0x96c02504',
 'jump 0x96c0252c','R9 = ##0x0','R11 = R0',
 'R11 = lsr(R11,#0x1b)','R11 = and(R11,##0x1f)','R11 = asl(R11,#0x1b)',
 'immext(##0x20000000)','P0 = cmp.eq(R11,##0x20000000)',
 'if (P0) R9 = ##0x1','P0 = cmp.eq(R9,##0x1)',
 'if (!P0.new) jump:nt 0x96c02538',
 'immext(##0xffffdfc0)','R0 = ##-0x2002','jump 0x96c02878',
]
assert len(EXPECTED)==44
# Exact opcode fields, compiled *independently* from Hexagon source.
# P0.new branch + compare packet samples are emitted in opcode priority
# order by LLVM; the list states the corresponding original word positions.
CASES=[
 ('{allocframe(#16)}',[0]),
 ('{r9=#0}',[1,10,20,31]),
 ('{r11=r0}',[2,11,21,32]),
 ('{r11=lsr(r11,#14)}',[3]),
 ('{r11=and(r11,#3)}',[4,23]),
 ('{r11=asl(r11,#30)}',[5,24]),
 ('{p0=cmp.eq(r11,#0)}',[6]),
 ('{if(p0)r9=#1}',[7,17,27,38]),
 ('{p0=cmp.eq(r9,#1)\n if(p0.new) jump:t target}\n.space 0x1d0\ntarget:\n{nop}',[9,8]),
 ('{r11=lsr(r11,#27)}',[12,33]),
 ('{r11=and(r11,#31)}',[13,34]),
 ('{r11=asl(r11,#27)}',[14,35]),
 ('{p0=cmp.eq(r11,##0x10000000)}',[15,16]),
 ('{p0=cmp.eq(r9,#1)\n if(!p0.new) jump:nt target}\n.space 0x2c\ntarget:\n{nop}',[19,18]),
 ('{r11=lsr(r11,#25)}',[22]),
 ('{p0=cmp.eq(r11,##0xc0000000)}',[25,26]),
 ('{p0=cmp.eq(r9,#1)\n if(p0.new) jump:nt target}\n.space 4\ntarget:\n{nop}',[29,28]),
 ('{jump target}\n.space 0x28\ntarget:\n{nop}',[30]),
 ('{p0=cmp.eq(r11,##0x20000000)}',[36,37]),
 ('{p0=cmp.eq(r9,#1)\n if(!p0.new) jump:nt target}\n.space 0xc\ntarget:\n{nop}',[40,39]),
 ('{r0=##-0x2002\n jump target}\n.space 0x340\ntarget:\n{nop}',[43,41,42]),
]

def assemble(mc:Path,source:str)->bytes:
 with tempfile.TemporaryDirectory() as td:
  d=Path(td)
  (d/'code.s').write_text('.text\n.global entry\nentry:\n'+source+'\n')
  p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
    '-o',str(d/'code.o'),str(d/'code.s')],capture_output=True,text=True)
  if p.returncode:raise ValueError('LLVM6 rejected '+repr(source)+': '+p.stderr)
  spec=importlib.util.spec_from_file_location('elf',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
  if not spec or not spec.loader:raise ValueError('missing ELF32 text extraction helper')
  mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
  return mod.parse_elf32_text((d/'code.o').read_bytes())

def verify(generated:Path,reference:Path,rz:Path,mc:Path)->None:
 ref=reference.read_bytes()
 if hashlib.sha256(ref).hexdigest()!=SHA:raise ValueError('Wrong pinned CDSP firmware revision')
 actual=generated.read_bytes()
 if len(actual)!=END-START or actual!=ref[START:END]:raise ValueError('Typed CDSP guard differs from reference')
 p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',actual.hex()],
    capture_output=True,text=True,check=True)
 dec=[]
 for line in p.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:dec.append((int(m[1],16),''.join(m[2].lower().split()).split('<endloop')[0]))
 if len(dec)!=44:raise ValueError('Rizin decoded only '+str(len(dec))+' of 44 words')
 for i,((pc,sem),expected) in enumerate(zip(dec,EXPECTED)):
  if pc!=PC+4*i or ''.join(expected.lower().split())!=sem:
   raise ValueError(f'Rizin semantics at index {i}: {sem!r}, expected {expected!r}')
 matched=set()
 for src,idxs in CASES:
  b=assemble(mc,src)
  # Repeated scalar samples reuse the first assembled word; explicit
  # multi-instruction groups list LLVM's reordered output word indices.
  compound=idxs in ([9,8],[19,18],[15,16],[25,26],
                    [29,28],[36,37],[40,39])
  count=3 if idxs==[43,41,42] else 2 if compound else 1
  if len(b)<4*count:raise ValueError('LLVM6 output too short for '+src)
  for j,i in enumerate(idxs):
   source_index=j if compound or count==3 else 0
   opcode=int.from_bytes(b[4*source_index:4*(source_index+1)],'little')&~0xc000
   expected=int.from_bytes(actual[4*i:4*i+4],'little')&~0xc000
   if opcode!=expected:
    raise ValueError(f'Independent LLVM6 word mismatch at {i} ({START+4*i:#x}): {opcode:#x} expected {expected:#x}')
   matched.add(i)
 if len(matched)!=44:raise ValueError(f'Only {len(matched)} of 44 LLVM6 words verified, missing {sorted(set(range(44))-matched)}')
 print(f'CDSP codec format guard: {len(actual)} byte exact; {len(dec)} Rizin words; {len(matched)} LLVM6 independently assembled operand encodings. Partial function.')

if __name__=='__main__':
 a=argparse.ArgumentParser()
 for n in ('rust-output','reference','rz-asm','llvm-mc'):a.add_argument('--'+n,required=True,type=Path)
 q=a.parse_args();verify(q.rust_output,q.reference,q.rz_asm,q.llvm_mc)
