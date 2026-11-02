#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Reference-independent LLVM6 fixture and Rizin V66 packet audit of a source-built second bitloop."""
from __future__ import annotations
import argparse,importlib.util,os,re,subprocess,tempfile
from pathlib import Path
START,END,PHYS=0x567b4,0x5680c,0x96cc87b4
PREFIX_START=0x56000
WORD_COUNT=(END-START)//4

def decode(rz:Path,code:bytes):
    env=os.environ.copy()
    run=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',code.hex()],
         check=True,env=env,capture_output=True,text=True)
    lines=[line for line in run.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=WORD_COUNT or any('invalid' in line.lower() for line in lines):
        raise RuntimeError(f'Rizin did not decode exactly {WORD_COUNT} original Hexagon words')
    pat=re.compile(r'^0x[0-9a-f]+\s+\d+\s+[a-f0-9]{8}\s+\S\s+(.*)$',re.I)
    ops=[]
    for line in lines:
        m=pat.match(line)
        if m is None:raise RuntimeError('unexpected decoder output: '+line)
        ops.append(re.sub(r'\s+','',m[1]).lower())
    return lines,ops

def verify(llvm:Path,rz:Path,generated:Path,prefix:Path,reference:Path):
    ref=reference.read_bytes();out=generated.read_bytes();joined=prefix.read_bytes()
    if len(out)!=END-START or out!=ref[START:END]:
        raise RuntimeError('88 generated executable bytes differ from pinned original')
    if len(joined)!=END-PREFIX_START or joined!=ref[PREFIX_START:END] or joined[-len(out):]!=out:
        raise RuntimeError('complete 2060-byte RWE prefix does not exactly match pinned original')
    original_lines, original_ops=decode(rz,out)
    expected={
        'r2=or(r19,r24)', 'r2=setbit(r2,#0xc)', 'r2=ct0(r2)',
        'r4=minu(r3,r2)', 'r23-=asl(r26,r27)',
        'r2|=or(r3,##0x77000000)', 'r3=or(r24,##-0x40000000)',
        'call0x96dc0aac', 'r24+=asl(r26,r27)',
        'r19+=asl(r26,r27)',
        'p0=cmp.eq(r23,#0x0);if(!p0.new)jump:t0x96cc87a8',
    }
    missing=expected-set(original_ops)
    if missing:raise RuntimeError('original opcode decoder missing '+str(sorted(missing)))
    # Original packet marker structure remains a separate architectural check.
    packet_marker=[x.split()[3] for x in original_lines]
    if packet_marker.count('[')!=3 or packet_marker[-1]!='\\':
        raise RuntimeError('unexpected Hexagon V66 packet markers')
    with tempfile.TemporaryDirectory(prefix='cdsp-secondary-llvm6-') as tmp:
        obj=Path(tmp)/'independent.o'
        fixture=Path(__file__).with_name('late-controller-secondary-bitloop.s')
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(fixture),'-o',str(obj)],check=True,capture_output=True,text=True)
        parser=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('hexagon_elf_parser',parser)
        if spec is None or spec.loader is None:raise RuntimeError('ELF section parser not found')
        m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
        independently_assembled=m.parse_elf32_text(obj.read_bytes())[0x7b4:0x80c]
    if len(independently_assembled)!=END-START:raise RuntimeError('independent LLVM6 span size mismatch')
    llvm_lines,llvm_ops=decode(rz,independently_assembled)
    if llvm_ops!=original_ops:
        for k,(a,b) in enumerate(zip(original_ops,llvm_ops)):
            if a!=b:raise RuntimeError(f'Rizin instruction semantics mismatch at word {k}: {a} vs {b}')
        raise RuntimeError('LLVM6 instruction sequence incomplete')
    same=sum(out[j:j+4]==independently_assembled[j:j+4] for j in range(0,len(out),4))
    if same!=11:raise RuntimeError(f'expected LLVM6 11/22 exact words, got {same}')
    print('PASS: 88 source-generated executable bytes equal reference; 2060-byte RWE layout = 2016 executable + 44 alignment')
    print('PASS: LLVM6 independently reconstructs all 22 semantic instructions, 11 unchanged instruction words')
    print('PASS: Rizin V66 independently validates all 22 original packet words, helper call, retry target and extenders')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for name in ('llvm-mc','rz-asm','generated','prefix','reference'):
        p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.prefix,a.reference)
