#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon decoder and LLVM6 duplex checks for CDSP compare.

The region is 144 executable bytes with register-indexed physical memory,
paired duplex assignment/ADD, and conditional new-value branches. Rizin V66
decodes privileged physical operations. LLVM6 checks the two duplex fields
and mixed extended assignment/ADD compound independently from Rust.
"""
from pathlib import Path
import argparse,importlib.util,re,subprocess,tempfile,struct

OFFSET=0x4364
PHYSICAL=0x96c0_1364
SIZE=0x90
ASM='''.text
.global v66_duplex_samples
v66_duplex_samples:
{ r2=#11; r7=#0 }
{ r1=add(r1,#4); r3=add(r3,#4) }:endloop0
{ r6=##0xfe01b8e0; r3=add(r3,r5) }
'''
EXPECTED={
 0x4364:'immext(##0x0)',0x4380:'p0=cmp.gt(r0,r6);if(!p0.new)jump:nt0x96c013f0',
 0x43b8:'immext(##0xfe01b8c0)',0x43bc:'r6=##0xfe01b8e0;r3=add(r3,r5)',
 0x43c0:'loop0(0x96c013c8,r2)',0x43c4:'r2=##0xb;r7=##0x0',
 0x43d0:'r5=memw_phys(r3,r4)',0x43e0:'memw(r6++#0x4)=r3',
 0x43e4:'memw(r6++#0x4)=r5',0x43ec:'r1=add(r1,##0x4);r3=add(r3,##0x4)',
 0x43f0:'jumprlr',
}

def extract(elf:Path)->bytes:
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_compare_elf',path)
    assert spec and spec.loader
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.parse_elf32_text(elf.read_bytes())

def verify(llvm:Path,rz:Path,rust:Path,reference:Path|None):
    data=rust.read_bytes()
    if len(data)!=SIZE:raise ValueError(f'Expected {SIZE} rebuilt bytes, found {len(data)}')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],capture_output=True,text=True,check=True)
    statements={}
    for row in decoded.stdout.splitlines():
        m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+[?\\/\[|]\s+(.*)',row)
        if m:statements[int(m[1],16)]=''.join(m[2].lower().split())
    if len(statements)!=SIZE//4:raise ValueError(f'Only {len(statements)} of {SIZE//4} V66 words decode')
    for offset,label in EXPECTED.items():
        got=statements.get(PHYSICAL+offset-OFFSET,'')
        if label not in got:raise ValueError(f'CDSP compare instruction {offset:#x} unexpected: {got} (wanted {label})')
    with tempfile.TemporaryDirectory(prefix='cdsp-compare-llvm-') as dir:
        src=Path(dir)/'checks.s';obj=Path(dir)/'checks.o';src.write_text(ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(src)],capture_output=True,text=True,check=True)
        assembled=extract(obj)
    if len(assembled)!=20:raise ValueError('LLVM assembler unexpectedly changed packet sizing')
    pairs=[(0x43c4,0,4),(0x43ec,8,4),(0x43b8,12,8)]
    for dest,src,n in pairs:
        if data[dest-OFFSET:dest-OFFSET+n]!=assembled[src:src+n]:
            raise ValueError(f'Independent LLVM6 duplex encoding differs at {dest:#x}')
    if reference is not None and reference.read_bytes()[OFFSET:OFFSET+SIZE]!=data:
        raise ValueError('Reference firmware does not match rebuilt CDSP compare code')
    print(f'CDSP compare: {SIZE} independently generated and matched bytes, {SIZE//4} Rizin decoded instructions, three LLVM6 duplex comparisons; privileged physical memory decoder-only')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
