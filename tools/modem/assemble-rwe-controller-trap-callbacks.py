#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Reassemble V60 controller traps and callback frame from original Hexagon source.

The three trap/self-loop vectors, callback-register dispatch, paired
stack stores/loads, and return are validated on architectural instruction
classes and declared registers. The OEM reference is an optional *comparison*
input and never supplies any source or output bytes.
"""
from __future__ import annotations
import argparse,hashlib,runpy,struct,subprocess,tempfile
from pathlib import Path
START=0x3aa1c
END=0x3aa8c
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'

def validate(b:bytes)->bytes:
    if len(b)!=END-START:raise ValueError(f'Unexpected trap/callback length {len(b)}')
    words=struct.unpack('<28I',b)
    for k,num in enumerate((10,8,9)):
        trap,jump=words[2*k:2*k+2]
        # trap0(#n), jump to same PC; the immediate alone varies per vector.
        if trap != (0x5400c0e0 + (num << 2)):
            raise ValueError(f'Invalid Hexagon trap0 immediate or packet bits at vector {k}')
        if (jump & ~0x3fff)==0:
            raise ValueError('Invalid self-loop opcode class')
        if jump != 0x5800c000:
            raise ValueError(f'Invalid trap self-loop branch at vector {k}')
    # Validate the architecture's 48-byte frame and callr register fields.
    if words[6]!=0xa09dc006:raise ValueError('Frame size is not 48 bytes')
    if (words[15],words[17]) != (0x50a4c000,0x50a5c000):
        raise ValueError('Expected callr r4 and callr r5 instructions')
    if words[26]!=0x901ec01e or words[27]!=0x529fc000:
        raise ValueError('Unexpected callback epilogue and register return')
    return b

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-trap-callbacks-') as td:
        obj=Path(td)/'callback.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('LLVM6 rejected Hexagon callback source: '+p.stderr)
        extractor=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        return validate(extractor(obj.read_bytes()))

def verify(ref:bytes,b:bytes)->None:
    digest=hashlib.sha256(ref).hexdigest()
    if digest!=PINNED_SHA256:raise ValueError('Wrong pinned modem revision: '+digest)
    validate(b)
    if b!=ref[START:END]:raise ValueError('Trap/callback bytes differ from reference modem')

def main()->int:
    parser=argparse.ArgumentParser(description=__doc__)
    repo=Path(__file__).resolve().parents[1]
    parser.add_argument('--llvm-mc',required=True,type=Path)
    parser.add_argument('--reference',type=Path)
    parser.add_argument('--source',type=Path,default=repo/'sources/xiaomi-nabu/modem/modem_rwe_controller_trap_callbacks.s')
    parser.add_argument('--output',required=True,type=Path)
    args=parser.parse_args()
    try:
        result=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),result)
        args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_bytes(result)
        print(f'TRAP CALLBACKS [0x{START:x},0x{END:x}) {len(result)} bytes SHA256 {hashlib.sha256(result).hexdigest()}')
        if args.reference:print('PINNED MODEM: ZERO BYTE DIFFERENCES')
        return 0
    except (OSError,ValueError) as e:parser.exit(1,f'ERROR: {e}\n')
if __name__=='__main__':raise SystemExit(main())
