#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Rizin/LLVM6 verification of indexed V66 dispatch helpers.

No original binary opcodes are copied into the Rust emitter. This verifier
checks all 21 Rizin semantics and the LLVM6 encoder for all 20 ordinary
instructions. The SGP0 read uses only Rizin/reference verification.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess
START=0x5aa8;END=0x5afc;PC=0x96c02aa8
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'R0 = -1 ; jumpr R31',
 'P0 = cmp.gtu(R5,#0x5); if (P0.new) jump:nt 0x96c02af8',
 'R7 = memw(R6+##0x100)',
 'R7 = memw(R7+##0x10)',
 'if (!cmp.eq(R7.new,#0x4)) jump:nt 0x96c02ac4',
 'immext(##0xc6c0)','jump 0x96c0f1a0',
 'P0 = cmp.eq(R5,#0x2); if (P0.new) jump:nt 0x96c02ad8',
 'immext(##0xfe024d40)','R7 = memw(R5<<#0x2+##0xfe024d60)',
 'jumpr R7','jumpr LR','allocframe(SP,#0x0):raw',
 'callr R7','R6 = SGP0','R6 = memw(R6+##0x0)',
 'immext(##0x80)','R3:2 = memd(R6+##0x88)',
 'LR:FP = deallocframe(FP):raw','jumpr LR',
 'R0 = -1 ; jumpr R31',
]
assert len(EXPECTED)==21

def normalize(s:str)->str:return ''.join(s.lower().split()).replace('r31','lr')

def verify(path:Path,reference:Path,rz:Path,llvm:Path)->None:
    orig=reference.read_bytes();actual=path.read_bytes()
    if hashlib.sha256(orig).hexdigest()!=SHA:raise ValueError('wrong CDSP source revision')
    if len(actual)!=END-START or actual!=orig[START:END]:raise ValueError('dispatch helper bytes differ')
    text=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',actual.hex()],check=True,capture_output=True,text=True).stdout
    parsed=[]
    for line in text.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:parsed.append((int(m[1],16),m[2].split('<endloop')[0]))
    if len(parsed)!=21:raise ValueError(f'Rizin found {len(parsed)} instead of 21 words')
    for i,((pc,semantic),want) in enumerate(zip(parsed,EXPECTED)):
        if pc!=PC+i*4 or normalize(semantic)!=normalize(want):
            raise ValueError(f'Rizin failed at {pc:#x}: {semantic!r} != {want!r}')
    spec=importlib.util.spec_from_file_location('hexagon_llvm_helper',Path(__file__).with_name('verify-early-hvx.py'))
    if not spec or not spec.loader:raise RuntimeError('LLVM helper missing')
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    # Assemble complete instruction packets for branches. Hexagon .new
    # consumers are packet-sensitive and must not be assembled in isolation.
    samples={
        0:('{r0=#-1;jumpr lr}',1),
        1:('{p0=cmp.gtu(r5,#5);if(p0.new)jump:nt target\n r7=memw(r6+#256)}\n.space 68\ntarget:\n{jumpr lr}',2),
        3:('{r7=memw(r7+#16)\n if(!cmp.eq(r7.new,#4))jump:nt target}\n.space 8\ntarget:\n{jumpr lr}',2),
        5:('{jump ##0xc6e4}',2),
        7:('{p0=cmp.eq(r5,#2);if(p0.new)jump:nt target\n r7=memw(r5<<#2+##0xfe024d60)}\n.space 8\ntarget:\n{jumpr lr}',3),
        10:('{jumpr r7}',1),11:('{jumpr lr}',1),
        12:('{allocframe(#0)}',1),13:('{callr r7}',1),
        15:('{r6=memw(r6+#0)}',1),
        16:('{r3:2=memd(r6+##136)}',2),
        18:('{deallocframe}',1),19:('{jumpr lr}',1),
        20:('{r0=#-1;jumpr lr}',1),
    }
    confirmed=set()
    for start,(asm,num) in samples.items():
        compiled=mod.llvm_bytes(llvm,asm)
        got=compiled[:num*4];want=actual[start*4:(start+num)*4]
        if got!=want:raise ValueError(f'LLVM6 packet mismatch at index {start}: {got.hex()} vs {want.hex()}')
        confirmed.update(range(start,start+num))
    if confirmed!=set(range(21))-{14}:
        raise ValueError(f'Only {len(confirmed)} LLVM6 words confirmed, remaining {sorted(set(range(21))-confirmed)}')
    print(f'CDSP indexed dispatcher: 84 reference-exact bytes; {len(parsed)}/21 Rizin semantics; {len(confirmed)}/21 independently LLVM6-assembled words, privileged SGP0 read Rizin/reference only.')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for a in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+a,type=Path,required=True)
    a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
