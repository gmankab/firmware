#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6/Hexagon V66 check of the worker callback register writes."""
import argparse,hashlib,importlib.util,re,subprocess,tempfile
from pathlib import Path
START=0x4e74;END=0x4ec8;PHYS=0x96c0_1e74
HASH='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=['r0=##0x2','immext(##0xfe024d00)','memw(r1=##0xfe024d00)=r0.new',
 'dccleana(r1)','call0x96c023dc','call0x96c02418','immext(##0x0)',
 'r0=##0x1e','immext(##0xfe000040)','memw(r1=##0xfe000044)=r0.new',
 'dccleana(r1)','immext(##0xfffff4c0)','call0x96c01364',
 'immext(##0x580)','call0x96c02458','immext(##0xaf00)','call0x96c0cdb0',
 'immext(##0xa140)','call0x96c0c004','call0x96c02044','call0x96c3736c']
assert len(EXPECTED)==21
ASM='''{ r0=#2; memw(r1=##0xfe024d00)=r0.new }
{ dccleana(r1) }
{ r0=##30; memw(r1=##0xfe000044)=r0.new }
{ dccleana(r1) }
'''
def assemble(tool,src):
 with tempfile.TemporaryDirectory(prefix='cdsp33-') as d:
  p=Path(d);(p/'a.s').write_text('.text\n.global a\na:\n'+src)
  subprocess.run([str(tool),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(p/'a.o'),str(p/'a.s')],capture_output=True,text=True,check=True)
  origin=Path(__file__).with_name('verify-cdsp-hexagon-entry.py');sp=importlib.util.spec_from_file_location('cdsp33_elf',origin);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
  return m.parse_elf32_text((p/'a.o').read_bytes())
def check(a):
 reference=a.reference.read_bytes();output=a.rust_output.read_bytes()
 if hashlib.sha256(reference).hexdigest()!=HASH:raise ValueError('pinned CDSP reference SHA mismatch')
 if len(output)!=END-START or output!=reference[START:END]:raise ValueError('source bytes mismatch CDSP reference')
 rz=subprocess.run([str(a.rz_asm),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',output.hex()],text=True,capture_output=True,check=True)
 rows=[]
 for l in rz.stdout.splitlines():
  m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+[?/\\\[|]\s+(.*)',l)
  if m:rows.append((int(m[1],16),''.join(m[2].lower().split())))
 if len(rows)!=21:raise ValueError(f'V66 decoded {len(rows)} of 21 words')
 for i,((addr,sem),word) in enumerate(zip(rows,EXPECTED)):
  if addr!=PHYS+4*i or word not in sem:raise ValueError(f'Hexagon unexpected {addr:#x}: {sem!r} expected {word!r}')
 llvm=assemble(a.llvm_mc,ASM)
 if len(llvm)!=9*4:raise ValueError('LLVM status packet emitted unexpected number of bytes')
 for i,offset in list(enumerate(range(0,4)))+list(enumerate(range(6,11),4)):
  if llvm[i*4:(i+1)*4]!=output[offset*4:(offset+1)*4]:
   raise ValueError(f'LLVM exact status encoding differs at reference {START+offset*4:#x}')
 total=9
 for here,target in [(0x96c01e84,0x96c023dc),(0x96c01e88,0x96c02418),
                     (0x96c01ec0,0x96c02044),(0x96c01ec4,0x96c3736c)]:
  delta=target-here
  if delta<=4:raise ValueError('test expects forward direct call')
  llvm_call=assemble(a.llvm_mc,'{ call finish }\n.space '+str(delta-4)+'\nfinish:\n{ jumpr lr }\n')[:4]
  off=here-PHYS
  if llvm_call!=output[off:off+4]:raise ValueError(f'LLVM call mismatch at {here:#x}')
  total+=1
 print(f'CDSP callback status: {END-START} source-authored byte-exact, 21/21 V66 Rizin decoded, {total} independently exact LLVM6 words; extended CALLs reference/Rizin-verified')
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__)
 for arg in ('reference','rust-output','rz-asm','llvm-mc'):p.add_argument('--'+arg,required=True,type=Path)
 check(p.parse_args())
