#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently audit 92 bytes of CDSP Hexagon controller bitplane loop.

The typed emitter is original word-exact. The LLVM6 semantic assembly uses
one-instruction packets for most operations, so packet markers and the signed
loop displacement differ; their independently decoded instruction meanings
must nevertheless match and both physical call/branch targets are checked.
"""
from __future__ import annotations
import argparse,importlib.util,re,subprocess,tempfile
from pathlib import Path
from collections import Counter

FILE_START=0x566d8
FILE_END=0x56734
PHYSICAL=0x96cc86d8
LENGTH=FILE_END-FILE_START

def text_from_elf(image:bytes)->bytes:
    file=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('parse_cdsp_elf32',file)
    if spec is None or spec.loader is None:raise RuntimeError('CDSP ELF32 parser unavailable')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(image)

def decode(rizin:Path,data:bytes)->tuple[list[str],Counter[str]]:
    cp=subprocess.run([str(rizin),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],check=True,capture_output=True,text=True)
    lines=[s.strip() for s in cp.stdout.splitlines() if s.startswith('0x')]
    if len(lines)!=LENGTH//4 or any('invalid' in s.lower() for s in lines):
        raise RuntimeError('invalid bitplane V66 packet or unexpected instruction count')
    rex=re.compile(r'^0x[0-9a-f]+\s+\d+\s+[a-f0-9]{8}\s+\S\s+(.*)$',re.I)
    ops=[]
    for line in lines:
        m=rex.match(line)
        if not m:raise RuntimeError('bad Rizin disassembly line '+line)
        ops.append(re.sub(r'\s+','',m[1]).lower())
    return lines,Counter(ops)

def run(llvm:Path,rizin:Path,generated:Path,prefix:Path,reference:Path)->None:
    orig=reference.read_bytes();code=generated.read_bytes();joined=prefix.read_bytes()
    if len(code)!=LENGTH or code!=orig[FILE_START:FILE_END]:
        raise RuntimeError('typed bitloop body must reproduce 92 original bytes')
    if len(joined)!=0x734 or joined!=orig[0x56000:FILE_END] or joined[-LENGTH:]!=code:
        raise RuntimeError('full RWE prefix (1844 = 1800 executable + 44 fill) mismatch')
    with tempfile.TemporaryDirectory(prefix='cdsp-bitloop-body-') as tmp:
        output=Path(tmp)/'bitloop.o'
        fixture=Path(__file__).with_name('late-rwe-controller-bitloop-body.s')
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(fixture),'-o',str(output)],check=True,capture_output=True,text=True)
        independently_assembled=text_from_elf(output.read_bytes())[0x6d8:0x734]
    if len(independently_assembled)!=LENGTH:raise RuntimeError('independent LLVM6 body size mismatch')
    exact=sum(code[i:i+4]==independently_assembled[i:i+4] for i in range(0,LENGTH,4))
    if exact!=12:raise RuntimeError(f'expected 12/23 unchanged LLVM6 words, got {exact}')
    lines,orig_ops=decode(rizin,code)
    llvm_lines,llvm_ops=decode(rizin,independently_assembled)
    if orig_ops!=llvm_ops:
        raise RuntimeError(f'original V66 and LLVM semantic ops differ: {orig_ops-llvm_ops} / {llvm_ops-orig_ops}')
    summary='\n'.join(lines).lower().replace(' ','')
    for operation in ['r2=or(r22,r23)','r2=setbit(r2,#0xc)','r2=ct0(r2)',
                      'r4=minu(r3,r2)','r26=clrbit(r4,#0x0)',
                      'r2=and(##0x3ffffe,asl(r2,#0x1))','r2|=or(r3,##0x70000000)',
                      'call0x96dc0aac','jump:t0x96cc86cc']:
        if operation not in summary:
            raise RuntimeError('missing original V66 operation '+operation)
    print(f'PASS {LENGTH} exact source-built Hexagon executable bytes, zero reference differences')
    print(f'PASS independent LLVM6 equivalent opcodes, {exact}/23 original words unchanged')
    print('PASS Rizin V66 original packet grouping, bitplane loop backedge and direct helper call')
    print('PASS joined CDSP RWE 1844 file bytes = 1800 source executable + 44 structural fill')

if __name__=='__main__':
    a=argparse.ArgumentParser(description=__doc__)
    for k in ['llvm-mc','rz-asm','generated','prefix','reference']:a.add_argument('--'+k,type=Path,required=True)
    args=a.parse_args()
    run(args.llvm_mc,args.rz_asm,args.generated,args.prefix,args.reference)
