#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Whole CDSP V66 format classifier final: independently check every byte,
semantic decoder instruction and LLVM6-assembled operand encoding.

Hexagon instruction packet scheduling differs in LLVM6 and the OEM binary,
so packets containing predicates or extended assignments are mapped by
semantic word, while the full source-vs-original check retains parse bits.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess

START=0x5680;END=0x5880;PC=0x96c0_2680
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
# Original format-mask field classification, in runtime/physical address order.
FORMATS=[
 (0x5680,29,7,29,0,0x5720,False),
 (0x56a4,8,255,24,0x3f000000,0x5720,False),
 (0x56cc,6,1,31,0x80000000,0x5720,False),
 (0x56f4,2,1,31,0x80000000,0x5720,False),
 (0x5720,29,7,29,0x20000000,0x57c4,False),
 (0x5748,8,255,24,0x1f000000,0x57c4,False),
 (0x5770,6,1,31,0x80000000,0x57c4,False),
 (0x5798,2,1,31,0x80000000,0x57c4,True),
 (0x57c4,29,7,29,0x40000000,0x5870,False),
 (0x57ec,8,255,24,0x3f000000,0x5870,False),
 (0x5814,6,1,31,0x80000000,0x5870,False),
 (0x583c,2,1,31,0x80000000,0x5870,True),
]
ERRORS=[(0x5864,-25,0x5878)]
JUMPS=[(0x571c,0x5864),(0x57c0,0x5864)]

def shared():
 path=Path(__file__).with_name('verify-agent20-09-format-guard-tail.py')
 spec=importlib.util.spec_from_file_location('format_tail',path)
 if not spec or not spec.loader:raise ValueError('Missing prior independent Hexagon test infrastructure')
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
 return mod

def expected_and_assembly_cases():
 x=shared(); expected={};cases=[]
 def emit(address,text,source=None,positions=None):
  if address in expected:raise ValueError(f'Address reused {address:#x}')
  expected[address]=text
  if source is not None:cases.append((source,[address] if positions is None else positions))
 for start,right,mask,left,fmt,target,taken in FORMATS:
  a=start
  def add(text,asm=None):
   nonlocal a
   emit(a,text,asm);a+=4
  add('R9 = ##0x0','{r9=#0}')
  add('R11 = R0','{r11=r0}')
  add(f'R11 = lsr(R11,#0x{right:x})',f'{{r11=lsr(r11,#{right})}}')
  add(f'R11 = and(R11,##0x{mask:x})',f'{{r11=and(r11,#{mask})}}')
  add(f'R11 = asl(R11,#0x{left:x})',f'{{r11=asl(r11,#{left})}}')
  if fmt==0:
   add('P0 = cmp.eq(R11,##0x0)','{p0=cmp.eq(r11,#0)}')
  else:
   extaddr=a
   add(f'immext(##0x{fmt&~63:x})')
   add(f'P0 = cmp.eq(R11,##{x.signed_hex(fmt)})')
   cases.append((f'{{p0=cmp.eq(r11,##0x{fmt:x})}}',[extaddr,extaddr+4]))
  add('if (P0) R9 = ##0x1','{if(p0)r9=#1}')
  cmppos=a;add('P0 = cmp.eq(R9,##0x1)')
  branchtype='t' if taken else 'nt'
  add(f'if (!P0.new) jump:{branchtype} 0x{0x96c00000+target-0x3000:x}')
  asm=f'{{p0=cmp.eq(r9,#1)\n if(!p0.new) jump:{branchtype} target}}\n.space {target-cmppos-8}\ntarget:\n{{nop}}'
  cases.append((asm,[cmppos+4,cmppos]))
 for start,target in JUMPS:
  emit(start,f'jump 0x{0x96c00000+target-0x3000:x}',
       f'{{jump target}}\n.space {target-start-4}\ntarget:\n{{nop}}')
 for start,val,target in ERRORS:
  u=val&0xffffffff
  emit(start,f'immext(##0x{u&~63:x})')
  emit(start+4,f'R0 = ##{x.signed_hex(u)}')
  emit(start+8,f'jump 0x{0x96c00000+target-0x3000:x}')
  cases.append((f'{{r0=##{val}\n jump target}}\n.space {target-start-12}\ntarget:\n{{nop}}',[start+8,start,start+4]))
 # The other error arm (-1) and the common original return.
 emit(0x5870,'immext(##0xffffffc0)')
 emit(0x5874,'R0 = ##-0x1')
 cases.append(('{r0=##-1}',[0x5870,0x5874]))
 emit(0x5878,'LR:FP = deallocframe(FP):raw','{deallocframe}')
 emit(0x587c,'jumpr LR','{jumpr lr}')
 assert len(expected)==128,len(expected)
 assert sorted(expected)==list(range(START,END,4))
 return expected,cases

def verify(generated:Path,reference:Path,rz:Path,mc:Path):
 original=reference.read_bytes()
 if hashlib.sha256(original).hexdigest()!=SHA:raise ValueError('Wrong pinned CDSP firmware')
 bytes_=generated.read_bytes()
 if bytes_!=original[START:END] or len(bytes_)!=END-START:raise ValueError('Source-built classifier tail differs from reference')
 expected,cases=expected_and_assembly_cases()
 out=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',bytes_.hex()],
     capture_output=True,text=True,check=True).stdout
 decoder=[]
 for line in out.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:decoder.append((int(m[1],16),''.join(m[2].lower().split()).split('<endloop')[0]))
 if len(decoder)!=128:raise ValueError(f'Rizin decoded {len(decoder)} of 128 words')
 asm=shared();compiled={};verified=set()
 for addr,semantic in decoder:
  file_address=START+addr-PC
  want=asm.sem(expected[file_address])
  if semantic!=want:raise ValueError(f'Rizin at {file_address:#x}: {semantic}, expected {want}')
 for source,positions in cases:
  if source not in compiled:compiled[source]=asm.assemble(mc,source)
  b=compiled[source]
  if len(b)<len(positions)*4:raise ValueError('LLVM emitted too few instructions for '+source)
  for j,addr in enumerate(positions):
   emitted=int.from_bytes(b[4*j:4*j+4],'little')&~0xc000
   reference_word=int.from_bytes(bytes_[addr-START:addr-START+4],'little')&~0xc000
   if emitted!=reference_word:raise ValueError(f'LLVM word at {addr:#x}: {emitted:#x}, expected {reference_word:#x} source={source}')
   verified.add(addr)
 if len(verified)!=128:raise ValueError(f'Only {len(verified)}/128 LLVM6 operand fields verified: missing {[hex(x) for x in sorted(set(expected)-verified)]}')
 print(f'CDSP codec format classifier final: {len(bytes_)} byte-exact source-built bytes, {len(decoder)} Rizin V66 semantic decodes, {len(verified)} LLVM6 independently assembled word operands, original return present')

if __name__=='__main__':
 p=argparse.ArgumentParser()
 for n in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+n,required=True,type=Path)
 a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
