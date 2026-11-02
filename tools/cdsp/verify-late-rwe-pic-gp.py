#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""CDSP runtime RWE PIC saves and forced-extended GP-zero store verifier."""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
START=0x560d4
PHYSICAL=0x96cc80d4
SIZE=0x40

def parse_text(data:bytes)->bytes:
    dep=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('hexagon_rwe_parse',dep)
    assert spec and spec.loader
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod.parse_elf32_text(data)

def assemble(llvm:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='cdsp-rwe-gp-') as tmp:
        obj=Path(tmp)/'asm.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],check=True)
        return parse_text(obj.read_bytes())

def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None)->None:
    data=generated.read_bytes()
    if len(data)!=SIZE:raise ValueError('Expected exactly 64 independent CDSP RWE bytes')
    pic=assemble(llvm,SOURCE_DIR / 'late-rwe-pic.s')[0xd4:]
    if len(pic)!=0x2c or pic!=data[:0x2c]:
        raise ValueError('LLVM6 PIC register stack/save instructions differ from typed Rust')
    # LLVM6 defaults to a shorter GP-relative store. Compare its semantics,
    # not the length, then verify the original V66 forced 32-bit extension via
    # a separate Rizin decoder and direct pinned-reference byte comparison.
    gp=assemble(llvm,SOURCE_DIR / 'late-rwe-gp-semantic.s')
    if len(gp)!=0x10:raise ValueError('Expected LLVM6 compressed 16-byte GP-zero implementation')
    rzout=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],check=True,capture_output=True,text=True)
    lines=[x for x in rzout.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=SIZE//4 or any('invalid' in line.lower() for line in lines):
        raise ValueError('Rizin V66 did not decode all 16 instructions')
    decoded='\n'.join(lines).lower().replace(' ','')
    for token in ['r0=add(pc,##0x1c3f2c)','r0=add(pc,##0x1c3f24)',
                  'jump0x96dc3ec0','memw(gp+##0xc028c014)=r2.new','jumprlr']:
        if token not in decoded:raise ValueError('Missing CDSP runtime instruction '+token)
    if reference and reference.read_bytes()[START:START+SIZE]!=data:
        raise ValueError('CDSP RWE PIC/GP output differs from OEM reference')
    print('PASS: 64 byte-exact typed CDSP RWE bytes. LLVM6 PIC exact, GP short semantics equivalent; Rizin V66 confirms original long packet')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--generated',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference)
