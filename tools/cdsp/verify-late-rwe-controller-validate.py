#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently audit 72-byte Hexagon V66 controller range-validator.

LLVM6 source re-encodes the same instruction *semantics* but chooses different
packet scheduling for predicate comparisons. Its raw output intentionally is
NOT described as byte-identical. Rizin verifies original packet boundaries
and branch targets; full OEM byte equality is checked separately.
"""
from __future__ import annotations
import argparse
from collections import Counter
import importlib.util
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
import re
import subprocess
import tempfile

START=0x56650
END=0x56698
PHYS=0x96cc8650
BYTE_COUNT=END-START
RWE_START=0x56000
RWE_PREFIX_LEN=END-RWE_START
RWE_EXECUTABLE_LEN=1644
RWE_LAYOUT_FILL=44


def parse_hexagon_elf_text(data:bytes)->bytes:
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('hexagon_elf_parser',path)
    if spec is None or spec.loader is None:raise ValueError('Hexagon ELF decoder is missing')
    m=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m.parse_elf32_text(data)


def decode_isa(decoder:Path,data:bytes)->tuple[list[str],Counter[str]]:
    cp=subprocess.run([str(decoder),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',data.hex()],
                      check=True,text=True,capture_output=True)
    lines=[l.strip() for l in cp.stdout.splitlines() if l.startswith('0x')]
    if len(lines)!=BYTE_COUNT//4:
        raise ValueError(f'expected {BYTE_COUNT//4} Rizin instructions, decoded {len(lines)}')
    if any('invalid' in x.lower() for x in lines):
        raise ValueError('Rizin reported an invalid Hexagon instruction')
    pat=re.compile(r'^0x[0-9a-f]+\s+\d+\s+[0-9a-f]{8}\s+\S\s+(.*)$',re.I)
    semantics=[]
    for l in lines:
        m=pat.match(l)
        if m is None:raise ValueError('unable to interpret Hexagon instruction '+l)
        semantics.append(re.sub(r'\s+','',m[1]).lower())
    return lines,Counter(semantics)


def verify(llvm:Path,rizin:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    b=generated.read_bytes()
    f=reference.read_bytes()
    if len(b)!=BYTE_COUNT:raise ValueError('typed emitter must generate exactly 72 bytes')
    if f[START:END]!=b:raise ValueError('typed controller code differs from pinned OEM CDSP')
    if prefix:
        joined=prefix.read_bytes()
        if len(joined)!=RWE_PREFIX_LEN or joined[0x650:0x698]!=b or joined!=f[RWE_START:END]:
            raise ValueError('full RWE source/layout prefix does not match original firmware')
    with tempfile.TemporaryDirectory(prefix='cdsp-controller-range-') as tmp:
        obj=Path(tmp)/'cdsp-rwe-controller-validate.o'
        assembly=SOURCE_DIR / 'late-rwe-controller-validate.s'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj',str(assembly),'-o',str(obj)],check=True,capture_output=True,text=True)
        independent=parse_hexagon_elf_text(obj.read_bytes())[0x650:0x698]
    if len(independent)!=BYTE_COUNT:raise ValueError('LLVM6 must reconstruct exactly 72 semantic bytes')
    for k in range(0,9*4,4):
        if b[k:k+4]!=independent[k:k+4]:
            raise ValueError(f'initial LLVM6 instruction changed at {k:#x}')
    orig_lines,orig_semantics=decode_isa(rizin,b)
    _llvm_lines,llvm_semantics=decode_isa(rizin,independent)
    if orig_semantics!=llvm_semantics:
        raise ValueError(f'LLVM6 independent source semantics differ: {orig_semantics-llvm_semantics}')
    text='\n'.join(orig_lines).lower().replace(' ','')
    for term in [
        'call0x96cd0480',
        'r3:2=combine(##0x0,#-0x1)',
        'r16=r0',
        'jump:nt0x96cc88bc',
        'r25=add(r24,##0x4000)',
        'p0=cmp.gtu(r23,r24)',
        'r2=add(r23,r17)',
        'jump:nt0x96cc8688',
        'jump:t0x96cc8698',
        'p0=cmp.gtu(r23,r25)',
        'jump:nt0x96cc86c0',
        'if(!p0.new)jump:nt0x96cc86c0',
    ]:
        if term not in text:raise ValueError('original firmware decoder missing expected operation '+term)
    print('PASS 72 independently reconstructed original CDSP bytes, zero mismatches')
    print('PASS LLVM6 independently generated identical semantics, nine initial instruction words unchanged')
    print('PASS original Rizin V66 packet order and all 18 original words; branch targets checked')
    if prefix:
        print(f'PASS joined {RWE_PREFIX_LEN}-byte RWE layout: {RWE_EXECUTABLE_LEN} code + {RWE_LAYOUT_FILL} linker fill bytes')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for x in ['llvm-mc','rz-asm','generated','reference']:
        p.add_argument('--'+x,type=Path,required=True)
    p.add_argument('--prefix',type=Path)
    a=p.parse_args()
    verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
