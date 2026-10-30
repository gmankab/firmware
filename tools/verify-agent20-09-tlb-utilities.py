#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check CDSP early TLB lookup/invalidate helpers independently (V66).

28 Rizin semantics and 18 ordinary LLVM6 operand encodings. The ten TLB
lock/read/write/unlock instructions are privileged and not LLVM6-assembled.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess
START=0x5afc;END=0x5b6c;PC=0x96c02afc
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'R1 = asl(R1,#0x14)','R0 = lsr(R0,#0xc)','R1 = insert(R0,#0x14,#0x0)',
 'tlblock','R0 = tlbp(R1)','P0 = tstbit(R0,#0x1f)','R11 = ##0x0',
 'R0 = extractu(R0,#0x8,#0x0)','if (P0) jump:nt 0x96c02b30',
 'R3:2 = tlbr(R0)','R3 = clrbit(R3,#0x1f)','tlbw(R3:2,R0)','isync',
 'tlbunlock','jumpr LR',
 'R5 = asl(R1,#0x14)','R4 = lsr(R0,#0xc)','R5 = insert(R4,#0x14,#0x0)',
 'tlblock','R6 = tlbp(R5)','P0 = tstbit(R6,#0x1f)',
 'R6 = extractu(R6,#0x8,#0x0)','if (P0) jump:nt 0x96c02b64',
 'R1:0 = tlbr(R6)','tlbunlock','jumpr LR','tlbunlock',
 'R0 = ##0x0 ; jumpr R31',
]
assert len(EXPECTED)==28
PRIVILEGED={3,4,9,11,13,18,19,23,24,26}

def norm(s):return ''.join(s.lower().split()).replace('r31','lr')

def verify(binary:Path,reference:Path,rz:Path,llvm:Path)->None:
    data=binary.read_bytes();orig=reference.read_bytes()
    if hashlib.sha256(orig).hexdigest()!=SHA:raise ValueError('reference revision SHA mismatch')
    if len(data)!=END-START or data!=orig[START:END]:raise ValueError('CDSP TLB utility output differs')
    text=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',data.hex()],check=True,capture_output=True,text=True).stdout
    pairs=[]
    for line in text.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:pairs.append((int(m[1],16),m[2].split('<endloop')[0]))
    if len(pairs)!=28:raise ValueError(f'Rizin parsed {len(pairs)} of 28 words')
    for i,((addr,semantic),want) in enumerate(zip(pairs,EXPECTED)):
        if addr!=PC+i*4 or norm(semantic)!=norm(want):
            raise ValueError(f'Rizin differs at {addr:#x}: {semantic!r} != {want!r}')
    spec=importlib.util.spec_from_file_location('h',Path(__file__).with_name('verify-agent20-09-hvx.py'))
    if not spec or not spec.loader:raise RuntimeError('LLVM helper missing')
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    samples={
      0:'{r1=asl(r1,#20)}',1:'{r0=lsr(r0,#12)}',2:'{r1=insert(r0,#20,#0)}',
      5:'{p0=tstbit(r0,#31)}',6:'{r11=#0}',
      10:'{r3=clrbit(r3,#31)}',12:'{isync}',14:'{jumpr lr}',
      15:'{r5=asl(r1,#20)}',16:'{r4=lsr(r0,#12)}',17:'{r5=insert(r4,#20,#0)}',
      20:'{p0=tstbit(r6,#31)}',25:'{jumpr lr}',
      27:'{r0=#0;jumpr lr}',
    }
    confirmed=set()
    for i,asm in samples.items():
        b=mod.llvm_bytes(llvm,asm)
        if len(b)!=4:raise ValueError(f'LLVM sample length mismatch index {i}')
        # Single-op compiler can pick a different legal end-of-packet marker.
        got=int.from_bytes(b,'little')&~0xc000
        want=int.from_bytes(data[4*i:4*i+4],'little')&~0xc000
        if got!=want:raise ValueError(f'LLVM V66 opcode-field mismatch index {i}: {got:#x} vs {want:#x}')
        confirmed.add(i)
    groups={
       7:'{r0=extractu(r0,#8,#0);if(p0)jump:nt target}\n.space 16\ntarget:\n{jumpr lr}',
       21:'{r6=extractu(r6,#8,#0);if(p0)jump:nt target}\n.space 12\ntarget:\n{jumpr lr}',
    }
    for i,asm in groups.items():
        b=mod.llvm_bytes(llvm,asm)
        if b[:8]!=data[4*i:4*(i+2)]:raise ValueError(f'LLVM grouped predicate differs at index {i}')
        confirmed.update((i,i+1))
    if confirmed!=set(range(28))-PRIVILEGED:
        raise ValueError(f'LLVM coverage only {len(confirmed)}; skipped {sorted(set(range(28))-confirmed)}')
    print(f'CDSP early TLB helpers: {len(data)} exact bytes; 28/28 Rizin V66 semantics; {len(confirmed)}/28 LLVM6 instruction opcode fields; ten privileged ops decoder/reference only.')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for arg in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+arg,type=Path,required=True)
    a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
