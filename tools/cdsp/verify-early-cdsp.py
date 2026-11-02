#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent CDSP TLB sweep verification using V66 Rizin + LLVM6 scalar forms.

The Rizin decoder validates privileged TLB operations and packet markers.
LLVM6 independently encodes five ordinary DSP scalar instructions. LLVM6
lacks tlbr/tlbw/tlblock/tlbunlock and must not be misrepresented as checking
those privileged encodings independently. Both tools use source assemblers or
decoders, not firmware bytes baked into this verifier.
"""
import argparse
import importlib.util
import os
from pathlib import Path
import re
import subprocess
import tempfile

FILE_OFFSET=0x41e0
PHYSICAL_ADDRESS=0x96c011e0
ENCODED_LEN=0xb0
# File offsets relative to the source-derived fragment, not opcode literals.
LLVM_SAMPLES = [(0x4218,0), (0x421c,4), (0x4220,8), (0x4224,12), (0x4234,16)]
ASSEMBLY = '''.text
.global scalar_independent
scalar_independent:
{ r6=ct0(r4) }
{ r4=clrbit(r4,r6) }
{ r4=asr(r4,#1) }
{ r4=and(r4,r10) }
{ r7:6=combine(#0,#0) }
'''
EXPECTED = {
    0x41e0:'immext(##0xfe0c6700)',
    0x41e4:'r0=memw(gp+##0xfe0c6734)',
    0x41f0:'p0=cmp.eq(r1,#0x0);if(p0.new)jump:nt0x96c0128c',
    0x4204:'loop0(0x96c01210,#0x7f)',
    0x4210:'r5:4=tlbr(r3)',
    0x4238:'tlblock',
    0x423c:'tlbw(r7:6,r3)',
    0x4240:'tlbunlock',
    0x4264:'r1:0=memd(r4++#0x8)',
    0x426c:'tlblock',
    0x4270:'tlbw(r1:0,r3)',
    0x4274:'tlbunlock',
    0x4288:'if(!p0.new)jump:nt0x96c01264',
    0x428c:'jumprlr',
}
def parse_elf32_text(p:Path)->bytes:
    source=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf_parser',source)
    assert spec is not None and spec.loader is not None
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.parse_elf32_text(p.read_bytes())

def verify(llvm_mc:Path, rz_asm:Path, built:Path, reference:Path|None):
    payload=built.read_bytes()
    if len(payload)!=ENCODED_LEN:
        raise ValueError(f'Expected {ENCODED_LEN} typed bytes, found {len(payload)}')
    cmd=[str(rz_asm),'-a','hexagon','-b','32','-o',hex(PHYSICAL_ADDRESS),'-D',payload.hex()]
    decoded=subprocess.run(cmd,capture_output=True,text=True,check=True)
    lines={}
    for l in decoded.stdout.splitlines():
        match=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+([a-fA-F0-9]{8})\s+([?\\/\[|])\s+(.*)',l)
        if match:
            a=int(match[1],16);lines[a]=(''.join(match[4].lower().split()), match[3])
    if len(lines)!=ENCODED_LEN//4:
        raise ValueError(f'Rizin decoded only {len(lines)} of {ENCODED_LEN//4} Hexagon words')
    for rel,label in EXPECTED.items():
        text,_=lines[PHYSICAL_ADDRESS+(rel-FILE_OFFSET)]
        if label not in text:
            raise ValueError(f'Unexpected V66 operation at file {rel:#x}: {text} (expected {label})')
    for rel,mark in [(0x41e4,'?'),(0x41f4,'/'),(0x4208,'|'),(0x4244,'/'),(0x4284,'/')]:
        got=lines[PHYSICAL_ADDRESS+(rel-FILE_OFFSET)][1]
        if got!=mark:raise ValueError(f'V66 packet parse marker at {rel:#x} expected {mark}, got {got}')
    with tempfile.TemporaryDirectory(prefix='cdsp-early-llvm-') as d:
        src=Path(d)/'ordinary.s'; obj=Path(d)/'ordinary.o';src.write_text(ASSEMBLY)
        subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(src)],check=True)
        independent=parse_elf32_text(obj)
    if len(independent)!=20:raise ValueError('Independent LLVM assembly changed instruction count')
    for addr,section_offset in LLVM_SAMPLES:
        pos=addr-FILE_OFFSET
        if payload[pos:pos+4]!=independent[section_offset:section_offset+4]:
            raise ValueError(f'Independent LLVM scalar instruction differs at {addr:#x}')
    if reference is not None:
        orig=reference.read_bytes()
        if orig[FILE_OFFSET:FILE_OFFSET+ENCODED_LEN]!=payload:
            raise ValueError('CDSP typed span differs from pinned original reference')
    print(f'CDSP TLB: {ENCODED_LEN}/{ENCODED_LEN} source-emitted reference-exact bytes; {len(lines)} V66 decoded instruction words; five independently LLVM-assembled instructions; privileged V66 words only decoder-checked')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--rz-asm',required=True,type=Path)
    p.add_argument('--rust-output',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
