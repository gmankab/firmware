#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V66 compilation of typed CDSP width dispatcher.
LLVM6 assembles all 21 instructions, including predicated byte/half/word/double
memory stores and a duplex return. The source is self-contained and consumes no
OEM byte literals. Rizin V66 checks semantics at original physical addresses.
"""
from pathlib import Path
import argparse,importlib.util,subprocess,tempfile,re
START=0x4b1c;SIZE=84;PHYS=0x96c01b1c
ASM=''' .text
.global sample
sample:
{ p0=cmp.gtu(r5,#3); if(p0.new) jump:nt failure }
{ r7=memw(r5<<#2+##0xfe026dc8) }
{ jumpr r7 }
failure:
{ r0=#-1; jumpr lr }
{ jump resume
  if(p0) memb(r0++#1)=r2
  if(p1) memb(r1+#0)=r2 }
{ jump resume
  if(p0) memh(r0++#2)=r2
  if(p1) memh(r1+#0)=r2 }
{ jump resume
  if(p0) memw(r0++#4)=r2
  if(p1) memw(r1+#0)=r2 }
{ jump resume
  if(p0) memd(r0++#8)=r3:2
  if(p1) memd(r1+#0)=r3:2 }
{ if(p0) memd(r0++#16)=r3:2
  if(p0) memd(r0+#8)=r3:2 }
{ if(p1) memd(r1+#0)=r3:2
  if(p1) memd(r1+#8)=r3:2 }
resume:
{ jumpr lr }
'''
EXPECTED={
 0x4b1c:'p0=cmp.gtu(r5,#0x3);if(p0.new)jump:nt0x96c01b2c',
 0x4b20:'immext(##0xfe026dc0)',0x4b24:'r7=memw(r5<<#0x2+##0xfe026dc8)',
 0x4b28:'jumprr7',0x4b2c:'r0=-1;jumprr31',
 0x4b34:'if(p0)memb(r0++#0x1)=r2',
 0x4b38:'if(p1)memb(r1+##0x0)=r2',
 0x4b40:'if(p0)memh(r0++#0x2)=r2',
 0x4b44:'if(p1)memh(r1+##0x0)=r2',
 0x4b4c:'if(p0)memw(r0++#0x4)=r2',
 0x4b50:'if(p1)memw(r1+##0x0)=r2',
 0x4b58:'if(p0)memd(r0++#0x8)=r3:2',
 0x4b5c:'if(p1)memd(r1+##0x0)=r3:2',
 0x4b60:'if(p0)memd(r0++#0x10)=r3:2',
 0x4b64:'if(p0)memd(r0+##0x8)=r3:2',
 0x4b68:'if(p1)memd(r1+##0x0)=r3:2',
 0x4b6c:'if(p1)memd(r1+##0x8)=r3:2',
}
def elf_text(p:Path)->bytes:
    spec=importlib.util.spec_from_file_location('hexagon_elf_reader',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
    assert spec and spec.loader
    obj=importlib.util.module_from_spec(spec);spec.loader.exec_module(obj)
    return obj.parse_elf32_text(p.read_bytes())
def verify(llvm:Path,rz:Path,built:Path,reference:Path):
    b=built.read_bytes()
    if len(b)!=SIZE or b!=reference.read_bytes()[START:START+SIZE]:
        raise ValueError('CDSP width dispatch was not reference exact')
    with tempfile.TemporaryDirectory(prefix='cdsp-width-') as folder:
        source=Path(folder)/'source.s';obj=Path(folder)/'result.o';source.write_text(ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(source)],capture_output=True,text=True,check=True)
        independent=elf_text(obj)
    if independent[:SIZE]!=b:
        diffs=[x for x in range(SIZE) if independent[x]!=b[x]]
        raise ValueError(f'21 LLVM6 independent instruction words differ at offsets {diffs[:10]}')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',b.hex()],capture_output=True,text=True,check=True)
    rows={}
    for line in decoded.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+([?/\\\[|])\s+(.*)',line)
        if m:rows[int(m[1],16)]=(''.join(m[3].lower().split()),m[2])
    if len(rows)!=SIZE//4:raise ValueError(f'Only {len(rows)} of {SIZE//4} Rizin V66 decoded')
    for offset,want in EXPECTED.items():
        got=rows[PHYS+(offset-START)][0]
        if want not in got:raise ValueError(f'Unexpected Rizin semantic at {offset:#x}: {got}, expected {want}')
    print('CDSP memory dispatch: 84 source bytes exact; all 21 Rizin V66 words decoded; independent LLVM6 assembly reproduces ALL 84 bytes including duplex and four data widths.')
if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for k in ('llvm-mc','rz-asm','rust-output','reference'):p.add_argument('--'+k,type=Path,required=True)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
