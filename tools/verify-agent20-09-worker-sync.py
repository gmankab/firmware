#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check MODECTL post-guard worker sync by independent V66 decode/LLVM6 forms.

All 23 reference byte-identical typed instructions are decoded with Rizin.
LLVM6 separately assembles 21 exact words including the extended absolute
pointer, status writes, cache maintenance, short CALL and return; HTID and
MODECTL are privileged and Rizin/reference checked only.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess,tempfile,struct

START=0x5004;END=0x5060;PHYS=0x96c02004
HASH='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
SEMANTICS=[
 'immext(##0xfe000040)','r1=##-0x1ffffbc',
 'immext(##0x0)','memw(r1+#0x0)=##0x1f','dccleana(r1)',
 'r1=htid','immext(##0xfe016e00)','r0=##-0x1fe9200',
 'r0+=mpyi(r1,##0x4)','r1=modectl','memw(r0+##0x0)=r1.new',
 'dccleana(r0)','dcinva(r0)','r1=memw(r0+##0x0)',
 'call0x96c3736c','jump0x96c02040',
 'r0=##0x1','immext(##0xfe024d00)','memw(r1=##0xfe024d00)=r0.new',
 'dccleana(r1)','dcinva(r1)','r0=memw(r1+##0x0)','jumprlr',
]
assert len(SEMANTICS)==23

def assemble(mc:Path,asm:str)->bytes:
 with tempfile.TemporaryDirectory(prefix='cdsp38-') as name:
  root=Path(name)
  (root/'sample.s').write_text('.text\n.global entry\nentry:\n'+asm+'\n')
  p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
    '-o',str(root/'sample.o'),str(root/'sample.s')],capture_output=True,text=True)
  if p.returncode:raise ValueError('LLVM6 independent assembly error: '+p.stderr)
  src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
  spec=importlib.util.spec_from_file_location('cdsp_sync_elf',src);assert spec and spec.loader
  mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
  return mod.parse_elf32_text((root/'sample.o').read_bytes())

def verify(out:Path,ref:Path,decoder:Path,llvm:Path):
 raw=ref.read_bytes();actual=out.read_bytes()
 if hashlib.sha256(raw).hexdigest()!=HASH:raise ValueError('incorrect CDSP reference revision')
 if len(actual)!=END-START or actual!=raw[START:END]:raise ValueError('typed source output differs from CDSP reference')
 dec=subprocess.run([str(decoder),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],
                    capture_output=True,text=True,check=True)
 rows=[]
 for line in dec.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:rows.append((int(m[1],16),''.join(m[2].lower().split())))
 if len(rows)!=len(SEMANTICS):raise ValueError(f'Rizin decoded {len(rows)} words, expected {len(SEMANTICS)}')
 for i,((address,decoded),expected) in enumerate(zip(rows,SEMANTICS)):
  if address!=PHYS+i*4 or expected not in decoded:
   raise ValueError(f'Rizin semantic mismatch file {START+i*4:#x}: got {decoded!r} expected {expected!r}')
 sample=assemble(llvm,'''{ r1=##0xfe000044 }
{ memw(r1+#0)=##31 }
{ dccleana(r1) }
{ r0 += mpyi(r1,#4) }
{ r1=#0; memw(r0+#0)=r1.new }
{ dccleana(r0) }
{ dcinva(r0) }
{ r1=memw(r0+#0) }
loop:
{ jump loop }
{ r0=#1; memw(r1=##0xfe024d00)=r0.new }
{ dccleana(r1) }
{ dcinva(r1) }
{ r0=memw(r1+#0) }
{ jumpr lr }''')
 words=[struct.unpack_from('<I',sample,i)[0] for i in range(0,len(sample),4)]
 pairs=[(0,0),(1,1),(2,2),(3,3),(4,4),(8,5),(10,7),(11,8),(12,9),(13,10),
        (15,11),(16,12),(17,13),(18,14),(19,15),(20,16),(21,17),(22,18)]
 if len(words)!=19:raise ValueError('LLVM6 packet sample emitted unexpected word count')
 confirmed=0
 for output_idx,llvm_idx in pairs:
  if words[llvm_idx]!=struct.unpack_from('<I',actual,output_idx*4)[0]:
   raise ValueError(f'independently assembled word differs at {START+output_idx*4:#x}')
  confirmed+=1
 ptr=assemble(llvm,'{ r0=##0xfe016e00 }')
 if ptr!=actual[6*4:8*4]:raise ValueError('MODECTL slot extended assignment differs from LLVM6')
 confirmed+=2
 src=PHYS+14*4;dest=0x96c3736c;delta=dest-src
 if delta<=4 or delta%4:raise ValueError('bad CALL target alignment')
 cb=assemble(llvm,'{ call target }\n.space '+str(delta-4)+'\ntarget:\n{ jumpr lr }')
 if cb[:4]!=actual[14*4:15*4]:raise ValueError('direct CALL encoding differs from LLVM6')
 confirmed+=1
 assert confirmed==21
 print(f'CDSP worker sync: {len(actual)} exact bytes; {len(rows)} V66 words decoded; {confirmed} LLVM6 independently reproduced; 2 privileged words decoder/reference verified only')

if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--rust-output',type=Path,required=True);p.add_argument('--reference',type=Path,required=True);p.add_argument('--rz-asm',type=Path,required=True);p.add_argument('--llvm-mc',type=Path,required=True)
 a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
