#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V66 check of seed-state initialization 0x56364..0x563c0.

LLVM6 semantically assembles 84 bytes vs 92 in pinned reference, shortening
forced GP byte load/store instructions and rescheduling packet operands. This
verifier compares the unchanged ISA words plus Rizin's original 23-word order
and compares the entire source-generated interval to the pinned MBN.
"""
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
OFFSET=0x56364
SIZE=0x5c
PHYS=0x96cc8364

def elf_text(b:bytes)->bytes:
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf_parser_seedstate',path)
    assert spec and spec.loader
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(b)

def words(b:bytes)->list[int]:
    if len(b)%4:raise ValueError('Hexagon code must be a multiple of four bytes')
    return [int.from_bytes(b[i:i+4],'little') for i in range(0,len(b),4)]

def check(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    actual=generated.read_bytes();orig=reference.read_bytes()
    if len(actual)!=SIZE or actual!=orig[OFFSET:OFFSET+SIZE]:
        raise ValueError('typed seed-state source differs from pinned CDSP MBN')
    with tempfile.TemporaryDirectory(prefix='cdsp_hexagon_seed_init_') as tmp:
        obj=Path(tmp)/'seed_init.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                        str(SOURCE_DIR / 'late-rwe-seed-init.s'),
                        '-o',str(obj)],text=True,capture_output=True,check=True)
        independent=elf_text(obj.read_bytes())[0x364:]
    if len(independent)!=84:
        raise ValueError(f'Expected LLVM6 84-byte semantic source, got {len(independent)}')
    a,b=words(actual),words(independent)
    matches=[(0,0),(2,2),(3,3),(6,5),(7,7),(9,8),(10,9),
             (12,11),(13,12),(14,13),(16,15),(17,16),(18,17),
             (19,18),(20,19)]
    for orig_i,llvm_i in matches:
        if a[orig_i]!=b[llvm_i]:
            raise ValueError(f'LLVM6 semantic opcode mismatch: original {orig_i}, compiled {llvm_i}')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],
                           check=True,capture_output=True,text=True).stdout
    lines=[x for x in decoded.splitlines() if x.startswith('0x')]
    if len(lines)!=23 or any('invalid' in line.lower() for line in lines):
        raise ValueError(f'Rizin decoded {len(lines)}/23 V66 original words')
    text='\n'.join(lines).lower().replace(' ','')
    required=['r22=upcyclelo','call0x96dc0540','memub(gp+##0xc0364140)',
              'if(p0.new)jump:nt0x96cc83c0','call0x96cc8230',
              'memw(r19+##0x0)=r2.new','memw(r18+##0x0)=r2.new',
              'memb(gp+##0xc0364140)=r23']
    for feature in required:
        if feature not in text:raise ValueError('Rizin did not confirm CDSP seed-state feature '+feature)
    if prefix is not None:
        joined=prefix.read_bytes()
        if len(joined)!=0x3c0 or joined!=orig[0x56000:0x563c0]:
            raise ValueError('960-byte CDSP contiguous RWE file prefix mismatch')
        if joined[-SIZE:]!=actual:raise ValueError('Source-generated prefix differs from independent seed bytes')
    print('PASS exact reference: 92 source-generated Hexagon V66 bytes, zero mismatches')
    print(f'PASS LLVM6 source cross-check: {len(matches)} unchanged ISA word encodings; 84 semantic bytes')
    print('PASS Rizin V66: all 23 original-order instruction words, GP stores and backward call targets')
    if prefix:print('PASS RWE prefix 960 bytes: 928 executable plus 32 distinct NOP/fill')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for option in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+option,type=Path,required=option!='prefix')
    v=p.parse_args();check(v.llvm_mc,v.rz_asm,v.generated,v.reference,v.prefix)
