#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V66 compilation of typed CDSP width dispatcher.
LLVM6 fully matches the first 21 instructions (84 bytes), including
predicated byte/half/word/double stores and a duplex return, and independently
reassembles 17 additional tail instruction operands plus the conditional
branch opcode family. The source contains no OEM byte literals. Rizin V66 checks semantics at original physical addresses.
"""
from pathlib import Path
import argparse,importlib.util,subprocess,tempfile,re
START=0x4b1c;SIZE=156;PREFIX_BYTES=84;PHYS=0x96c01b1c
ALIGNMENT_END=0x4bc0
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
TAIL_ASM=''' .text
.global sample
sample:
{ p0=cmp.gtu(r1,r0)
  if(!p0.new) jump:nt done }
{ r3=or(r1,r0) }
loop0start:
{ r2=ct0(r3)
  r4=sub(#0,r3)
  r7=add(r1,#-32)
  if(!cmp.gtu(r2.new,#4)) jump:nt alt }
alt:
{ p0=cmp.eq(r0,r7)
  if(!p0.new) jump:t alt }
{ r0=add(r0,#32)
  dczeroa(r0) }
done:
{ jumpr lr }
{ r5=add(##0xfe001b30,mpyi(r2,#12))
  p1=tstbit(r1,r2)
  r3=and(r3,r4) }
{ p0=tstbit(r0,r2)
  jumpr r5
  r1=and(r1,~r3)
  r3:2=combine(#0,#0) }
'''
TAIL_EXPECTED={
    0x4b70:'p0=cmp.gtu(r1,r0);if(!p0.new)jump:nt0x96c01b94',
    0x4b74:'r3=or(r1,r0)',0x4b78:'r2=ct0(r3)',
    0x4b7c:'r4=sub(##0x0,r3)',0x4b80:'r7=add(r1,##-0x20)',
    0x4b84:'if(!cmp.gtu(r2.new,#0x4))jump:nt0x96c01b98',
    0x4b88:'p0=cmp.eq(r0,r7);if(!p0.new)jump:t0x96c01b88',
    0x4b8c:'r0=add(r0,##0x20)',0x4b90:'dczeroa(r0)',
    0x4b94:'jumprlr',0x4b98:'immext(##0xfe001b00)',
    0x4b9c:'r5=add(##0xfe001b30,mpyi(r2,#0xc))',
    0x4ba0:'p1=tstbit(r1,r2)',0x4ba4:'r3=and(r3,r4)',
    0x4ba8:'p0=tstbit(r0,r2)',0x4bac:'jumprr5',
    0x4bb0:'r1=and(r1,~r3)',0x4bb4:'r3:2=combine(##0x0,#0x0)',
}
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
        tail_source=Path(folder)/'tail.s';tail_obj=Path(folder)/'tail.o'
        tail_source.write_text(TAIL_ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(tail_obj),str(tail_source)],capture_output=True,text=True,check=True)
        tail_independent=elf_text(tail_obj)
    if independent[:PREFIX_BYTES]!=b[:PREFIX_BYTES]:
        diffs=[x for x in range(PREFIX_BYTES) if independent[x]!=b[x]]
        raise ValueError(f'21 LLVM6 independent instruction words differ at offsets {diffs[:10]}')
    for i in range(0,SIZE-PREFIX_BYTES,4):
        actual=int.from_bytes(b[PREFIX_BYTES+i:PREFIX_BYTES+i+4],'little')
        assembly=int.from_bytes(tail_independent[i:i+4],'little')
        if i==0x14: # Same new-value GTU branch family, different source-assembly forward label.
            mask=~0xc0ff
        else:
            mask=~0xc000  # LLVM packet scheduler may change parse bits.
        if (actual&mask)!=(assembly&mask):
            raise ValueError(f'LLVM6 tail opcode/operand mismatch at file {0x4b70+i:#x}: {actual:#x}!={assembly:#x}')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',b.hex()],capture_output=True,text=True,check=True)
    rows={}
    for line in decoded.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+([?/\\\[|])\s+(.*)',line)
        if m:rows[int(m[1],16)]=(''.join(m[3].lower().split()),m[2])
    if len(rows)!=SIZE//4:raise ValueError(f'Only {len(rows)} of {SIZE//4} Rizin V66 decoded')
    for offset,want in EXPECTED.items():
        got=rows[PHYS+(offset-START)][0]
        if want not in got:raise ValueError(f'Unexpected Rizin semantic at {offset:#x}: {got}, expected {want}')
    for offset,want in TAIL_EXPECTED.items():
        got=rows[PHYS+(offset-START)][0]
        if want not in got:raise ValueError(f'Unexpected Rizin tail at {offset:#x}: {got}, expected {want}')
    if reference.read_bytes()[START+SIZE:ALIGNMENT_END]!=b'\x00'*8:
        raise ValueError('CDSP eight-byte structural alignment after memory dispatch is nonzero')
    print('CDSP memory dispatch: 156 source-exact bytes (39 Rizin V66 words); LLVM6 reproduces all 84 prefix bytes and all 18 tail opcode families (17 complete operands and one branch-family comparison); eight linker zeros excluded.')
if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for k in ('llvm-mc','rz-asm','rust-output','reference'):p.add_argument('--'+k,type=Path,required=True)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
