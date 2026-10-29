#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""LLVM6 semantics and Rizin V66 independent CDSP status-path checks.

LLVM6 shrinks a source GP-relative memory access and rearranges packet
markers/branch displacements. We only compare truly identical scalar words,
check absolute branch targets semantically and verify original address/packet
fields using Rizin, never claiming full LLVM binary identity.
"""
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
FILE=0x569fc
END=0x56a30
PHYSICAL=0x96cc89fc

def parse_text(blob:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('check_cdsp_elf32',p)
    if not spec or not spec.loader:raise ValueError('independent ELF parser unavailable')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.parse_elf32_text(blob)

def words(blob:bytes)->list[int]:
    assert len(blob)%4==0
    return [int.from_bytes(blob[n:n+4],'little') for n in range(0,len(blob),4)]

def decode(rz:Path,blob:bytes)->list[str]:
    res=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',blob.hex()],
        check=True,capture_output=True,text=True)
    lines=[x for x in res.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=len(blob)//4 or any('invalid' in x.lower() for x in lines):
        raise ValueError('Rizin V66 decode incomplete or invalid')
    return lines

def verify(llvm:Path,rz:Path,binary:Path,reference:Path,prefix:Path|None):
    generated=binary.read_bytes()
    original=reference.read_bytes()
    if len(generated)!=END-FILE or generated!=original[FILE:END]:
        raise ValueError('typed CDSP status bytes differ from pinned firmware')
    if prefix is not None:
        layout=prefix.read_bytes()
        if len(layout)!=END-0x56000 or layout!=original[0x56000:END]:
            raise ValueError('CDSP RWE full 2608-byte prefix does not match pinned image')
    source=Path(__file__).with_name('agent20-10-cdsp-controller-status-semantic.s')
    with tempfile.TemporaryDirectory(prefix='nabu-cdsp-status-') as d:
        obj=Path(d)/'status.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(source),'-o',str(obj)],
            check=True,capture_output=True,text=True)
        assembled=parse_text(obj.read_bytes())[:48]
    if len(assembled)!=48:raise ValueError('expected 12 compiled LLVM6 scalar words')
    oem,compiled=words(generated),words(assembled)
    if len(oem)!=13 or len(compiled)!=12:raise ValueError('status word counts changed')
    # LLVM compacted the original forced-extended GP+0xc028c0f0 to one word,
    # so those first two original words have no direct LLVM word counterpart.
    unchanged={2:1,3:2,5:4,6:5,7:6,9:8,10:9,12:11}
    for i,j in unchanged.items():
        if oem[i]!=compiled[j]:raise ValueError(f'LLVM scalar mismatch reference={i}, independent={j}')
    if oem[11]&~0xc000!=compiled[10]&~0xc000:
        raise ValueError('LLVM pair initialization differs beyond parse markers')
    o=decode(rz,generated)
    c=decode(rz,assembled)
    a='\n'.join(o).lower().replace(' ','')
    b='\n'.join(c).lower().replace(' ','')
    for expr in ['0xc028c0f0','r2=lsr(r2,#0xc)','memw(r17+##0x20)=r2.new',
        'call0x96ddf2c0','combine(##-0x3fdd9e1f,r17)',
        'jump:nt0x96cc8abc','0xc036415c','combine(##0x20,#0x0)']:
        if expr not in a:raise ValueError('original V66 status missing '+expr)
    for expr in ['r2=lsr(r2,#0xc)','memw(r17+##0x20)=r2.new',
        'call0x96ddf2c0','jump:nt0x96cc8abc',
        '0xc036415c','combine(##0x20,#0x0)']:
        if expr not in b:raise ValueError('independently compiled status missing '+expr)
    if 'r2=memw(gp+##0xc0f0)' not in b:
        raise ValueError('LLVM compact GP-relative alternative not identified')
    print('PASS: CDSP 52 bytes source-generated, original 13 V66 words decoded')
    print('PASS: LLVM6 independent semantic fixture: 8 matching words, 1 parse-only difference')
    print('PASS: LLVM6 compact GP load, original extended GP pointer and branch targets separately checked')
    if prefix:print('PASS: 2608 exact RWE bytes = 2552 executable + 56 structural layout')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for item in ('llvm-mc','rz-asm','generated','reference','prefix'):
        p.add_argument('--'+item,type=Path,required=item!='prefix')
    args=p.parse_args();verify(args.llvm_mc,args.rz_asm,args.generated,args.reference,args.prefix)
