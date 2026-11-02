#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 and Rizin V66 audit of CDSP bounded byte compare.

LLVM6 reorders the two-word unsigned-bounds predicate packet, preserving
semantics but changing the original parse markers. All other ten words match
exactly with the OEM reference. Original ordering is independently decoded
by the Rizin V66 backend; no OEM instruction bytes are imported as source.
"""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
FILE=0x56590;PHYS=0x96cc8590;SIZE=48;PREFIX_LEN=0x5c0

def text(blob:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf_text',p)
    if spec is None or spec.loader is None:raise ValueError('missing ELF32 parser')
    x=importlib.util.module_from_spec(spec);spec.loader.exec_module(x)
    return x.parse_elf32_text(blob)

def words(blob:bytes)->list[int]:
    if len(blob)%4:raise ValueError('unaligned instructions')
    return [int.from_bytes(blob[i:i+4],'little') for i in range(0,len(blob),4)]

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    ours=generated.read_bytes();ref=reference.read_bytes()
    if len(ours)!=SIZE or ours!=ref[FILE:FILE+SIZE]:
        raise ValueError('48-byte bounded comparison differs from original CDSP')
    if prefix:
        total=prefix.read_bytes()
        if len(total)!=PREFIX_LEN or total!=ref[0x56000:FILE+SIZE]:
            raise ValueError('contiguous RWE prefix does not match pinned OEM reference')
        if total[-SIZE:]!=ours:raise ValueError('generated fragment differs from its prefix')
    with tempfile.TemporaryDirectory(prefix='cdsp_bounded_compare_') as td:
        obj=Path(td)/'bounded.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(Path(__file__).with_name('late-rwe-bounded-compare.s')),
            '-o',str(obj)],check=True,capture_output=True,text=True)
        independent=text(obj.read_bytes())[0x590:0x5c0]
    original_words=words(ours);llvm_words=words(independent)
    if len(original_words)!=12 or len(llvm_words)!=12:
        raise ValueError('LLVM6 must emit exactly 12 words for bounded comparison')
    for i in [0,3,4,5,6,7,8,9,10,11]:
        if original_words[i]!=llvm_words[i]:
            raise ValueError(f'Unexpected LLVM semantic word mismatch at {i}')
    # Original: predicate then its branch. LLVM: reordered branch then predicate.
    # Both condition and immediate displacement are unchanged after normalizing
    # only the packet-marker field and instruction positions.
    predicate_mask=0xffff3fff
    if (original_words[1]&predicate_mask)!=(llvm_words[2]&predicate_mask):
        raise ValueError('P0 unsigned bounds predicate differs beyond packet marker')
    if (original_words[2]&predicate_mask)!=(llvm_words[1]&predicate_mask):
        raise ValueError('P0 branch differs beyond packet marker')
    dis=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',ours.hex()],
        check=True,capture_output=True,text=True).stdout
    lines=[line for line in dis.splitlines() if line.startswith('0x')]
    if len(lines)!=12 or any('invalid' in line.lower() for line in lines):
        raise ValueError('Rizin failed to decode all twelve original V66 words')
    normalized='\n'.join(lines).lower().replace(' ','')
    for term in ['p0=!cmp.gtu(r2,r0)','0x96cc85b8','0x96cc85bc',
                 'memb(r1+##0x0)','memb(r3++#0x1)','0x96cc8594',
                 'jumprlr','jumprr31']:
        if term not in normalized:
            raise ValueError('Rizin V66 did not confirm '+term)
    print('PASS 48 source-generated executable bytes match pinned CDSP')
    print('PASS LLVM6 10 unchanged opcode words; 2 proven packet-order changes')
    print('PASS Rizin V66 all 12 original words, byte-load signs, physical branches')
    if prefix:print('PASS 1472-byte contiguous RWE prefix: 1432 code + 40 layout bytes')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+name,type=Path,required=name!='prefix')
    a=p.parse_args()
    verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
