#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Compile and verify Hexagon V60 mode follow-up, preserving packet order.

Only the two independently compiled compare/branch operand words are
reordered and have parse markers updated, after exact architecture checks.
The OEM firmware is read only by optional revision/byte comparison.
"""
from __future__ import annotations
import argparse,hashlib,struct,subprocess,tempfile,runpy
from pathlib import Path

FILE_START=0x3a444
FILE_END=0x3a47c
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
BRANCH_ASSEMBLED=0x5c004806
COMPARE_ASSEMBLED=0x7562fff0

def normalize_llvm6_packet(data:bytes)->bytes:
    if len(data)!=FILE_END-FILE_START:
        raise ValueError('V60 mode follow-up instruction-length mismatch')
    result=bytearray(data)
    branch,compare=struct.unpack_from('<II',result,0x20)
    if (branch,compare)!=(BRANCH_ASSEMBLED,COMPARE_ASSEMBLED):
        raise ValueError(f'Unexpected LLVM6 V60 predicate/branch operands: {branch:08x}, {compare:08x}')
    if (branch&0xc000)!=0x4000 or (compare&0xc000)!=0xc000:
        raise ValueError('Unexpected original LLVM6 packet markers')
    # Producer comes before consumer; preserve both exact ISA op class and
    # register/branch operands, changing only the legal parse-marker fields.
    struct.pack_into('<II',result,0x20,(compare&~(1<<15)),(branch|(1<<15)))
    return bytes(result)

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-mode-followup-') as tmp:
        obj=Path(tmp)/'code.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60',
                         '-filetype=obj',str(source),'-o',str(obj)],
                         capture_output=True,text=True)
        if p.returncode:raise ValueError('LLVM6 refused mode follow-up: '+p.stderr)
        get=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        generated=normalize_llvm6_packet(get(obj.read_bytes()))
        checks={0x00:0x57e04000,0x04:0x0c1a728f,0x08:0x4980c280,
                0x0c:0x80005442,0x10:0x5c204014,0x14:0x7c00c000,
                0x18:0x6c20c020,0x1c:0x6c83c002,
                0x20:0x75627ff0,0x24:0x5c00c806,
                0x28:0x6c42c000,0x2c:0x6c20c040,
                0x30:0x57e0c000,0x34:0x57e0c000}
        for off,word in checks.items():
            if struct.unpack_from('<I',generated,off)[0]!=word:
                raise ValueError(f'Controller mode opcode/operand mismatch at 0x{FILE_START+off:x}')
        return generated

def verify(ref:bytes,built:bytes)->None:
    digest=hashlib.sha256(ref).hexdigest()
    if digest!=PINNED_SHA256:raise ValueError('Wrong modem source revision: '+digest)
    if len(built)!=FILE_END-FILE_START or ref[FILE_START:FILE_END]!=built:
        if len(built)!=FILE_END-FILE_START:raise ValueError('Bad reconstructed length')
        i=next(i for i,(a,b) in enumerate(zip(built,ref[FILE_START:FILE_END])) if a!=b)
        raise ValueError(f'Modem mode follow-up mismatch at file 0x{FILE_START+i:x}')

def main():
    a=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    a.add_argument('--llvm-mc',required=True,type=Path)
    a.add_argument('--source',type=Path,default=root/'sources/xiaomi-nabu/modem/modem_rwe_mode_followup.s')
    a.add_argument('--output',required=True,type=Path)
    a.add_argument('--reference',type=Path)
    args=a.parse_args()
    try:
        data=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),data)
        args.output.parent.mkdir(parents=True,exist_ok=True)
        args.output.write_bytes(data)
        print(f'REBUILT_MODE_FOLLOWUP [0x{FILE_START:x},0x{FILE_END:x}): {len(data)} bytes sha256 {hashlib.sha256(data).hexdigest()}')
        if args.reference:print('PINNED REFERENCE: 0 byte mismatches')
        return 0
    except (ValueError,OSError) as e:a.exit(1,f'ERROR: {e}\n')
if __name__=='__main__':raise SystemExit(main())
