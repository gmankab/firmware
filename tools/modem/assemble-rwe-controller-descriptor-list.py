#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V60 doubly-linked descriptor list primitives with checked packet order.

One legal predicate/branch packet is reordered by the historical LLVM6
scheduler. Only three exactly checked ISA words are permuted. The input
firmware is never consulted during generation; optional reference comparison
is pin-checked and separate.
"""
from __future__ import annotations
import argparse,hashlib,struct,runpy,subprocess,tempfile
from pathlib import Path
START=0x3a650
END=0x3a72c
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
BRANCH=0x5c00430c
COMPARE=0xf2064202
PREDICATED_ZERO=0x7e604008
PACKET_POSITION=0x3a6f8-START

def normalize(data:bytes)->bytes:
    if len(data)!=END-START:raise ValueError('Wrong descriptor-list instruction size')
    actual=struct.unpack_from('<III',data,PACKET_POSITION)
    if actual!=(BRANCH,COMPARE,PREDICATED_ZERO):
        raise ValueError('Unexpected predicate/branch packet operands or parse bits')
    for x in actual:
        if x&0xc000!=0x4000:raise ValueError('Packet parser marker unexpectedly changed')
    out=bytearray(data)
    struct.pack_into('<III',out,PACKET_POSITION,COMPARE,PREDICATED_ZERO,BRANCH)
    return bytes(out)

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-controller-list-') as td:
        obj=Path(td)/'descriptor-list.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('LLVM6 assembler rejected source: '+p.stderr)
        helper=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        b=normalize(helper(obj.read_bytes()))
    # Check separate return forms and nonduplicated func starts.
    for off,expected in {0x20:0x529f4000,0x50:0x529f4000,0x80:0x529f4000,0xd4:0x529f4000}.items():
        if struct.unpack_from('<I',b,off)[0]!=expected:
            raise ValueError(f'Unexpected Hexagon jump-return at +0x{off:x}')
    return b

def verify(ref:bytes,b:bytes)->None:
    digest=hashlib.sha256(ref).hexdigest()
    if digest!=PINNED_SHA256:raise ValueError('Wrong modem reference revision: '+digest)
    if len(b)!=END-START or b!=ref[START:END]:raise ValueError('Controller descriptor list differs from pinned reference')

def main()->int:
    a=argparse.ArgumentParser(description=__doc__)
    repo=Path(__file__).resolve().parents[1]
    a.add_argument('--llvm-mc',type=Path,required=True)
    a.add_argument('--reference',type=Path)
    a.add_argument('--source',type=Path,default=repo/'sources/xiaomi-nabu/modem/modem_rwe_controller_descriptor_list.s')
    a.add_argument('--output',type=Path,required=True)
    args=a.parse_args()
    try:
        b=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),b)
        args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_bytes(b)
        print(f'LINKED DESCRIPTORS [0x{START:x},0x{END:x}) {len(b)} bytes SHA256 {hashlib.sha256(b).hexdigest()}')
        if args.reference:print('PINNED MODEM: EXACT')
        return 0
    except (OSError,ValueError) as exc:a.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
