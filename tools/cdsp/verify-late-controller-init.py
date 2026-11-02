#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6/Rizin V66 validation of CDSP 0x56930..0x56954.

LLVM6 proves nine semantic instruction words but schedules one P0 compare
before the extended input byte-load. Rizin confirms the OEM packet order,
physical calls and new-value consumer. This verifier never inserts OEM code
into source generation.
"""
from pathlib import Path
import argparse,importlib.util,os,subprocess,tempfile
FILE=0x56930
LENGTH=36
PHYSICAL=0x96cc8930

def get_object_text(obj:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('independent_cdsp_elf',p)
    if spec is None or spec.loader is None:raise RuntimeError('Missing ELF32 independent parser')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(obj)

def words(raw:bytes)->list[int]:
    if len(raw)%4:raise ValueError('not 32-bit Hexagon words')
    return [int.from_bytes(raw[i:i+4],'little') for i in range(0,len(raw),4)]

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None):
    result=generated.read_bytes();firmware=reference.read_bytes()
    if len(result)!=LENGTH or firmware[FILE:FILE+LENGTH]!=result:
        raise ValueError('controller-init emitted bytes are not exact reference')
    if prefix:
        p=prefix.read_bytes()
        if len(p)!=FILE+LENGTH-0x56000 or p!=firmware[0x56000:FILE+LENGTH]:
            raise ValueError('controller-init full RWE prefix comparison failed')
    fixture=Path(__file__).with_name('late-controller-init-semantic.s')
    with tempfile.TemporaryDirectory(prefix='nabu-cdsp-init-') as temp:
        obj=Path(temp)/'controller.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(fixture),'-o',str(obj)],check=True,capture_output=True,text=True)
        alt=get_object_text(obj.read_bytes())[:LENGTH]
    if len(alt)!=LENGTH:raise ValueError('LLVM6 controller fixture not 9 words')
    model=words(result);independent=words(alt)
    # LLVM ends the call packet whereas the original continues the packet,
    # which is precisely and only a 0x8000 parse-marker difference.
    if model[0]^independent[0]!=0x8000:
        raise ValueError('controller call semantic fields do not agree with LLVM6')
    # The extended controller-log store and predicate-to-register stack store
    # are unchanged. LLVM moves the predicate branch before the byte load.
    indices={1:1,2:2,3:5,4:6,5:4,7:7,8:8}
    for a,b in indices.items():
        if model[a]!=independent[b]:
            raise ValueError(f'LLVM6 instruction fields differ OEM word {a}, independent word {b}')
    # The transformed branch must keep the same actual absolute target.
    disassemble=lambda raw:subprocess.run([str(rz),'-a','hexagon','-b','32',
        '-o',hex(PHYSICAL),'-D',raw.hex()],check=True,capture_output=True,text=True).stdout
    actual=disassemble(result)
    compiler=disassemble(alt)
    lines=[x for x in actual.splitlines() if x.startswith('0x')]
    if len(lines)!=9 or any('invalid' in x.lower() for x in lines):
        raise ValueError('Rizin failed to decode all 9 OEM packet words')
    norm='\n'.join(lines).lower().replace(' ','')
    for marker in ['call0x96dc4248','0xc0364ae0','0xc02a0e40',
        'combine(##0x0,r20)','jump:nt0x96cc8ad0','r2=p0','memw(sp+##0x14)=r2.new']:
        if marker not in norm:raise ValueError(f'Rizin missing original semantic fragment {marker}')
    if 'jump:nt 0x96cc8ad0' not in compiler:
        raise ValueError('LLVM6 branch does not target original absolute destination')
    print('PASS: exact 36 source-generated controller bytes and physical call/skip targets')
    print('PASS: LLVM6 nine operations, original schedule/packet verified with Rizin V66')
    if prefix:print('PASS: full 2388-byte RWE prefix matches pinned CDSP image')

if __name__=='__main__':
    ap=argparse.ArgumentParser(description=__doc__)
    for field in ('llvm-mc','rz-asm','generated','reference','prefix'):
        ap.add_argument('--'+field,type=Path,required=field!='prefix')
    x=ap.parse_args()
    verify(x.llvm_mc,x.rz_asm,x.generated,x.reference,x.prefix)
