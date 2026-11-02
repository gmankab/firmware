#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent V66 verification for packed 64-bit TLB descriptor builder.

Unlike the privileged TLB helper routines, these 21 instructions can all be
assembled with LLVM6. Match opcode fields excluding legal packet parse bits,
then separately verify the complete compound predicate/branch word.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess
START=0x5b6c;END=0x5bc0;PC=0x96c02b6c
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'R6 = memw(SP+##0x0)','R8 = lsr(R1,#0xc)',
 'R12 = asl(R6,#0x14)','R8 = or(R8,R12)','R8 = asl(R8,#0x1)',
 'R14 = ##0x1','R9 = asl(R14,R2)','R10 = asl(R3,#0x18)',
 'R11 = asl(R4,#0x1d)','R8 = or(R8,R9)','R8 = or(R8,R10)',
 'R8 = or(R8,R11)','R9 = lsr(R0,#0xc)','R10 = asl(R5,#0x14)',
 'P0 = cmp.eq(R5,-1); if (!P0.new) jump:t 0x96c02bb0',
 'R9 = setbit(R9,#0x1e)','R10 = ##0x0','R9 = or(R9,R10)',
 'R9 = setbit(R9,#0x1f)','R1:0 = combine(R9,R8)','jumpr LR',
]
assert len(EXPECTED)==21

def norm(s):return ''.join(s.lower().split()).replace('r31','lr')

def verify(binary:Path,reference:Path,rz:Path,llvm:Path)->None:
    b=binary.read_bytes();orig=reference.read_bytes()
    if hashlib.sha256(orig).hexdigest()!=SHA:raise ValueError('incorrect CDSP firmware revision')
    if len(b)!=END-START or b!=orig[START:END]:raise ValueError('TLB descriptor bytes not reference-exact')
    output=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',b.hex()],check=True,capture_output=True,text=True).stdout
    instructions=[]
    for line in output.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:instructions.append((int(m[1],16),m[2].split('<endloop')[0]))
    if len(instructions)!=21:raise ValueError(f'Rizin decoded {len(instructions)} of 21 words')
    for i,((addr,semantic),want) in enumerate(zip(instructions,EXPECTED)):
        if addr!=PC+i*4 or norm(semantic)!=norm(want):
            raise ValueError(f'Rizin differs at {addr:#x}: {semantic!r} vs {want!r}')
    spec=importlib.util.spec_from_file_location('llvm_helper',Path(__file__).with_name('verify-early-hvx.py'))
    if not spec or not spec.loader:raise RuntimeError('LLVM assembler helper missing')
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    samples=[
      '{r6=memw(sp+#0)}','{r8=lsr(r1,#12)}',
      '{r12=asl(r6,#20)}','{r8=or(r8,r12)}','{r8=asl(r8,#1)}',
      '{r14=#1}','{r9=asl(r14,r2)}','{r10=asl(r3,#24)}',
      '{r11=asl(r4,#29)}','{r8=or(r8,r9)}','{r8=or(r8,r10)}',
      '{r8=or(r8,r11)}','{r9=lsr(r0,#12)}','{r10=asl(r5,#20)}',
      '{p0=cmp.eq(r5,#-1);if(!p0.new)jump:t target}\n{nop}\n{nop}\ntarget:\n{jumpr lr}',
      '{r9=setbit(r9,#30)}','{r10=#0}','{r9=or(r9,r10)}',
      '{r9=setbit(r9,#31)}','{r1:0=combine(r9,r8)}','{jumpr lr}',
    ]
    for i,asm in enumerate(samples):
        encoded=mod.llvm_bytes(llvm,asm)
        if len(encoded)<4:raise ValueError(f'LLVM opcode missing at {i}')
        got=int.from_bytes(encoded[:4],'little')&~0xc000
        expected=int.from_bytes(b[4*i:4*i+4],'little')&~0xc000
        if got!=expected:raise ValueError(f'LLVM mismatch at {i}: {got:#x} vs {expected:#x}')
    # The packetized compound branch is entirely byte-identical including
    # destination, not just the masked operand fields.
    assert mod.llvm_bytes(llvm,samples[14])[:4]==b[56:60]
    print(f'CDSP 64-bit TLB entry construction: 84 exact bytes; 21/21 Rizin V66 semantics; 21/21 LLVM6 independently assembled instruction opcode fields (compound branch fully exact).')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for arg in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+arg,type=Path,required=True)
    a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
