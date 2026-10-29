#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble checked Hexagon V60 controller mode entry, optionally verify OEM revision.

The source contains checked privileged ISA field encoders and ordinary LLVM6
Hexagon assembly. The verifier does NOT read reference bytes for generation.
"""
from __future__ import annotations
import argparse, hashlib, runpy, struct, subprocess, tempfile
from pathlib import Path

FILE_START=0x3a4c4
FILE_END=0x3a50c
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-modem-mode-entry-') as td:
        obj=Path(td)/'mode-entry.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('Hexagon LLVM6 rejected source: '+p.stderr)
        emitted=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object'](obj.read_bytes())
    if len(emitted)!=FILE_END-FILE_START:raise ValueError('wrong source-built interval length')
    words=struct.unpack('<'+'I'*(len(emitted)//4),emitted)
    # Enforce the sensitive privileged-register fields, and that two compound
    # four-word packets are bounded by their exact architected parse bits.
    for pos,expected in {
        0:(0x6c200000,0x40),1:(0x6c200000,0x60),2:(0x6e800000,0x10),
        6:(0x6e800000,0x3),10:(0x67000000,0x6),14:(0x6c200000,0x80),
        15:(0x67000000,0x3),16:(0x67000000,0x4),
    }.items():
        base,field=expected;word=words[pos]
        if pos in (0,1,14):correct=(word&~0xc000)==(base|field)
        elif pos in (2,6):correct=(word&0xffc03fe0)==base and ((word>>16)&0x3f)==field and (word&0x1f)<=31
        else:correct=(word&0xffc03fe0)==base and (word&0x3f)==field
        if not correct:raise ValueError(f'Invalid privileged instruction at +0x{pos*4:x}: {word:08x}')
    if not (words[2]&0xc000==0x4000 and words[5]&0xc000==0xc000 and
            words[6]&0xc000==0x4000 and words[9]&0xc000==0xc000 and
            words[10]&0xc000==0x4000 and words[13]&0xc000==0xc000):
        raise ValueError('mode-entry compound packet markers differ')
    return emitted

def verify(reference:bytes,built:bytes)->None:
    sha=hashlib.sha256(reference).hexdigest()
    if sha!=PINNED_SHA256:raise ValueError('Wrong modem reference revision: '+sha)
    if len(built)!=FILE_END-FILE_START:raise ValueError('Invalid emitted size')
    original=reference[FILE_START:FILE_END]
    if original!=built:
        diff=next((i for i,(a,b) in enumerate(zip(original,built)) if a!=b),0)
        raise ValueError(f'Byte mismatch at modem file 0x{FILE_START+diff:x}')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    repo=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--source',type=Path,default=repo/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_mode_entry.s')
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',required=True,type=Path)
    args=p.parse_args()
    try:
        b=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),b)
        args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_bytes(b)
        print(f'HEXV60 MODE ENTRY [0x{FILE_START:x},0x{FILE_END:x}) {len(b)} bytes, sha256 {hashlib.sha256(b).hexdigest()}')
        if args.reference:print('PINNED MODEM: ZERO BYTE DIFFERENCES')
        return 0
    except (ValueError,OSError) as exc:p.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
