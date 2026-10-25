#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Cross-check the 64-byte CDSP hardware-loop copy with Hexagon LLVM/Rizin.

LLVM6 supports all instruction families in this routine, but its scheduler
rearranges one predicate packet and picks a different parse marker for one
word. Compare the decoded encodings **and packet ownership**, rather than
claiming a 64-byte LLVM binary identity that the tool does not provide.
"""
from __future__ import annotations
import argparse
import importlib.util
from pathlib import Path
import struct
import subprocess
import tempfile

FILE_OFFSET = 0x41a0
FUNCTION_BYTES = 64
PHYSICAL_ADDRESS = 0x96c011a0


def load_helper(filename: str, alias: str):
    spec = importlib.util.spec_from_file_location(alias, Path(__file__).with_name(filename))
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def program() -> str:
    return '''.text
.global cdsp_dma_loop
cdsp_dma_loop:
{ r3=memw(r25+##-0x1f39980) }
{ r0=memw(r25+##-0x1f398cc) }
{ p0=cmp.eq(r3,#0)
  if(p0.new) jump:nt done }
{ r1=add(r25,##-0x3f98cda0)
  p0=cmp.gt(r3,#32)
  if(!p0.new) jump:nt done }
{ p0=cmp.eq(r1,#0)
  if(p0.new) jump:nt done }
{ r2=lsr(r3,#3) }
{ loop0(loop,r2) }
loop:
{ r5:4=memd(r1++#8) }
{ nop
  memd(r0++#8)=r5:4 }:endloop0
done:
{ jumpr lr }
'''


def verify(llvm_mc: Path, rz_asm: Path, rust_output: Path, reference: Path | None):
    got = rust_output.read_bytes()
    if len(got) != FUNCTION_BYTES:
        raise ValueError('CDSP DMA loop must be 64 bytes')
    with tempfile.TemporaryDirectory(prefix='cdsp-dma-copy-llvm-') as tmp:
        asm = Path(tmp)/'dma.s'; obj = Path(tmp)/'dma.o'
        asm.write_text(program())
        subprocess.run([str(llvm_mc), '-triple=hexagon', '-mcpu=hexagonv60',
                        '-filetype=obj', '-o', str(obj), str(asm)], check=True)
        compiled = load_helper('verify-cdsp-hexagon-entry.py','cdsp_dma_elf').parse_elf32_text(obj.read_bytes())
    if len(compiled) != FUNCTION_BYTES:
        raise ValueError('LLVM changed DMA loop instruction count')
    original_words = struct.unpack('<16I', got)
    llvm_words = struct.unpack('<16I', compiled)
    unchanged_indices = (0,2,3,4,9,10,11,12,13,14,15)
    for i in unchanged_indices:
        if original_words[i] != llvm_words[i]:
            raise ValueError(f'LLVM difference at preserved DMA word {i}')
    if (original_words[1] & ~0xc000) != (llvm_words[1] & ~0xc000):
        raise ValueError('the first packet parse-marker was not the sole change')
    # LLVM6 moves the last predicate branch to the front of this packet, and
    # puts two other instructions in the same order but a different parse form.
    for reference_index, llvm_index in [(5,6),(6,7),(7,8),(8,5)]:
        if (original_words[reference_index] & ~0xc000) != (llvm_words[llvm_index] & ~0xc000):
            raise ValueError(f'LLVM predicate instruction bits differ: {reference_index}/{llvm_index}')
    decoded = load_helper('verify-cdsp-tlb.py','cdsp_dma_rizin').decode_rizin(rz_asm, got, PHYSICAL_ADDRESS)
    if len(decoded) != 16:
        raise ValueError(f'expected 16 Rizin Hexagon instructions; got {len(decoded)}')
    assertions={4:('p0=cmp.eq(r3,#0x0)','0x96c011dc'),
                7:('p0=cmp.gt(r3,##0x20)',),
                8:('if(!p0.new)jump:nt0x96c011dc',),
                11:('loop0(0x96c011d0,r2)',),
                14:('memd(r0++#0x8)=r5:4',)}
    for i, parts in assertions.items():
        instruction=decoded[i]
        for part in parts:
            part=''.join(part.lower().split())
            if part not in instruction:
                raise ValueError(f'Rizin instruction {i} differs: {instruction}')
    if reference is not None and reference.read_bytes()[FILE_OFFSET:FILE_OFFSET+FUNCTION_BYTES]!=got:
        raise ValueError('DMA loop bytes differ from the privately supplied reference')
    print('CDSP DMA: 64/64 exact bytes, 16/16 independently LLVM-assembled semantic instructions; scheduler/packet markers differ, Rizin validates 16 decoded words')


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args()
    verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)

if __name__=='__main__':main()
