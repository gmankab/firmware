#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild the complete 620-byte ADSP controller slot bitmap scanner.

The function starts immediately after four linker-fill bytes and ends with
its original dealloc_return. All 155 Hexagon V66 instruction words are built
from independently authored LLVM6 mnemonic packets. Only the *first two*
parse fields require a checked architectural continuation transformation.
The OEM ADSP reference is read only during optional verification.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))

START=0x121c0
END=0x1242c
SIZE=END-START
PHYSICAL=0x8be0f1c0
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
PARTS=(
    ('adsp-agent20-rwe-next-controller-slot-scan-prefix.py',0x60),
    ('adsp-agent20-rwe-next-controller-slot-scan-operations.py',0xf0),
    ('adsp-agent20-rwe-next-controller-slot-scan-bitmap.S',0x90),
    ('adsp-agent20-rwe-next-controller-slot-scan-classes.S',0x50),
    ('adsp-agent20-rwe-next-controller-slot-scan-epilogue.S',0x3c),
)

def _import_script(name:str):
    spec=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    if not spec or not spec.loader:raise RuntimeError('Missing source assembler')
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod

def render(llvm_mc:str)->bytes:
    from_source=_import_script('emit-adsp-entry.py')
    fragments=[]
    for name,length in PARTS:
        if name.endswith('.py'):
            body=_import_script(name).render(llvm_mc)
        else:
            body=from_source.assemble(llvm_mc,T/name,length)
        if len(body)!=length:raise ValueError('Scanner component has incorrect output size')
        fragments.append(body)
    result=b''.join(fragments)
    if len(result)!=SIZE:raise ValueError('Scanner function has unexpected boundaries')
    return result

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware revision')
    if len(built)!=SIZE or built!=reference[START:END]:
        raise ValueError('Complete controller slot scan differs from ADSP reference')
    if reference[START-4:START]!=bytes(4):
        raise ValueError('Expected linker fill before scanner function')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'Complete controller slot scanner: {SIZE} source-built bytes '
          f'file [{START:#x},{END:#x}), physical [{PHYSICAL:#x},{PHYSICAL+SIZE:#x}), '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0

if __name__=='__main__':raise SystemExit(main())
