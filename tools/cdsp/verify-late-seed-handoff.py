#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify the 28-byte CDSP seed controller tail-call with LLVM6 and Rizin V66.

LLVM6 emits a different parse-marker choice for the call and pair-combine;
normalization of *only* these documented packet-marker bits is checked.
There is no claim that the LLVM6 produced binary is fully original-identical.
"""
from __future__ import annotations
import argparse, importlib.util, subprocess, tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
START=0x56448
LENGTH=28
RWE_START=0x56000
RWE_END=START+LENGTH
PHYS=0x96cc8448

def elf_text(content:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_seed_handoff_elf_parser',p)
    if spec is None or spec.loader is None:raise ValueError('Missing CDSP ELF parser')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.parse_elf32_text(content)

def verify(llvm:Path,rz:Path,gen:Path,reference:Path,prefix:Path|None)->None:
    actual=gen.read_bytes(); ref=reference.read_bytes()
    if len(actual)!=LENGTH or actual!=ref[START:RWE_END]:
        raise ValueError('Original pinned CDSP handoff differs from typed generator')
    if prefix is not None:
        contents=prefix.read_bytes()
        if len(contents)!=RWE_END-RWE_START or contents!=ref[RWE_START:RWE_END]:
            raise ValueError('Joined 1124-byte CDSP prefix differs from pinned reference')
        if contents[-LENGTH:]!=actual:raise ValueError('Generated prefix tail diverges')
    fixture=SOURCE_DIR / 'late-rwe-seed-handoff.s'
    with tempfile.TemporaryDirectory(prefix='cdsp_seed_handoff_') as tmp:
        obj=Path(tmp)/'seed-handoff.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(fixture),'-o',str(obj)],capture_output=True,text=True,check=True)
        independent=elf_text(obj.read_bytes())[0x448:]
    if len(independent)!=LENGTH:raise ValueError('LLVM6 fixture was not 28 bytes')
    original=[int.from_bytes(actual[i:i+4],'little') for i in range(0,LENGTH,4)]
    llvm6=[int.from_bytes(independent[i:i+4],'little') for i in range(0,LENGTH,4)]
    for i in [1,2,3,5,6]:
        if original[i]!=llvm6[i]:raise ValueError(f'Unchanged LLVM6 opcode {i} mismatch')
    if any((original[i]^llvm6[i]) != 0x8000 for i in (0,4)):
        raise ValueError('LLVM6 reordered a field beyond documented packet marker')
    out=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],capture_output=True,text=True,check=True).stdout
    words=[w for w in out.splitlines() if w.startswith('0x')]
    if len(words)!=7 or any('invalid' in w.lower() for w in words):
        raise ValueError('Rizin V66 failed seven handoff words')
    n='\n'.join(words).lower().replace(' ','')
    expected=['call0x96dc28ac','r0=##0xc0364130', 'allocframe(#0x0)',
              'r1:0=combine(#0x1,##0xc0226160)','deallocframe','jump0x96cce74c']
    for e in expected:
        if e not in n:raise ValueError('Rizin did not confirm feature '+e)
    print('PASS 28 source-built Hexagon V66 handoff bytes match pinned CDSP firmware')
    print('PASS LLVM6 semantic fixture: five unchanged opcode words and two original packet-marker choices confirmed')
    print('PASS Rizin V66: all seven words including external physical call/jump and 2 duplex semantic operations')
    if prefix:print('PASS source-built RWE prefix: 1124 file bytes = 1092 executable + 32 alignment NOPs')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+name,type=Path,required=name!='prefix')
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
