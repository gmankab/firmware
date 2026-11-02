#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""LLVM6/Rizin cross-check of the CDSP late controller's 120-byte mask commit."""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
FILE_OFFSET=0x552b4
PHYSICAL=0x96cc72b4
LENGTH=0x78

def semantic_asm()->str:
    lines=['.text','.global cdsp_controller_commit','cdsp_controller_commit:']
    lines.extend(['{ r0=##0x08300204\n  r1=r14 }','{ memw(r0+#0)=r1 }','{ barrier }',
                  '{ r0=##0x08300208 }','{ r1=r15 }','{ memw(r0+#0)=r1 }','{ barrier }'])
    for addr,mask in [(0x08300200,1),(0x08300238,7)]:
        lines.extend([f'{{ r0=##{addr} }}','{ r7=r0\n  r5.h=#0 }',f'{{ r5.l=#{mask} }}',
                      '{ r5=sub(#-1,r5)\n  r4=memw(r0+#0) }','{ r4=and(r4,r5) }',
                      '{ memw(r0+#0)=r4 }','{ barrier }'])
    return '\n'.join(lines)+'\n'

def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None)->None:
    inp=generated.read_bytes()
    if len(inp)==0x32c:code=inp[0x2b4:]
    elif len(inp)==LENGTH:code=inp
    else:raise ValueError('Expected 120-byte mask commit or 812-byte controller prefix')
    with tempfile.TemporaryDirectory(prefix='hexagon-cdsp-register-') as temp:
        asm=Path(temp)/'commit.s';obj=Path(temp)/'commit.o';asm.write_text(semantic_asm())
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(asm)],check=True)
        modulepath=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('hexagon_elf_mask_verifier',modulepath)
        assert spec is not None and spec.loader is not None
        mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
        independent=mod.parse_elf32_text(obj.read_bytes())
    if independent!=code:
        ix=next(i for i,(x,y) in enumerate(zip(independent,code)) if x!=y)
        raise ValueError(f'LLVM6 differs at file {FILE_OFFSET+ix:#x}')
    p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',code.hex()],
                     capture_output=True,text=True,check=True)
    lines=[l for l in p.stdout.splitlines() if l.startswith('0x')]
    if len(lines)!=30 or any('invalid' in l.lower() for l in lines):
        raise ValueError('Rizin must decode 30 valid Hexagon instruction words')
    decomp='\n'.join(lines).lower().replace(' ','')
    for term in ['r1=r14','r1=r15','r5.h=#0x0','r5.l=#0x1','r5.l=#0x7',
                 'r5=sub(##-0x1,r5)','r4=and(r4,r5)','barrier']:
        if term not in decomp:raise ValueError('Decoder missing semantic instruction '+term)
    if reference and reference.read_bytes()[FILE_OFFSET:FILE_OFFSET+LENGTH]!=code:
        raise ValueError('Source-generated mask code differs from original')
    print('PASS: 120 exact CDSP MMIO commit/mask bytes, 30 independently LLVM6-assembled and Rizin V66-decoded words')

if __name__=='__main__':
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--llvm-mc',type=Path,required=True)
    ap.add_argument('--rz-asm',type=Path,required=True);ap.add_argument('--generated',type=Path,required=True)
    ap.add_argument('--reference',type=Path);a=ap.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference)
