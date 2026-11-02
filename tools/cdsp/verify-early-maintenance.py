#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify typed CDSP STOP controller wrappers and TLB snapshot/clean helper.

Rizin V66 checks the full instruction/packet stream including privileged
S54, TLB reads and stop. LLVM6 reproduces ordinary packet operands separately;
unsupported privileged forms are not falsely claimed independently assembled.
"""
from pathlib import Path
import argparse,importlib.util,re,subprocess,tempfile
START=0x4ab0
PHYSICAL=0x96c0_1ab0
SIZE=108
EXPECTED={
 0x4ab0:'call0x96c01a38',0x4ab4:'call0x96c01a84',
 0x4ab8:'r0=##0x0',0x4abc:'jump0x96c01ad4',
 0x4ac0:'call0x96c01a38',0x4ac4:'call0x96c01a08',0x4ac8:'call0x96c01a84',
 0x4acc:'r0=##0x0',0x4ad0:'jump0x96c01ad4',0x4ad4:'stop(r0)',
 0x4ad8:'jump0x96c01ad4',0x4adc:'r23=s54',0x4ae0:'r0=##0x80',
 0x4ae4:'immext(##0xfe024900)',0x4ae8:'r1=add(r23,##-0x1fdb700)',
 0x4aec:'r4=lsr(r0,#0x2)',0x4af0:'loop0(0x96c01af8,r0)',
 0x4af4:'r0=##0x0',0x4af8:'r3:2=tlbr(r0)',
 0x4afc:'r0=add(r0,##0x1)',0x4b00:'memd(r1++#0x8)=r3:2',
 0x4b04:'immext(##0xfe024900)',0x4b08:'r1=add(r23,##-0x1fdb700)',
 0x4b0c:'loop0(0x96c01b10,r4)',0x4b10:'r1=add(r1,##0x20)',
 0x4b14:'dccleana(r1)',0x4b18:'jumprlr',
}
ASM='''.text
.global sample
sample:
{ call next }
{ r0=#0 }
{ jump next }
next:
{ r0=#128 }
{ r4=lsr(r0,#2) }
{ loop0(loopstart,r0) }
{ r0=#0 }
loopstart:
{ r0=add(r0,#1) }
{ memd(r1++#8)=r3:2 }:endloop0
{ r1=add(r1,#32) }
{ dccleana(r1) }
{ jumpr lr }
'''
LLVM_MAP={0x4ab8:4,0x4ae0:12,0x4aec:16,0x4af0:20,0x4af4:24,
          0x4afc:28,0x4b00:36,0x4b10:40,0x4b14:44,0x4b18:48}
def elf_text(path:Path)->bytes:
    loader=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf_for_maintenance',loader)
    assert spec and spec.loader
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod.parse_elf32_text(path.read_bytes())
def verify(llvm:Path,rz:Path,built:Path,reference:Path):
    out=built.read_bytes()
    if len(out)!=SIZE or reference.read_bytes()[START:START+SIZE]!=out:
        raise ValueError('CDSP maintenance output differs from original reference')
    d=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',out.hex()],capture_output=True,text=True,check=True)
    lines={}
    for line in d.stdout.splitlines():
        m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+([?/\\\[|])\s+(.*)',line)
        if m:lines[int(m[1],16)]=(''.join(m[3].lower().split()),m[2])
    if len(lines)!=SIZE//4:raise ValueError(f'Rizin decoded {len(lines)}/{SIZE//4} instructions')
    for addr,expected in EXPECTED.items():
        got=lines[PHYSICAL+addr-START][0]
        if expected not in got:raise ValueError(f'Rizin semantic difference at {addr:#x}: {got} not {expected}')
    with tempfile.TemporaryDirectory(prefix='cdsp-maintenance-') as t:
        a=Path(t)/'sample.s';o=Path(t)/'sample.o';a.write_text(ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(o),str(a)],check=True,capture_output=True,text=True)
        independent=elf_text(o)
    for addr,mapped in LLVM_MAP.items():
        actual=int.from_bytes(out[addr-START:addr-START+4],'little')
        other=int.from_bytes(independent[mapped:mapped+4],'little')
        if actual&~0xc000!=other&~0xc000:
            raise ValueError(f'LLVM scalar mismatch at {addr:#x}: {actual:#x} vs {other:#x}')
    print('CDSP maintenance: 108/108 source exact; 27 Rizin V66 decoded; 10 independent LLVM6 ordinary word encodings; privileged S54, TLB read, stop Rizin-only.')
if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for k in ('llvm-mc','rz-asm','rust-output','reference'):p.add_argument('--'+k,type=Path,required=True)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
