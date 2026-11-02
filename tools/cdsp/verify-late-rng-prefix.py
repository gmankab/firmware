#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 and Rizin V66 validation of CDSP seed/PRNG prologue.

LLVM6 emits 64 semantic bytes; the source reference has 68 due to an
original forced two-word GP-relative load and packet scheduling. This checks
unchanged semantic word encodings, exact Rust-vs-reference, and independent
Rizin decoding of the original packet order without equating LLVM output
with a byte-identical reference build.
"""
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
START=0x56320
LENGTH=68
PHYS=0x96cc8320

def parse_section(binary:bytes)->bytes:
    file=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf_parser_rng',file)
    assert spec and spec.loader
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(binary)

def words(data:bytes)->list[int]:
    assert len(data)%4==0
    return [int.from_bytes(data[i:i+4],'little') for i in range(0,len(data),4)]

def validate(llvm:Path,rz:Path,reference:Path,generated:Path,prefix:Path|None)->None:
    original=reference.read_bytes()
    produced=generated.read_bytes()
    if len(produced)!=LENGTH or produced!=original[START:START+LENGTH]:
        raise ValueError('typed RNG prefix does not match CDSP private reference')
    with tempfile.TemporaryDirectory(prefix='cdsp_rng_') as tmp:
        obj=Path(tmp)/'rng.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                        str(SOURCE_DIR / 'late-rwe-random-prefix.s'),
                        '-o',str(obj)],check=True,capture_output=True,text=True)
        assembler=parse_section(obj.read_bytes())[0x320:]
    if len(assembler)!=64:
        raise ValueError(f'expected 64 LLVM6 semantic bytes, got {len(assembler)}')
    a,b=words(produced),words(assembler)
    mapping=[(2,2),(3,0),(4,3),(5,4),(6,5),(7,6),(8,7),
             (10,10),(12,11),(13,12),(14,13),(15,14),(16,15)]
    for actual_i,llvm_i in mapping:
        if a[actual_i]!=b[llvm_i]:
            raise ValueError(f'LLVM6 opcode at original word {actual_i} did not match semantic word {llvm_i}')
    disassembled=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',produced.hex()],
                                capture_output=True,text=True,check=True).stdout
    decoded=[x for x in disassembled.splitlines() if x.startswith('0x')]
    if len(decoded)!=17 or any('invalid' in x.lower() for x in decoded):
        raise ValueError(f'Rizin decoded {len(decoded)}/17 original V66 words')
    normalized='\n'.join(decoded).lower().replace(' ','')
    for needed in ['memw(gp+##0xfe0ad660)','allocframe(sp,#0x30)',
                   'if(!p0)r20=##-0x3fc9bea8', 'if(p0)jump:nt0x96cc8438',
                   'r18=##-0x3fc9beb0','r19=##-0x3fc9beb8']:
        if needed not in normalized:
            raise ValueError('Rizin failed to confirm intended RNG field '+needed)
    if prefix:
        joined=prefix.read_bytes()
        if len(joined)!=0x364 or joined!=original[0x56000:0x56364]:
            raise ValueError('full source-generated RWE prefix through RNG prologue differs')
        if joined[-LENGTH:]!=produced:raise ValueError('RWE generated prefix does not contain new code')
    print('PASS reference equality: 68 source-generated CDSP V66 seed prologue bytes')
    print(f'PASS LLVM6 semantic fixture: {len(mapping)} unchanged word encodings; 64-byte shortened assembly')
    print('PASS Rizin V66 decodes all 17 original packet-order words, branch and GP operands')
    if prefix:print('PASS 868 contiguous RWE file bytes: 836 executable and 32 NOP/fill')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for arg in ['llvm-mc','rz-asm','reference','generated','prefix']:
        p.add_argument('--'+arg,type=Path,required=arg!='prefix')
    z=p.parse_args();validate(z.llvm_mc,z.rz_asm,z.reference,z.generated,z.prefix)
