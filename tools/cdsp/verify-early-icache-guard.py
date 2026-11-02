#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""CDSP I-cache guard: 12 V66 words decoded, 11 independently LLVM-assembled.

The REV special-register read has no supported LLVM6 assembler spelling;
it is verified by the independent Rizin V66 decoder + reference only.
"""
import argparse,hashlib,importlib.util,re,subprocess,tempfile
from pathlib import Path
START=0x5268;END=0x5298;PC=0x96c0_2268
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=['R0=REV','R0=and(R0,##0xff)','P0=cmp.gt(R0,##0x55)',
          'if(!P0.new)jumpr:ntLR','R0=##0x1','immext(##0xfe016e00)',
          'R2=memw(R1=##0xfe016e20)','P0=cmp.eq(R2,#0x0);if(!P0.new)jumpr:ntR31',
          'memw(R1+##0x0)=R0','allocframe(SP,#0x10)','call0x96c03388','jump0x96c0239c']
SOURCE='''
.text
.global entry
entry:
{ r0=and(r0,#255) }
{ p0=cmp.gt(r0,#85); if (!p0.new) jumpr:nt lr }
{ r0=#1; r2=memw(r1=##0xfe016e20) }
{ p0=cmp.eq(r2,#0); if (!p0.new) jumpr:nt lr }
{ memw(r1+#0)=r0 }
{ allocframe(#16) }
{ call target }
{ jump exit }
.space (0x96c03388 - 0x96c02268 - 48)
target:
{ jumpr lr }
.space 4
exit:
{ jumpr lr }
'''

def assemble(mc:Path,src:str)->bytes:
    with tempfile.TemporaryDirectory() as d:
        d=Path(d);(d/'s.s').write_text(src)
        q=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(d/'s.o'),str(d/'s.s')],capture_output=True,text=True)
        if q.returncode:raise ValueError('LLVM6 assembler failed: '+q.stderr)
        spec=importlib.util.spec_from_file_location('helper',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
        if spec is None or spec.loader is None:raise ValueError('Could not load ELF .text extractor')
        helper=importlib.util.module_from_spec(spec);spec.loader.exec_module(helper)
        return helper.parse_elf32_text((d/'s.o').read_bytes())

def check(b:bytes,reference:Path,rz:Path,mc:Path):
    ref=reference.read_bytes()
    if hashlib.sha256(ref).hexdigest()!=SHA:raise ValueError('Unexpected CDSP revision')
    if len(b)!=48 or b!=ref[START:END]:raise ValueError('I-cache guard differs from pinned CDSP bytes')
    q=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',b.hex()],capture_output=True,text=True,check=True)
    rows=[]
    for line in q.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:rows.append((int(m.group(1),16),''.join(m.group(2).lower().split())))
    if len(rows)!=12:raise ValueError(f'Rizin decoded {len(rows)} of 12 words')
    for n,((pc,text),expected) in enumerate(zip(rows,EXPECTED)):
        if pc!=PC+n*4 or ''.join(expected.lower().split()) not in text:
            raise ValueError(f'Rizin mismatch word {n}: {text} (expected {expected})')
    sample=assemble(mc,SOURCE)
    # LLVM's ordinary instructions are in words 0..9, corresponding to OEM
    # positions 1..10; privileged REV is original word 0.
    for n in range(10):
        if sample[4*n:4*n+4]!=b[4*(n+1):4*(n+2)]:
            raise ValueError(f'LLVM6 ordinary instruction mismatch at OEM word {n+1}')
    jump=assemble(mc,'.text\n.global entry\nentry:\n{ jump dest }\n.space 0x104\ndest:\n{ jumpr lr }\n')
    if jump[:4]!=b[-4:]:raise ValueError('LLVM6 I-cache exit jump opcode mismatch')
    print('CDSP I-cache revision guard: 48 exact bytes, 12 Rizin V66 decodes, 11 independent LLVM6 exact words; REV decoder/reference only')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for field in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+field,required=True,type=Path)
    args=p.parse_args()
    check(args.rust_output.read_bytes(),args.reference,args.rz_asm,args.llvm_mc)
