#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V66 checks for CDSP GP-state multiplier at 0x56540.

The independently authored LLVM6 semantic fixture is 32 bytes; the original
has two forced GP immext words and original V66 packet marker choices. LLVM6
does NOT emit a byte-identical original, and the verifier does not claim so.
"""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
FILE_OFFSET=0x56540
LEN=40
PHYS=0x96cc8540
RWE_START=0x56000

def read_elf(blob:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    s=importlib.util.spec_from_file_location('gp_multiplier_independent_elf',p)
    if s is None or s.loader is None:raise ValueError('Independent ELF reader unavailable')
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
    return m.parse_elf32_text(blob)

def decode_words(blob:bytes)->list[int]:
    if len(blob)%4:raise ValueError('Hexagon code must be a multiple of 4 bytes')
    return [int.from_bytes(blob[i:i+4],'little') for i in range(0,len(blob),4)]

def verify(llvm:Path,rz:Path,gen:Path,ref:Path,prefix:Path|None)->None:
    ours=gen.read_bytes();src=ref.read_bytes()
    if len(ours)!=LEN or src[FILE_OFFSET:FILE_OFFSET+LEN]!=ours:
        raise ValueError('40-byte GP-state code does not match source firmware')
    if prefix:
        joined=prefix.read_bytes()
        if len(joined)!=0x568 or joined!=src[RWE_START:FILE_OFFSET+LEN]:
            raise ValueError('RWE prefix 1384 bytes differs from pinned firmware')
        if joined[-LEN:]!=ours:raise ValueError('RWE prefix does not contain 40-byte GP multiplier')
    with tempfile.TemporaryDirectory(prefix='cdsp_gp_multiplier_') as d:
        output=Path(d)/'semantics.o'
        asm=Path(__file__).with_name('late-rwe-state-multiplier.s')
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj',str(asm),'-o',str(output)],
                       capture_output=True,text=True,check=True)
        semantic=read_elf(output.read_bytes())[0x540:]
    if len(semantic)!=32:raise ValueError(f'Unexpected {len(semantic)} LLVM6 semantic bytes')
    a,b=decode_words(ours),decode_words(semantic)
    if a[2]!=b[1] or a[3]!=b[2] or a[9]!=b[7]:
        raise ValueError('LLVM6 differs in unchanged duplex, extender or result store words')
    for i,j in [(4,3),(6,5)]:
        if a[i]^b[j]!=0x8000:raise ValueError('Unexpected differences beyond original packet marker')
    # LLVM6 omits the two GP extenders. Compiled negative branch offset is 4
    # bytes shorter: original halfword displacement 0xfe vs LLVM 0xfa.
    if a[5]^b[4]!=0x4:raise ValueError('LLVM6 branch displacement discrepancy not explained by shorter source')
    raw=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',ours.hex()],
                       check=True,capture_output=True,text=True).stdout
    lines=[x for x in raw.splitlines() if x.startswith('0x')]
    if len(lines)!=10 or any('invalid' in ln.lower() for ln in lines):
        raise ValueError(f'Rizin V66 decoded {len(lines)}/10 original words')
    norm='\n'.join(lines).lower().replace(' ','')
    features=['memw(gp+##0xc028c02c)',
              'r3=##0x1;r4=r3',
              'r3+=mpyi(r4,##0x15a4e35)',
              'if(p0.new)jump:t0x96cc8548',
              'memw(gp+##0xc028c02c)=r3',
              'memw(r0+##0x0)=r4']
    for item in features:
        if item not in norm:raise ValueError('Rizin does not confirm '+item)
    print('PASS CDSP GP multiplier exact source output: 40 bytes, zero differences')
    print('PASS LLVM6: verified unchanged duplex/extender/store, original packet markers and shifted branch displacement')
    print('PASS Rizin V66: all ten original packet-order words and physical backward branch')
    if prefix:print('PASS RWE layout: 1384 bytes = 1352 executable + 32 linker NOPs')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for a in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+a,type=Path,required=a!='prefix')
    v=p.parse_args();verify(v.llvm_mc,v.rz_asm,v.generated,v.reference,v.prefix)
