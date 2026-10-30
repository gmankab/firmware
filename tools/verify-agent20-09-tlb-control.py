#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify CDSP TLB probe/read/write helpers and trailing 44-byte ELF fill.

21 Rizin V66 semantic words, 12 LLVM6-reassembled ordinary word encodings.
Privileged TLB registers are only decoder/reference validated.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess
START=0x5bc0;END=0x5c14;PAD_END=0x5c40;PC=0x96c02bc0
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'tlblock','R1 = asl(R1,#0x14)','R0 = lsr(R0,#0xc)',
 'R1 = insert(R0,#0x14,#0x0)','R0 = tlbp(R1)','tlbunlock',
 'P0 = tstbit(R0,#0x1f)','if (P0.new) jump:nt 0x96c02be4',
 'jumpr LR','R0 = ##-0x1','jumpr LR',
 'tlblock','tlbw(R3:2,R0)','isync','tlbunlock',
 'R0 = ##0x0','jumpr LR',
 'tlblock','R1:0 = tlbr(R0)','tlbunlock','jumpr LR',
]
PRIVILEGED={0,4,5,11,12,14,17,18,19}
assert len(EXPECTED)==21

def norm(s):return ''.join(s.lower().split()).replace('r31','lr')

def verify(binary:Path,reference:Path,rz:Path,llvm:Path)->None:
    orig=reference.read_bytes();out=binary.read_bytes()
    if hashlib.sha256(orig).hexdigest()!=SHA:raise ValueError('CDSP SHA mismatch')
    if len(out)!=END-START or orig[START:END]!=out:raise ValueError('TLB control instruction bytes mismatch')
    if orig[END:PAD_END]!=bytes(PAD_END-END):raise ValueError('44 alignment bytes not zero')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',out.hex()],check=True,capture_output=True,text=True).stdout
    pairs=[]
    for line in decoded.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:pairs.append((int(m[1],16),m[2].split('<endloop')[0]))
    if len(pairs)!=21:raise ValueError(f'Rizin decoded {len(pairs)} of 21 words')
    for i,((pc,op),want) in enumerate(zip(pairs,EXPECTED)):
        if pc!=PC+i*4 or norm(op)!=norm(want):raise ValueError(f'Rizin difference at {pc:#x}: {op!r} vs {want!r}')
    spec=importlib.util.spec_from_file_location('helper',Path(__file__).with_name('verify-agent20-09-hvx.py'))
    if not spec or not spec.loader:raise RuntimeError('missing LLVM6 helper')
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    singles={
        1:'{r1=asl(r1,#20)}',2:'{r0=lsr(r0,#12)}',3:'{r1=insert(r0,#20,#0)}',
        8:'{jumpr lr}',9:'{r0=#-1}',10:'{jumpr lr}',13:'{isync}',
        15:'{r0=#0}',16:'{jumpr lr}',20:'{jumpr lr}',
    }
    verified=set()
    for i,asm in singles.items():
        b=mod.llvm_bytes(llvm,asm)
        if len(b)!=4 or b[:4]!=out[4*i:4*i+4]:raise ValueError(f'LLVM exact mismatch for word {i}')
        verified.add(i)
    branch=mod.llvm_bytes(llvm,'{p0=tstbit(r0,#31);if(p0.new)jump:nt target}\n{nop}\ntarget:\n{jumpr lr}')
    if branch[:8]!=out[6*4:8*4]:raise ValueError('LLVM predicate consumer packet mismatch')
    verified.update((6,7))
    if verified != set(range(21))-PRIVILEGED:raise ValueError(f'LLVM verified {len(verified)} words, expected 12')
    print(f'CDSP TLB probe/read/write: {len(out)} exact code bytes, {PAD_END-END} separate alignment zeros; 21 Rizin V66 semantics and {len(verified)} LLVM6 exact instruction words. Privileged TLB ops Rizin/reference only.')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for arg in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+arg,type=Path,required=True)
    a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
