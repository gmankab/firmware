#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""CDSP controller state packet: pinned reference, Rizin V66, LLVM6 evidence.

System-register operations are outside LLVM6's supported assembler symbols;
those four words are tested against original bytes and Rizin semantics only.
"""
from pathlib import Path
import argparse
import importlib.util
import re
import subprocess
import tempfile
START=0x4c64
END=0x4cb4
PHYS=0x96c01c64
EXPECTED={
 0x4c64:'r0=##0x2',
 0x4c68:'immext(##0xfe024d00)',
 0x4c6c:'memw(r1=##0xfe024d00)=r0.new',
 0x4c70:'dccleana(r1)',
 0x4c74:'call0x96c02418',
 0x4c78:'immext(##0xa380)',
 0x4c7c:'call0x96c0c004',
 0x4c80:'call0x96c02044',
 0x4c84:'r0=syscfg',
 0x4c88:'r0=clrbit(r0,#0x4)',
 0x4c8c:'r1=##0x0',
 0x4c90:'syscfg=r0',
 0x4c94:'r0=ssr',
 0x4c98:'r0=insert(r1,#0x3,#0x10)',
 0x4c9c:'ssr=r0',
 0x4ca0:'immext(##0x0)',
 0x4ca4:'r0=##0x24',
 0x4ca8:'immext(##0xfe000040)',
 0x4cac:'memw(r1=##0xfe000044)=r0.new',
 0x4cb0:'dccleana(r1)',
}

def elf_text(path:Path)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('agent09_elf',p)
    assert spec and spec.loader
    t=importlib.util.module_from_spec(spec);spec.loader.exec_module(t)
    return t.parse_elf32_text(path.read_bytes())

def compile_sample(tool:Path,code:str)->bytes:
    with tempfile.TemporaryDirectory(prefix='cdsp-worker-state-llvm-') as tmp:
        source=Path(tmp)/'x.s'; obj=Path(tmp)/'x.o'
        source.write_text('.text\n.global sample\nsample:\n'+code)
        subprocess.run([str(tool),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                        '-o',str(obj),str(source)],check=True,capture_output=True,text=True)
        return elf_text(obj)

def check_llvm(llvm:Path,generated:bytes)->int:
    verified=0
    def check(where:int,data:bytes):
        nonlocal verified
        got=generated[where-START:where-START+len(data)]
        if got!=data:raise ValueError(f'LLVM6 mismatch at file {where:#x}: {got.hex()} / {data.hex()}')
        verified+=len(data)//4
    coupled=compile_sample(llvm,
        '{ r0=##2; memw(r1=##0xfe024d00)=r0.new }\n'
        '{ dccleana(r1) }\n'
        '{ r0=clrbit(r0,#4); r1=#0 }\n'
        '{ r0=insert(r1,#3,#16) }\n'
        '{ r0=##36; memw(r1=##0xfe000044)=r0.new }\n'
        '{ dccleana(r1) }\n')
    # LLVM emits an unnecessary `immext #0` before R0=##2. The reference
    # has a legal shorter producer within the *same* three-word packet.
    check(0x4c64,coupled[4:16])
    check(0x4c70,coupled[16:20])
    check(0x4c88,coupled[20:28])
    check(0x4c98,coupled[28:32])
    check(0x4ca0,coupled[32:48])
    check(0x4cb0,coupled[48:52])
    for where,distance in [(0x4c74,0x96c02418-0x96c01c74),
                           (0x4c80,0x96c02044-0x96c01c80)]:
        assert distance>=4 and distance%4==0
        b=compile_sample(llvm,f'{{ call target }}\n.space {distance-4}\ntarget:\n{{ jumpr lr }}\n')
        check(where,b[:4])
    b=compile_sample(llvm,'{ call ##0xa38c }\n')
    check(0x4c78,b[:8])
    if verified!=16:raise ValueError(f'expected 16 LLVM6 words, matched {verified}')
    return verified

def check_rizin(rz:Path,generated:bytes)->int:
    cp=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',generated.hex()],
                      capture_output=True,text=True,check=True)
    observed={}
    for line in cp.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:observed[int(m[1],16)]=''.join(m[2].split()).lower()
    if len(observed)!=20:raise ValueError(f'expected 20 Rizin V66 words, found {len(observed)}')
    for where,want in EXPECTED.items():
        got=observed[PHYS+where-START]
        if want not in got:
            raise ValueError(f'Rizin semantic mismatch file {where:#x}: {got} expected {want}')
    return len(observed)

def verify(llvm:Path,rz:Path,generated:Path,reference:Path)->None:
    b=generated.read_bytes()
    if len(b)!=END-START or reference.read_bytes()[START:END]!=b:
        raise ValueError('CDSP worker state source not exact relative to pinned original')
    observed=check_rizin(rz,b);llvmcount=check_llvm(llvm,b)
    print(f'CDSP worker state {len(b)} exact bytes; {observed} Rizin semantic words, '
          f'{llvmcount} LLVM6 assembled words, 4 privileged system-register words Rizin only.')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ('llvm-mc','rz-asm','rust-output','reference'):
        p.add_argument('--'+name,type=Path,required=True)
    q=p.parse_args();verify(q.llvm_mc,q.rz_asm,q.rust_output,q.reference)
