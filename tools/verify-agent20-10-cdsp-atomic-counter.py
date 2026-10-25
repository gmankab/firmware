#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6/Rizin check for CDSP atomic counter 0x56464..0x564b8.

LLVM6 assembles an 88-byte semantic equivalent but splits the original V66
fused absolute-address/load word into two standalone words. It also changes
branch displacement and packet-marker bits. Exact raw LLVM byte equality is
*not* asserted; checked original word fields and V66 packet flow are.
"""
from __future__ import annotations
import argparse, importlib.util, subprocess, tempfile
from pathlib import Path
START=0x56464
SIZE=84
PHYS=0x96cc8464
RWE_START=0x56000

def text_from_object(b:bytes)->bytes:
    spec=importlib.util.spec_from_file_location('cdsp_atomic_elf',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
    if spec is None or spec.loader is None:raise ValueError('Missing independent ELF text reader')
    helper=importlib.util.module_from_spec(spec);spec.loader.exec_module(helper)
    return helper.parse_elf32_text(b)

def words(v:bytes)->list[int]:
    if len(v)%4:raise ValueError('Expected aligned Hexagon words')
    return [int.from_bytes(v[i:i+4],'little') for i in range(0,len(v),4)]

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    original=reference.read_bytes();source=generated.read_bytes()
    if len(source)!=SIZE or source!=original[START:START+SIZE]:
        raise ValueError('Source-generated CDSP atomic counter differs from original pinned bytes')
    if prefix:
        joined=prefix.read_bytes()
        if len(joined)!=0x4b8 or joined!=original[RWE_START:START+SIZE]:
            raise ValueError('Full RWE prefix through atomic counter is not byte-identical')
        if joined[-SIZE:]!=source:raise ValueError('RWE prefix emitted a different atomic counter')
    with tempfile.TemporaryDirectory(prefix='cdsp_atomic_checker_') as tmp:
        obj=Path(tmp)/'cdsp-atomic.o'
        asm=Path(__file__).with_name('agent20-10-cdsp-rwe-atomic-semantics.s')
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj',str(asm),'-o',str(obj)],
                        check=True,capture_output=True,text=True)
        semantic=text_from_object(obj.read_bytes())[0x464:]
    if len(semantic)!=88:raise ValueError('Expected 88 LLVM6 semantic bytes (84 original)')
    a,b=words(source),words(semantic)
    matches=[(0,0),(3,4),(4,5),(5,6),(6,7),(7,8),(11,12),
        (12,13),(13,14),(14,15),(15,16),(16,17),(17,18),
        (18,19),(19,20),(20,21)]
    for i,j in matches:
        if a[i]!=b[j]:raise ValueError(f'Original source word {i} vs independent LLVM6 word {j}')
    raw=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',source.hex()],
                       check=True,capture_output=True,text=True).stdout
    decoded=[row for row in raw.splitlines() if row.startswith('0x')]
    if len(decoded)!=21 or any('invalid' in row.lower() for row in decoded):
        raise ValueError(f'Rizin V66 decoded {len(decoded)}/21 atomic counter words')
    n='\n'.join(decoded).lower().replace(' ','')
    features=['memw(r2=##0xc0363e00)','memw_locked(r2)',
        'memw_locked(r2,p0)=r3','memw_locked(r2,p0)=r4',
        'jump:nt0x96cc849c','jump:nt0x96cc8474',
        'jump:nt0x96cc84a0','if(!p0.new)r3=##0x1',
        'memw(r3<<#0x2+##0xc0363e08)','memw(r0+##0x0)=r2.new']
    for feature in features:
        if feature not in n:raise ValueError('Rizin did not prove atomic feature '+feature)
    print('PASS 84 byte-identical source-generated CDSP counter bytes')
    print(f'PASS LLVM6 semantic cross-check: {len(matches)} original words unchanged in 88-byte scheduled source')
    print('PASS Rizin V66: all 21 words, atomic retry loops and indexed output store')
    if prefix:print('PASS RWE prefix 1208 bytes = 1176 executable + 32 separately accounted fill')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for arg in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+arg,type=Path,required=arg!='prefix')
    opt=p.parse_args();verify(opt.llvm_mc,opt.rz_asm,opt.generated,opt.reference,opt.prefix)
