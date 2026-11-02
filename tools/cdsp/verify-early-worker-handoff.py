#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Corroborate the 152-byte CDSP V66 worker handoff and callback cascade.

Byte identity to SHA256-pinned CDSP + independent Rizin V66 decoding of all
38 words; ordinary packet forms and signed/long control transfers are
independently assembled by LLVM6. Privileged registers and forced extended
CALL instruction variants remain source-encoder/reference/Rizin checked only.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,struct,subprocess,tempfile
START=0x5060;END=0x50f8;PHYS=0x96c02060
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'r1=add(r1,##0x10)','jump0x96c323c4',
 'r1=##0x0','r0=ssr','r0=insert(r1,#0x3,#0x10)','ssr=r0',
 'call0x96c029b0','immext(##0xfffff280)','call0x96c01318',
 'immext(##0x9f80)','call0x96c0c004',
 'r0=##0x2','immext(##0xfe024d00)','memw(r1=##0xfe024d00)=r0.new',
 'dccleana(r1)','call0x96c02418',
 'immext(##0xfffff2c0)','call0x96c01364',
 'immext(##0xad00)','call0x96c0cdb0',
 'immext(##0x9f40)','call0x96c0c004',
 'call0x96c02044',
 'r0=syscfg','r0=clrbit(r0,#0x4)','syscfg=r0',
 'r0=ssr','r1=##0x0','r0=insert(r1,#0x3,#0x10)','ssr=r0',
 'immext(##0x0)','r0=##0x24','immext(##0xfe000040)',
 'memw(r1=##0xfe000044)=r0.new','dccleana(r1)',
 'immext(##0x305c0)','call0x96c326d0',
 'jump0x96c020f4'
]
assert len(EXPECTED)==38

def assemble(mc:Path,asm:str)->bytes:
 with tempfile.TemporaryDirectory(prefix='cdsp39-') as directory:
  root=Path(directory)
  (root/'test.s').write_text('.text\n.global entry\nentry:\n'+asm+'\n')
  p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(root/'test.o'),str(root/'test.s')],capture_output=True,text=True)
  if p.returncode:raise ValueError('LLVM6 independent source rejected: '+p.stderr)
  helper=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
  spec=importlib.util.spec_from_file_location('cdsp_handoff_elf',helper);assert spec and spec.loader
  mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
  return mod.parse_elf32_text((root/'test.o').read_bytes())

def verify(actual_path:Path,reference:Path,decoder:Path,mc:Path):
 raw=reference.read_bytes();actual=actual_path.read_bytes()
 if hashlib.sha256(raw).hexdigest()!=SHA:raise ValueError('Wrong CDSP reference revision')
 if len(actual)!=END-START or actual!=raw[START:END]:raise ValueError('typed CDSP worker handoff bytes do not match reference')
 p=subprocess.run([str(decoder),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],capture_output=True,text=True,check=True)
 rows=[]
 for line in p.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:rows.append((int(m[1],16),''.join(m[2].lower().split())))
 if len(rows)!=len(EXPECTED):raise ValueError(f'Rizin decoded {len(rows)}, expected {len(EXPECTED)}')
 for i,((pc,meaning),exp) in enumerate(zip(rows,EXPECTED)):
  if pc!=PHYS+i*4 or exp not in meaning:
   raise ValueError(f'V66 decode mismatch at file {START+i*4:#x}: {meaning} vs {exp}')
 sample=assemble(mc,'''{ r1=add(r1,#16) }
{ r1=#0 }
{ r0=insert(r1,#3,#16) }
{ r0=#2; memw(r1=##0xfe024d00)=r0.new }
{ dccleana(r1) }
{ r0=clrbit(r0,#4) }
{ r0=##36; memw(r1=##0xfe000044)=r0.new }
{ dccleana(r1) }
end:
{ jump end }''')
 sample_words=list(struct.unpack('<'+'I'*(len(sample)//4),sample))
 matches=[(0,0),(2,1),(4,2),(11,3),(12,4),(13,5),(14,6),(24,7),
          (30,8),(31,9),(32,10),(33,11),(34,12),(37,13)]
 assert len(sample_words)==14
 for i,j in matches:
  got=struct.unpack_from('<I',actual,i*4)[0]
  if sample_words[j]!=got:raise ValueError(f'LLVM6 scalar sample differs at {START+i*4:#x}')
 confirmed=len(matches)
 # Explicit packet-offset check for the large forward branch in the helper.
 delta=0x96c323c4-0x96c02064
 jump=assemble(mc,'{ jump destination }\n.space '+str(delta-4)+'\ndestination:\n{ jumpr lr }')
 if jump[:4]!=actual[4:8]:raise ValueError('Long forward JUMP differs from LLVM6')
 confirmed+=1
 for i,target in [(6,0x96c029b0),(15,0x96c02418)]:
  source=PHYS+i*4;delta=target-source
  word=assemble(mc,'{ call destination }\n.space '+str(delta-4)+'\ndestination:\n{ jumpr lr }')[:4]
  if word!=actual[i*4:i*4+4]:raise ValueError(f'CALL differs at {START+i*4:#x}')
  confirmed+=1
 # Signed backward call: high sign bit differs from the forward-only
 # subset previously used in older branches.
 byte_delta=0x96c020b8-0x96c02044
 back=assemble(mc,'destination:\n{ jumpr lr }\n.space '+str(byte_delta-4)+'\n{ call destination }')
 if back[-4:]!=actual[22*4:23*4]:raise ValueError('Backwards signed CALL differs from LLVM6')
 confirmed+=1
 # A separately scheduled zero producer and insert instruction preserve
 # independent scalar checks on the second SSR-reset block.
 pair=assemble(mc,'{ r1=#0 }\n{ r0=insert(r1,#3,#16) }')
 if pair!=actual[27*4:29*4]:raise ValueError('SSR clear producer/insert differs from LLVM6')
 confirmed+=2
 print(f'CDSP worker handoff: {len(actual)} exact bytes; {len(rows)} Rizin decoded; {confirmed} exact LLVM6 independently assembled words; privileged and forced extended CALLs not all LLVM reproduced')

if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--rust-output',type=Path,required=True);p.add_argument('--reference',type=Path,required=True);p.add_argument('--rz-asm',type=Path,required=True);p.add_argument('--llvm-mc',type=Path,required=True)
 a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
