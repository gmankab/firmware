#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check source-encoded CDSP exception transition against V66 and LLVM6.

Independent LLVM6 reorders the new-predicate comparison/branch within its
packet; only opcode/operand fields are compared, not synthetic branch targets.
"""
import argparse, importlib.util, re, subprocess, tempfile
from pathlib import Path

FILE_OFFSET=0x4860
PHYSICAL=0x96c0_1860
SIZE=0x44
EXPECTED={
  0x4860:'immext(##0xfe0258c0)',
  0x4864:'r0=add(r23,##-0x1fda71c)',
  0x4868:'r1=##0x3',
  0x486c:'memw(r0+##0x0)=r1',
  0x4870:'immext(##0xfe0391c0)',
  0x4874:'r1=add(r23,##-0x1fc6e38)',
  0x4878:'r1=memw(r1+##0x0)',
  0x487c:'immext(##0xfe024d00)',
  0x4880:'r24=add(r1,##-0x1fdb300)',
  0x4884:'r24=memw(r24+##0x0)',
  0x4888:'p0=cmp.eq(r24,##0x0)',
  0x488c:'if(!p0.new)jump:nt0x96c01924',
  0x4890:'call0x96c01adc',
  0x4894:'immext(##0xfe0258c0)',
  0x4898:'r0=add(r23,##-0x1fda71c)',
  0x489c:'r1=##0x4',
  0x48a0:'memw(r0+##0x0)=r1',
}
ASM='''.text
.global sample
sample:
{ r0=add(r23,##-0x1fda71c)
  r1=#3 }
{ memw(r0+#0)=r1 }
{ r1=add(r23,##-0x1fc6e38) }
{ r1=memw(r1+#0) }
{ r24=add(r1,##-0x1fdb300) }
{ r24=memw(r24+#0) }
{ p0=cmp.eq(r24,#0)
  if (!p0.new) jump:nt target }
{ call callback }
{ r0=add(r23,##-0x1fda71c)
  r1=#4 }
{ memw(r0+#0)=r1 }
target:
{ nop }
callback:
{ nop }
'''

def elf_text(path:Path)->bytes:
 helper=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
 spec=importlib.util.spec_from_file_location('agent20_transition_elf',helper)
 assert spec and spec.loader
 module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
 return module.parse_elf32_text(path.read_bytes())

def verify(llvm:Path,rz:Path,generated:Path,reference:Path):
 b=generated.read_bytes()
 if len(b)!=SIZE or reference.read_bytes()[FILE_OFFSET:FILE_OFFSET+SIZE]!=b:
  raise ValueError('CDSP exception transition generated/reference byte mismatch')
 p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',b.hex()],capture_output=True,text=True,check=True)
 rows={}
 for line in p.stdout.splitlines():
  m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+[?\\/\[|]\s+(.*)',line)
  if m:rows[int(m[1],16)]=''.join(m[2].lower().split())
 if len(rows)!=SIZE//4:raise ValueError(f'Rizin decoded {len(rows)}, expected 17 instructions')
 for address,semantics in EXPECTED.items():
  actual=rows.get(PHYSICAL+(address-FILE_OFFSET),'')
  if semantics not in actual:
   raise ValueError(f'Wrong V66 operand at {address:#x}: got {actual!r}, expected {semantics!r}')
 with tempfile.TemporaryDirectory(prefix='hexagon-transition-') as tmp:
  source=Path(tmp)/'samples.s';obj=Path(tmp)/'samples.o';source.write_text(ASM)
  subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(source)],capture_output=True,text=True,check=True)
  coded=elf_text(obj)
 for ref_addr,llvm_addr in [(0x4860,0),(0x4864,4),(0x4868,8),(0x486c,12),
   (0x4870,16),(0x4874,20),(0x4878,24),(0x487c,28),
   (0x4880,32),(0x4884,36),(0x4888,44),
   (0x4894,52),(0x4898,56),(0x489c,60),(0x48a0,64)]:
  direct=int.from_bytes(b[ref_addr-FILE_OFFSET:ref_addr-FILE_OFFSET+4],'little')
  independent=int.from_bytes(coded[llvm_addr:llvm_addr+4],'little')
  if direct&~0xc000!=independent&~0xc000:
   raise ValueError(f'LLVM6 register/immediate encoding mismatch at {ref_addr:#x}')
 # Synthetic labels differ, but the independent branch/call opcode fields
 # must match before accounting for parse markers and displacement bits.
 branch=int.from_bytes(b[0x488c-FILE_OFFSET:0x4890-FILE_OFFSET],'little')
 llvm_branch=int.from_bytes(coded[40:44],'little')
 if (branch&~0xc1ff)!=(llvm_branch&~0xc1ff):
  raise ValueError('Conditional P0.new branch opcode family mismatched LLVM6')
 call=int.from_bytes(b[0x4890-FILE_OFFSET:0x4894-FILE_OFFSET],'little')
 llvm_call=int.from_bytes(coded[48:52],'little')
 if call&0xfe000000!=llvm_call&0xfe000000:
  raise ValueError('Controller callback call opcode family mismatched LLVM6')
 print('CDSP exception transition: 68 reference-exact typed bytes; 17 Rizin V66 semantic decodes; 15 independent LLVM6 encodings, plus branch/call opcode family checks.')

def main():
 ap=argparse.ArgumentParser(description=__doc__)
 for opt in ('llvm-mc','rz-asm','rust-output','reference'):
  ap.add_argument('--'+opt,required=True,type=Path)
 a=ap.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
