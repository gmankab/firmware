#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon physical-copy routine independent instruction/packet verification.
LLVM6 independently assembles the variable-right-shift and postincrement
store, but cannot assemble privileged `memw_phys`, which Rizin V66 decodes.
"""
import argparse
import importlib.util
from pathlib import Path
import re
import subprocess
import tempfile

FILE_OFFSET=0x4318
PHYSICAL=0x96c01318
EXPECTED_LENGTH=0x4c
ASSEMBLY='''.text
.global smoke
smoke:
{ r4=lsr(r0,r2) }
{ memw(r1++#4)=r5 }
'''
EXPECTED={0x4318:'immext(##0xfe0a5e40)',0x4348:'loop0(0x96c01350,r2)',
          0x4350:'r4=lsr(r0,r2)',0x4354:'r5=memw_phys(r0,r4)',
          0x435c:'memw(r1++#0x4)=r5',0x4360:'jumprlr'}

def verify(llvm_mc:Path,rz_asm:Path,built:Path,reference:Path|None):
    data=built.read_bytes()
    if len(data)!=EXPECTED_LENGTH:raise ValueError('CDSP physical-copy length differs')
    rz=subprocess.run([str(rz_asm),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],capture_output=True,text=True,check=True)
    lines={}
    for line in rz.stdout.splitlines():
        match=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+([?\\/\[|])\s+(.*)',line)
        if match:lines[int(match[1],16)]=(''.join(match[3].lower().split()),match[2])
    if len(lines)!=EXPECTED_LENGTH//4:raise ValueError(f'Rizin V66 decoded {len(lines)} of {EXPECTED_LENGTH//4} words')
    for file_offset,wanted in EXPECTED.items():
        text,_=lines[PHYSICAL+file_offset-FILE_OFFSET]
        if wanted not in text:raise ValueError(f'Verification at {file_offset:#x} expected {wanted}, got {text}')
    # Hardware-loop end is the previous pointer increment, not the store.
    if lines[PHYSICAL+0x4358-FILE_OFFSET][1]!='/':
        raise ValueError('unexpected Hexagon loop-end packet ownership')
    with tempfile.TemporaryDirectory(prefix='cdsp-physical-copy-llvm-') as tmp:
        asm=Path(tmp)/'check.s';obj=Path(tmp)/'check.o';asm.write_text(ASSEMBLY)
        subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(asm)],check=True)
        parser=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        sp=importlib.util.spec_from_file_location('cdsp_physcopy_parse',parser)
        assert sp and sp.loader
        mod=importlib.util.module_from_spec(sp);sp.loader.exec_module(mod)
        generated=mod.parse_elf32_text(obj.read_bytes())
    if len(generated)!=8 or generated[:4]!=data[0x4350-FILE_OFFSET:0x4354-FILE_OFFSET] or generated[4:8]!=data[0x435c-FILE_OFFSET:0x4360-FILE_OFFSET]:
        raise ValueError('LLVM6 independently encoded scalar opcodes differ')
    if reference is not None and reference.read_bytes()[FILE_OFFSET:FILE_OFFSET+EXPECTED_LENGTH]!=data:
        raise ValueError('pinned CDSP reference does not match typed physical-copy code')
    print('CDSP physical copy: 76/76 reference-matching bytes, 19 Rizin-decoded V66 words, 2 LLVM6 independently encoded scalar instructions; privileged MEMW_PHYS only decoder corroborated')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
