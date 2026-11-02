#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""LLVM6/Rizin check of CDSP controller item enumeration at 0x56954..0x56990.

Original V66 packets include a compressed duplex and `.new` predicated
loads unsupported by the LLVM6 scheduler's chosen syntax. LLVM validates
the original instructions' semantic operation forms but not its byte order.
Rizin validates every OEM word, packet and physical branch/call destination.
"""
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
import argparse,subprocess,tempfile,importlib.util

FILE=0x56954;START=0x56930;END=0x56990;PHYSICAL=0x96cc8954

def parse(obj:bytes)->bytes:
    f=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elfparser',f)
    if spec is None or spec.loader is None: raise ValueError('missing independent ELF parser')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(obj)

def decode(rz:Path,data:bytes,physical:int)->list[str]:
    p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(physical),'-D',data.hex()],
        capture_output=True,text=True,check=True)
    lines=[x for x in p.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=len(data)//4 or any('invalid' in x.lower() for x in lines):
        raise ValueError('incomplete Hexagon V66 instruction decoding')
    return lines

def validate(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None):
    b=generated.read_bytes(); firmware=reference.read_bytes()
    if len(b)!=END-START or b!=firmware[START:END]:
        raise ValueError('96-byte source-built controller prefix does not match pinned CDSP')
    if prefix:
        p=prefix.read_bytes()
        if len(p)!=END-0x56000 or p!=firmware[0x56000:END]:
            raise ValueError('2448-byte RWE prefix mismatch')
    orig=b[FILE-START:]
    fixture=SOURCE_DIR / 'late-controller-enumeration-semantic.s'
    with tempfile.TemporaryDirectory(prefix='nabu-cdsp-loop-') as tmp:
        o=Path(tmp)/'fixture.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(fixture),'-o',str(o)],capture_output=True,text=True,check=True)
        alt=parse(o.read_bytes())[:64]
    ours=decode(rz,orig,PHYSICAL)
    compiler=decode(rz,alt,PHYSICAL)
    output='\n'.join(ours).lower().replace(' ','')
    cc='\n'.join(compiler).lower().replace(' ','')
    for token in ['jump:nt0x96cc8968','call0x96ddf2c0',
        'r1=add(r29,#0x18)','r0=r16','r17=add(r17,##0x1)',
        'r16=add(r16,##0xa0)','r2=memb(r16+##0xa0)',
        'jump:t0x96cc8954','memw(sp+##0x14)','r21=##0x0',
        'jump:nt0x96cc8ad0']:
        if token not in output:raise ValueError('original V66 packet missing '+token)
    for token in ['call0x96ddf2c0','r1=add(fp,##0x18)','r0=r16',
        'r16=add(r16,##0xa0)','jump:t0x96cc8954','jump:nt0x96cc8ad0']:
        if token not in cc:raise ValueError('independent LLVM fixture missing '+token)
    # LLVM6 expands original dual R1/R0 operation, so there is one extra
    # word. Compare operands for identical semantic operations after stripping
    # only packet parse bits (not arithmetic or register fields).
    refwords=[int.from_bytes(orig[i:i+4],'little') for i in range(0,len(orig),4)]
    altwords=[int.from_bytes(alt[i:i+4],'little') for i in range(0,len(alt),4)]
    for i,j in [(3,4),(4,5),(5,6),(6,7),(7,8),(9,10),(11,12)]:
        if refwords[i]&~0xc000 != altwords[j]&~0xc000:
            raise ValueError(f'LLVM6 scalar operand mismatch: OEM {i}, fixture {j}')
    print('PASS: CDSP 60 additional typed Hexagon bytes, zero OEM differences')
    print('PASS: LLVM6 independent semantic scalar encodings and Rizin V66 15 original packet words')
    if prefix:print('PASS: 2448-byte RWE file prefix (2392 code + 56 layout) exact')

if __name__=='__main__':
    a=argparse.ArgumentParser(description=__doc__)
    for f in ('llvm-mc','rz-asm','generated','reference','prefix'):
        a.add_argument('--'+f,type=Path,required=f!='prefix')
    z=a.parse_args();validate(z.llvm_mc,z.rz_asm,z.generated,z.reference,z.prefix)
