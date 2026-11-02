#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V66 verification for CDSP TLB writer, reader, cleaner.
Both privileged TLBW and TLBR are Rizin/reference verified only. All
remaining packet forms are independently reassembled by LLVM6 where possible.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess
START=0x5990;END=0x59f0;PC=0x96c02990
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
'R4 = ##0x7f','immext(##0xfe024900)','R1 = ##0xfe024900 ; R0 = ##0x0',
'R3:2 = memd(R1++#0x8)','tlbw(R3:2,R0)',
'P0 = cmp.eq(R0,R4); if (!P0.new) jump:t 0x96c0299c','R0 = add(R0,##0x1)',
'jumpr LR','R0 = ##0x80','immext(##0xfe024900)','R1 = ##-0x1fdb700',
'R4 = lsr(R0,#0x2)','loop0(0x96c029c8,R0)','R0 = ##0x0',
'R3:2 = tlbr(R0)','R0 = add(R0,##0x1)','memd(R1++#0x8) = R3:2',
'immext(##0xfe024900)','R1 = ##-0x1fdb700','loop0(0x96c029e0,R4)',
'R1 = add(R1,##0x20)','dccleana(R1)','jumpr LR','jumpr LR',
]
assert len(EXPECTED)==24

def normalized(s):return ''.join(s.lower().split()).replace('r31','lr')

def verify(ours:Path,reference:Path,rz:Path,mc:Path):
    data=ours.read_bytes();ref=reference.read_bytes()
    if hashlib.sha256(ref).hexdigest()!=SHA:raise ValueError('CDSP reference SHA mismatch')
    if len(data)!=END-START or data!=ref[START:END]:raise ValueError('non-byte-exact TLB state output')
    output=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',data.hex()],check=True,capture_output=True,text=True).stdout
    parsed=[]
    for line in output.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:parsed.append((int(m[1],16),m[2].split('< endloop')[0]))
    if len(parsed)!=24:raise ValueError(f'expected 24 Rizin words, got {len(parsed)}')
    for i,((address,semantic),expect) in enumerate(zip(parsed,EXPECTED)):
        if address!=PC+i*4 or normalized(semantic)!=normalized(expect):
            raise ValueError(f'Rizin discrepancy at {address:#x}: {semantic!r} != {expect!r}')
    import importlib.util
    helper_path=Path(__file__).with_name('verify-early-hvx.py')
    spec=importlib.util.spec_from_file_location('hvx_helper',helper_path)
    if not spec or not spec.loader:raise ValueError('HVX helper missing')
    helper=importlib.util.module_from_spec(spec);spec.loader.exec_module(helper)
    samples={
      0:'{r4=#127}',
      3:'{r3:2=memd(r1++#8)}',
      6:'{r0=add(r0,#1)}',
      7:'{jumpr lr}',
      8:'{r0=#128}',
      11:'{r4=lsr(r0,#2)}',
      13:'{r0=#0}',
      15:'{r0=add(r0,#1)}',
      16:'{memd(r1++#8)=r3:2}',
      20:'{r1=add(r1,#32)}',
      21:'{dccleana(r1)}',
      22:'{jumpr lr}',
      23:'{jumpr lr}',
    }
    done=set()
    for i,asm in samples.items():
        b=helper.llvm_bytes(mc,asm)
        if len(b)!=4:raise ValueError(f'LLVM sample at {i} emitted {len(b)} bytes')
        got=int.from_bytes(b,'little')&~0xc000
        want=int.from_bytes(data[4*i:4*i+4],'little')&~0xc000
        if got!=want:raise ValueError(f'LLVM field mismatch at word {i}: {got:#x}!={want:#x}')
        done.add(i)
    for asm,idx in [('{r1=##0xfe024900\n r0=#0}',[1,2]),
                    ('{r1=##0xfe024900}',[9,10]),
                    ('{r1=##0xfe024900}',[17,18])]:
        b=helper.llvm_bytes(mc,asm)
        if len(b)!=4*len(idx):raise ValueError('LLVM relocation/duplex packet length mismatch')
        for j,i in enumerate(idx):
            got=int.from_bytes(b[4*j:4*j+4],'little')&~0xc000
            want=int.from_bytes(data[4*i:4*i+4],'little')&~0xc000
            if got!=want:raise ValueError(f'LLVM duplex or extender mismatch word {i}: {got:#x} vs {want:#x}')
            done.add(i)
    # Loop labels are forward by 8 and 4 bytes, respectively.
    for i,asm in [(12,'{loop0(dest,r0)\n r0=#0}\ndest:\n{nop}'),
                  (19,'{loop0(dest,r4)}\ndest:\n{nop}')]:
        b=helper.llvm_bytes(mc,asm)
        got=int.from_bytes(b[:4],'little')&~0xc000
        want=int.from_bytes(data[4*i:4*i+4],'little')&~0xc000
        if got!=want:raise ValueError(f'LLVM loop target mismatch at {i}: {got:#x} vs {want:#x}')
        done.add(i)
    # Need the -8-byte predicate branch with two predecessor bytes in memory.
    branch='prior:\n{nop}\n{nop}\n{p0=cmp.eq(r0,r4);if(!p0.new)jump:t prior\n r0=add(r0,#1)}'
    b=helper.llvm_bytes(mc,branch)
    if len(b)!=16:raise ValueError('LLVM predicate backward-branch group incomplete')
    for j,i in enumerate([5,6]):
        got=int.from_bytes(b[8+j*4:12+j*4],'little')&~0xc000
        want=int.from_bytes(data[4*i:4*i+4],'little')&~0xc000
        if got!=want:raise ValueError(f'LLVM predicate grouped mismatch at {i}: {got:#x} vs {want:#x}')
        done.add(i)
    if len(done)<20:raise ValueError(f'Only {len(done)} LLVM words checked')
    print(f'CDSP TLB helpers: {len(data)} exact bytes; {len(parsed)} Rizin V66 semantics; {len(done)} LLVM6 reassembled instruction fields; privileged TLBW/TLBR decoder/reference only.')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for a in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+a,type=Path,required=True)
    a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
