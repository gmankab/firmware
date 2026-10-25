#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6+Rizin byte verification for first 84 CDSP RWE startup bytes."""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
START=0x56000
PHYSICAL=0x96cc8000
SIZE=0x54

def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None):
    source=Path(__file__).with_name('agent20-10-cdsp-rwe-entry.s')
    with tempfile.TemporaryDirectory(prefix='cdsp-rwe-entry-') as tmp:
        obj=Path(tmp)/'entry.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],check=True)
        dep=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('cdsp_parse_elf32',dep)
        assert spec and spec.loader
        module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
        independent=module.parse_elf32_text(obj.read_bytes())
    actual=generated.read_bytes()
    if len(actual)!=SIZE or actual!=independent:
        i=next((j for j,(a,b) in enumerate(zip(actual,independent)) if a!=b),0)
        raise ValueError(f'Hexagon typed emitter differs from LLVM6 at CDSP file {START+i:#x}')
    dec=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',actual.hex()],
                       capture_output=True,text=True,check=True)
    lines=[line for line in dec.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=21 or any('invalid' in line.lower() for line in lines):
        raise ValueError('Rizin V66 must decode 21 4-byte executable words')
    text='\n'.join(lines).replace(' ','').lower()
    for token in ['allocframe(sp,#0x8)','r3=##0x0;r1=##0x0','callrr28','jump0x96cc8034','jumprlr']:
        if token not in text: raise ValueError('Rizin decoder did not confirm '+token)
    if reference and reference.read_bytes()[START:START+SIZE]!=actual:
        raise ValueError('CDSP RWE startup differs from pinned original reference')
    print('PASS: 84 typed Hexagon RWE startup bytes exactly assembled LLVM6 and independently decoded Rizin V66')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True);p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--generated',type=Path,required=True);p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference)
