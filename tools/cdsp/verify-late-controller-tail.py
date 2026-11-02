#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon LLVM6 + Rizin V66 168-byte CDSP controller tail verifier."""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path

START=0x55408
PHYSICAL=0x96cc7408
NBYTES=0xa8


def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None)->None:
    contents=generated.read_bytes()
    if len(contents)==0x4b0:sub=contents[0x408:]
    elif len(contents)==NBYTES:sub=contents
    else:raise ValueError('Expected 168-byte controller tail or entire 1200-byte executable segment')
    asm=Path(__file__).with_name('late-controller-tail.s')
    with tempfile.TemporaryDirectory(prefix='cdsp-tail-') as td:
        obj=Path(td)/'tail.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj',str(asm),'-o',str(obj)],check=True)
        spec=importlib.util.spec_from_file_location('hexagon_text',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
        assert spec and spec.loader
        module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
        independently_assembled=module.parse_elf32_text(obj.read_bytes())[0x408:]
    if sub!=independently_assembled:
        ix=next(i for i,(a,b) in enumerate(zip(sub,independently_assembled)) if a!=b)
        raise ValueError(f'Hexagon LLVM6 semantic reassembly differs at file {START+ix:#x}')
    output=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',sub.hex()],
                          capture_output=True,text=True,check=True)
    lines=[line for line in output.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=NBYTES//4 or any('invalid' in line.lower() for line in lines):
        raise ValueError('Rizin V66 failed to decode all 42 controller words')
    decoded='\n'.join(lines).lower().replace(' ','')
    for token in ['r2=##0x700','r3=##0x1f','r3=asl(r16,#0x0)',
                  'r0=##0x830002c','jump:t0x96cc746c','r0=##0x83a0048',
                  'jumprlr']:
        if token not in decoded:raise ValueError('Rizin decoder missing '+token)
    if reference and reference.read_bytes()[START:START+NBYTES]!=sub:
        raise ValueError('CDSP controller tail differs from pinned original bytes')
    print('PASS: 168 exact controller MMIO/shutdown bytes: 42 LLVM6-assembled words, Rizin V66 decoded')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True);p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--generated',type=Path,required=True);p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference)
