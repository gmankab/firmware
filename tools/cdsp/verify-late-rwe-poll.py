#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently cross-check source-built 152-byte CDSP Hexagon V66 poll.

LLVM6 semantic source is 144 bytes, whereas the OEM file uses 152 because
it retains two original forced-extender packet words. LLVM also recalculates
internal branch displacements accordingly. Equal words are compared by
explicit semantic index mapping; no unchecked code words are imported.
Rizin independently decodes all 38 original-order words, with branch targets.
"""
import argparse
import importlib.util
import subprocess
import tempfile
from pathlib import Path

OFF=0x56230
N=152
BASE=0x96cc8230

def parse_text(data:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf_parser',p)
    if spec is None or spec.loader is None:raise RuntimeError('CDSP parser unavailable')
    m=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m.parse_elf32_text(data)

def decode(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    src=generated.read_bytes()
    original=reference.read_bytes()
    if len(src)!=N:raise ValueError(f'expected exactly {N} typed executable bytes, got {len(src)}')
    if original[OFF:OFF+N]!=src:
        raise ValueError('generated V66 executable differs from pinned CDSP reference')
    with tempfile.TemporaryDirectory(prefix='nabu-agent20-10-poll-') as tmp:
        obj=Path(tmp)/'cdsp-callback-poll.o'
        cp=subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                           '-filetype=obj',str(Path(__file__).with_name('late-rwe-poll.s')),
                           '-o',str(obj)],capture_output=True,text=True,check=True)
        independent=parse_text(obj.read_bytes())[0x230:]
    if len(independent)!=144:
        raise ValueError(f'expected 144-byte LLVM6 semantic equivalent, got {len(independent)}')
    def words(b:bytes):return [int.from_bytes(b[i:i+4],'little') for i in range(0,len(b),4)]
    actual=words(src)
    semantic=words(independent)
    pairs=[(n,n) for n in range(0,3)]
    pairs += [(n,n) for n in range(4,9)]
    pairs += [(n,n-1) for n in range(11,15)]
    pairs += [(n,n-2) for n in range(17,25)]
    pairs += [(n,n-2) for n in range(26,38)]
    assert len(pairs)==32
    for a,b in pairs:
        if actual[a]!=semantic[b]:
            raise ValueError(f'Hexagon LLVM6 semantic instruction index [{a}] != [{b}]: {actual[a]:#010x} vs {semantic[b]:#010x}')
    output=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(BASE),'-D',src.hex()],
                          capture_output=True,text=True,check=True).stdout
    decoded=[line for line in output.splitlines() if line.startswith('0x')]
    if len(decoded)!=38 or any('invalid' in line.lower() for line in decoded):
        raise ValueError(f'Rizin decoded {len(decoded)} of 38 words; invalid instruction encountered')
    text='\n'.join(decoded).lower().replace(' ','')
    for feature in ['call0x96dfd9a0','call0x96cc8120','memub(gp+##0xc0364128)',
                    'memb(##0xc0364128)=r19','memw(r20++#0x4)',
                    'jump:t0x96cc827c','jump:t0x96cc8254',
                    'call0x96cc8230','dealloc_return']:
        if feature not in text:raise ValueError('Rizin did not decode expected CDSP feature '+feature)
    if prefix:
        full=prefix.read_bytes()
        if len(full)!=712 or full!=original[0x56000:0x562c8] or full[0x230:0x2c8]!=src:
            raise ValueError('joined RWE 712-byte prefix differs from independently built function or reference')
    print('PASS exact reference: 152 executable bytes; 0 differing bytes')
    print(f'PASS LLVM6 independently assembles 144-byte semantic source; {len(pairs)} unchanged original words')
    print('PASS Rizin V66 decodes all 38 words, original packet boundaries and callback destinations')
    if prefix:print('PASS contiguous RWE layout 712 bytes: 688 executable + 24 explicit NOP/fill')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for label in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+label,type=Path,required=label!='prefix')
    a=p.parse_args()
    decode(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
