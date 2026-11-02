#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon assembler and V66 decoder check of CDSP TLB walk.

LLVM Hexagon independently assembles 12 of 13 instructions. The V66 tlbw
is replaced by an assembler-only `nop` for layout but NEVER copied into the
source implementation; that remaining instruction is separately decoded by
Rizin and all 52 Rust bytes are compared against the reference.
"""
import argparse
import importlib.util
from pathlib import Path

START=0x3d94
PHYSICAL=0x96c00d94
LENGTH=52
TLBW_OFFSET=32  # 0x3db4 - 0x3d94
EXPECTED=(
    'r0=##0x0', 'immext(##0xfe024900)', 'r1=add(r25,##-0x1fdb700)',
    'r3:2=memd(r1++#0x8)', 'r4=extractu(r3,#0x7,#0x14)',
    'r4=sub(##0x7d,r4)', 'if(!cmp.eq(r4.new,#0x0))jump:t0x96c00db4',
    'r3=insert(r4,#0x7,#0x14)', 'tlbw(r3:2,r0)',
    'p0=cmp.eq(r2,#0x0);if(!p0.new)jump:t0x96c00da0',
    'r0=add(r0,##0x1)', 'isync', 'jumprlr',
)


def make_asm():
    return '''.text
.global cdsp_tlb_walk
cdsp_tlb_walk:
{
 r0 = #0
 r1 = add(r25,##-0x1fdb700)
}
again:
{ r3:2 = memd(r1++#8) }
{ r4 = extractu(r3,#7,#20) }
{
 r4 = sub(#0x7d,r4)
 if (!cmp.eq(r4.new,#0)) jump:t store
}
{ r3 = insert(r4,#7,#20) }
store:
{ nop }
{
 p0 = cmp.eq(r2,#0)
 if (!p0.new) jump:t again
 r0 = add(r0,#1)
}
{ isync }
{ jumpr lr }
'''


def helper():
    src=Path(__file__).with_name('verify-cdsp-tlb.py')
    spec=importlib.util.spec_from_file_location('cdsp_tlb_helpers',src)
    if spec is None or spec.loader is None:raise ValueError('missing TLB helper')
    mod=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def verify(llvm_mc:Path,decoder:Path,generated:Path,reference:Path|None):
    lib=helper()
    data=generated.read_bytes()
    if len(data)!=LENGTH: raise ValueError('CDSP TLB walker expected 52 bytes')
    assembled=lib.assemble(llvm_mc,make_asm())
    if len(assembled)!=LENGTH or assembled[:TLBW_OFFSET]!=data[:TLBW_OFFSET] or assembled[TLBW_OFFSET+4:]!=data[TLBW_OFFSET+4:]:
        raise ValueError('CDSP TLB walk ordinary instructions disagree with LLVM')
    decoded=lib.decode_rizin(decoder,data,physical_address=PHYSICAL)
    if decoded!=list(EXPECTED):raise ValueError(f'CDSP V66 TLB walk unexpected decoded instructions: {decoded}')
    if reference is not None:
        if reference.read_bytes()[START:START+LENGTH]!=data:
            raise ValueError('CDSP TLB walker differs from original reference')
    print('CDSP TLB walker: 52/52 exact reference bytes; 48 LLVM-assembled instruction bytes and all 13 Rizin-decoded instructions')


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)

if __name__=='__main__':main()
