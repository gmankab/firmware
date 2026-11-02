#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build the bounded Hexagon V60 vector load and byte-loop variants.

LLVM6 schedules three producer/branch packets differently. The normalizer
requires exactly the original source-assembled operand classes/packet markers,
then changes only their order and legal parse bits; no OEM reference bytes are
used for generation. A complete whole-file SHA and byte check is optional.
"""
from __future__ import annotations
import argparse,hashlib,runpy,struct,subprocess,tempfile
from pathlib import Path
START=0x3a778
END=0x3a8ec
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
ENTRY_BRANCH=0x5c00489c
ENTRY_PAIR=0xf5014006
ENTRY_MOV=0x7062c008
SIGNED_BRANCHES={0x3a8a0:0x5cdf60f6,0x3a8d8:0x5cdf60f2}
BYTE_LOOP_ADD=0xb009c109

def normalize(data:bytes)->bytes:
    if len(data)!=END-START:raise ValueError(f'Vector load length should be {END-START}, received {len(data)}')
    out=bytearray(data)
    off=0x3a77c-START
    words=struct.unpack_from('<III',out,off)
    if words!=(ENTRY_BRANCH,ENTRY_PAIR,ENTRY_MOV):
        raise ValueError(f'Unexpected source-assembled vector entry packet {words!r}')
    if words[0]&0xc000!=0x4000 or words[1]&0xc000!=0x4000 or words[2]&0xc000!=0xc000:
        raise ValueError('Vector entry parse markers invalid')
    struct.pack_into('<III',out,off,words[1],words[2]&~0x8000,words[0]|0x8000)
    for addr,expected_branch in SIGNED_BRANCHES.items():
        off=addr-START
        branch,increment=struct.unpack_from('<II',out,off)
        if (branch,increment)!=(expected_branch,BYTE_LOOP_ADD):
            raise ValueError(f'Unexpected source-assembled byte-loop packet 0x{addr:x}')
        if branch&0xc000!=0x4000 or increment&0xc000!=0xc000:
            raise ValueError(f'Unexpected byte-loop markers at 0x{addr:x}')
        struct.pack_into('<II',out,off,increment&~0x8000,branch|0x8000)
    return bytes(out)

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-rwe-vector-load-') as td:
        obj=Path(td)/'vector-load.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('LLVM6 refused vector source: '+p.stderr)
        extract=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        return normalize(extract(obj.read_bytes()))

def verify(ref:bytes,built:bytes)->None:
    digest=hashlib.sha256(ref).hexdigest()
    if digest!=PINNED_SHA256:raise ValueError('Wrong pinned modem revision: '+digest)
    if len(built)!=END-START:raise ValueError('Invalid vector source output length')
    if ref[START:END]!=built:
        index=next(i for i,(x,y) in enumerate(zip(ref[START:END],built)) if x!=y)
        raise ValueError(f'Hexagon vector source mismatch at file 0x{START+index:x}')

def main()->int:
    a=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    a.add_argument('--llvm-mc',type=Path,required=True)
    a.add_argument('--reference',type=Path)
    a.add_argument('--source',type=Path,default=root/'sources/xiaomi-nabu/modem/modem_rwe_controller_vector_load.s')
    a.add_argument('--output',type=Path,required=True)
    args=a.parse_args()
    try:
        b=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),b)
        args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_bytes(b)
        print(f'VECTOR LOAD [0x{START:x},0x{END:x}) {len(b)} source-assembled bytes SHA256 {hashlib.sha256(b).hexdigest()}')
        if args.reference:print('PINNED MODEM: ZERO BYTE DIFFERENCES')
        return 0
    except (OSError,ValueError) as exc:a.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
