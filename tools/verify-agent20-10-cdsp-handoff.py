#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently assemble and decode CDSP controller PIC wait handoff."""
from __future__ import annotations
import argparse, importlib.util, subprocess, tempfile
from pathlib import Path

START=0x5532c
PHYSICAL=0x96cc732c
LENGTH=0x70


def semantic_asm()->str:
    return '''.text
.global wait_base
wait_base:
.space 0x2f4
controller_stage:
{ r0=##0x08300200 }
{ r7=r0
  r5.h=#0 }
{ r5.l=#4
  dcinva(r7) }
{ r6=sub(#-1,r5)
  r4=memw(r0+#0) }
{ r4=and(r4,r6) }
{ r4=or(r4,r5) }
{ memw(r0+#0)=r4 }
{ barrier }
{ r0=##0x08300238 }
{ r1=memw(r0+#0) }
{ r2=##1 }
{ r1=or(r1,r2) }
{ memw(r0+#0)=r1 }
{ barrier }
{ r4=##0x08300200 }
{ r2=##-0x80000000 }
{ r0=##0xc0 }
{ r13=add(pc,#8) }
{ jump wait_base }
'''


def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None)->None:
    contents=generated.read_bytes()
    if len(contents)==0x39c: fragment=contents[0x32c:]
    elif len(contents)==LENGTH: fragment=contents
    else: raise ValueError('expected full 924-byte controller source or 112-byte handoff')
    with tempfile.TemporaryDirectory(prefix='cdsp-pic-') as td:
        td=Path(td);src=td/'pic.s';obj=td/'pic.o'
        src.write_text(semantic_asm())
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(src),'-o',str(obj)],check=True)
        dep=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('parse_cdsp_elf_text',dep)
        assert spec and spec.loader
        m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
        full=m.parse_elf32_text(obj.read_bytes())
    independent=full[0x2f4:]
    if independent!=fragment:
        mismatches=[i for i,(x,y) in enumerate(zip(independent,fragment)) if x!=y]
        raise ValueError(f'Hexagon LLVM6 independent assembler differs at file {START+mismatches[0]:#x}')
    dec=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',fragment.hex()],check=True,text=True,capture_output=True)
    lines=[x for x in dec.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=28 or any('invalid' in x.lower() for x in lines):
        raise ValueError('Rizin V66 must decode all 28 instruction words')
    decoded='\n'.join(lines).replace(' ','').lower()
    for value in ['dcinva(r7)','r13=add(pc,##0x8)','jump0x96cc7038','memw(r0+##0x0)=r4','barrier']:
        if value not in decoded:raise ValueError('Missing decoded Hexagon operation '+value)
    if fragment[-8:-4]!=(0x6a49c40d).to_bytes(4,'little'):
        raise ValueError('Hexagon PIC register encoding missing bit 10')
    if reference and reference.read_bytes()[START:START+LENGTH]!=fragment:
        raise ValueError('CDSP pinned reference differs in handoff')
    print('PASS: 112 exact Hexagon controller PIC handoff bytes, independently assembled with LLVM6, decoded Rizin V66')


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--generated',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference)
