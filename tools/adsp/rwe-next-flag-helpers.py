#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Three Hexagon ADSP source-built controller flag helpers, disjoint intervals.

The generator writes three independent files and never fills the intervening
alignment, diagnostics or other OEM data. All 20 instruction words are built
from original authored assembly source with Hexagon LLVM6, no V66 fallbacks.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,json,sys
from pathlib import Path
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
PARTS={
 'dispatch-stub':(0x113f0,0x113f8,ASSEMBLY_DIR / 'rwe-next-dispatch-stub.S'),
 'flag-window':(0x11400,0x11428,ASSEMBLY_DIR / 'rwe-next-flag-window.S'),
 'flag-sync':(0x11430,0x11450,ASSEMBLY_DIR / 'rwe-next-flag-sync.S'),
}
REQUIRED_ZERO_FILL=((0x113f8,0x11400),(0x11428,0x11430))
DATA_START=0x11394
DATA_END=0x113f0
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def base():
    spec=importlib.util.spec_from_file_location('adsp_core',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('No source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def render(llvm_mc:str)->dict[str,bytes]:
    m=base()
    return {name:m.assemble(llvm_mc,asm,end-start)
        for name,(start,end,asm) in PARTS.items()}

def verify(original:bytes,generated:dict[str,bytes])->None:
    if hashlib.sha256(original).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong pinned ADSP firmware SHA-256')
    if set(generated)!=set(PARTS):
        raise ValueError('Missing one of the source-generated ADSP flag functions')
    for name,(start,end,_) in PARTS.items():
        code=generated[name]
        if len(code)!=end-start or code!=original[start:end]:
            raise ValueError(f'ADSP {name} differs from firmware at [0x{start:x},0x{end:x})')
    for a,b in REQUIRED_ZERO_FILL:
        if original[a:b]!=bytes(b-a):
            raise ValueError('Expected ADSP linker fill is nonzero')
    if not original[DATA_START:DATA_END].startswith(b'trampoline index out of range!'):
        raise ValueError('Diagnostic string pool missing before flag helper')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output-dir',required=True,type=Path)
    a=p.parse_args();pieces=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),pieces)
    a.output_dir.mkdir(parents=True,exist_ok=True)
    manifest=[]
    for name,(start,end,_) in PARTS.items():
        piece=pieces[name]
        path=a.output_dir/(f'adsp-{name}-{len(piece)}.bin')
        path.write_bytes(piece)
        manifest.append({'name':name,'file_start':hex(start),'file_end':hex(end),
                         'physical_start':hex(0x8be03000+start-0x6000),
                         'source_code_bytes':len(piece),'sha256':hashlib.sha256(piece).hexdigest()})
    (a.output_dir/'source-coverage.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(f'ADSP flag helpers: {sum(len(x) for x in pieces.values())} source-built '
          f'bytes in {len(pieces)} disjoint intervals, no copied fill')
    return 0
if __name__=='__main__':sys.exit(main())
