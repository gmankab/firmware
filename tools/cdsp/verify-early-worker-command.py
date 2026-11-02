#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V66 controller command packet check (19 words)."""
import argparse,hashlib,importlib.util,re,struct,subprocess,tempfile
from pathlib import Path
START=0x4e28;END=0x4e74;PHYS=0x96c0_1e28
REFSHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
SEM=['r2=##0x2','r1=insert(r2,#0x8,#0x10)','r1=setbit(r1,#0x1e)',
     'r1=setbit(r1,#0x1d)','r0=add(r0,##0x8)',
     'r1=memw(r23+#0x0);memw(r0+#0x0)=r1','call0x96c32464',
     'immext(##0x0)','r0=##0x20','immext(##0xfe000040)',
     'memw(r1=##0xfe000044)=r0.new','dccleana(r1)',
     'call0x96c02184','call0x96c02268','call0x96c029b0',
     'immext(##0xfffff480)','call0x96c01318','immext(##0xa180)','call0x96c0c004']
assert len(SEM)==19
ASM='''{ r2=#2 }
{ r1=insert(r2,#8,#16) }
{ r1=setbit(r1,#30) }
{ r1=setbit(r1,#29) }
{ r0=add(r0,#8); r1=memw(r23+#0); memw(r0+#0)=r1 }
{ r0=##32; memw(r1=##0xfe000044)=r0.new }
{ dccleana(r1) }
'''
def compile_asm(mc,src):
 with tempfile.TemporaryDirectory(prefix='agent09-cdsp-llvm-') as d:
  p=Path(d);s=p/'source.s';o=p/'source.o';s.write_text('.text\n.global _start\n_start:\n'+src)
  subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(o),str(s)],capture_output=True,text=True,check=True)
  elf=Path(__file__).with_name('verify-cdsp-hexagon-entry.py');sp=importlib.util.spec_from_file_location('reader',elf);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
  return m.parse_elf32_text(o.read_bytes())
def run(a):
 original=a.reference.read_bytes();out=a.rust_output.read_bytes()
 if hashlib.sha256(original).hexdigest()!=REFSHA:raise ValueError('CDSP reference SHA256 mismatch')
 if len(out)!=END-START or out!=original[START:END]:raise ValueError('Rust source bytes differ from pinned CDSP')
 rz=subprocess.run([str(a.rz_asm),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',out.hex()],capture_output=True,text=True,check=True)
 lines=[]
 for l in rz.stdout.splitlines():
  m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',l)
  if m:lines.append((int(m.group(1),16),''.join(m.group(2).lower().split())))
 if len(lines)!=19:raise ValueError(f'Rizin decoded only {len(lines)}/19 instructions')
 for i,((addr,text),want) in enumerate(zip(lines,SEM)):
  if addr!=PHYS+4*i or want not in text:raise ValueError(f'Rizin word at {addr:#x}: {text} missing {want}')
 llvm=compile_asm(a.llvm_mc,ASM)
 for idx,source_index in [(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),
                          (6,7),(7,8),(8,9),(9,10),(10,11)]:
  if llvm[idx*4:idx*4+4]!=out[source_index*4:source_index*4+4]:
   raise ValueError(f'LLVM6 byte mismatch for command word {source_index}')
 matched=11
 # Independently assemble the four ordinary CALLs using exact packet-relative distances.
 for addr,target in [(0x96c0_1e40,0x96c3_2464),(0x96c0_1e58,0x96c0_2184),
                     (0x96c0_1e5c,0x96c0_2268),(0x96c0_1e60,0x96c0_29b0)]:
  delta=target-addr
  word=compile_asm(a.llvm_mc,'{ call target }\n.space '+str(delta-4)+'\ntarget:\n{ jumpr lr }\n')[:4]
  idx=(addr-PHYS)//4
  if word!=out[4*idx:4*idx+4]:raise ValueError(f'LLVM6 CALL encoding differs at {addr:#x}')
  matched+=1
 print(f'CDSP worker command {len(out)} exact source bytes; 19/19 Rizin V66 words; {matched} LLVM6 exact word validations, 4 extended CALL words independently decoded/reference-checked')
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__)
 for name in ('reference','rust-output','rz-asm','llvm-mc'):p.add_argument('--'+name,required=True,type=Path)
 run(p.parse_args())
