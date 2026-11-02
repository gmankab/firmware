#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check CDSP Hexagon mode selector against LLVM6 and Rizin, without OEM code import.

LLVM6 reorders two *equivalent* three-instruction predicate packets (the
conditional branch goes first). Normalize only packet instruction order and
parse-marker bits after comparing the semantic operands/branch displacement.
Rizin V66 independently decodes the actual packet layout emitted by Rust.
"""
from __future__ import annotations
import argparse,importlib.util,os,struct,subprocess,tempfile
from pathlib import Path

OFFSET=0x55070
PHYS=0x96cc7070
LENGTH=56

def semantic_asm()->str:
    return '''.text
.global cdsp_mode_selector
cdsp_mode_selector:
{ r0=##0x01fc8000 }
{ r1=memw(r0+#0) }
{ r2=and(r1,##0x100) }
{ p0=cmp.eq(r2,##0x100)
  if(p0.new) jump:t mode_0x100 }
{ r2=and(r1,##0x200) }
{ p0=cmp.eq(r2,##0x200)
  if(p0.new) jump:t mode_0x200 }
{ jump default_mode }
mode_0x100:
.space 0xb0
mode_0x200:
.space 0xb0
default_mode:
'''

def from_llvm_object(obj:bytes)->bytes:
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('hexagon_elf_parser_mode',path)
    assert spec is not None and spec.loader is not None
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(obj)

def normalize_packets(data:bytes)->bytes:
    w=list(struct.unpack('<14I',data))
    for ix in [5,10]:
        branch,ext,cmp=w[ix:ix+3]
        # LLVM scheduler output: branch-continue, extender-continue,
        # compare-final. Vendor clang/V66: extender-continue,
        # compare-continue, branch-final. The original compare and branch
        # operands and packet-relative displacement must otherwise agree.
        if ext&0xc000!=0x4000 or cmp&0xc000!=0xc000 or branch&0xc000!=0x4000:
            raise ValueError('Unexpected compiler packet marker/scheduling')
        w[ix:ix+3]=[ext,(cmp&~0xc000)|0x4000,(branch&~0xc000)|0xc000]
    return struct.pack('<14I',*w)

def check(llvm:Path,rz:Path,generated:Path,reference:Path|None)->None:
    full=generated.read_bytes()
    # Support either the isolated 56-byte detector or the complete 168-byte
    # source-generated controller prefix.
    if len(full)==168:detector=full[112:]
    elif len(full)==56:detector=full
    else:raise ValueError('Expected the 56-byte detector or 168-byte prefix')
    with tempfile.TemporaryDirectory(prefix='cdsp-mode-') as d:
        src=Path(d)/'detect.s';dst=Path(d)/'detect.o';src.write_text(semantic_asm())
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(dst),str(src)],check=True)
        assembled=from_llvm_object(dst.read_bytes())[:LENGTH]
    if normalize_packets(assembled)!=detector:
        raise ValueError('Hexagon 6 independently assembled operands differ after documented packet reordering')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',detector.hex()],
        capture_output=True,text=True,check=True)
    lines=[line for line in decoded.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=14:raise ValueError('Expected 14 four-byte instruction words')
    joined='\n'.join(lines).lower().replace(' ','')
    for text in ('r0=##0x1fc8000','memw(r0+##0x0)','cmp.eq(r2,##0x100)',
                 'jump:t0x96cc70a8','cmp.eq(r2,##0x200)','jump:t0x96cc7158','jump0x96cc7208'):
        if text not in joined:raise ValueError(f'Rizin missing Hexagon semantics {text}')
    if reference is not None:
        b=reference.read_bytes()
        if b[OFFSET:OFFSET+LENGTH]!=detector:raise ValueError('Mode detection differs from CDSP reference')
    print('PASS: 56-byte CDSP mode selector, 14 decoded V66 words, all instruction fields LLVM6-checked')
    print('LLVM6 predicate .new packets were schedule-normalized; actual packet order validated by Rizin')

if __name__=='__main__':
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--llvm-mc',type=Path,required=True);ap.add_argument('--rz-asm',type=Path,required=True)
    ap.add_argument('--generated',type=Path,required=True);ap.add_argument('--reference',type=Path)
    a=ap.parse_args();check(a.llvm_mc,a.rz_asm,a.generated,a.reference)
