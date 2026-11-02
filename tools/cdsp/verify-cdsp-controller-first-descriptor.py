#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently reassemble first 108-byte CDSP descriptor with LLVM6.

Hexagon V60 LLVM assembles the complete packets, including the .new branch
conditions and extended MMIO word-store instructions. The CDSP V66 Rizin
backend verifies the interpreted condition and branch destinations.
"""
from __future__ import annotations
import argparse
import importlib.util
from pathlib import Path
import subprocess
import tempfile

FILE_OFFSET=0x3fec
LENGTH=108
PHYSICAL=0x96c00fec
SOURCE='''.text
.global cdsp_first_controller_descriptor
cdsp_first_controller_descriptor:
{ r21=memw(r25+##-0x1fe92a4) }
{ r23=#170
  memw(r21+##44)=r23.new }
{ r23=#10
  memw(r21+##48)=r23.new }
{ r23=#5
  r22=memw(r25+##-0x1fd92bc)
  if(cmp.eq(r22.new,#0)) jump:t n1 }
{ r23=#10 }
n1:
{ memw(r21+##0)=r23 }
{ r23=#5
  r22=memw(r25+##-0x1fd92b8)
  if(cmp.eq(r22.new,#0)) jump:t n2 }
{ r23=#10 }
n2:
{ r22=memw(r25+##-0x1fd92b4) }
{ r22=asl(r22,#4) }
{ r23=add(r23,r22) }
{ memw(r21+##40)=r23 }
{ r21=add(r21,#256) }
'''

def load(name,alias):
    spec=importlib.util.spec_from_file_location(alias,Path(__file__).with_name(name))
    assert spec and spec.loader
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None):
    out=generated.read_bytes()
    if len(out)!=LENGTH:raise ValueError('descriptor must emit 108 bytes')
    with tempfile.TemporaryDirectory(prefix='cdsp-first-descriptor-') as d:
        source=Path(d)/'descriptor.s';obj=Path(d)/'descriptor.o'
        source.write_text(SOURCE)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
           '-filetype=obj','-o',str(obj),str(source)],check=True)
        data=load('verify-cdsp-hexagon-entry.py','cdsp_first_descriptor_elf')\
          .parse_elf32_text(obj.read_bytes())
    if data!=out:raise ValueError('LLVM independently emitted a different CDSP descriptor')
    decoded=load('verify-cdsp-tlb.py','cdsp_first_descriptor_decoder')\
        .decode_rizin(rz,out,physical_address=PHYSICAL)
    if len(decoded)!=LENGTH//4:raise ValueError('Rizin decoded wrong instruction count')
    mandatory=(
       'memw(r25+##-0x1fe92a4)',
       'memw(r21+##0x2c)=r23.new',
       'memw(r21+##0x30)=r23.new',
       'if(cmp.eq(r22.new,#0x0))jump:t0x96c01020',
       'if(cmp.eq(r22.new,#0x0))jump:t0x96c0103c',
       'r21=add(r21,##0x100)',
    )
    for item in mandatory:
        if not any(item in line for line in decoded):raise ValueError(f'Rizin did not confirm {item}')
    if reference is not None:
        original=reference.read_bytes()
        if original[FILE_OFFSET:FILE_OFFSET+LENGTH]!=out:
            raise ValueError('CDSP original first controller descriptor differs')
    print('CDSP first controller descriptor: 108/108 source-rebuilt bytes independently matched LLVM6, all 27 V66 instructions decoded by Rizin')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--rust-output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
