#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V66/LLVM6 byte validation of worker ownership decision."""
import argparse,importlib.util,subprocess,hashlib,re,tempfile
from pathlib import Path
START=0x4e04;END=0x4e28;PHYS=0x96c0_1e04
REFSHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
MNEMONICS=['r23=r0;r1:0=memd(r29+#0x0)','r18=memw(r0+##0x0)',
 'r19=extract(r18,#0x3,#0x0)','p0=tstbit(r18,#0x1c)','p1=cmp.eq(r19,r1)',
 'p2=and(p0,!p1)','if(p2.new)jump:nt0x96c02004','if(p1)jump:nt0x96c01e28',
 'p0=cmp.eq(r1,#0x0);if(!p0.new)jump:t0x96c02004']
ASM='''{ r23=r0; r1:0=memd(r29+#0) }
{ r18=memw(r0+#0) }
{ r19=extract(r18,#3,#0); p0=tstbit(r18,#28) }
{ p1=cmp.eq(r19,r1) }
{ p2=and(p0,!p1); if(p2.new) jump:nt label }
{ if(p1) jump:nt label2 }
{ p0=cmp.eq(r1,#0); if(!p0.new) jump:t label }
label2:
{ jumpr lr }
.space 472
label:
{ jumpr lr }
'''
def elf_text(b):
 src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
 spec=importlib.util.spec_from_file_location('readhexagon',src);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
 return m.parse_elf32_text(b)
def main():
 a=argparse.ArgumentParser(description=__doc__)
 for name in ['reference','rust-output','rz-asm','llvm-mc']:
  a.add_argument('--'+name,required=True,type=Path)
 args=a.parse_args()
 reference=args.reference.read_bytes()
 if hashlib.sha256(reference).hexdigest()!=REFSHA:raise ValueError('CDSP reference SHA256 mismatch')
 r=args.rust_output.read_bytes()
 if len(r)!=END-START or r!=reference[START:END]:raise ValueError('Rust output differs from CDSP reference')
 p=subprocess.run([str(args.rz_asm),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',r.hex()],capture_output=True,text=True,check=True)
 found=[]
 for line in p.stdout.splitlines():
  m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:found.append((int(m.group(1),16),''.join(m.group(2).lower().split())))
 if len(found)!=9:raise ValueError(f'Rizin decoded {len(found)} words, expected nine')
 for i,((vma,text),want) in enumerate(zip(found,MNEMONICS)):
  if vma!=PHYS+4*i or want not in text:
   raise ValueError(f'Decode mismatch at {vma:#x}: {text!r} expected {want!r}')
 with tempfile.TemporaryDirectory() as dirname:
  d=Path(dirname);src=d/'sample.s';obj=d/'sample.o';src.write_text('.text\n.global sample\nsample:\n'+ASM)
  subprocess.run([str(args.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(src)],capture_output=True,text=True,check=True)
  asm=elf_text(obj.read_bytes())[:END-START]
 if asm!=r:raise ValueError(f'LLVM6 independent assembler mismatch {asm.hex()} != {r.hex()}')
 print('CDSP worker decision: 36/36 source bytes exact, 9 Rizin V66 words, 9/9 LLVM6 exact words, no OEM code embedded')
if __name__=='__main__':main()
