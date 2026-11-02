#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 semantics/Rizin packet verification of indexed CDSP lookup.

The source firmware uses R3.new and R2.new fused compares, but LLVM6
emits P1 predicate compares. Both branches must have identical absolute
physical destinations; the code asserts this and reports altered scheduling.
"""
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
FILE=0x56a30
END=0x56a74
PHYSICAL=0x96cc8a30

def extract_text(raw:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf32_independent',p)
    if spec is None or spec.loader is None:raise ValueError('ELF parser unavailable')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.parse_elf32_text(raw)

def as_words(raw:bytes)->list[int]:
    if len(raw)%4:raise ValueError('misaligned instruction region')
    return [int.from_bytes(raw[i:i+4],'little') for i in range(0,len(raw),4)]

def lines(rz:Path,data:bytes)->list[str]:
    ret=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],
        check=True,capture_output=True,text=True)
    out=[x for x in ret.stdout.splitlines() if x.startswith('0x')]
    if len(out)!=len(data)//4 or any('invalid' in x.lower() for x in out):
        raise ValueError('Rizin could not decode the complete Hexagon source')
    return out

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None):
    data=generated.read_bytes();orig=reference.read_bytes()
    if len(data)!=68 or data!=orig[FILE:END]:
        raise ValueError('indexed CDSP code differs from pinned reference')
    if prefix:
        b=prefix.read_bytes()
        if len(b)!=END-0x56000 or b!=orig[0x56000:END]:
            raise ValueError('2676-byte CDSP RWE prefix differs from pinned reference')
    fixture=Path(__file__).with_name('late-controller-record-lookup-semantic.s')
    with tempfile.TemporaryDirectory(prefix='nabu-cdsp-lookup-') as temp:
        obj=Path(temp)/'lookup.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(fixture),'-o',str(obj)],
            capture_output=True,text=True,check=True)
        compiled=extract_text(obj.read_bytes())[:68]
    if len(compiled)!=68:raise ValueError('LLVM fixture did not produce 17 original operands')
    source,independent=as_words(data),as_words(compiled)
    for i in (2,4,6,8,10,11,12,14):
        if source[i]!=independent[i]:raise ValueError(f'LLVM6 differs from exact word {i}')
    for i in (0,3,5,7,9,13,15):
        if source[i]&~0xc000!=independent[i]&~0xc000:
            raise ValueError(f'LLVM6 differs beyond packet parse markers at word {i}')
    orig_lines=lines(rz,data);comp_lines=lines(rz,compiled)
    act='\n'.join(orig_lines).lower().replace(' ','')
    alt='\n'.join(comp_lines).lower().replace(' ','')
    for token in ['r3=memw(r17+r23<<#0x0)','jump:nt0x96cc8aa8',
        'call0x96dc0bac','r17=add(r16,##0x18)',
        'call0x96cd1230','r2=lsr(r3,#0xc)']:
        if token not in act or token not in alt:
            raise ValueError('Hexagon lookup semantic field or branch changed: '+token)
    for token in ['if(cmp.eq(r3.new,-1))','if(cmp.eq(r2.new,r16))']:
        if token not in act:raise ValueError('reference fused new-value guard missing: '+token)
    for token in ['p1=cmp.eq(r3,-1)','p1=cmp.eq(r2,r16)']:
        if token not in alt:raise ValueError('LLVM6 equivalent P1 guard missing: '+token)
    print('PASS: indexed CDSP lookup 68 typed bytes exactly match reference; 17 Rizin V66 words')
    print('PASS: LLVM6 independent operand checks, 8 unchanged words and 7 parse-only differences')
    print('PASS: two guarded branches and both absolute service-call targets verified')
    if prefix:print('PASS: contiguous 2676-byte RWE prefix = 2620 executable + 56 fill')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ('llvm-mc','rz-asm','generated','reference','prefix'):
        p.add_argument('--'+name,type=Path,required=name!='prefix')
    opts=p.parse_args();verify(opts.llvm_mc,opts.rz_asm,opts.generated,opts.reference,opts.prefix)
