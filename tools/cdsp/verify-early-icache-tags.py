#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent CDSP V66 I-cache tag sweep validation.

1. SHA-256 pins the original CDSP revision and compares all 268 source-built
   bytes with the reference. 2. Rizin independently decodes *every one* of
   the 67 packet words. 3. LLVM6 independently reassembles ordinary instruction
   samples, including forced extended calls and the new-predicate branch.
   Unsupported privileged IC* / SYSCFG words are *not* called LLVM-verified.
"""
from __future__ import annotations
import argparse, hashlib, importlib.util, re, subprocess, tempfile, os
from pathlib import Path
START=0x5298;END=0x53a4;PC=0x96c0_2298
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED='R7 = ##0x0\nimmext(##0xfe016e00)\nR8 = ##-0x1fe91d0\nimmext(##0xfe037dc0)\nR1 = memw(GP+##0xfe037dc0)\nR8 = add(R8,##0x4)\nP3 = cmp.eq(R1,##0x80)\nimmext(##0xfe023900)\nR14 = ##-0x1fdc700\nimmext(##0xfe01b900)\nR15 = ##-0x1fe4700\nR0 = SYSCFG\nR0 = clrbit(R0,#0x1)\nSYSCFG = R0\nisync\nif (P3) R10 = ##0x80\nif (!P3) R10 = ##0x100\nloop1(0x96c022e0,R10)\nR10 = LC1\nR12 = add(R10,##-0x1)\nR10 = LC0\nR13 = add(R10,##-0x1)\nR10 = add(R10,##-0x1)\nR1 = ictagr(R10)\nP0 = tstbit(R1,#0x1)\nif (!P0.new) jump:nt 0x96c0235c\nR4 = ##0x0\nR4 = ##0x0\nR1 = icdatar(R10)\nP0 = cmp.gt(R7,#0x2); if (P0.new) jump:t 0x96c02350\nR0 = R1\nimmext(##0x140)\ncall 0x96c02488\nR16 = and(R0,R1)\nR0 = R6\nimmext(##0x140)\ncall 0x96c02488\nR11 = and(R0,R6)\nP0 = cmp.eq(R16,R11)\nif (P0.new) jump:t 0x96c02350\nmemw(R8++#0x4) = R3\nmemw(R8++#0x4) = R2\nmemw(R8++#0x4) = R10\nmemw(R8++#0x4) = R6\nmemw(R8++#0x4) = R1\nR7 = add(R7,##0x1)\nR10 = add(R10,##0x4)\nR4 = add(R4,##0x1)\nif (!cmp.eq(R4.new,#0x8)) jump:t 0x96c02308\nnop\nnop\nnop\nR1 = SYSCFG\nR1 = setbit(R1,#0x1)\nSYSCFG = R1\nisync\nR1 = ##0x2\nimmext(##0xfe016e00)\nmemw(R0=##0xfe016e20) = R1\ndccleana(R0)\nR0 = add(R0,##0x20)\ndccleana(R0)\nimmext(##0xfe016e00)\nmemw(R0=##0xfe016e30) = R7\ndccleana(R0)\nLR:FP = deallocframe(FP):raw\njumpr LR'.splitlines()

# Distinct independent LLVM6 source samples; they are assembly mnemonics,
# not reference instruction words. Packet parse bits may differ when LLVM
# builds a sample as a singleton rather than inside its original packet.
SAMPLE_CASES=[

 ('{ r7=#0 }',[0]),
 ('{ r8=##0xfe016e30 }',[1,2]),
 ('{ r8=add(r8,#4)\n p3=cmp.eq(r1,#128) }',[5,6]),
 ('{ r14=##0xfe023900\n r15=##0xfe01b900 }',[7,8,9,10]),
 ('{ r0=clrbit(r0,#1) }',[12]),
 ('{ isync }',[14]),
 ('{ if (p3) r10=#128\n if (!p3) r10=#256 }',[15,16]),
 ('{ loop1(dest,r10) }\ndest:\n{ nop }',[17]),
 ('{ r12=add(r10,#-1) }',[19]),
 ('{ r13=add(r10,#-1) }',[21]),
 ('{ r10=add(r10,#-1) }',[22]),
 ('{ p0=tstbit(r1,#1) }',[24]),
 ('{ r4=#0 }',[26]),
 ('{ r0=r1 }',[30]),
 ('{ r16=and(r0,r1) }',[33]),
 ('{ r0=r6 }',[34]),
 ('{ r11=and(r0,r6) }',[37]),
 ('{ p0=cmp.eq(r16,r11) }',[38]),
 ('{ memw(r8++#4)=r3 }',[40]),
 ('{ memw(r8++#4)=r2 }',[41]),
 ('{ memw(r8++#4)=r10 }',[42]),
 ('{ memw(r8++#4)=r6 }',[43]),
 ('{ memw(r8++#4)=r1 }',[44]),
 ('{ r7=add(r7,#1) }',[45]),
 ('{ r10=add(r10,#4) }',[46]),
 ('{ r4=add(r4,#1) }',[47]),
 ('{ nop }',[49]),
 ('{ r1=setbit(r1,#1) }',[53]),
 ('{ r1=#2 }',[56]),
 ('{ memw(r0=##0xfe016e20)=r1 }',[57,58]),
 ('{ dccleana(r0) }',[59]),
 ('{ r0=add(r0,#32) }',[60]),
 ('{ memw(r0=##0xfe016e30)=r7 }',[62,63]),
 ('{ deallocframe }',[65]),
 ('{ jumpr lr }',[66]),
 ('{ call ##0x174 }',[31,32]),
 ('{ call ##0x164 }',[35,36]),
 ('{ r10=lc1 }',[18]),
 ('{ r10=lc0 }',[20]),
 ('{ p0=tstbit(r1,#1)\nif(!p0.new)jump:nt dest }\n.space 92\ndest:\n{ jumpr lr }',[24,25]),
 ('{ p0=cmp.gt(r7,#2);if(p0.new)jump:t dest }\n.space 64\ndest:\n{ jumpr lr }',[29]),
]


def assemble(mc:Path,source:str)->bytes:
    with tempfile.TemporaryDirectory(prefix='icache-llvm6-') as td:
        td=Path(td);(td/'s.s').write_text('.text\n.global entry\nentry:\n'+source+'\n')
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj','-o',str(td/'s.o'),str(td/'s.s')],capture_output=True,text=True)
        if p.returncode:raise ValueError('LLVM6 failed assembling '+repr(source)+': '+p.stderr)
        spec=importlib.util.spec_from_file_location('elf_reader',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
        if not spec or not spec.loader:raise ValueError('Missing ELF text helper')
        helper=importlib.util.module_from_spec(spec);spec.loader.exec_module(helper)
        return helper.parse_elf32_text((td/'s.o').read_bytes())


def verify(generated:Path,reference:Path,rz:Path,mc:Path):
    ref=reference.read_bytes();ours=generated.read_bytes()
    if hashlib.sha256(ref).hexdigest()!=SHA:raise ValueError('Wrong CDSP firmware SHA-256')
    if len(ours)!=END-START or ours!=ref[START:END]:
        raise ValueError('CDSP I-cache tag executable mismatch or incorrect length')
    q=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',ours.hex()],
        capture_output=True,text=True,check=True)
    rows=[]
    for line in q.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:rows.append((int(m[1],16),''.join(m[2].lower().split()).split('<endloop')[0]))
    if len(rows)!=len(EXPECTED):raise ValueError(f'Rizin returned {len(rows)} words instead of 67')
    for i,((address,decoded),expected) in enumerate(zip(rows,EXPECTED)):
        if address!=PC+i*4 or ''.join(expected.lower().split()) not in decoded:
            raise ValueError(f'Rizin mismatch word {i} at {address:#x}: {decoded} != {expected}')
    matched=set()
    for source,indices in SAMPLE_CASES:
        sample=assemble(mc,source)
        if len(sample)<4*len(indices):raise ValueError('LLVM6 source shorter than the expected sample')
        for i,at in enumerate(indices):
            got=int.from_bytes(sample[i*4:i*4+4],'little')&~0xc000
            expected=int.from_bytes(ours[at*4:at*4+4],'little')&~0xc000
            if got!=expected:raise ValueError(f'LLVM6 independent mismatch at word {at}: {got:#x} != {expected:#x}')
            matched.add(at)
    if len(matched)<50:raise ValueError(f'Only {len(matched)} LLVM6 confirmed words, expected >=50')
    print(f'CDSP I-cache sweep: {len(ours)} exact bytes; all {len(rows)} Rizin V66 words; {len(matched)} LLVM6-tested instruction fields. Privileged forms Rizin/reference only.')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for arg in ('rust-output','reference','rz-asm','llvm-mc'):
        p.add_argument('--'+arg,required=True,type=Path)
    a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
