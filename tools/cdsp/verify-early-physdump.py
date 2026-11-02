#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check typed CDSP physical-memory snapshot and cache-publish functions.

Rizin V66 decodes the actual 30-word packet stream, including privileged
S54/physical-memory operations. LLVM6 independently assembles 15 standard
scalar forms but not privileged memw_phys/S54/new-value forms.
"""
from pathlib import Path
import argparse,importlib.util,re,subprocess,tempfile
START=0x4a38
PHYSICAL=0x96c0_1a38
SIZE=120
EXPECTED={
 0x4a38:'r23=s54',0x4a3c:'immext(##0xc0673240)',
 0x4a40:'r1=memw(r23+##-0x3f98cda0)',
 0x4a48:'r3=memw(r23+##-0x1f39980)',
 0x4a4c:'p0=cmp.gt(r3,##0x20)',
 0x4a50:'if(!p0.new)jump:nt0x96c01a80',
 0x4a54:'r1=add(r1,r23)',
 0x4a5c:'r0=memw(r23+##-0x1f398cc)',
 0x4a60:'r2=lsr(r3,#0x2)',
 0x4a64:'p0=cmp.eq(r0,#0x0);if(p0.new)jump:nt0x96c01a80',
 0x4a68:'loop0(0x96c01a70,r2)',
 0x4a70:'r4=lsr(r0,r2)',
 0x4a74:'r5=memw_phys(r0,r4)',
 0x4a7c:'memw(r1++#0x4)=r5',
 0x4a80:'jumprlr',
 0x4a84:'r23=s54',
 0x4a8c:'r22=add(r23,##-0x1fc6e38)',
 0x4a90:'r22=memw(r22+##0x0)',
 0x4a98:'r23=add(r22,##-0x1fdb300)',
 0x4aa0:'memw(r23+##0x0)=r22.new',
 0x4aa4:'dccleana(r23)',0x4aa8:'dcinva(r23)',0x4aac:'jumprlr',
}
ASM=''' .text
.global sample
sample:
{ p0=cmp.gt(r3,#32) }
{ r1=add(r1,r23) }
{ r2=lsr(r3,#2) }
{ loop0(loopstart,r2) }
{ r2=#11 }
loopstart:
{ r4=lsr(r0,r2) }
{ r0=add(r0,#4) }
{ memw(r1++#4)=r5 }:endloop0
{ jumpr lr }
{ r22=memw(r22+#0) }
{ r23=add(r22,##-0x1fdb300) }
{ r22=#4 }
{ dccleana(r23) }
{ dcinva(r23) }
'''
LLVM_WORDS={
    0x4a4c:0,0x4a54:4,0x4a60:8,0x4a68:12,0x4a6c:16,
    0x4a70:20,0x4a78:24,0x4a7c:32,0x4a80:36,
    0x4a90:40,0x4a94:44,0x4a98:48,0x4a9c:52,
    0x4aa4:56,0x4aa8:60,
}

def elf_text(obj:Path)->bytes:
    script=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_shared_elf',script)
    assert spec and spec.loader
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod.parse_elf32_text(obj.read_bytes())

def verify(llvm:Path,rz:Path,built:Path,reference:Path):
    out=built.read_bytes()
    if len(out)!=SIZE or reference.read_bytes()[START:START+SIZE]!=out:
        raise ValueError('CDSP physical dump / cache publish not byte-identical')
    proc=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',out.hex()],text=True,capture_output=True,check=True)
    parsed={}
    for line in proc.stdout.splitlines():
        m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+([?/\\\[|])\s+(.*)',line)
        if m:parsed[int(m[1],16)]=(''.join(m[3].lower().split()),m[2])
    if len(parsed)!=SIZE//4:raise ValueError(f'Rizin only decoded {len(parsed)} of {SIZE//4} words')
    for offset,want in EXPECTED.items():
        got=parsed[PHYSICAL+offset-START][0]
        if want not in got:raise ValueError(f'Rizin V66 mismatch at {offset:#x}: {got}; expected {want}')
    with tempfile.TemporaryDirectory(prefix='cdsp-physdump-') as tmp:
        source=Path(tmp)/'demo.s';obj=Path(tmp)/'demo.o';source.write_text(ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(source)],capture_output=True,text=True,check=True)
        independent=elf_text(obj)
    for off,other in LLVM_WORDS.items():
        actual=int.from_bytes(out[off-START:off-START+4],'little')
        alt=int.from_bytes(independent[other:other+4],'little')
        if actual&~0xc000!=alt&~0xc000:
            raise ValueError(f'LLVM6 did not reproduce operand encoding at {off:#x}: {actual:#x} vs {alt:#x}')
    print('CDSP physdump: 120 exact bytes, 30 Rizin V66 words and 15 independent LLVM6 scalar encodings; privileged physical reads / S54 decoder-only.')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for flag in ('llvm-mc','rz-asm','rust-output','reference'):p.add_argument('--'+flag,type=Path,required=True)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
