#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble the disjoint modem controller-mode release with strict reference check.

All emitted instruction bytes are produced from LLVM6 Hexagon source. Reference
firmware may only be read for optional pinned SHA-256 and exact byte checking.
"""
from __future__ import annotations
import argparse, hashlib, runpy, subprocess, tempfile
from pathlib import Path

FILE_START=0x3a3d8
FILE_END=0x3a404
VIRTUAL_START=0xc08423d8
PHYSICAL_START=0x8e0423d8
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'

def assemble(llvm_mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-mode-release-') as tmp:
        output=Path(tmp)/'mode-release.o'
        proc=subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
                             '-filetype=obj',str(source),'-o',str(output)],capture_output=True,text=True)
        if proc.returncode:raise ValueError('LLVM6 Hexagon assembly: '+proc.stderr)
        helper=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))
        generated=helper['text_from_object'](output.read_bytes())
        if len(generated)!=FILE_END-FILE_START:raise ValueError('Incorrect typed mode-release byte count')
        return generated

def verify(original:bytes,generated:bytes)->None:
    sha=hashlib.sha256(original).hexdigest()
    if sha!=PINNED_SHA256:raise ValueError('Wrong modem source revision: '+sha)
    ref=original[FILE_START:FILE_END]
    if len(generated)!=FILE_END-FILE_START or len(ref)!=len(generated):
        raise ValueError('Mode-release fragment has invalid size')
    if ref!=generated:
        i=next(i for i,(a,b) in enumerate(zip(ref,generated)) if a!=b)
        raise ValueError(f'First mismatch at modem file 0x{FILE_START+i:x}')

def main()->int:
    root=Path(__file__).resolve().parents[1]
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--llvm-mc',required=True,type=Path)
    parser.add_argument('--source',type=Path,default=root/'sources/xiaomi-nabu/modem/modem_rwe_early_mode_release.s')
    parser.add_argument('--output',required=True,type=Path)
    parser.add_argument('--reference',type=Path)
    args=parser.parse_args()
    try:
        b=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),b)
        args.output.write_bytes(b)
    except (OSError,ValueError) as err:
        parser.exit(1,f'ERROR: {err}\n')
    print(f'Modem RWE mode release [0x{FILE_START:x},0x{FILE_END:x}): {len(b)} bytes, sha256 {hashlib.sha256(b).hexdigest()}')
    if args.reference:print('Pinned modem reference matches exactly')
    return 0
if __name__=='__main__':raise SystemExit(main())
