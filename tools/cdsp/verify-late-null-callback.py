#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent V66 audit of null-guarded CDSP callback, 0x56568..0x56588.

LLVM6's semantic assembly is forty bytes including eight linker NOPs, while
original packet markers differ for P0 and extended call and LLVM6 chooses a
slightly different extended call low displacement. Rizin validates original
packet order and physical target. No exact LLVM6 binary claim is made.
"""
from __future__ import annotations
import argparse, importlib.util, subprocess,tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
FILE_START=0x56568
FUNC_LEN=32
FILL_LEN=8
PHYS=0x96cc8568
RWE_START=0x56000

def elf_text(blob:bytes)->bytes:
    spec=importlib.util.spec_from_file_location('cdsp_null_callback_loader',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
    if spec is None or spec.loader is None:raise ValueError('Missing validated CDSP ELF text reader')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(blob)

def words(blob:bytes)->list[int]:
    if len(blob)%4:raise ValueError('Unaligned code')
    return [int.from_bytes(blob[i:i+4],'little') for i in range(0,len(blob),4)]

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    ours=generated.read_bytes();ref=reference.read_bytes()
    if len(ours)!=FUNC_LEN or ours!=ref[FILE_START:FILE_START+FUNC_LEN]:
        raise ValueError('32-byte CDSP null callback differs from original')
    if prefix:
        joined=prefix.read_bytes()
        if len(joined)!=0x590 or joined!=ref[RWE_START:FILE_START+FUNC_LEN+FILL_LEN]:
            raise ValueError('CDSP null callback 1424-byte RWE file prefix mismatch')
        if joined[-FILL_LEN:]!=b'\x00\xc0\x00\x7f'*2:
            raise ValueError('Last eight layout bytes are not the expected NOP/fill')
        if joined[-(FUNC_LEN+FILL_LEN):-FILL_LEN]!=ours:
            raise ValueError('Full prefix generated inconsistent callback executable')
    with tempfile.TemporaryDirectory(prefix='cdsp_null_callback_') as tmp:
        obj=Path(tmp)/'null.o'
        asm=SOURCE_DIR / 'late-rwe-null-callback.s'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(asm),'-o',str(obj)],check=True,capture_output=True,text=True)
        independent=elf_text(obj.read_bytes())[0x568:]
    if len(independent)!=40:raise ValueError('LLVM6 semantic program must be 40 bytes')
    a,b=words(ours),words(independent)
    for i in [1,2,3,5,6,7]:
        if a[i]!=b[i]:raise ValueError(f'Expected unchanged LLVM6 semantic word {i}')
    if a[0]^b[0]!=0x8000:raise ValueError('P0 packet marker difference not explained')
    if a[4]^b[4]!=0x8008:raise ValueError('Predicated extended call low displacement/marker unexpected')
    if b[8]!=0x7f00c000 or b[9]!=0x7f00c000:
        raise ValueError('LLVM6 did not independently generate two layout NOPs')
    data=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',ours.hex()],check=True,capture_output=True,text=True).stdout
    lines=[l for l in data.splitlines() if l.startswith('0x')]
    if len(lines)!=8 or any('invalid' in l.lower() for l in lines):
        raise ValueError('Rizin V66 cannot decode all eight original callback instruction words')
    norm='\n'.join(lines).lower().replace(' ','')
    for pat in ['p0=cmp.eq(r2,##0x0)','if(!p0)call0x96ddb630',
                'r17:16=memd(r29+#0x0)','dealloc_return']:
        if pat not in norm:raise ValueError('Rizin did not confirm '+pat)
    print('PASS exact original MBN: 32 source-generated null callback bytes')
    print('PASS independent LLVM6: 6 unchanged opcode words, documented P0/call packet differences, 2 NOPs')
    print('PASS Rizin V66: all 8 executable words and physical conditional-call target')
    if prefix:print('PASS contiguous file prefix: 1424 bytes = 1384 executable + 40 linker NOPs')

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ['llvm-mc','rz-asm','generated','reference','prefix']:
        parser.add_argument('--'+name,type=Path,required=name!='prefix')
    a=parser.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
