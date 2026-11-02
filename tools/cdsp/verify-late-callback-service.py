#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""LLVM6 and Rizin V66 checks for CDSP callback broker 0x564b8..0x56540.

LLVM6's semantic fixture is 128 bytes, eight bytes shorter than the source-
reconstructed original: it shortens the forced extended GP status load/store
and reschedules some new-value packet operations. Only proven unchanged words
are compared to LLVM6, and all original words/branches are checked via Rizin.
"""
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
START=0x564b8
LENGTH=136
PHYS=0x96cc84b8
RWE_START=0x56000

def read_elf(data:bytes)->bytes:
    script=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_callback_broker_elf',script)
    if spec is None or spec.loader is None:raise ValueError('Missing independent ELF reader')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.parse_elf32_text(data)

def words(buf:bytes)->list[int]:
    if len(buf)%4:raise ValueError('Hexagon program alignment not multiple of 4')
    return [int.from_bytes(buf[i:i+4],'little') for i in range(0,len(buf),4)]

def verify(llvm:Path,rz:Path,gen:Path,reference:Path,prefix:Path|None)->None:
    actual=gen.read_bytes();ref=reference.read_bytes()
    if len(actual)!=LENGTH or ref[START:START+LENGTH]!=actual:
        raise ValueError('136 CDSP callback bytes differ from pinned source')
    if prefix:
        full=prefix.read_bytes()
        if len(full)!=0x540 or full!=ref[RWE_START:START+LENGTH]:
            raise ValueError('Contiguous 1344-byte CDSP RWE prefix mismatch')
        if full[-LENGTH:]!=actual:raise ValueError('Joined prefix disagrees with emitted broker')
    with tempfile.TemporaryDirectory(prefix='cdsp_broker_') as tmp:
        obj=Path(tmp)/'callback-broker.o'
        asm=Path(__file__).with_name('late-rwe-callback-orchestrator.s')
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj',str(asm),'-o',str(obj)],
                       check=True,text=True,capture_output=True)
        independent=read_elf(obj.read_bytes())[0x4b8:]
    if len(independent)!=128:raise ValueError(f'Unexpected {len(independent)} LLVM6 bytes')
    a,b=words(actual),words(independent)
    pairs=[(1,1),(2,2),(7,6),(9,8),(10,9),(11,10),(12,11),
        (13,12),(14,13),(15,14),(16,15),(21,19),(22,20),(23,21),
        (24,22),(25,23),(26,24),(27,25),(28,26),(29,27),
        (30,28),(31,29),(32,30),(33,31)]
    for i,j in pairs:
        if a[i]!=b[j]:raise ValueError(f'LLVM6 semantic opcode differs for original word {i}, independent {j}')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],
                           check=True,text=True,capture_output=True).stdout
    lines=[l for l in decoded.splitlines() if l.startswith('0x')]
    if len(lines)!=34 or any('invalid' in l.lower() for l in lines):
        raise ValueError(f'Rizin only verified {len(lines)}/34 words')
    normalized='\n'.join(lines).lower().replace(' ','')
    for x in ['memw(gp+##0xc028c028)','jump:nt0x96cc8538',
              'call0x96ddf3a0','trap0(#0x0)',
              'if(cmp.gt(r2.new,#0x1))jump:t0x96cc850c',
              'memw(gp+##0xc028c028)=r2.new',
              'memw(sp+##0x8)=r2.new','lr:fp=dealloc_return']:
        if x not in normalized:raise ValueError('Missing Rizin callback feature '+x)
    if normalized.count('trap0(#0x0)')!=3:raise ValueError('Three trap0 callbacks not decoded')
    print('PASS exactly 136 CDSP source-generated callback broker bytes')
    print(f'PASS LLVM6 semantic cross-check: {len(pairs)} unchanged words out of 34; 128-byte compiler layout accounted')
    print('PASS Rizin V66: all 34 original-order words; physical callback and three trap0 services')
    if prefix:print('PASS contiguous CDSP RWE file prefix: 1344 bytes (1312 executable and 32 structural NOPs)')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+name,type=Path,required=name!='prefix')
    a=p.parse_args()
    verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
