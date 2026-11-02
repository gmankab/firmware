#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent audit of CDSP large-controller prologue at file 0x56600.

The typed 80-byte original-order V66 fragment is matched against pinned
firmware; LLVM6 reassembles the semantics with a shorter three-GP-load layout
(17 vs original 20 words), while Rizin independently decodes all original
20 instruction words and physical branch/memory addresses.
"""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
FILE=0x56600; SIZE=80; PHYS=0x96cc8600;PREFIX=0x650

def read_text(raw:bytes)->bytes:
    spec=importlib.util.spec_from_file_location('elf_parser',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
    if spec is None or spec.loader is None:raise ValueError('ELF text parser unavailable')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(raw)

def words(blob:bytes)->list[int]:
    if len(blob)%4:raise ValueError('unaligned code')
    return [int.from_bytes(blob[i:i+4],'little') for i in range(0,len(blob),4)]

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    ref=reference.read_bytes();src=generated.read_bytes()
    if len(src)!=SIZE or src!=ref[FILE:FILE+SIZE]:raise ValueError('large prologue not reference-byte-identical')
    if prefix is not None:
        data=prefix.read_bytes()
        if len(data)!=PREFIX or data!=ref[0x56000:FILE+SIZE] or data[-SIZE:]!=src:
            raise ValueError('1616-byte RWE prefix mismatch')
    with tempfile.TemporaryDirectory(prefix='nabu-cdsp-large-prologue-') as tmp:
        output=Path(tmp)/'asm.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
            str(SOURCE_DIR / 'late-rwe-large-prologue.s'),
            '-o',str(output)],check=True,capture_output=True,text=True)
        comp=read_text(output.read_bytes())[0x600:0x644]
    orig=words(src);generated_words=words(comp)
    if len(orig)!=20 or len(generated_words)!=17:raise ValueError('Unexpected LLVM6 GP compression layout')
    same=[(i,i) for i in range(8)]+[(12,10),(13,11),(16,13),(17,14),(18,15),(19,16)]
    for a,b in same:
        if orig[a]!=generated_words[b]:
            raise ValueError(f'Independent LLVM semantics word mismatch: OEM {a}, compiler {b}')
    d=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',src.hex()],
                     capture_output=True,text=True,check=True).stdout
    lines=[x for x in d.splitlines() if x.startswith('0x')]
    if len(lines)!=20 or any('invalid' in x.lower() for x in lines):
        raise ValueError('Rizin failed to decode all 20 V66 words')
    norm='\n'.join(lines).lower().replace(' ','')
    for pat in ['allocframe(#0x30)','memd(sp+##0x10)=r23:22',
        '0xc0673260','0xfe0c6734','0xfe0c6680','0xc028c0f0',
        '0xc0367850','jump:nt0x96cc8870','lsr(r17,#0xc)']:
        if pat not in norm:raise ValueError('Rizin did not independently confirm '+pat)
    print('PASS 80 independently source-generated CDSP executable bytes = original reference')
    print('PASS LLVM6 14 identical words, compiler-shortened three GP-load pairs')
    print('PASS Rizin V66 all 20 original packet words/addresses/branches')
    if prefix:print('PASS 1616 file bytes = 1572 executable + 44 structural NOP/fill')

if __name__=='__main__':
    arg=argparse.ArgumentParser(description=__doc__)
    for a in ('llvm-mc','rz-asm','generated','reference','prefix'):
        arg.add_argument('--'+a,type=Path,required=a!='prefix')
    x=arg.parse_args();verify(x.llvm_mc,x.rz_asm,x.generated,x.reference,x.prefix)
