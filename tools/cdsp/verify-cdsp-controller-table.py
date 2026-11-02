#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Reassemble the CDSP 32-byte controller-table clear loop using LLVM6.

The fixture emits original semantics with explicit addresses/operands, not a
raw opcode fixture. An optional CDSP reference checks the typed Rust output.
"""
from __future__ import annotations
import argparse
import importlib.util
from pathlib import Path
import subprocess
import tempfile

OFFSET=0x3fcc
SIZE=32
LLVM_ASM='''.text
.global cdsp_clear_controller_table
cdsp_clear_controller_table:
{ r21=memw(r25+##-0x1fe92a0) }
{ r19=memw(r25+##-0x1fe929c) }
{ r23:22=combine(#0,#0) }
again:
{ memd(r21++#8)=r23:22 }
{ r19=add(r19,#-8)
 if(cmp.gtu(r19.new,#0)) jump:t again }
'''
EXPECTED=(
 'immext(##0xfe016d40)','r21=memw(r25+##-0x1fe92a0)',
 'immext(##0xfe016d40)','r19=memw(r25+##-0x1fe929c)',
 'r23:22=combine(##0x0,#0x0)','memd(r21++#0x8)=r23:22',
 'r19=add(r19,##-0x8)',
 'if(cmp.gtu(r19.new,#0x0))jump:t0x96c00fe0',
)

def verify(llvm:Path,rz:Path,output:Path,reference:Path|None):
    data=output.read_bytes()
    if len(data)!=SIZE:raise ValueError('CDSP table clear must emit exactly 32 bytes')
    with tempfile.TemporaryDirectory(prefix='cdsp-table-clear-') as folder:
        asm=Path(folder)/'clear.s';obj=Path(folder)/'clear.o'
        asm.write_text(LLVM_ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj','-o',str(obj),str(asm)],check=True)
        path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        s=importlib.util.spec_from_file_location('elf_cdsp',path);assert s and s.loader
        mod=importlib.util.module_from_spec(s);s.loader.exec_module(mod)
        actual=mod.parse_elf32_text(obj.read_bytes())
    if actual!=data:
        raise ValueError('LLVM independent reassembly differs from Rust generated table-clear bytes')
    path=Path(__file__).with_name('verify-cdsp-tlb.py')
    s=importlib.util.spec_from_file_location('v66_cdsp',path);assert s and s.loader
    mod=importlib.util.module_from_spec(s);s.loader.exec_module(mod)
    decoded=mod.decode_rizin(rz,data,physical_address=0x96c00fcc)
    if tuple(decoded)!=EXPECTED:raise ValueError(f'Rizin V66 decoded table-clear instructions differ: {decoded}')
    if reference is not None:
        orig=reference.read_bytes()
        if orig[OFFSET:OFFSET+SIZE]!=data:raise ValueError('CDSP reference table-clear bytes differ')
    print('CDSP 32/32 reference bytes independently assembled by LLVM6; all eight instructions decoded with Rizin')

def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--llvm-mc',required=True,type=Path)
    ap.add_argument('--rz-asm',required=True,type=Path)
    ap.add_argument('--rust-output',required=True,type=Path)
    ap.add_argument('--reference',type=Path)
    a=ap.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
