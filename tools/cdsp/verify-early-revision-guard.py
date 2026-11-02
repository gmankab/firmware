#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent revision guard checks: 12 Rizin V66 words, 11 LLVM6 words.

REV is a privileged system-register read; LLVM6 cannot independently
assemble it. No private OEM instruction data is embedded in this script.
"""
import argparse,hashlib,importlib.util,re,subprocess,tempfile
from pathlib import Path
START=0x5184;END=0x51b4;PC=0x96c0_2184
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'R0=REV','R0=and(R0,##0xff)','P0=cmp.gt(R0,##0x60)',
 'if(!P0.new)jumpr:ntLR','R0=##0x1','R12=##0xa',
 'immext(##0xfe016e00)','R2=memw(R1=##0xfe016e24)',
 'P0=cmp.eq(R2,#0x0);if(!P0.new)jumpr:ntR31',
 'memw(R1+#0x0)=R0;allocframe(#0x0)','call0x96c034e8','jump0x96c02260',
]
SOURCE='''
{ r0=and(r0,#255) }
{ p0=cmp.gt(r0,#96); if(!p0.new) jumpr:nt lr }
{ r0=#1; r12=#10; r2=memw(r1=##0xfe016e24) }
{ p0=cmp.eq(r2,#0); if(!p0.new) jumpr:nt lr }
{ memw(r1+#0)=r0; allocframe(#0) }
{ jump exit }
.space (0x96c02260 - 0x96c021b0 - 4)
exit:
{ jumpr lr }
'''

def assemble(mc:Path,src:str)->bytes:
 with tempfile.TemporaryDirectory() as d:
  d=Path(d);(d/'a.s').write_text('.text\n.global entry\nentry:\n'+src)
  result=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
     '-o',str(d/'a.o'),str(d/'a.s')],capture_output=True,text=True)
  if result.returncode:raise ValueError('LLVM6 assembler failed '+result.stderr)
  spec=importlib.util.spec_from_file_location('mod',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
  assert spec and spec.loader
  mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
  return mod.parse_elf32_text((d/'a.o').read_bytes())

def verify(ours:Path,reference:Path,rz:Path,mc:Path):
 original=reference.read_bytes();data=ours.read_bytes()
 if hashlib.sha256(original).hexdigest()!=SHA:raise ValueError('Wrong CDSP reference firmware')
 if data!=original[START:END] or len(data)!=END-START:raise ValueError('Typed revision guard differs from pinned reference')
 dec=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',data.hex()],capture_output=True,text=True,check=True)
 rows=[]
 for line in dec.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:rows.append((int(m.group(1),16),''.join(m.group(2).lower().split())))
 if len(rows)!=12:raise ValueError('Rizin did not decode all 12 V66 words')
 for n,((pc,text),expected) in enumerate(zip(rows,EXPECTED)):
  if pc!=PC+n*4 or ''.join(expected.lower().split()) not in text:
   raise ValueError(f'Rizin disagreement at file {START+n*4:#x}: {text}, expected {expected}')
 sample=assemble(mc,SOURCE)
 # Ignore the first, privileged REV. Independent sample has all other
 # instructions except CALL, checked with its actual relative address below.
 for j,i in enumerate([1,2,3,4,5,6,7,8,9,11]):
  if sample[4*j:4*j+4]!=data[i*4:i*4+4]:
   raise ValueError(f'LLVM6 sample mismatch at word {i}')
 direct=assemble(mc,'{ call destination }\n.space (0x133c-4)\ndestination:\n{ jumpr lr }')
 if direct[:4]!=data[10*4:11*4]:raise ValueError('LLVM6 direct callback call mismatch')
 print('Revision guard: 48 exact bytes, 12 Rizin V66 words, 11 LLVM6-assembled words; REV privileged read decoder/reference verified only')

if __name__=='__main__':
 p=argparse.ArgumentParser()
 for name in ('rust-output','reference','rz-asm','llvm-mc'):
  p.add_argument('--'+name,required=True,type=Path)
 a=p.parse_args()
 verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
