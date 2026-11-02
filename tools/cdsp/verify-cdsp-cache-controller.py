#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 and Rizin checks for 80-byte CDSP CFGBASE setup.

LLVM independently assembles the 16 unprivileged words, with one packet parse
marker difference (LLVM emits separate ASL packet). Rizin V66 confirms the
four remaining privileged words and all packet boundaries. This is *not* an
independent LLVM assembly of the entire V66 routine.
"""
from __future__ import annotations
import argparse
import importlib.util
from pathlib import Path
import re
import subprocess
import tempfile
import struct

FILE_OFFSET=0x3f24
SIZE=80
PHYS=0x96c00f24
LLVM_ASM='''.text
.global cache_cfg
cache_cfg:
{ r21=asl(r21,#5) }
{ r22=##0x4c }
{ r23=asl(r23,#16)
  memw(r25 + ##-0x1fe92a4)=r23.new }
{ r22=##0x38 }
{ r23=asl(r23,#16)
  memw(r25 + ##-0x1fe92a0)=r23.new }
{ r22=##0x3c }
{ r23=asl(r23,#10)
  memw(r25 + ##-0x1fe929c)=r23.new }
'''
LLVM_TO_RUST=(1,2,3,5,6,7,8,9,11,12,13,14,15,17,18,19)
PRIVILEGED=(0,4,10,16)
EXPECTED=(
 'r21=cfgbase', 'r21=asl(r21,#0x5)', 'immext(##0x40)', 'r22=##0x4c',
 'r23=memw_phys(r22,r21)', 'r23=asl(r23,#0x10)',
 'immext(##0xfe016d40)', 'memw(r25+##-0x1fe92a4)=r23.new',
 'immext(##0x0)', 'r22=##0x38', 'r23=memw_phys(r22,r21)',
 'r23=asl(r23,#0x10)', 'immext(##0xfe016d40)',
 'memw(r25+##-0x1fe92a0)=r23.new', 'immext(##0x0)', 'r22=##0x3c',
 'r23=memw_phys(r22,r21)', 'r23=asl(r23,#0xa)',
 'immext(##0xfe016d40)', 'memw(r25+##-0x1fe929c)=r23.new',
)

def helper(module_filename:str,alias:str):
    spec=importlib.util.spec_from_file_location(alias,Path(__file__).with_name(module_filename))
    assert spec and spec.loader
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def run(llvm_mc:Path,rz_asm:Path,out_file:Path,reference:Path|None):
    output=out_file.read_bytes()
    if len(output)!=SIZE:raise ValueError('CFGBASE fragment must be exactly 80 bytes')
    elf=helper('verify-cdsp-hexagon-entry.py','cdsp_elf_parser')
    with tempfile.TemporaryDirectory(prefix='cdsp-cfgbase-') as t:
        asm=Path(t)/'cfgbase.s'; obj=Path(t)/'cfgbase.o'
        asm.write_text(LLVM_ASM)
        subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj','-o',str(obj),str(asm)],check=True)
        llvm=elf.parse_elf32_text(obj.read_bytes())
    if len(llvm)!=len(LLVM_TO_RUST)*4:raise ValueError('LLVM fixture changed length')
    llvmwords=struct.unpack('<16I',llvm)
    rustwords=struct.unpack('<20I',output)
    for i,index in enumerate(LLVM_TO_RUST):
        if index==1:
            # LLVM schedules the first ASL as a one-instruction packet;
            # firmware schedules it with the following extended constant.
            if (llvmwords[i]&~0xc000)!=(rustwords[index]&~0xc000) \
                    or rustwords[index]&0xc000!=0x4000:
                raise ValueError('first ASL instruction or parse marker differs')
        elif llvmwords[i]!=rustwords[index]:
            raise ValueError(f'LLVM disagrees on ordinary instruction at file {FILE_OFFSET+4*index:#x}')
    decoder=helper('verify-cdsp-tlb.py','cdsp_rizin_parser')
    decoded=decoder.decode_rizin(rz_asm,output,physical_address=PHYS)
    if tuple(decoded)!=EXPECTED:
        for i,(x,y) in enumerate(zip(decoded,EXPECTED)):
            if x!=y:raise ValueError(f'Rizin {i}: {x!r}, expected {y!r}')
        raise ValueError(f'Rizin decoded {len(decoded)} words, expected {len(EXPECTED)}')
    if reference is not None:
        orig=reference.read_bytes()
        if orig[FILE_OFFSET:FILE_OFFSET+SIZE]!=output:
            raise ValueError('Generated CFGBASE prefix differs from CDSP reference')
    print('CDSP CFGBASE setup: 80/80 bytes match; LLVM assembles 16 nonprivileged words with one parse-marker difference, Rizin confirms all 20 V66 words')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--rz-asm',required=True,type=Path)
    p.add_argument('--rust-output',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    a=p.parse_args()
    run(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
