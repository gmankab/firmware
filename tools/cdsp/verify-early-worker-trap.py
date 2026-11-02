#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify fatal CDSP controller path with original, Rizin V66 and LLVM6."""
from pathlib import Path
import argparse,importlib.util,re,subprocess,tempfile
START=0x4cb4
END=0x4cd8
PHYS=0x96c0_1cb4
EXPECTED={
 0x4cb4:'immext(##0x30a00)',
 0x4cb8:'call0x96c326d0',
 0x4cbc:'immext(##0x5d80)',
 0x4cc0:'call0x96c07a44',
 0x4cc4:'r2=##-0x1',
 0x4cc8:'r3=##-0x1',
 0x4ccc:'r0=##0x18;r1=-1',
 0x4cd0:'trap0(#0x0)',
 0x4cd4:'jump0x96c01cd4',
}

def elf_text(b:bytes)->bytes:
 p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
 spec=importlib.util.spec_from_file_location('cdsp_worker_trap_elf',p)
 assert spec and spec.loader
 obj=importlib.util.module_from_spec(spec);spec.loader.exec_module(obj)
 return obj.parse_elf32_text(b)

def verify(reference:Path,generated:Path,asmtool:Path,llvm:Path)->None:
 ref=reference.read_bytes()[START:END];out=generated.read_bytes()
 if len(out)!=36 or out!=ref:raise ValueError('CDSP fatal supervisor source does not match pinned reference')
 x=subprocess.run([str(asmtool),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',out.hex()],
                  text=True,capture_output=True,check=True)
 observed={}
 for line in x.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:observed[int(m[1],16)]=''.join(m[2].split()).lower()
 if len(observed)!=9:raise ValueError(f'Rizin decoded {len(observed)} of 9 words')
 for at,want in EXPECTED.items():
  got=observed[PHYS+at-START]
  if want not in got:raise ValueError(f'wrong decoded op at {at:#x}: {got}; expected {want}')
 asm='''.text
.global sample
sample:
{ call ##0x30a1c }
{ call ##0x5d88 }
{ r2=#-1 }
{ r3=#-1 }
{ r0=#24; r1=#-1 }
{ trap0(#0) }
again:
{ jump again }
'''
 with tempfile.TemporaryDirectory(prefix='cdsp-worker-trap-') as t:
  source=Path(t)/'case.s';dest=Path(t)/'case.o';source.write_text(asm)
  subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                  '-o',str(dest),str(source)],check=True,capture_output=True,text=True)
  independent=elf_text(dest.read_bytes())
 if len(independent)!=len(out):raise ValueError('wrong LLVM6 sample word count')
 for i in range(0,len(out),4):
  actual=int.from_bytes(out[i:i+4],'little')
  llvmword=int.from_bytes(independent[i:i+4],'little')
  if i in (0x10,0x14):
   # Two signed -1 assignments share a single packet in reference;
   # LLVM6 emitted singleton packets in its minimal sample.
   actual &=~0xc000;llvmword &=~0xc000
  if actual!=llvmword:
   raise ValueError(f'LLVM6 independent mismatch file {START+i:#x}: {actual:#x}/{llvmword:#x}')
 print('CDSP fatal path: 36 exact bytes; 9 Rizin V66 semantic words; '
       '9 LLVM6 assembled words (two normalized packet parse markers).')

if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__)
 for arg in ('reference','rust-output','rz-asm','llvm-mc'):
  p.add_argument('--'+arg,type=Path,required=True)
 q=p.parse_args();verify(q.reference,q.rust_output,q.rz_asm,q.llvm_mc)
