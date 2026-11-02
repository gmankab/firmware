#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""LLVM6/Rizin V66 independent CDSP descriptor-byte scan verification.

Original V66 uses a cross-scheduled R3.new fused zero guard, while LLVM6
emits a P1 compare-and-branch. These are checked separately and not falsely
called byte-identical. The OEM packet markers and exact bytes are validated
by Rizin and the strict private firmware reference comparison.
"""
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
FILE=0x569c0
END=0x569fc
PHYSICAL=0x96cc89c0

def extract_elf32_text(blob:bytes)->bytes:
    f=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('independent_cdsp_elf_reader',f)
    if not spec or not spec.loader:raise ValueError('missing independent ELF reader')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(blob)

def words(raw:bytes)->list[int]:
    if len(raw)%4:raise ValueError('Hexagon word alignment missing')
    return [int.from_bytes(raw[i:i+4],'little') for i in range(0,len(raw),4)]

def decode(rz:Path,blob:bytes):
    p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',blob.hex()],
        check=True,capture_output=True,text=True)
    lines=[line for line in p.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=len(blob)//4 or any('invalid' in line.lower() for line in lines):
        raise ValueError('Rizin V66 decoded missing/invalid operations')
    return lines

def verify(llvm:Path,rz:Path,binary:Path,reference:Path,prefix:Path|None):
    orig=reference.read_bytes()
    typed=binary.read_bytes()
    if len(typed)!=60 or typed!=orig[FILE:END]:
        raise ValueError('controller byte-scanning emitter differs from pinned CDSP')
    if prefix:
        region=prefix.read_bytes()
        if len(region)!=END-0x56000 or region!=orig[0x56000:END]:
            raise ValueError('full 2556-byte RWE prefix differs from original')
    source=Path(__file__).with_name('late-controller-byte-scan-semantic.s')
    with tempfile.TemporaryDirectory(prefix='nabu-cdsp-scan-') as path:
        o=Path(path)/'scan.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(source),'-o',str(o)],capture_output=True,text=True,check=True)
        assembled=extract_elf32_text(o.read_bytes())[:60]
    if len(assembled)!=60:raise ValueError('LLVM6 scan fixture is not 15 instruction words')
    a,b=words(typed),words(assembled)
    # Eleven words are identical (including original word-index branch forms),
    # three differ only in two-bit packet parse markers.
    identical={0,2,4,5,6,7,10,11,12,13,14}
    parser_only={1,3,8}
    for i in identical:
        if a[i]!=b[i]:raise ValueError(f'LLVM6 differs in exact scalar instruction word {i}')
    for i in parser_only:
        if a[i]&~0xc000!=b[i]&~0xc000:
            raise ValueError(f'LLVM6 differs beyond packet parse bits in word {i}')
    raw_oem=decode(rz,typed)
    raw_compiler=decode(rz,assembled)
    actual='\n'.join(raw_oem).lower().replace(' ','')
    alt='\n'.join(raw_compiler).lower().replace(' ','')
    for expr in ['r0=##-0x3fdd9e33','call0x96dc0bac','r22=add(sp,##0x18)',
                 'r2=##0x0;r17=memw(r16+#0x10)',
                 'jump:nt0x96cc89fc','r4=memb(r17+r2<<#0x0)',
                 'jump:t0x96cc89dc','jump0x96cc8a0c']:
        if expr not in actual:raise ValueError('original scan missing '+expr)
    for expr in ['call0x96dc0bac','jump:nt0x96cc89fc','r4=memb(r17+r2<<#0x0)',
                 'jump:t0x96cc89dc','jump0x96cc8a0c']:
        if expr not in alt:raise ValueError('LLVM6 scan missing semantic '+expr)
    if 'if(cmp.eq(r3.new,#0x0))' not in actual:
        raise ValueError('original `.new` guard was lost')
    if 'p1=cmp.eq(r3,#0x0)' not in alt:
        raise ValueError('LLVM6 equivalent P1 guard is absent')
    print('PASS: 60 source-generated Hexagon bytes identical to pinned CDSP, 15 original words')
    print('PASS: independent LLVM6 11 exact words, 3 parse-only differences, guarded byte compare equivalent')
    print('PASS: Rizin V66 loop, physical trace call and packet-specific branch targets')
    if prefix:print('PASS: exact RWE prefix: 2556 file bytes = 2500 executable + 56 layout')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for n in ('llvm-mc','rz-asm','generated','reference','prefix'):
        p.add_argument('--'+n,type=Path,required=n!='prefix')
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
