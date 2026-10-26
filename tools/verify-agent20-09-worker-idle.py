#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 idle-controller continuation: reference + Rizin + LLVM6.

LLVM6 canonicalizes three explicit 32-bit GP-absolute loads into two-word
(short-address) packets, so independent LLVM checks apply to the remaining
10 source instructions and the opcode families of the three conditional
branches. The original three extended GP packets are all Rizin-correlated.
"""
from pathlib import Path
import argparse,importlib.util,re,subprocess,tempfile
START=0x4cd8;END=0x4d24;PHYS=0x96c0_1cd8
EXPECTED={
 0x4cd8:'immext(##0xfe0ac080)',0x4cdc:'r0=memw(gp+##0xfe0ac0a0)',
 0x4ce0:'if(cmp.eq(r0.new,#0x0))jump:t0x96c01d20',
 0x4ce4:'immext(##0xfe0a5dc0)',0x4ce8:'r0=memw(gp+##0xfe0a5ddc)',
 0x4cec:'if(cmp.eq(r0.new,#0x0))jump:t0x96c01d20',
 0x4cf0:'immext(##0xfe037ac0)',0x4cf4:'r0=memw(gp+##0xfe037ad0)',
 0x4cf8:'if(cmp.eq(r0.new,#0x0))jump:nt0x96c01d20',
 0x4cfc:'memw(r0+#0x24)=#0;memw(r0+#0x20)=#0',
 0x4d00:'memw(r0+#0x10)=#0;memw(r0+#0x14)=#0',
 0x4d04:'memw(r0+#0x40)=##0x0',0x4d08:'memw(r0+#0x3c)=##0x0',
 0x4d0c:'memw(r0+#0x5c)=##0x0',0x4d10:'memw(r0+#0x58)=##0x0',
 0x4d14:'memw(r0+#0x78)=##0x0',0x4d18:'memw(r0+#0x74)=##0x0',
 0x4d1c:'memw(r0+##0x4)=clrbit(#0x15)',0x4d20:'jump0x96c01d20',
}

def extract_llvm(obj:Path)->bytes:
 src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
 spec=importlib.util.spec_from_file_location('cdsp_idle_elf_reader',src)
 assert spec and spec.loader
 m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
 return m.parse_elf32_text(obj.read_bytes())

def llvm_tail(compiler:Path,data:bytes)->int:
 asm=''' .text
.global sample
sample:
{ memw(r0+#36)=#0; memw(r0+#32)=#0 }
{ memw(r0+#16)=#0; memw(r0+#20)=#0 }
{ memw(r0+#64)=#0; memw(r0+#60)=#0 }
{ memw(r0+#92)=#0; memw(r0+#88)=#0 }
{ memw(r0+#120)=#0; memw(r0+#116)=#0 }
{ memw(r0+#4)=clrbit(#21) }
loop:
{ jump loop }
'''
 with tempfile.TemporaryDirectory(prefix='cdsp-idle-llvm-') as t:
  s=Path(t)/'check.s';o=Path(t)/'check.o';s.write_text(asm)
  subprocess.run([str(compiler),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                  '-o',str(o),str(s)],check=True,capture_output=True,text=True)
  independently_assembled=extract_llvm(o)
 expected=data[0x4cfc-START:]
 if independently_assembled!=expected:
  raise ValueError(f'LLVM6 memory-zero tail mismatch: {independently_assembled.hex()}/{expected.hex()}')
 return len(expected)//4

def rizin_decode(tool:Path,data:bytes)->int:
 out=subprocess.run([str(tool),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',data.hex()],
                    check=True,text=True,capture_output=True)
 decoded={}
 for line in out.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:decoded[int(m[1],16)]=''.join(m[2].split()).lower()
 if len(decoded)!=19:raise ValueError(f'Rizin decoded {len(decoded)} of 19 words')
 for file,want in EXPECTED.items():
  actual=decoded[PHYS+file-START]
  if want not in actual:
   raise ValueError(f'Rizin semantic mismatch at CDSP file {file:#x}: {actual} not {want}')
 return len(decoded)

def verify(source:Path,rebuilt:Path,rz:Path,llvm:Path):
 actual=rebuilt.read_bytes();original=source.read_bytes()[START:END]
 if len(actual)!=76 or actual!=original:
  raise ValueError('typed worker idle bytes differ from pinned original')
 print('CDSP idle:',len(actual),'exact bytes;',rizin_decode(rz,actual),'Rizin V66 semantic words;',
       llvm_tail(llvm,actual),'independently LLVM6-assembled zeroing/return words;',
       'nine extended GP load/branch words Rizin-only')

if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__)
 for k in ('reference','rust-output','rz-asm','llvm-mc'):
  p.add_argument('--'+k,type=Path,required=True)
 q=p.parse_args();verify(q.reference,q.rust_output,q.rz_asm,q.llvm_mc)
