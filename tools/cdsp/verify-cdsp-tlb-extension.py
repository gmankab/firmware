#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check the newly reconstructed CDSP TLB clear/index routines.

LLVM6 builds a 64-byte independently authored fixture with two `nop`
placeholders because LLVM6 lacks V66 privileged `tlbw`; it also schedules
one independent add/jump pair in reverse. We compare the 48 LLVM-identical
bytes exactly, check the two reordered instruction operand fields with the
Hexagon packet parse-marker bits masked, and decode all 16 original words
using Rizin's V66 decoder. Crucially Rizin decoding is NOT an independently
assembled encoding of the two privileged TLB writes.
"""
from __future__ import annotations
import argparse
from pathlib import Path
import struct
import subprocess
import tempfile
import importlib.util

START=0x3e38
LENGTH=64
PHYS=0x96c00e38
LLVM_ASM='''.text
.global cdsp_tlb_extension
cdsp_tlb_extension:
{
 r5 = #0x7f
 r0 = #0
 r3:2 = combine(#0,#0)
}
loop:
{
 p0 = cmp.eq(r0,r5)
 if (p0.new) jumpr:nt lr
}
{ nop }
{
 r0 = add(r0,#1)
 jump loop
}
{
 r0 = #0x7f
 r1 = ##-0x1fdb700
}
{ r2 = asl(r0,#3) }
{ r1 = add(r1,r2) }
{
 r0 = #0x7f
 r3:2 = memd(r1++#0)
}
{ nop }
{ jumpr lr }
'''
EXPECTED=(
 'r5=##0x7f', 'r0=##0x0;r3:2=combine(#0,#0x0)',
 'p0=cmp.eq(r0,r5)', 'if(p0.new)jumpr:ntlr',
 'tlbw(r3:2,r0)', 'r0=add(r0,##0x1)', 'jump0x96c00e40',
 'r0=##0x7f', 'immext(##0xfe024900)', 'r1=##-0x1fdb700',
 'r2=asl(r0,#0x3)', 'r1=add(r1,r2)', 'r0=##0x7f',
 'r3:2=memd(r1++#0x0)', 'tlbw(r3:2,r0)', 'jumprlr',
)

def independent_bytes(llvm_mc:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='cdsp-hex-v66-ext-') as tmp:
        source=Path(tmp)/'extension.s';obj=Path(tmp)/'extension.o'
        source.write_text(LLVM_ASM)
        subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj','-o',str(obj),str(source)], check=True)
        helper=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('cdsp_elf_parser',helper)
        if spec is None or spec.loader is None:raise ValueError('missing CDSP ELF parser')
        module=importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module.parse_elf32_text(obj.read_bytes())

def verify(llvm_mc:Path,rz_asm:Path,rust_output:Path,reference:Path|None)->None:
    actual=rust_output.read_bytes()
    if len(actual)!=LENGTH:raise ValueError(f'expected {LENGTH} bytes, got {len(actual)}')
    independent=independent_bytes(llvm_mc)
    if len(independent)!=LENGTH:raise ValueError('LLVM fixture length differs')
    noncanonical={4,5,6,14}
    for idx, (rust,llvm) in enumerate(zip(struct.unpack('<16I',actual),struct.unpack('<16I',independent))):
        if idx not in noncanonical and rust!=llvm:
            raise ValueError(f'LLVM identical instruction differs at {START+idx*4:#x}')
    asm_words=struct.unpack('<16I',independent)
    words=struct.unpack('<16I',actual)
    mask=0xffff3fff
    if ((words[5]&mask)!=(asm_words[6]&mask) or
        (words[6]&mask)!=(asm_words[5]&mask)):
        raise ValueError('reordered loop packet fields disagree with LLVM')
    helper=Path(__file__).with_name('verify-cdsp-tlb.py')
    spec=importlib.util.spec_from_file_location('cdsp_v66_decoder',helper)
    if spec is None or spec.loader is None:raise ValueError('missing Hexagon decoder helper')
    mod=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    decoded=mod.decode_rizin(rz_asm,actual,physical_address=PHYS)
    if tuple(decoded)!=EXPECTED:
        raise ValueError(f'Rizin V66 decode mismatch:\n {decoded}')
    if reference is not None:
        ref=reference.read_bytes()
        if len(ref)<START+LENGTH or ref[START:START+LENGTH]!=actual:
            raise ValueError(f'CDSP private reference differs at file {START:#x}')
    print('Verified 64 CDSP TLB bytes: 48 byte-for-byte LLVM, 8 LLVM-normalized operand fields, 2 privileged TLB words Rizin-decoded, 16 Rizin instruction words, optional full reference match')

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--llvm-mc',type=Path,required=True)
    parser.add_argument('--rz-asm',type=Path,required=True)
    parser.add_argument('--rust-output',type=Path,required=True)
    parser.add_argument('--reference',type=Path)
    a=parser.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)

if __name__=='__main__':main()
