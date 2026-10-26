#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6/Rizin V66 audit of source-built controller guard bytes."""
import argparse,importlib.util,os,re,subprocess,tempfile
from pathlib import Path
START,END,PHYS=0x56784,0x567b4,0x96cc8784

def decode(rz:Path,code:bytes):
    p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',code.hex()],
                     check=True,capture_output=True,text=True)
    lines=[x for x in p.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=12 or any('invalid' in x.lower() for x in lines):
        raise RuntimeError('invalid or unexpected original V66 packet structure')
    pat=re.compile(r'^0x[0-9a-f]+\s+\d+\s+[a-f0-9]{8}\s+\S\s+(.*)$',re.I)
    operations=[]
    for line in lines:
        m=pat.match(line)
        if m is None:raise RuntimeError('invalid decoded line '+line)
        op=re.sub(r'\s+','',m[1]).lower()
        if not op.startswith('immext('):operations.append(op)
    return lines,operations

def verify(llvm:Path,rz:Path,generated:Path,prefix:Path,reference:Path):
    ref=reference.read_bytes();out=generated.read_bytes();joined=prefix.read_bytes()
    if len(out)!=END-START or out!=ref[START:END]:
        raise RuntimeError('48 source-built executable bytes differ from reference')
    if len(joined)!=0x7b4 or joined!=ref[0x56000:END] or joined[-len(out):]!=out:
        raise RuntimeError('contiguous 1972 byte RWE prefix differs from reference')
    lines,ops=decode(rz,out)
    summary='\n'.join(lines).lower().replace(' ','')
    for text in ['r23,#0x0);if(p0.new)jump:nt0x96cc8810',
                 'cmp.gtu(r4.new,r3))jump:nt0x96cc8810',
                 'r19=and(r19,##0xfff00)', 'r3=cl0(r23)',
                 'cmp.gtu(r25,r2.new))jump:nt0x96cc88ac']:
        if text not in summary:raise RuntimeError('original reference lacks '+text)
    with tempfile.TemporaryDirectory(prefix='cdsp-guard-llvm6-') as td:
        obj=Path(td)/'llvm.o'
        fixture=Path(__file__).with_name('agent20-10-cdsp-controller-mode-guard.s')
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(fixture),'-o',str(obj)],check=True,capture_output=True,text=True)
        parser=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('verified_cdsp_elf',parser)
        if spec is None or spec.loader is None:raise RuntimeError('CDSP ELF parser unavailable')
        m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
        independent=m.parse_elf32_text(obj.read_bytes())[0x784:0x7b4]
    llvm_lines,llvm_ops=decode(rz,independent)
    if ops!=llvm_ops:raise RuntimeError('original and independently assembled semantic operations differ')
    same=sum(out[i:i+4]==independent[i:i+4] for i in range(0,48,4))
    if same!=7:raise RuntimeError(f'expected 7/12 identical original LLVM instructions, got {same}')
    print('PASS 48 original-byte-identical controller guard bytes; 1972 joined file bytes = 1928 source executable + 44 layout')
    print('PASS LLVM6 12 independently assembled semantic instructions, 7/12 unchanged instruction words')
    print('PASS Rizin V66 12 original instructions, all three guarded branch destinations and packet boundaries')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for name in ['llvm-mc','rz-asm','reference','generated','prefix']:
        p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args()
    verify(a.llvm_mc,a.rz_asm,a.generated,a.prefix,a.reference)
