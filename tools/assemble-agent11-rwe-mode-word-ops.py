#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble controller mode-word/bitfield routine and verify reference identity.

One legitimate 3-word Hexagon predicate/branch packet is sorted differently by
LLVM6. The normalizer swaps only two exact checked source-assembled words,
without changing their opcodes, target displacement or marker bits.
"""
from __future__ import annotations
import argparse,hashlib,runpy,struct,subprocess,tempfile
from pathlib import Path
FILE_START=0x3a47c
FILE_END=0x3a4c4
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
LLVM_BRANCH=0x5c00481a
LLVM_PREDICATE=0x75637ff0

def normalize_llvm6(data:bytes)->bytes:
    if len(data)!=FILE_END-FILE_START:raise ValueError('Invalid mode-bitfield instruction count')
    out=bytearray(data)
    branch,predicate=struct.unpack_from('<II',out,0x14)
    if (branch,predicate)!=(LLVM_BRANCH,LLVM_PREDICATE):
        raise ValueError(f'Unexpected source-assembled Hexagon predicate/branch: {branch:08x} {predicate:08x}')
    if (branch & 0xc000)!=0x4000 or (predicate & 0xc000)!=0x4000:
        raise ValueError('Unexpected LLVM6 packet parse markers')
    struct.pack_into('<II',out,0x14,predicate,branch)
    return bytes(out)

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-v60-mode-bitfields-') as td:
        obj=Path(td)/'source.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('Hexagon LLVM6 failed: '+p.stderr)
        get=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        data=normalize_llvm6(get(obj.read_bytes()))
        architecture_words={0:0x6c20c020,4:0x8f21470f,8:0x80004c22,12:0x706fc00c,
            16:0x6c82c003,20:0x75637ff0,24:0x5c00481a,28:0x7060c004,
            32:0x6c43c000,36:0x8c404085,40:0x80004242,44:0x7c0ec186,
            48:0x838058e2,52:0xc405c727,56:0xca04c602,60:0xc382c642,
            64:0x6c20c040,68:0x57e0c000}
        for offset,word in architecture_words.items():
            if struct.unpack_from('<I',data,offset)[0]!=word:
                raise ValueError(f'Original V60 mode-bitfield operand/marker mismatch at file 0x{FILE_START+offset:x}')
        return data

def verify(ref:bytes,built:bytes)->None:
    digest=hashlib.sha256(ref).hexdigest()
    if digest!=PINNED_SHA256:raise ValueError('Wrong modem reference revision: '+digest)
    if len(built)!=FILE_END-FILE_START:raise ValueError('Incorrect source-built length')
    original=ref[FILE_START:FILE_END]
    if original!=built:
        i=next(i for i,(x,y) in enumerate(zip(original,built)) if x!=y)
        raise ValueError(f'Mismatch at file 0x{FILE_START+i:x}')

def main():
    p=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    p.add_argument('--source',type=Path,default=root/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_mode_word_ops.s')
    p.add_argument('--output',required=True,type=Path)
    args=p.parse_args()
    try:
        b=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),b)
        args.output.parent.mkdir(parents=True,exist_ok=True)
        args.output.write_bytes(b)
        print(f'MODE_WORD_OPERATIONS [0x{FILE_START:x},0x{FILE_END:x}) {len(b)} bytes SHA256 {hashlib.sha256(b).hexdigest()}')
        if args.reference:print('PINNED MODEM: EXACT')
        return 0
    except (ValueError,OSError) as e:p.exit(1,f'ERROR: {e}\n')
if __name__=='__main__':raise SystemExit(main())
