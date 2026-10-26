#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent reference/Hexagon V66 evidence for Agent20-09 worker runtime.

Checks all 18 emitted Hexagon words with Rizin semantics and independent LLVM6
assembly. OEM CDSP bytes enter only as an external verification input.
"""
from __future__ import annotations
import argparse
import importlib.util
import re
import subprocess
import tempfile
from pathlib import Path

FILE_START=0x4c1c
FILE_END=0x4c64
PHYS_BASE=0x96c0_1c1c
EXPECTED={
 0x4c1c:'call0x96c328bc',
 0x4c20:'r23:22=combine(r1,r0)',
 0x4c24:'call0x96c02184',
 0x4c28:'call0x96c02268',
 0x4c2c:'r1:0=combine(r23,r22)',
 0x4c30:'p0=cmp.eq(r0,#0x2);if(p0.new)jump:nt0x96c01f24',
 0x4c34:'p0=cmp.eq(r0,#0x1);if(p0.new)jump:t0x96c01cd8',
 0x4c38:'immext(##0xfe016f40)',
 0x4c3c:'sp=##-0x1fe90a0',
 0x4c40:'call0x96c029b0',
 0x4c44:'immext(##0xfffff6c0)',
 0x4c48:'call0x96c01318',
 0x4c4c:'immext(##0xfffff700)',
 0x4c50:'call0x96c01364',
 0x4c54:'immext(##0xb140)',
 0x4c58:'call0x96c0cdb0',
 0x4c5c:'immext(##0xa380)',
 0x4c60:'call0x96c0c004',
}
assert len(EXPECTED)==18

def elf_text(data:bytes)->bytes:
    src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('agent09_hexagon_elf_reader',src)
    assert spec and spec.loader
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.parse_elf32_text(data)

def llvm_asm(mctool:Path,source:str)->bytes:
    with tempfile.TemporaryDirectory(prefix='cdsp-worker-runtime-llvm-') as temp:
        a=Path(temp)/'check.s';o=Path(temp)/'check.o'
        a.write_text(source)
        subprocess.run([str(mctool),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                        '-o',str(o),str(a)],text=True,capture_output=True,check=True)
        return elf_text(o.read_bytes())

def single_call(mctool:Path,delta:int)->bytes:
    if not 0<delta<0x100000 or delta%4:
        raise ValueError('LLVM short direct call sample needs positive aligned displacement')
    return llvm_asm(mctool,'.text\n.global sample\nsample:\n{ call target }\n'
                    f'.space {delta-4}\ntarget:\n{{ jumpr lr }}\n')[:4]

def independent_llvm(mctool:Path,generated:bytes)->int:
    # Displacements are encoded relative to physical packet start, so these
    # tests place only the *offset* in a temporary ELF .text section.
    matched=0
    def check(address:int,code:bytes):
        nonlocal matched
        at=address-FILE_START
        got=generated[at:at+len(code)]
        if got!=code:
            raise ValueError(f'independent LLVM6 mismatch at file {address:#x}: '
                             f'generator {got.hex()}, assembler {code.hex()}')
        matched+=len(code)//4
    check(0x4c1c,single_call(mctool,0x96c328bc-0x96c01c1c))
    check(0x4c24,single_call(mctool,0x96c02184-0x96c01c24))
    check(0x4c28,single_call(mctool,0x96c02268-0x96c01c28))
    check(0x4c40,single_call(mctool,0x96c029b0-0x96c01c40))
    pair=llvm_asm(mctool,'.text\n.global sample\nsample:\n'
                  '{ r23:22=combine(r1,r0) }\n'
                  '{ r1:0=combine(r23,r22) }\n'
                  '{ sp=##-0x1fe90a0 }\n')
    check(0x4c20,pair[:4]);check(0x4c2c,pair[4:8]);check(0x4c38,pair[8:16])
    b1=llvm_asm(mctool,'.text\n.global sample\nsample:\n'
                '{ p0=cmp.eq(r0,#2); if(p0.new) jump:nt target }\n'
                '.space 0x2f0\ntarget:\n{ jumpr lr }\n')
    check(0x4c30,b1[:4])
    b2=llvm_asm(mctool,'.text\n.global sample\nsample:\n'
                '{ p0=cmp.eq(r0,#1); if(p0.new) jump:t target }\n'
                '.space 0xa0\ntarget:\n{ jumpr lr }\n')
    check(0x4c34,b2[:4])
    # Explicit extended CALL displacements force two-word V66 packet forms.
    bundles=llvm_asm(mctool,'.text\n.global sample\nsample:\n'
        '{ call ##-0x92c }\n{ call ##-0x8e8 }\n'
        '{ call ##0xb15c }\n{ call ##0xa3a8 }\n')
    for i,off in enumerate((0x4c44,0x4c4c,0x4c54,0x4c5c)):
        check(off,bundles[8*i:8*(i+1)])
    if matched!=18:raise AssertionError(f'expected 18 LLVM6 checked words, matched {matched}')
    return matched

def independent_rizin(asmtool:Path,generated:bytes)->int:
    cmd=[str(asmtool),'-a','hexagon','-b','32','-o',hex(PHYS_BASE),'-D',generated.hex()]
    res=subprocess.run(cmd,capture_output=True,text=True,check=True)
    observed={}
    for line in res.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:observed[int(m.group(1),16)]= ''.join(m.group(2).split()).lower()
    if len(observed)!=18:raise ValueError(f'Rizin decoded {len(observed)} of 18 words')
    for off,want in EXPECTED.items():
        address=PHYS_BASE+off-FILE_START
        got=observed[address]
        if want not in got:
            raise ValueError(f'Rizin semantic mismatch file {off:#x}: got {got}, expected {want}')
    return len(observed)

def verify(llvm:Path,rizin:Path,generated:Path,reference:Path)->None:
    b=generated.read_bytes();f=reference.read_bytes()
    if len(b)!=FILE_END-FILE_START:raise ValueError(f'expected 72 bytes, got {len(b)}')
    if f[FILE_START:FILE_END]!=b:raise ValueError('CDSP new worker bytes differ from pinned source')
    n=independent_rizin(rizin,b)
    v=independent_llvm(llvm,b)
    if f[0x4bb8:0x4bc0] != b'\0'*8:
        raise ValueError('earlier eight linker-alignment bytes were changed')
    print(f'CDSP worker runtime {len(b)} exact bytes, {n} Rizin V66 semantic checks, '
          f'{v} independent LLVM6 assembled words; 240 linker-fill bytes excluded.')

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    for arg in ('llvm-mc','rz-asm','rust-output','reference'):
        parser.add_argument('--'+arg,type=Path,required=True)
    q=parser.parse_args();verify(q.llvm_mc,q.rz_asm,q.rust_output,q.reference)
