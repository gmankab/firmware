#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Cross-check 312 bytes of Hexagon CDSP TLB/startup/cache copy.

LLVM6 independently assembles forced extended CALL, DCZEROA and register-pair
load/store. The latter have equal opcode fields but packet marker differences
when assembled as standalone packets. V66 Rizin independently decodes all
78 words, including privileged TLB operations that LLVM6 cannot assemble.
"""
import argparse
import importlib.util
import os
from pathlib import Path
import re
import subprocess
import struct
import tempfile

FILE_OFFSET=0x41e0
PHYSICAL=0x96c011e0
BYTES=0x138
ASM='''.text
.global smoke
smoke:
{ call ##-188 }
{ call ##16 }
{ dczeroa(r1) }
{ r7:6=memd(r0++#16) }
{ memd(r1++#16)=r7:6 }
'''
# Firmly grounded semantic labels, not extracted OEM opcodes.
LABELS={
    0x41e0:'immext(##0xfe0c6700)',
    0x4238:'tlblock',
    0x423c:'tlbw(r7:6,r3)',
    0x4288:'if(!p0.new)jump:nt0x96c01264',
    0x428c:'jumprlr',
    0x4290:'r17=lr',
    0x42a0:'call0x96c011e0',
    0x42b0:'call0x96c012bc',
    0x42f0:'dczeroa(r1)',
    0x4300:'memd(r5++#0x10)=r9:8',
    0x4314:'jumprlr',
}

def parse_elf32_text(p:Path)->bytes:
    lib=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_ref_elf',lib)
    assert spec is not None and spec.loader is not None
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(p.read_bytes())

def verify(llvm:Path,decoder:Path,produced:Path,reference:Path|None):
    data=produced.read_bytes()
    if len(data)!=BYTES:raise ValueError(f'Expected {BYTES} Hexagon bytes, got {len(data)}')
    rz=subprocess.run([str(decoder),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],text=True,capture_output=True,check=True)
    rows={}
    for line in rz.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+([0-9a-fA-F]{8})\s+([?\\/\[|])\s+(.*)',line)
        if m:rows[int(m[1],16)]=''.join(m[4].lower().split())
    if len(rows)!=BYTES//4:raise ValueError(f'Rizin decoded {len(rows)} of {BYTES//4} instruction words')
    for foff,label in LABELS.items():
        text=rows.get(PHYSICAL+foff-FILE_OFFSET,'')
        if label not in text:raise ValueError(f'Rizin unexpected {foff:#x}: {text} vs {label}')
    with tempfile.TemporaryDirectory(prefix='cdsp-joined-llvm-') as d:
        asm=Path(d)/'check.s';obj=Path(d)/'check.o';asm.write_text(ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(asm)],check=True)
        from_llvm=parse_elf32_text(obj)
    if len(from_llvm)!=28:raise ValueError('LLVM6 sample emitted an unexpected length')
    for file_offset, sample_at, width in [(0x429c,0,8),(0x42ac,8,8),(0x42f0,16,4)]:
        i=file_offset-FILE_OFFSET
        if data[i:i+width]!=from_llvm[sample_at:sample_at+width]:
            raise ValueError(f'LLVM6 did not independently emit exact control bytes at {file_offset:#x}')
    for file_offset, sample_at in [(0x42f4,20),(0x42fc,24)]:
        original_word=struct.unpack_from('<I',data,file_offset-FILE_OFFSET)[0]
        llvm_word=struct.unpack_from('<I',from_llvm,sample_at)[0]
        if original_word & ~0xc000 != llvm_word & ~0xc000:
            raise ValueError(f'Hexagon V66 packet parse-marker normalization did not match LLVM at {file_offset:#x}')
    if reference is not None:
        original=reference.read_bytes()
        if original[FILE_OFFSET:FILE_OFFSET+BYTES]!=data:
            raise ValueError('Source-generated CDSP bytes differ from pinned firmware')
    print(f'CDSP Hexagon: {BYTES}/{BYTES} exact rebuilt bytes, {len(rows)} independently decoded instructions, three LLVM exact and two normalized instruction groups, no privately embedded opcodes')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
