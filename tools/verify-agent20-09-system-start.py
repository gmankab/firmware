#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent V66 decoding and LLVM6 scalar checks for CDSP system init.

The first 104 bytes at file 0x4500 are executable, but are DISJOINT from
0x41e0..0x4418 by 232 structurally zero-aligned bytes. This is not a
continuous 800-byte source-reconstructed code region.
"""
import argparse
import importlib.util
from pathlib import Path
import re
import subprocess
import tempfile

START=0x4500
PHYSICAL=0x96c01500
LEN=0x68
EXPECTED={
 0x4500:'r23=syscfg',
 0x4504:'pmucfg=r23',
 0x4508:'r0=##0x28',
 0x450c:'syscfg=r0',
 0x4510:'isync',
 0x4514:'r23=s54',
 0x4518:'immext(##0xfe025400)',
 0x451c:'r0=add(r23,##-0x1fdabe0)',
 0x4520:'r25=add(r0,##0x4)',
 0x4524:'immext(##0x0)',
 0x4528:'r24=##0x25',
 0x452c:'r0=memw(r25+##0x0)',
 0x4530:'immext(##0x0)',
 0x4534:'p0=cmp.eq(r0,##0x25)',
 0x4538:'if(p0.new)jump:nt0x96c01ad4',
 0x453c:'immext(##0xfe0258c0)',
 0x4540:'r0=add(r23,##-0x1fda71c)',
 0x4544:'memw(r25+##0x0)=r24',
 0x4548:'r24=##0x1',
 0x454c:'memw(r0+##0x0)=r24.new',
 0x4550:'immext(##0xfe025400)',
 0x4554:'r0=add(r23,##-0x1fdabe0)',
 0x4558:'r0=add(r0,##0x0)',
 0x455c:'r24=memw(r0+##0x0)',
 0x4560:'r24=setbit(r24,#0x1d)',
 0x4564:'memw(r0+##0x0)=r24.new',
}
BRANCH_ASSEMBLY='''.text
.global branch_probe
branch_probe:
{ p0=cmp.eq(r0,##37)
  if (p0.new) jump:nt far_away }
.space 0x598
far_away:
{ nop }
'''
ASSEMBLY='''.text
.global samples
samples:
{ r25=add(r0,#4) }
{ r0=memw(r25+#0) }
{ r0=#0x28 }
{ r24=#0x25 }
'''

def parse_elf(obj:Path)->bytes:
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('agent9_system_elf',path)
    assert spec and spec.loader
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.parse_elf32_text(obj.read_bytes())

def verify(llvm:Path,rz:Path,rust:Path,reference:Path|None):
    blob=rust.read_bytes()
    if len(blob)!=LEN:raise ValueError(f'Expected {LEN} authored bytes, got {len(blob)}')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',blob.hex()],capture_output=True,text=True,check=True)
    texts={}
    for row in decoded.stdout.splitlines():
        m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+([?\\/\[|])\s+(.*)',row)
        if m:texts[int(m[1],16)]=''.join(m[3].lower().split())
    if len(texts)!=LEN//4:raise ValueError(f'Rizin V66 decoded only {len(texts)} words')
    for offset,pattern in EXPECTED.items():
        txt=texts.get(PHYSICAL+offset-START,'')
        if pattern not in txt:raise ValueError(f'V66 {offset:#x} expected {pattern}, got {txt}')
    with tempfile.TemporaryDirectory(prefix='cdsp-system-init-') as path:
        src=Path(path)/'x.s';obj=Path(path)/'x.o'
        src.write_text(ASSEMBLY)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(src)],capture_output=True,text=True,check=True)
        built=parse_elf(obj)
    for i,addr in enumerate((0x4520,0x452c,0x4508,0x4528)):
        actual=int.from_bytes(blob[addr-START:addr-START+4],'little')
        independent=int.from_bytes(built[i*4:i*4+4],'little')
        if (actual&~0xc000)!=(independent&~0xc000):raise ValueError(f'LLVM6 differs at {addr:#x}')
    # LLVM V60 schedules the same packet in different instruction order:
    # branch, extender, predicate, while V66 OEM stores extender, predicate,
    # branch. The independently decoded target and *operand fields* must match.
    with tempfile.TemporaryDirectory(prefix='cdsp-system-branch-') as path:
        src=Path(path)/'branch.s';obj=Path(path)/'branch.o'
        src.write_text(BRANCH_ASSEMBLY)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(src)],capture_output=True,text=True,check=True)
        llvm_branch=parse_elf(obj)
    for ll,oem_addr in ((0,0x4538),(4,0x4530),(8,0x4534)):
        encoded=int.from_bytes(llvm_branch[ll:ll+4],'little')
        reference_word=int.from_bytes(blob[oem_addr-START:oem_addr-START+4],'little')
        if (encoded&~0xc000)!=(reference_word&~0xc000):
            raise ValueError(f'Independent LLVM branch packet operand fields differ at {oem_addr:#x}')
    if reference:
        original=reference.read_bytes()
        if blob!=original[START:START+LEN]:raise ValueError('Pinned system init differs from independently encoded bytes')
        if original[0x4418:START]!=bytes(232):raise ValueError('Linker-alignment gap is not empty as expected')
    print('CDSP system init: 104/104 typed bytes exact, 26 V66 decoded instructions, 4 LLVM6 scalar forms plus independent 3-word branch packet; 232 adjacent ELF zero bytes excluded')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--rz-asm',required=True,type=Path)
    p.add_argument('--rust-output',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
