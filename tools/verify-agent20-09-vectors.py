#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Validate CDSP supervisor exception vectors: 87 V66 decoded instructions.

Checks packet grouping of 60 executable NOPs and seven vector branches.
Six supported LLVM6 assembly groups are checked independently; privileged
register copy/restore and start() instructions remain Rizin-only corroborated.
"""
import argparse, importlib.util,re,subprocess,tempfile
from pathlib import Path

OFFSET=0x45c0
PHYSICAL=0x96c0_15c0
SIZE=0x15c
NOP_OFFSET=0x4610
NOP_COUNT=60
VECTOR_OFFSET=0x4700
EXPECTED={
    0x45c0:'immext(##0xfe001700)',
    0x45c4:'r1=add(r23,##-0x1ffe900)',
    0x45c8:'evb=r1',
    0x45cc:'call0x96c00d30',
    0x45d0:'immext(##0xfe016d40)',
    0x45d4:'r25=memw(r23+##-0x1fe92bc)',
    0x45d8:'p0=cmp.eq(r25,##0x2)',
    0x45dc:'p1=cmp.eq(r25,##0x0)',
    0x45e0:'if(p0)jump:nt0x96c01ab0',
    0x45e4:'if(!p1)jump:nt0x96c01ac0',
    0x45e8:'r24=pmuevtcfg',
    0x45ec:'p3:0=r24',
    0x45f0:'r1:0=s49:48',
    0x45f4:'r25:24=s45:44',
    0x45f8:'sp=s47',
    0x45fc:'lr:fp=s51:50',
    0x4600:'immext(##0xffffffc0)',
    0x4604:'r23=##-0x1',
    0x4608:'start(r23)',
    0x460c:'jump0x96c0171c',
    0x4700:'jump0x96c0171c',
}
ASM='''.text
.global sample
sample:
{ r1=add(r23,##-0x1ffe900) }
{ r25=memw(r23+##-0x1fe92bc) }
{ p0=cmp.eq(r25,#2) }
{ p1=cmp.eq(r25,#0) }
{ r23=##-1 }
{ nop }
{ jump target }
{ nop }
target:
{ nop }
'''

def parse_elf(path:Path)->bytes:
    src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('agent20_cdsp_supervisor_elf',src)
    assert spec and spec.loader
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod.parse_elf32_text(path.read_bytes())

def test(llvm:Path,rz:Path,source:Path,reference:Path):
    data=source.read_bytes()
    if len(data)!=SIZE:raise ValueError(f'Expected {SIZE} V66 bytes, got {len(data)}')
    if data!=reference.read_bytes()[OFFSET:OFFSET+SIZE]:
        raise ValueError('supervisor exception-vector code does not equal pinned reference')
    result=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],capture_output=True,text=True,check=True)
    decoded={}
    for line in result.stdout.splitlines():
        m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+[?\\/\[|]\s+(.*)',line)
        if m:decoded[int(m[1],16)]=''.join(m[2].lower().split())
    if len(decoded)!=SIZE//4:raise ValueError(f'Only {len(decoded)} of {SIZE//4} V66 words decode')
    for a,expected in EXPECTED.items():
        actual=decoded.get(PHYSICAL+a-OFFSET,'')
        if expected not in actual:raise ValueError(f'Hexagon decode differs at file {a:#x}: {actual!r} vs {expected!r}')
    for i in range(NOP_COUNT):
        a=PHYSICAL+NOP_OFFSET-OFFSET+i*4
        if 'nop' not in decoded.get(a,''):
            raise ValueError(f'Misaligned/missing Hexagon NOP at {a:#x}')
        word=int.from_bytes(data[NOP_OFFSET-OFFSET+i*4:NOP_OFFSET-OFFSET+i*4+4],'little')
        expected_marker=(0xc000 if i in (2,3) or (i>=4 and (i-4)%4==3) else 0x4000)
        if word&0xc000 != expected_marker:
            raise ValueError(f'Hexagon NOP group end marker differs at {a:#x}')
    for i in range(7):
        a=PHYSICAL+VECTOR_OFFSET-OFFSET+i*4
        target=PHYSICAL+0x471c-OFFSET if i==0 else a
        expected=f'jump0x{target:08x}'
        if expected not in decoded.get(a,''):
            raise ValueError(f'Unexpected vector at {a:#x}: {decoded.get(a)!r}')
    with tempfile.TemporaryDirectory(prefix='cdsp-vectors-llvm-') as temp:
        asm=Path(temp)/'check.s';obj=Path(temp)/'check.o';asm.write_text(ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(asm)],capture_output=True,text=True,check=True)
        independent=parse_elf(obj)
    # Map instructions in the independent LLVM6 suite to their original slots.
    # Parser markers may differ (one shared packet versus separate packets),
    # so restrict comparison to operand fields.
    for dst,src in [(0x45c0,0),(0x45c4,4),(0x45d0,8),(0x45d4,12),
                    (0x45d8,16),(0x45dc,20),(0x4600,24),(0x4604,28),
                    (0x4610,32),(0x4700,36)]:
        word=int.from_bytes(data[dst-OFFSET:dst-OFFSET+4],'little')
        other=int.from_bytes(independent[src:src+4],'little')
        # The standalone LLVM short-jump offset here targets a different
        # local label, so only validate its opcode family, not target bits.
        if dst==0x4700:
            if (word&0xfc00_0000)!=(other&0xfc00_0000):
                raise ValueError('LLVM6 different jump opcode family')
        elif (word&~0xc000)!=(other&~0xc000):
            raise ValueError(f'Independent LLVM6 operand encoding mismatch at {dst:#x}')
    print('CDSP exception vectors: 348/348 exact bytes, 87 V66 words, 60 executable NOPs, 7 jumps, 10 independent LLVM6 instruction field checks. Privileged register operations decoder-correlated only.')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path,required=True)
    a=p.parse_args();test(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
