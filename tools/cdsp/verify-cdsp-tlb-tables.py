#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently verify two Hexagon table-walk TLB invalidation loops.

LLVM 6 assembles the ordinary V60 math, memory, compare/branch and return
words, substituting harmless `nop` for the V66-only `tlbw`. It does *not*
independently assemble the latter; Rizin independently decodes the original
16 words with their V66 privilege operands. All 64 emitted bytes are also
checked against the supplied private firmware when provided.
"""
from __future__ import annotations
import argparse
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import struct

START=0x3e78
LENGTH=64
PHYS=0x96c00e78
TLBW_INDICES=(5,12)
LLVM_ASM='''.text
.global cdsp_tlb_tables
cdsp_tlb_tables:
{
 r1=add(r25,##-0x1f39958)
 r0=#0
}
loop1:
{ r3:2=memd(r1++#8) }
{
 p0=cmp.eq(r2,#0)
 if(p0.new) jump:t done1
}
{ nop }
{ r0=add(r0,#1) }
{ jump loop1 }
done1:
{ r1=add(r25,##-0x1f39918) }
loop2:
{ r3:2=memd(r1++#8) }
{
 p0=cmp.eq(r2,#0)
 if(p0.new) jump:t done2
}
{ nop }
{ r0=add(r0,#1) }
{ jump loop2 }
done2:
{ jumpr lr }
'''
EXPECTED=(
 'immext(##0xfe0c6680)', 'r1=add(r25,##-0x1f39958)',
 'r0=##0x0', 'r3:2=memd(r1++#0x8)',
 'p0=cmp.eq(r2,#0x0);if(p0.new)jump:t0x96c00e98',
 'tlbw(r3:2,r0)', 'r0=add(r0,##0x1)', 'jump0x96c00e84',
 'immext(##0xfe0c66c0)', 'r1=add(r25,##-0x1f39918)',
 'r3:2=memd(r1++#0x8)',
 'p0=cmp.eq(r2,#0x0);if(p0.new)jump:t0x96c00eb4',
 'tlbw(r3:2,r0)', 'r0=add(r0,##0x1)',
 'jump0x96c00ea0', 'jumprlr'
)

def load_helper(name:str,module_name:str):
    path=Path(__file__).with_name(name)
    spec=importlib.util.spec_from_file_location(module_name,path)
    if spec is None or spec.loader is None:raise ValueError(f'missing {name}')
    obj=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(obj)
    return obj

def assemble(llvm_mc:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='cdsp-tlb-tables-') as d:
        src=Path(d)/'tlb_tables.s';dst=Path(d)/'tlb_tables.o';src.write_text(LLVM_ASM)
        subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj','-o',str(dst),str(src)],check=True)
        return load_helper('verify-cdsp-hexagon-entry.py','hexagon_elf_parser').parse_elf32_text(dst.read_bytes())

def verify(llvm_mc:Path,rz_asm:Path,rust_output:Path,reference:Path|None)->None:
    got=rust_output.read_bytes()
    if len(got)!=LENGTH:raise ValueError('Expected 64 bytes of TLB-table code')
    llvm=assemble(llvm_mc)
    if len(llvm)!=LENGTH:raise ValueError('LLVM fixture must be 64 bytes')
    w=struct.unpack('<16I',got);others=struct.unpack('<16I',llvm)
    for idx in range(16):
        if idx not in TLBW_INDICES and w[idx]!=others[idx]:
            raise ValueError(f'LLVM roundtrip disagrees at file {START+4*idx:#x}')
    decoded=load_helper('verify-cdsp-tlb.py','hexagon_tlb_decoder').decode_rizin(
        rz_asm,got,physical_address=PHYS)
    if tuple(decoded)!=EXPECTED:raise ValueError(f'V66 TLB table decoded forms changed: {decoded}')
    if reference is not None:
        original=reference.read_bytes()
        if original[START:START+LENGTH]!=got:raise ValueError('Original CDSP TLB table walk differs')
    print('64 CDSP TLB-table bytes matched reference: 56 LLVM-assembled bytes; two decoded but not LLVM-assembled privileged TLB writes; all 16 V66 words confirmed by Rizin')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
