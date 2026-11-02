#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Complete original Hexagon ADSP controller-global startup function.

Source-builds all 584 bytes [ADSP file 0x11450,0x11698) using five
contiguous independently authored assembly/typed-code sections. Input OEM
firmware is read only for strict revision and byte-equality comparisons;
there is no original binary payload splicing, padding or authentication.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
TOOLS=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
START=0x11450
END=0x11698
SIZE=END-START
PHYSICAL_START=0x8be0e450
FILL_START=END
FILL_END=0x116a0
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
PARTS=(
    ('startup',0x11450,0x114a8,'rwe-next-controller-global-startup.py'),
    ('memory',0x114a8,0x11548,'rwe-next-controller-global-memory.py'),
    ('layout',0x11548,0x11624,'rwe-next-controller-global-layout.py'),
    ('handoff',0x11624,0x1167c,'rwe-next-controller-global-handoff.py'),
)
LAST_SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-global-complete.S'

def load_source(name:str):
    path=TOOLS/name
    spec=importlib.util.spec_from_file_location('adsp_fragment_'+name.replace('.','_').replace('-','_'),path)
    if not spec or not spec.loader:raise RuntimeError(f'Missing source emitter {path}')
    obj=importlib.util.module_from_spec(spec);spec.loader.exec_module(obj);return obj

def render(llvm_mc:str)->bytes:
    out=bytearray()
    for name,begin,end,filename in PARTS:
        if begin!=START+len(out):raise ValueError(f'Nonadjacent {name} starts at 0x{begin:x}')
        piece=load_source(filename).render(llvm_mc)
        if len(piece)!=end-begin:raise ValueError(f'{name}: wrong source length')
        out+=piece
    if len(out)!=0x1167c-START:raise ValueError('Missing controller handoff')
    piece=load_source('emit-adsp-entry.py').assemble(llvm_mc,LAST_SOURCE,END-0x1167c)
    out+=piece
    if len(out)!=SIZE:raise ValueError('Wrong complete function size')
    return bytes(out)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP reference SHA-256')
    if len(built)!=SIZE or built!=reference[START:END]:
        raise ValueError('Global ADSP controller function differs from reference')
    if reference[FILL_START:FILL_END]!=bytes(FILL_END-FILL_START):
        raise ValueError('Expected 8-byte linker fill is nonzero')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();data=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),data)
    args.output.write_bytes(data)
    print(f'ADSP controller-global complete: {len(data)} source-built bytes '
          f'[file 0x{START:x},0x{END:x}) [physical 0x{PHYSICAL_START:x},'
          f'0x{PHYSICAL_START+SIZE:x}) sha256={hashlib.sha256(data).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
