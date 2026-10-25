#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify CFGBASE V66 function and distinct 0x4418..0x4500 zero alignment.

All 9 instruction words must decode as expected on Rizin's V66 backend;
LLVM6 independently encodes 3 ordinary instructions after removing packet
marker bits. CFGBASE and MEMW_PHYS are independently *decoded*, not
assembled, because LLVM6 rejects this privileged ISA subset.
"""
import argparse
import importlib.util
from pathlib import Path
import re
import subprocess
import tempfile

FILE_OFFSET=0x43f4
PHYSICAL=0x96c013f4
FUNCTION_SIZE=36
ALIGN_END=0x4500
EXPECTED={
    0x43f4:'r1=cfgbase',
    0x43f8:'r1=asl(r1,#0x5)',
    0x43fc:'immext(##0x0)',
    0x4400:'r0=##0x0',
    0x4404:'r2=memw_phys(r0,r1)',
    0x4408:'r2=asl(r2,#0x10)',
    0x440c:'jumprlr',
    0x4410:'immext(##0xfe0c6700)',
    0x4414:'memw(gp+##0xfe0c6734)=r2.new',
}
ASM='''.text
.global cfgbase_samples
cfgbase_samples:
{ r1=asl(r1,#5) }
{ r2=asl(r2,#16) }
{ jumpr lr }
'''

def parse_elf(obj:Path)->bytes:
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cfgbase_elf',path)
    assert spec and spec.loader
    m=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m.parse_elf32_text(obj.read_bytes())

def check(llvm:Path,rizin:Path,generated:Path,reference:Path|None):
    data=generated.read_bytes()
    if len(data)!=FUNCTION_SIZE:raise ValueError(f'Need {FUNCTION_SIZE} source-emitted bytes')
    rz=subprocess.run([str(rizin),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],capture_output=True,text=True,check=True)
    insns={}
    for line in rz.stdout.splitlines():
        m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+([?\\/\[|])\s+(.*)',line)
        if m:insns[int(m[1],16)]=(''.join(m[3].lower().split()),m[2])
    if len(insns)!=9:raise ValueError(f'Expected nine Rizin decoded words, saw {len(insns)}')
    for addr,text in EXPECTED.items():
        actual=insns[PHYSICAL+addr-FILE_OFFSET][0]
        if text not in actual:raise ValueError(f'Unexpected V66 at {addr:#x}: {actual}; expected {text}')
    # Last instruction's consumer shares packet with the produced R2.new;
    # GP + reserved offset is a source address constant, not a source blob.
    if insns[PHYSICAL+0x4414-FILE_OFFSET][1]!='\\':
        raise ValueError('Final GP store did not terminate Hexagon packet')
    with tempfile.TemporaryDirectory(prefix='cdsp-cfgbase-llvm-') as tmp:
        asm=Path(tmp)/'ordinary.s'; obj=Path(tmp)/'ordinary.o'
        asm.write_text(ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(asm)],capture_output=True,text=True,check=True)
        compiled=parse_elf(obj)
    if len(compiled)!=12:raise ValueError('Expected 3 LLVM6-generated instruction words')
    for a,b in ((0,0x43f8),(4,0x4408),(8,0x440c)):
        llvm_word=int.from_bytes(compiled[a:a+4],'little')
        ref_word=int.from_bytes(data[b-FILE_OFFSET:b-FILE_OFFSET+4],'little')
        if llvm_word&~0xc000 != ref_word&~0xc000:
            raise ValueError(f'Independent LLVM6 operand bits differ at {b:#x}')
    if reference:
        all_bytes=reference.read_bytes()
        if all_bytes[FILE_OFFSET:FILE_OFFSET+FUNCTION_SIZE]!=data:
            raise ValueError('Pinned reference CDSP CFGBASE bytes differ')
        zero=all_bytes[FILE_OFFSET+FUNCTION_SIZE:ALIGN_END]
        if zero!=bytes(ALIGN_END-(FILE_OFFSET+FUNCTION_SIZE)):
            raise ValueError('Reference ELF linker alignment bytes differ')
    print('CFGBASE: 36/36 typed bytes matched; nine Rizin-decoded V66 words; '
          'three LLVM independently encoded ordinary instructions; '
          '232 external structural zero-fill bytes are not claimed as executable')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();check(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
