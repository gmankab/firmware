#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Reassemble CDSP's conditional TLB controller with LLVM and decode Rizin.

LLVM 6 reorders the two-instruction .new predicate packet and does not
support privileged tlbw. The comparison checks identical fields with the
architecture's packet parse bits removed for the reordered predicate pair,
checks every other independently assemblable instruction exactly, and checks
that Rizin decodes all 13 emitted instructions. Never claim an independent
LLVM assembly of the 4-byte tlbw instruction.
"""
import argparse
import importlib.util
from pathlib import Path
import struct

START=0x3dc8
PHYSICAL=0x96c00dc8
LENGTH=52
EXPECTED=(
    'p0=cmp.eq(r25,##0x0)', 'if(p0.new)jump:nt0x96c00df8',
    'r2=lsr(r24,#0xb)', 'r3=lsr(r24,#0xc)',
    'r4=##0x7c0', 'r5=##0xc00',
    'r2=insert(r4,#0xb,#0x15)', 'r3=insert(r5,#0xc,#0x14)',
    'r2=setbit(r2,#0x1)', 'r0=##0x7f',
    'tlbw(r3:2,r0)', 'isync', 'jumprlr',
)


def asm_source():
    return '''.text
.global conditional_tlb
conditional_tlb:
{
 p0 = cmp.eq(r25,#0)
 if (p0.new) jump:nt tlb_end
}
{
 r2 = lsr(r24,#11)
 r3 = lsr(r24,#12)
 r4 = #0x7c0
 r5 = #0xc00
}
{
 r2 = insert(r4,#11,#21)
 r3 = insert(r5,#12,#20)
}
{
 r2 = setbit(r2,#1)
 r0 = #0x7f
}
{ nop }
{ isync }
tlb_end:
{ jumpr lr }
'''


def helper():
    src=Path(__file__).with_name('verify-cdsp-tlb.py')
    spec=importlib.util.spec_from_file_location('other_tlb',src)
    if spec is None or spec.loader is None:raise ValueError('no TLB helper')
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod


def verify(llvm_mc: Path, decoder: Path, generated: Path, reference: Path|None):
    library=helper()
    data=generated.read_bytes()
    if len(data)!=LENGTH:raise ValueError(f'CDSP conditional TLB expected {LENGTH} bytes')
    asm=library.assemble(llvm_mc,asm_source())
    if len(asm)!=LENGTH:raise ValueError('LLVM conditional-TLB fixture wrong length')
    words=struct.unpack('<13I',data)
    expected=struct.unpack('<13I',asm)
    # LLVM schedules the `.new` branch before its predicate producer, while
    # the original packet has producer first. Only compare semantic bitfields.
    if (words[0]&~0xc000)!=(expected[1]&~0xc000) or (words[1]&~0xc000)!=(expected[0]&~0xc000):
        raise ValueError('LLVM predicate and conditional-branch fields mismatch')
    if data[8:40]!=asm[8:40] or data[44:]!=asm[44:]:
        raise ValueError('LLVM-assembled non-TLB instructions mismatch')
    decoded=library.decode_rizin(decoder,data,physical_address=PHYSICAL)
    if decoded!=list(EXPECTED):raise ValueError(f'Rizin CDSP conditional TLB instruction mismatch: {decoded}')
    if reference is not None:
        if reference.read_bytes()[START:START+LENGTH]!=data:
            raise ValueError('CDSP reference differs in conditional TLB region')
    print('Verified 52/52 CDSP conditional TLB bytes: 40 exact LLVM bytes, 8 LLVM predicate/branch fields, all 13 Rizin-decoded instructions')


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)

if __name__=='__main__':main()
