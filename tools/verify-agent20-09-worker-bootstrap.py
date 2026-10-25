#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent CDSP worker-init decoder / LLVM6 ordinary instruction checks.

Both branches of the 92-byte source-encoded worker routine are compared
against the pinned original bytes and Rizin V66. Privileged SGP0/SYSCFG
forms and the far direct controller jump are decoder-checked only.
"""
from pathlib import Path
import argparse,importlib.util,re,subprocess,tempfile
START=0x4bc0;PHYS=0x96c01bc0;SIZE=92
ASM=''' .text
.global sample
sample:
{ allocframe(#8) }
{ memd(sp+#0)=r1:0 }
{ r1=clrbit(r1,#4) }
{ r0=memw(r0+#0) }
{ r0=memw(r0+##0x100) }
{ r0=memw(r0+##0x10) }
{ r1:0=memd(sp+#0) }
{ deallocframe }
{ r0=r18 }
{ jump after }
after:
{ jumpr lr }
'''
# Corresponding offset of the first word of each LLVM generated semantic unit;
# the independent branch at 0x4c0c has a different sample displacement.
SCALARS={0x4bc0:0,0x4bcc:4,0x4bd8:8,0x4be4:12,
         0x4be8:16,0x4bec:20,0x4bf4:24,0x4bf8:28,
         0x4c00:32,0x4c04:36,0x4c08:40}
EXPECTED={
 0x4bc0:'allocframe(sp,#0x8)',
 0x4bc4:'immext(##0x58c0)',0x4bc8:'call0x96c074b8',
 0x4bcc:'memd(sp+##0x0)=r1:0',
 0x4bd0:'p0=cmp.eq(r0,#0x0);if(!p0.new)jump:nt0x96c01be0',
 0x4bd4:'r1=syscfg',0x4bd8:'r1=clrbit(r1,#0x4)',0x4bdc:'syscfg=r1',
 0x4be0:'r0=sgp0',0x4be4:'r0=memw(r0+##0x0)',
 0x4be8:'immext(##0x100)',0x4bec:'r0=memw(r0+##0x100)',
 0x4bf0:'if(cmp.eq(r0.new,#0x0))jump:t0x96c01c10',
 0x4bf4:'immext(##0x0)',0x4bf8:'r0=memw(r0+##0x10)',
 0x4bfc:'if(cmp.eq(r0.new,#0x0))jump:t0x96c01c10',
 0x4c00:'r1:0=memd(sp+##0x0)',0x4c04:'lr:fp=deallocframe(fp)',
 0x4c08:'r0=r18',0x4c0c:'jump0x96c01c1c',
 0x4c10:'r1:0=memd(sp+##0x0)',0x4c14:'lr:fp=deallocframe(fp)',
 0x4c18:'jump0x96c047a8',
}
def elf_text(p:Path)->bytes:
 spec=importlib.util.spec_from_file_location('hexagon_elf_reader',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
 assert spec and spec.loader
 x=importlib.util.module_from_spec(spec);spec.loader.exec_module(x)
 return x.parse_elf32_text(p.read_bytes())
def verify(llvm:Path,rz:Path,generated:Path,ref:Path):
 b=generated.read_bytes();original=ref.read_bytes()
 if len(b)!=SIZE or original[START:START+SIZE]!=b:
  raise ValueError('CDSP worker fragment not reference exact')
 d=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',b.hex()],capture_output=True,text=True,check=True)
 decoded={}
 for line in d.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+([?/\\\[|])\s+(.*)',line)
  if m:decoded[int(m[1],16)]=(''.join(m[3].lower().split()),m[2])
 if len(decoded)!=SIZE//4:raise ValueError(f'Rizin decoded {len(decoded)} of {SIZE//4}')
 for addr,want in EXPECTED.items():
  got=decoded[PHYS+addr-START][0]
  if want not in got:raise ValueError(f'wrong Rizin semantic {addr:#x} got {got}, expected {want}')
 with tempfile.TemporaryDirectory(prefix='cdsp-worker-bootstrap-') as d:
  p=Path(d)/'sample.s';o=Path(d)/'sample.o';p.write_text(ASM)
  subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(o),str(p)],check=True,capture_output=True,text=True)
  independent=elf_text(o)
 for addr,index in SCALARS.items():
  actual=int.from_bytes(b[addr-START:addr-START+4],'little')
  other=int.from_bytes(independent[index:index+4],'little')
  if (actual & ~0xc000)!=(other & ~0xc000):
   raise ValueError(f'LLVM6 ordinary encoding differs at file {addr:#x}: {actual:#x} vs {other:#x}')
 if original[0x4bb8:0x4bc0]!=b'\0'*8:
  raise ValueError('linker alignment immediately before worker not zero')
 print('CDSP worker bootstrap: 92 reference-exact typed bytes, 23 Rizin V66 semantic checks, 11 LLVM6 independently assembled ordinary encodings; privileged and far controller jump Rizin-only; 8 alignment zeros excluded.')
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__)
 for arg in ('llvm-mc','rz-asm','rust-output','reference'):p.add_argument('--'+arg,type=Path,required=True)
 a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
