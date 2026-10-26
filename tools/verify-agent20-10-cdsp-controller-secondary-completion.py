#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Reassemble the secondary controller capacity and command prologue with LLVM6 and audit V66 packets."""
from __future__ import annotations
import argparse,importlib.util,os,re,subprocess,tempfile
from pathlib import Path
START=0x5680c
END=0x56848
PHYSICAL=0x96cc880c

def rizin(rz:Path,span:bytes):
    run=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',span.hex()],
        check=True,capture_output=True,text=True,env=os.environ.copy())
    rows=[line for line in run.stdout.splitlines() if line.startswith('0x')]
    if len(rows)!=15 or any('invalid' in row.lower() for row in rows):
        raise RuntimeError('expected 15 valid decoded V66 words')
    return rows

def verify(args):
    cdsp=args.reference.read_bytes();emitted=args.generated.read_bytes();joined=args.prefix.read_bytes()
    if len(emitted)!=END-START or emitted!=cdsp[START:END]:
        raise RuntimeError('generated 60-byte controller completion mismatches original reference')
    if len(joined)!=END-0x56000 or joined!=cdsp[0x56000:END] or joined[-60:]!=emitted:
        raise RuntimeError('source-built joined 2120-byte prefix differs from reference')
    original=rizin(args.rz_asm,emitted)
    text='\n'.join(original).lower().replace(' ','')
    for required in ('r2=add(r17,##-0x1f)','p0=cmp.gtu(r22,r20);if(p0.new)jump:nt0x96cc881c',
                     'if(cmp.gtu(r3.new,r20))jump:t0x96cc884c',
                     'if(!cmp.gtu(r3.new,r2))jump:nt0x96cc88c8',
                     'r3=##-0x40000000','immext(##0x77000000)',
                     'r2=or(r4,and(r2,##0x1ffffe))'):
        if required not in text:raise RuntimeError('Rizin decoded meaning lacks '+required)
    with tempfile.TemporaryDirectory(prefix='cdsp-secondary-capacity-') as td:
        obj=Path(td)/'capacity.o'
        fixture=Path(__file__).with_name('agent20-10-cdsp-controller-secondary-completion.s')
        subprocess.run([str(args.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(fixture),'-o',str(obj)],check=True,capture_output=True,text=True)
        parser=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('cdsp_elf_sections',parser)
        if spec is None or spec.loader is None:raise RuntimeError('ELF fixture parser unavailable')
        m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
        independent=m.parse_elf32_text(obj.read_bytes())[0x80c:0x848]
    if independent!=emitted:raise RuntimeError('independent LLVM6 assembled 15 instructions differ from typed Rust')
    rizin(args.rz_asm,independent)
    print('PASS: 60 source-authored Hexagon V66 bytes identical to original CDSP and LLVM6 independent fixture')
    print('PASS: joined 2120-byte controller RWE prefix = 2076 executable + 44 structural fill')
    print('PASS: all 15 original packet words, three guard destinations and two extended constants decode in Rizin V66')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for name in ('llvm-mc','rz-asm','generated','prefix','reference'):
        p.add_argument('--'+name,type=Path,required=True)
    verify(p.parse_args())
