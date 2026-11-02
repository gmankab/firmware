#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 reconstruction of all three later CDSP MMIO config cases."""
from __future__ import annotations
import argparse, importlib.util, os, subprocess,tempfile
from pathlib import Path

FILE_OFFSET=0x550a8
PHYS=0x96cc70a8
LEN=0x20c

def semantic_asm()->str:
    lines=['.text','.global cdsp_late_three_modes','cdsp_late_three_modes:']
    for mode,code,extra in [('feature100',0x31,2),('feature200',0x39,0),('default',0x39,0)]:
        lines.append(f'{mode}:')
        lines.extend([f'{{ r14=##{code} }}','{ r15=##0x44 }','{ r16=##1 }'])
        settings=[
            (0x18,0x20485699),(0x1c,0x2267),(0x20,0x24),(0x0c,0),
            (0x10,0x805),(0x14,0xd0),(0x24,0),(0x28,extra),(0x2c,0x20),
        ]
        for off,val in settings:
            lines.append(f'{{ r0=##{0x08300200+off} }}')
            lines.append(f'{{ memw(r0+#0)=##{val} }}')
        lines.append('{ barrier }')
        if mode!='default':lines.append('{ jump shared_exit }')
    lines.append('shared_exit:')
    return '\n'.join(lines)+'\n'

def get_text(obj:bytes)->bytes:
    modulepath=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('hexagon_parser_configuration',modulepath)
    assert spec is not None and spec.loader is not None
    x=importlib.util.module_from_spec(spec);spec.loader.exec_module(x)
    return x.parse_elf32_text(obj)

def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None)->None:
    data=generated.read_bytes()
    if len(data)==0x2b4:fragment=data[0xa8:]
    elif len(data)==LEN:fragment=data
    else:raise ValueError('Expected 524-byte configuration fragment or 692-byte full prefix')
    assert len(fragment)==LEN
    with tempfile.TemporaryDirectory(prefix='hexagon-controller-config-') as d:
        source=Path(d)/'config.s';obj=Path(d)/'config.o';source.write_text(semantic_asm())
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(source)],check=True)
        independent=get_text(obj.read_bytes())
    if independent!=fragment:
        mismatch=next(i for i,(a,b) in enumerate(zip(independent,fragment)) if a!=b)
        raise ValueError(f'LLVM6 independent byte mismatch file {FILE_OFFSET+mismatch:#x}: {independent[mismatch]:02x} != {fragment[mismatch]:02x}')
    dec=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',fragment.hex()],capture_output=True,text=True,check=True)
    lines=[line for line in dec.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=LEN//4 or any('invalid' in line.lower() for line in lines):
        raise ValueError('Rizin did not recognize every Hexagon controller configuration instruction')
    text='\n'.join(lines).lower().replace(' ','')
    for expected in ['r14=##0x31','r14=##0x39','memw(r0+#0x0)=##0x20485699',
                     'memw(r0+#0x0)=##0x2267','memw(r0+#0x0)=##0x2',
                     'jump0x96cc72b4','barrier']:
        if expected not in text:raise ValueError('Rizin missing '+expected)
    if reference is not None:
        b=reference.read_bytes()
        if b[FILE_OFFSET:FILE_OFFSET+LEN]!=fragment:raise ValueError('Reference CDSP config mismatch')
    print('PASS: all 524 CDSP configuration bytes independently LLVM6-assembled, all 131 Hexagon words decoded by Rizin V66, reference matches')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True);p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--generated',type=Path,required=True);p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference)
