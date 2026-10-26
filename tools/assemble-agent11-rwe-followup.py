#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble and strictly compare Agent 11's next *disjoint* Hexagon helper.

Instruction operands and packet semantics are authored in Hexagon assembly.
The original reference is optional, is hash-pinned, and is never copied into
an emitted byte fragment. The gap before this fragment is explicitly omitted.
"""
import argparse
import hashlib
import runpy
import subprocess
import tempfile
from pathlib import Path

FILE_START=0x39920
FILE_END=0x39a0c
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'


def assemble(llvm_mc:Path, source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='modem-rwe-lock-') as tmp:
        obj=Path(tmp)/'followup.o'
        p=subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:
            raise ValueError('Hexagon V60 assembler failed: '+p.stderr)
        data=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object'](obj.read_bytes())
        if len(data)!=FILE_END-FILE_START:
            raise ValueError(f'Unexpected helper length {len(data)}, expected {FILE_END-FILE_START}')
        return data


def verify(original:bytes,generated:bytes)->None:
    digest=hashlib.sha256(original).hexdigest()
    if digest!=PINNED_SHA256:
        raise ValueError(f'Wrong modem source revision: {digest}')
    data=original[FILE_START:FILE_END]
    if data!=generated:
        at=next((i for i,(a,b) in enumerate(zip(data,generated)) if a!=b),0)
        raise ValueError(f'First mismatch at file 0x{FILE_START+at:x}')


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--source',type=Path,default=root/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_followup.s')
    p.add_argument('--output',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    args=p.parse_args()
    try:
        data=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),data)
        args.output.write_bytes(data)
    except (OSError,ValueError) as exc:
        p.exit(1,f'ERROR: {exc}\n')
    print(f'Modem second exception helper: [file 0x{FILE_START:x},0x{FILE_END:x}), {len(data)} independently assembled bytes, sha256 {hashlib.sha256(data).hexdigest()}')
    if args.reference:print('Pinned reference matches exactly')
    return 0
if __name__=='__main__':raise SystemExit(main())
