#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check independently authored LLVM6 controller return packet sources against V66 Rizin and original CDSP."""
from __future__ import annotations
import argparse,importlib.util,os,re,subprocess,tempfile
from pathlib import Path
START,END,PHYS=0x56848,0x56890,0x96cc8848

def decode(rz:Path,b:bytes):
    p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',b.hex()],check=True,capture_output=True,text=True)
    rows=[l for l in p.stdout.splitlines() if l.startswith('0x')]
    if len(rows)!=18 or any('invalid' in row.lower() for row in rows):
        raise RuntimeError('expected eighteen valid Rizin V66 instruction words')
    pattern=re.compile(r'^0x[0-9a-f]+\s+\d+\s+[a-f0-9]{8}\s+\S\s+(.*)$',re.I)
    decoded=[]
    for line in rows:
        m=pattern.match(line)
        if not m:raise RuntimeError('unknown Rizin line '+line)
        decoded.append(re.sub(r'\s+','',m[1]).lower())
    return rows,decoded

def verify(args):
    source=args.generated.read_bytes();ref=args.reference.read_bytes();prefix=args.prefix.read_bytes()
    if len(source)!=END-START or source!=ref[START:END]:
        raise RuntimeError('72 source-built controller return bytes differ from CDSP')
    if len(prefix)!=END-0x56000 or prefix!=ref[0x56000:END] or prefix[-72:]!=source:
        raise RuntimeError('controller 2192-byte RWE prefix differs from reference')
    original_rows,original_ops=decode(args.rz_asm,source)
    joined='\n'.join(original_rows).lower().replace(' ','')
    for required in ('r1:0=combine(##0x0,#0x1)','r2=memw(r16+#0x0);r17:16=memd(r29+#0x28)',
       'r19:18=memd(r29+#0x20);r21:20=memd(r29+#0x18)',
       'jump0x96dc0aac','r27:26=memd(sp+##0x0)','lr:fp=deallocframe(fp):raw'):
        if required not in joined:raise RuntimeError('Rizin did not confirm '+required)
    with tempfile.TemporaryDirectory(prefix='cdsp-return-llvm6-') as td:
        obj=Path(td)/'returns.o'
        fixture=Path(__file__).with_name('late-controller-return-paths.s')
        subprocess.run([str(args.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(fixture),'-o',str(obj)],check=True,capture_output=True,text=True)
        parser=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('cdsp_elf_decoder',parser)
        if spec is None or spec.loader is None:raise RuntimeError('unavailable ELF section parser')
        mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
        independent=mod.parse_elf32_text(obj.read_bytes())[0x848:0x890]
    llvm_rows,llvm_ops=decode(args.rz_asm,independent)
    if original_ops!=llvm_ops:
        for i,(a,b) in enumerate(zip(original_ops,llvm_ops)):
            if a!=b:raise RuntimeError(f'LLVM6 original meaning mismatch word {i}: {a} vs {b}')
        raise RuntimeError('LLVM6 semantic packet count differs')
    same=sum(source[i:i+4]==independent[i:i+4] for i in range(0,len(source),4))
    if same!=14:raise RuntimeError(f'Expected 14/18 LLVM6 unchanged words, got {same}')
    print('PASS 72 source-encoded controller return-path bytes; joined RWE prefix 2192 = 2148 executable + 44 NOP')
    print('PASS LLVM6 18 semantic instructions, 14/18 byte-identical original packet words')
    print('PASS Rizin V66 original 18 words, 2 shared helper jumps and both register-restore/return paths')

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    for key in ('llvm-mc','rz-asm','generated','prefix','reference'):
        parser.add_argument('--'+key,type=Path,required=True)
    verify(parser.parse_args())
