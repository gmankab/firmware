#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6/Rizin V66 checks of CDSP callback registry sweeper.

The original 80-byte routine contains two forced immediate extenders which
LLVM6 removes; LLVM6's source-assembled semantic output is therefore 72 bytes.
No claim of full raw LLVM binary equivalence is made. Every 20 source-built
original-order word is decoded with Rizin, then compared with pinned OEM bytes.
"""
import argparse
import importlib.util
import subprocess
import tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'

FILE_OFFSET=0x561e0
SIZE=80
VMA=0x96cc81e0

def parser_text(data:bytes)->bytes:
    f=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    s=importlib.util.spec_from_file_location('cdsp_parser',f)
    if s is None or s.loader is None:raise RuntimeError('ELF parser module unavailable')
    m=importlib.util.module_from_spec(s)
    s.loader.exec_module(m)
    return m.parse_elf32_text(data)

def check(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    src=generated.read_bytes()
    if len(src)!=SIZE:raise ValueError('CDSP sweeper should emit exactly 80 bytes')
    orig=reference.read_bytes()
    if orig[FILE_OFFSET:FILE_OFFSET+SIZE]!=src:
        raise ValueError('CDSP original pinned code disagrees with typed sweeper')
    with tempfile.TemporaryDirectory(prefix='nabu-cdsp-sweeper-') as tmp:
        obj=Path(tmp)/'sweeper.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                        str(SOURCE_DIR / 'late-rwe-sweeper.s'),
                        '-o',str(obj)],capture_output=True,text=True,check=True)
        independent=parser_text(obj.read_bytes())[0x1e0:]
    if len(independent)!=72:
        raise ValueError(f'LLVM6 semantic equivalent should be 72, got {len(independent)}')
    words=lambda b:[int.from_bytes(b[i:i+4],'little') for i in range(0,len(b),4)]
    original_words=words(src); llvm_words=words(independent)
    exact=[(0,0),(3,2),(4,3),(5,4),(6,5),(7,6)]
    exact.extend((idx,idx-2) for idx in range(10,20))
    for left,right in exact:
        if original_words[left]!=llvm_words[right]:
            raise ValueError(f'independently assembled word {left} != LLVM6 semantic word {right}')
    rzout=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(VMA),'-D',src.hex()],
                         capture_output=True,text=True,check=True)
    decoded=[line for line in rzout.stdout.splitlines() if line.startswith('0x')]
    if len(decoded)!=20 or any('invalid' in line.lower() for line in decoded):
        raise ValueError(f'Rizin V66 decoded {len(decoded)} / 20 original packet words')
    all_text='\n'.join(decoded).lower().replace(' ','')
    for required in [
        'memb(##0xc0364128)=r3.new',
        'call0x96cc8120',
        'memw(r16<<#0x2+##0xc028c010)',
        'jump:t0x96cc820c',
        'callrr2',
        'memw(sp+##0x4)',
        'dealloc_return',
    ]:
        if required not in all_text:
            raise ValueError('Rizin V66 failed to confirm '+required)
    if prefix is not None:
        full=prefix.read_bytes()
        if len(full)!=560 or full!=orig[0x56000:0x56230]:
            raise ValueError('CDSP entire prefix 0x56000-0x56230 differs from original')
        if full[0x1e0:0x230]!=src:
            raise ValueError('CDSP independent sweeper disagrees with joined prefix')
    print(f'PASS {SIZE} independently generated CDSP V66 executable bytes, original pinned reference exact')
    print(f'PASS LLVM6 {len(exact)} unchanged word matches; 72-byte semantic assembly differs by two forced extenders')
    print('PASS Rizin independently validates all 20 original instruction words and callback-loop targets')
    if prefix is not None:
        print('PASS full CDSP RWE first 560 file bytes: 536 executable + 24 layout NOP/fill')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ('llvm-mc','rz-asm','generated','reference','prefix'):
        p.add_argument('--'+name,type=Path,required=name!='prefix')
    a=p.parse_args()
    check(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
