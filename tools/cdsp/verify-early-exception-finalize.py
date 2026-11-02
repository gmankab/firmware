#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify CDSP Hexagon SYSCFG controller-finalization source. Rizin checks
privileged register access and packet layout; LLVM6 independently assembles
portable scalar, extender, insertion and new-value predicate forms.
"""
import argparse,importlib.util,re,subprocess,tempfile
from pathlib import Path
START=0x48e8;PHYSICAL=0x96c0_18e8;SIZE=116
EXPECTED={
0x48e8:'immext(##0xfe0258c0)',0x48ec:'r0=add(r23,##-0x1fda71c)',
0x48f0:'r1=##0x5',0x48f4:'memw(r0+##0x0)=r1',0x48f8:'call0x96c01a08',
0x48fc:'immext(##0xfe0258c0)',0x4900:'r0=add(r23,##-0x1fda71c)',
0x4904:'r1=##0x6',0x4908:'memw(r0+##0x0)=r1',0x490c:'call0x96c01a84',
0x4910:'r23=s54',0x4914:'immext(##0xfe0258c0)',
0x4918:'r0=add(r23,##-0x1fda71c)',0x491c:'r1=##0x7',
0x4920:'memw(r0+##0x0)=r1',0x4924:'immext(##0x0)',
0x4928:'r0=##0x0',0x492c:'if(cmp.eq(r0.new,#0x0))jump:nt0x96c01ad4',
0x4930:'r0=syscfg',0x4934:'immext(##0xfe026cc0)',
0x4938:'r1=memw(r23+##-0x1fd9334)',0x493c:'r0=insert(r1,#0x3,#0x10)',
0x4940:'syscfg=r0',0x4944:'isync',0x4948:'immext(##0x80)',
0x494c:'r0=##0x80',0x4950:'immext(##0x1ffe680)',
0x4954:'call0x98c00000',0x4958:'jump0x96c01ad4',
}
ASM='''.text
.global sample
sample:
{ r0=add(r23,##-0x1fda71c)
  r1=#5 }
{ memw(r0+#0)=r1 }
{ call callback }
{ r0=add(r23,##-0x1fda71c)
  r1=#6 }
{ memw(r0+#0)=r1 }
{ call callback }
{ r0=add(r23,##-0x1fda71c)
  r1=#7 }
{ memw(r0+#0)=r1 }
{ r0=##0
  if(cmp.eq(r0.new,#0)) jump:nt done }
{ r1=memw(r23+##-0x1fd9334) }
{ r0=insert(r1,#3,#16) }
{ isync }
{ r0=##128 }
{ call callback }
{ jump done }
callback:
{ nop }
done:
{ nop }
'''
def elf_text(path:Path)->bytes:
 helper=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
 spec=importlib.util.spec_from_file_location('cdsp_finalizer_elf',helper)
 assert spec and spec.loader
 module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
 return module.parse_elf32_text(path.read_bytes())
def verify(llvm:Path,rz:Path,emitted:Path,reference:Path):
 b=emitted.read_bytes()
 if len(b)!=SIZE or reference.read_bytes()[START:START+SIZE]!=b:
  raise ValueError('CDSP generated controller finalization does not match original MBN')
 p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',b.hex()],capture_output=True,text=True,check=True)
 decoded={}
 for line in p.stdout.splitlines():
  m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+[?\\/\[|]\s+(.*)',line)
  if m:decoded[int(m[1],16)]=''.join(m[2].lower().split())
 if len(decoded)!=SIZE//4:raise ValueError(f'Only {len(decoded)} of {SIZE//4} V66 instructions decoded')
 for addr,expected in EXPECTED.items():
  got=decoded.get(PHYSICAL+(addr-START),'')
  if expected not in got:raise ValueError(f'Malformed decoded V66 semantic at {addr:#x}: {got}, wanted {expected}')
 with tempfile.TemporaryDirectory(prefix='hexagon-finalize-') as tmp:
  s=Path(tmp)/'llvm.s';o=Path(tmp)/'llvm.o';s.write_text(ASM)
  subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(o),str(s)],capture_output=True,text=True,check=True)
  independently_built=elf_text(o)
 matching=[(0x48e8,0),(0x48ec,4),(0x48f0,8),(0x48f4,12),
   (0x48fc,20),(0x4900,24),(0x4904,28),(0x4908,32),
   (0x4914,40),(0x4918,44),(0x491c,48),(0x4920,52),
   (0x4924,56),(0x4928,60),(0x492c,64),
   (0x4934,68),(0x4938,72),(0x493c,76),
   (0x4944,80),(0x4948,84),(0x494c,88)]
 for offset,mapped in matching:
  actual=int.from_bytes(b[offset-START:offset-START+4],'little')
  independently=int.from_bytes(independently_built[mapped:mapped+4],'little')
  strip=0xc000
  if offset==0x492c:strip|=0x1ff
  if actual&~strip!=independently&~strip:
   raise ValueError(f'Hexagon LLVM6 independent operand mismatch at {offset:#x}: {actual:#x}!={independently:#x}')
 for offset,mapped in [(0x48f8,16),(0x490c,36),(0x4954,92),(0x4958,96)]:
  actual=int.from_bytes(b[offset-START:offset-START+4],'little')
  independently=int.from_bytes(independently_built[mapped:mapped+4],'little')
  if (actual&0xfe00_0000)!=(independently&0xfe00_0000):
   raise ValueError(f'Hexagon LLVM6 call/jump family mismatch at {offset:#x}')
 print('CDSP exception finalization: 116/116 byte-exact typed instructions, 29 Rizin V66 decoded, 21 independent LLVM6 operand comparisons and four branch-family checks; SYSCFG/S54 privileged only decoder-verified.')
def main():
 ap=argparse.ArgumentParser(description=__doc__)
 for name in ('llvm-mc','rz-asm','rust-output','reference'):ap.add_argument('--'+name,type=Path,required=True)
 a=ap.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
