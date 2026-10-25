#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon LLVM+Rizin cross-check of the 60-byte CDSP TLB code.

LLVM 6 assembles the ordinary arithmetic, bit-field, return and synchronize
packets; Rizin decodes all 15 instructions including the V66-only `tlbw`.
This is *not* a claim that LLVM independently assembled the two TLB writes.
"""
import argparse
import importlib.util
from pathlib import Path
import re
import subprocess
import tempfile

START_FILE_OFFSET=0x3dfc
PHYSICAL_ADDRESS=0x96c00dfc
CODE_SIZE=60
MNEMONICS=(
    'r2=lsr(r24,#0xb)', 'r3=lsr(r26,#0xc)', 'r4=##0x7c0',
    'r5=##0xc00', 'r2=insert(r4,#0xb,#0x15)',
    'r3=insert(r5,#0xc,#0x14)', 'r2=setbit(r2,#0x1)',
    'r0=##0x7e', 'tlbw(r3:2,r0)', 'isync', 'jumprlr',
    'r3:2=combine(##0x0,#0x0)', 'tlbw(r3:2,r0)', 'isync', 'jumprlr',
)


def original_llvm_asm() -> str:
    return '''.text
.global cdsp_tlb_setup
cdsp_tlb_setup:
{
 r2 = lsr(r24,#11)
 r3 = lsr(r26,#12)
 r4 = #0x7c0
 r5 = #0xc00
}
{
 r2 = insert(r4,#11,#21)
 r3 = insert(r5,#12,#20)
}
{
 r2 = setbit(r2,#1)
 r0 = #0x7e
}
'''


def trailing_llvm_asm() -> str:
    return '''.text
.global cdsp_tlb_returns
cdsp_tlb_returns:
{ isync }
{ jumpr lr }
{ r3:2 = combine(#0,#0) }
{ isync }
{ jumpr lr }
'''


def parse_elf(data: bytes) -> bytes:
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('hex_entry', path)
    if spec is None or spec.loader is None:
        raise ValueError('missing CDSP ELF text parser')
    mod=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.parse_elf32_text(data)


def assemble(llvm_mc: Path, source: str) -> bytes:
    with tempfile.TemporaryDirectory(prefix='cdsp-hexagon-tlb-') as d:
        asm=Path(d)/'check.s';out=Path(d)/'check.o'
        asm.write_text(source)
        subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj','-o',str(out),str(asm)],check=True)
        return parse_elf(out.read_bytes())


def decode_rizin(decoder: Path, data: bytes) -> list[str]:
    result=subprocess.run([str(decoder),'-a','hexagon','-b','32',
                           '-o',f'0x{PHYSICAL_ADDRESS:x}','-D',data.hex()],
                          check=True,capture_output=True,text=True)
    found=[]
    for line in result.stdout.splitlines():
        token=line.split()
        if len(token)<5 or not re.fullmatch(r'0x[0-9a-f]+',token[0],re.I):
            continue
        if token[1]!='4' or not re.fullmatch(r'[0-9a-fA-F]{8}',token[2]):
            raise ValueError('non-32-bit Hexagon instruction detected')
        if token[3] not in ('[','/','|','\\','?'):
            raise ValueError(f'unknown Hexagon packet parse marker {token[3]}')
        found.append(''.join(token[4:]).lower())
    return found


def verify(llvm_mc: Path, decoder: Path, rust_output: Path, reference: Path|None):
    actual=rust_output.read_bytes()
    if len(actual)!=CODE_SIZE:raise ValueError('CDSP typed TLB output must be 60 bytes')
    prefix=assemble(llvm_mc,original_llvm_asm())
    tail=assemble(llvm_mc,trailing_llvm_asm())
    if len(prefix)!=32 or len(tail)!=20 or prefix!=actual[:32] or tail!=actual[36:48]+actual[52:60]:
        raise ValueError('LLVM Hexagon did not match independently assembled non-TLB words')
    decoded=decode_rizin(decoder,actual)
    if decoded!=list(MNEMONICS):
        raise ValueError(f'Hexagon V66 instructions differ from expected: {decoded!r}')
    if reference is not None:
        source=reference.read_bytes()[START_FILE_OFFSET:START_FILE_OFFSET+CODE_SIZE]
        if source!=actual:raise ValueError('CDSP 60-byte TLB region differs from private reference')
    print('CDSP TLB region: 60/60 exact reference bytes; 52 LLVM-assembled bytes and 15 Rizin-decoded instructions')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--llvm-mc',type=Path,required=True)
    parser.add_argument('--rz-asm',type=Path,required=True)
    parser.add_argument('--rust-output',type=Path,required=True)
    parser.add_argument('--reference',type=Path)
    args=parser.parse_args()
    verify(args.llvm_mc,args.rz_asm,args.rust_output,args.reference)

if __name__=='__main__':main()
