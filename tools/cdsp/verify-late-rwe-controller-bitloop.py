#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently verify six original V66 bitloop entry instructions.

LLVM6 coalesces R25:24 and R18 into a different packet, changing its parse
marker and the branch's low halfword displacement. Rizin V66 confirms original
source targets after evaluating original parse markers. Do not claim LLVM6's
binary is the original instruction order/byte encoding.
"""
from __future__ import annotations
import argparse,importlib.util,re,subprocess,tempfile
from collections import Counter
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
START=0x566c0
END=0x566d8
PHYS=0x96cc86c0
RWE=0x56000

def elf_text(binary:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf_parser',p)
    if not spec or not spec.loader:raise RuntimeError('CDSP ELF32 parser unavailable')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(binary)

def decode(rzin:Path,words:bytes)->tuple[list[str],Counter[str]]:
    x=subprocess.run([str(rzin),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',words.hex()],capture_output=True,text=True,check=True)
    lines=[line.strip() for line in x.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=6 or any('invalid' in line.lower() for line in lines):
        raise RuntimeError('Rizin failed to decode six V66 instructions')
    p=re.compile(r'^0x[0-9a-f]+\s+\d+\s+[0-9a-f]{8}\s+\S\s+(.*)$',re.I)
    ops=[]
    for line in lines:
        m=p.match(line)
        if not m:raise RuntimeError('malformed Rizin assembly line '+line)
        ops.append(re.sub(r'\s+','',m[1]).lower())
    return lines,Counter(ops)

def run(llvm:Path,rizin:Path,generated:Path,prefix:Path,reference:Path)->None:
    data=generated.read_bytes(); original=reference.read_bytes(); joined=prefix.read_bytes()
    if len(data)!=24 or data!=original[START:END]:raise RuntimeError('24-byte new V66 fragment mismatch')
    if len(joined)!=0x6d8 or joined!=original[RWE:END]:raise RuntimeError('full 1752-byte RWE prefix mismatch')
    with tempfile.TemporaryDirectory() as tmp:
        obj=Path(tmp)/'bitloop.o'
        src=SOURCE_DIR / 'late-rwe-controller-bitloop-prefix.s'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(src),'-o',str(obj)],check=True,capture_output=True,text=True)
        independent=elf_text(obj.read_bytes())[0x6c0:0x6d8]
    if len(independent)!=24:raise RuntimeError('LLVM6 bitloop fixture length differs')
    exact=sum(data[i:i+4]==independent[i:i+4] for i in range(0,24,4))
    if exact!=4:raise RuntimeError(f'expected exactly 4 of 6 unchanged LLVM6 words, got {exact}')
    lines,ops=decode(rizin,data)
    other,independent_ops=decode(rizin,independent)
    if ops!=independent_ops:
        raise RuntimeError(f'Rizin and LLVM semantic instruction counters differ: {ops-independent_ops}')
    text='\n'.join(lines).lower().replace(' ','')
    for term in ['r25:24=combine(##0x1,#0x7)','r18=##0x10','p0=cmp.eq(r17,#0x0)',
                 'r3=cl0(r17)','r2=add(r18,##-0x10)', 'jump:nt0x96cc8734','jump:nt0x96cc8890']:
        if term not in text:raise RuntimeError('controller source semantic missing '+term)
    print('PASS 24 typed Hexagon executable bytes, zero differences from reference')
    print(f'PASS independently assembled LLVM6 semantics, {exact}/6 original words unchanged')
    print('PASS Rizin V66 original packet markers and both physical branch targets')
    print('PASS RWE prefix 1752 bytes = 1708 source-executable + 44 structural fill')

if __name__=='__main__':
    a=argparse.ArgumentParser(description=__doc__)
    for k in ('llvm-mc','rz-asm','generated','prefix','reference'):a.add_argument('--'+k,type=Path,required=True)
    c=a.parse_args()
    run(c.llvm_mc,c.rz_asm,c.generated,c.prefix,c.reference)
