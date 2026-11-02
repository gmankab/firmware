#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check new V66 supervisor dispatcher against pinned CDSP, Rizin and LLVM6.

Original machine words are never imported into Rust emission. Rizin checks all
18 instructions; LLVM6 independently produces 13 complete A32/V66 Hexagon
instruction words, including the two-word indexed absolute table load.
The privileged SIAD and four alternate extended-immediate consumer encodings
remain semantic-decoder/pinned-reference checked only.
"""
from pathlib import Path
import argparse, hashlib, importlib.util, re, subprocess
START=0x5a60
END=0x5aa8
PC=0x96c02a60
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'allocframe(SP,#0x0):raw',
 'immext(##0x4c4b400)','R0 = ##0x4c4b400','siad(R0)',
 'immext(##0xfe026a40)','R0 = ##-0x1fd9590',
 'immext(##0x0)','R1 = ##0x2f',
 'immext(##0x2f940)','call 0x96c323c4',
 'R0 = ##0x0','LR:FP = deallocframe(FP):raw','jumpr LR',
 'P0 = cmp.gtu(R5,#0xd); if (P0.new) jump:nt 0x96c02aa8',
 'immext(##0xfe024d00)','R8 = memw(R5<<#0x2+##0xfe024d28)',
 'jumpr R8','jumpr LR',
]
assert len(EXPECTED)==18

def normalize(s:str)->str:
    return ''.join(s.lower().split()).replace('r31','lr')

def verify(output:Path,reference:Path,rz:Path,llvm:Path)->None:
    actual=output.read_bytes();ref=reference.read_bytes()
    if hashlib.sha256(ref).hexdigest()!=SHA:raise ValueError('CDSP SHA differs from pinned revision')
    if len(actual)!=END-START or actual!=ref[START:END]:raise ValueError('V66 supervisor output bytes differ from original')
    lines=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',actual.hex()],
        check=True,capture_output=True,text=True).stdout
    parsed=[]
    for line in lines.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m: parsed.append((int(m[1],16),m[2].split('<endloop')[0]))
    if len(parsed)!=18:raise ValueError(f'Rizin expected 18 semantics, got {len(parsed)}')
    for idx,((pc,s),expected) in enumerate(zip(parsed,EXPECTED)):
        if pc!=PC+idx*4 or normalize(s)!=normalize(expected):
            raise ValueError(f'Rizin differs at {pc:#x}: {s!r} vs {expected!r}')
    p=Path(__file__).with_name('verify-early-hvx.py')
    spec=importlib.util.spec_from_file_location('hexagon_llvm_helper',p)
    if not spec or not spec.loader:raise RuntimeError('missing LLVM6 helper')
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    singles={
        0:'{allocframe(#0)}',1:'{r0=##0x4c4b400}',
        4:'{r0=##0xfe026a70}',6:'{r1=##47}',8:'{call ##0x2f944}',
        10:'{r0=#0}',11:'{deallocframe}',12:'{jumpr lr}',
        13:'{p0=cmp.gtu(r5,#13);if(p0.new)jump:nt target}\n{nop}\n{nop}\n{nop}\n{nop}\ntarget:\n{jumpr lr}',
        16:'{jumpr r8}',17:'{jumpr lr}',
    }
    count=0
    for idx,source in singles.items():
        encoded=mod.llvm_bytes(llvm,source)[:4]
        if encoded!=actual[4*idx:4*(idx+1)]:raise ValueError(f'LLVM6 mismatch word {idx}: {encoded.hex()} vs {actual[4*idx:4*(idx+1)].hex()}')
        count+=1
    pair=mod.llvm_bytes(llvm,'{r8=memw(r5<<#2+##0xfe024d28)}')
    if pair!=actual[14*4:16*4]:raise ValueError('LLVM6 indexed memw extender/body differ')
    count+=2
    if count!=13:raise ValueError(f'Expected 13 LLVM-independent complete-word checks, got {count}')
    print(f'CDSP supervisor 72 bytes exact; 18/18 Rizin V66 words; {count}/18 LLVM6 independent complete instruction words; privileged SIAD and extender consumers decoder/reference verified only')

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    for key in ('rust-output','reference','rz-asm','llvm-mc'):parser.add_argument('--'+key,type=Path,required=True)
    args=parser.parse_args()
    verify(args.rust_output,args.reference,args.rz_asm,args.llvm_mc)
