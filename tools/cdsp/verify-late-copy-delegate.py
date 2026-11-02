#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently check the CDSP bounded copy/hardware-loop fallback.

LLVM6's source assembly has the same instruction semantics and full length,
while its scheduler swaps the SUB vs P0 branch and the second P0 branch vs
comparison, altering four packet markers. Rizin V66 validates the OEM order.
"""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
FILE=0x565c0;PHYS=0x96cc85c0;CODE=60;PREFIX=1536;FILL=4

def elf_text(raw:bytes)->bytes:
    spec=importlib.util.spec_from_file_location('cdsp_elf_parser',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
    if spec is None or spec.loader is None:raise ValueError('missing trusted ELF32 text parser')
    parser=importlib.util.module_from_spec(spec);spec.loader.exec_module(parser)
    return parser.parse_elf32_text(raw)

def words(blob:bytes)->list[int]:
    if len(blob)%4:raise ValueError('unaligned executable')
    return [int.from_bytes(blob[i:i+4],'little') for i in range(0,len(blob),4)]

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    got=generated.read_bytes();ref=reference.read_bytes()
    if len(got)!=CODE or got!=ref[FILE:FILE+CODE]:
        raise ValueError('CDSP copy/delegate executable differs from pinned original')
    if prefix:
        total=prefix.read_bytes()
        if len(total)!=PREFIX or total!=ref[0x56000:FILE+CODE+FILL]:
            raise ValueError('source-built 1536-byte CDSP prefix not byte identical')
        if total[-(CODE+FILL):-FILL]!=got or total[-FILL:]!=b'\x00\xc0\x00\x7f':
            raise ValueError('incorrect executable/NOP boundary in RWE prefix')
    with tempfile.TemporaryDirectory(prefix='cdsp-copy-delegate-') as tmp:
        obj=Path(tmp)/'copy.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj',str(SOURCE_DIR / 'late-rwe-copy-delegate.s'),
                        '-o',str(obj)],capture_output=True,text=True,check=True)
        independently_assembled=elf_text(obj.read_bytes())[0x5c0:0x600]
    theirs=words(got);other=words(independently_assembled)
    if len(theirs)!=15 or len(other)!=16:
        raise ValueError('expected 15 original executable words and 16 semantic words incl NOP')
    for i in [0,1,2,5,8,9,10,11,12,13,14]:
        if theirs[i]!=other[i]:raise ValueError(f'unexplained LLVM6 opcode mismatch in word {i}')
    mask=0xffff_3fff
    for a,b in [(3,4),(4,3),(6,7),(7,6)]:
        if (theirs[a]&mask)!=(other[b]&mask):
            raise ValueError(f'Hexagon packet permutation changed instruction operands {a}/{b}')
    if other[15]!=0x7f00_c000:
        raise ValueError('LLVM6 failed to independently assemble trailing NOP')
    data=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',got.hex()],
                        capture_output=True,text=True,check=True).stdout
    decoded=[line for line in data.splitlines() if line.startswith('0x')]
    if len(decoded)!=15 or any('invalid' in line.lower() for line in decoded):
        raise ValueError('Rizin failed to decode the 15 original Hexagon packet words')
    normalized='\n'.join(decoded).lower().replace(' ','')
    for item in [
        'r16=r2','p0=cmp.gtu(r1,r0)','loop0(0x96cc85e0,r16)',
        'p0=!cmp.gt(r16,r2)','memb(r0++#0x1)=r2.new',
        'call0x96ddb740','dealloc_return','0x96cc85ec','0x96cc85f4',
    ]:
        if item not in normalized:
            raise ValueError('Rizin did not confirm '+item)
    print('PASS direct 60-byte CDSP reference comparison')
    print('PASS LLVM6 11 unchanged executable words, four packet-order-only differences, original NOP')
    print('PASS Rizin V66 original fifteen words, hardware loop, delegate call and branch targets')
    if prefix:print('PASS complete 1536 RWE bytes = 1492 source-generated executable + 44 structural NOP/fill')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+name,type=Path,required=name!='prefix')
    x=p.parse_args();verify(x.llvm_mc,x.rz_asm,x.generated,x.reference,x.prefix)
