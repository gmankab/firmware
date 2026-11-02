#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build bounded V60 vector stores and their compact two-register return.

The source constructs the original add-duplex from field-encoded 16-bit
ops. LLVM6 schedules one predicate/new branch ahead of its producers;
correct the order only after exact operand checks, then clear the branch's
end-of-packet marker because the source-encoded duplex terminates it.
"""
from __future__ import annotations
import argparse,hashlib,runpy,struct,subprocess,tempfile
from pathlib import Path
START=0x3a8ec
END=0x3aa1c
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
ENTRY=(0x5c00487a,0xf5014006,0x7062c008)
RETURN_MARKER=0x529fc000
DUAL_ADD=0x38603861

def normalize(data:bytes)->bytes:
    if len(data)!=END-START:raise ValueError('Wrong original vector-store length')
    b=bytearray(data)
    before=struct.unpack_from('<III',b,4)
    if before!=ENTRY:raise ValueError(f'Unexpected Hexagon V60 entry packet: {before!r}')
    if before[0]&0xc000!=0x4000 or before[1]&0xc000!=0x4000 or before[2]&0xc000!=0xc000:
        raise ValueError('Invalid source-generated entry parse markers')
    struct.pack_into('<III',b,4,before[1],before[2]&~0x8000,before[0]|0x8000)
    # The source constructs the compact instruction using two 0x3800
    # 16-bit ADD encoding expressions. Only its packet predecessor needs
    # the marker corrected; the compact operand fields are not modified.
    final=struct.unpack_from('<II',b,END-START-8)
    if final!=(RETURN_MARKER,DUAL_ADD):
        raise ValueError('Unexpected compact return / dual-register ADD operands')
    struct.pack_into('<I',b,END-START-8,RETURN_MARKER&~0x8000)
    return bytes(b)

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-vector-store-') as td:
        obj=Path(td)/'store.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('LLVM6 refused bounded vector-store source: '+p.stderr)
        extract=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        return normalize(extract(obj.read_bytes()))

def verify(ref:bytes,b:bytes)->None:
    sha=hashlib.sha256(ref).hexdigest()
    if sha!=PINNED_SHA256:raise ValueError('Wrong pinned modem revision: '+sha)
    if len(b)!=END-START or b!=ref[START:END]:raise ValueError('Vector store output differs from pinned modem')

def main()->int:
    repo=Path(__file__).resolve().parents[1]
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    p.add_argument('--source',type=Path,default=repo/'sources/xiaomi-nabu/modem/modem_rwe_controller_vector_store.s')
    p.add_argument('--output',required=True,type=Path)
    args=p.parse_args()
    try:
        data=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),data)
        args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_bytes(data)
        print(f'VECTOR STORE [0x{START:x},0x{END:x}) {len(data)} bytes SHA256 {hashlib.sha256(data).hexdigest()}')
        if args.reference:print('PINNED MODEM: ZERO BYTE DIFFERENCES')
        return 0
    except (OSError,ValueError) as exc:p.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
