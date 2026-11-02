#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble two V60 callback trampolines preserving source packet semantics.

LLVM6 schedules jump ahead of an extended destination register write,
although the original puts it at the end of the packet. The normalizer
checks each source-emitted architectural instruction, moves the jump to the
last slot, and corrects packet-end flags. No reference bytes are imported.
"""
from __future__ import annotations
import argparse,hashlib,runpy,struct,subprocess,tempfile
from pathlib import Path
START=0x3aa8c
END=0x3aab4
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
# Expected assembly forms derive from Hexagon signed immext, R5 move and
# the branch displacement to the common callback frame at file 0x3aa34.
FIRST=(0x59ff7fd0,0x0c0850a3,0x7800c585)
SECOND=(0x59ff7fc6,0x0c0850a8,0x7800c485)

def normalize(data:bytes)->bytes:
    if len(data)!=END-START:raise ValueError('Incorrect two-trampoline size')
    b=bytearray(data)
    for ix,expected in ((8,FIRST),(28,SECOND)):
        packet=struct.unpack_from('<III',b,ix)
        if packet!=expected:raise ValueError(f'Unexpected jump/extended R5 operands at +{ix:x}: {packet!r}')
        jump,ext,mov=packet
        # The original callback-frame branch finishes the three-word packet.
        # Source immext remains the first packet word; r5 is intermediate.
        struct.pack_into('<III',b,ix,ext,mov&~0x8000,jump|0x8000)
    # Both packed dual-register MOV inputs must remain unchanged.
    if struct.unpack_from('<I',b,4)[0]!=0x2ac43040 or struct.unpack_from('<I',b,24)[0]!=0x2b843040:
        raise ValueError('Unexpected compact dual-register callback setup')
    return bytes(b)

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-controller-callback-') as td:
        obj=Path(td)/'trampolines.o'
        proc=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if proc.returncode:raise ValueError('LLVM6 rejected callback trampolines: '+proc.stderr)
        extract=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        return normalize(extract(obj.read_bytes()))

def verify(ref:bytes,b:bytes)->None:
    got=hashlib.sha256(ref).hexdigest()
    if got!=PINNED_SHA256:raise ValueError('Wrong modem reference revision: '+got)
    if len(b)!=END-START or b!=ref[START:END]:raise ValueError('Callback trampolines differ from pinned modem')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    repo=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--source',type=Path,default=repo/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_callback_trampolines.s')
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args()
    try:
        out=assemble(a.llvm_mc,a.source)
        if a.reference:verify(a.reference.read_bytes(),out)
        a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_bytes(out)
        print(f'CALLBACK TRAMPOLINES [0x{START:x},0x{END:x}) {len(out)} bytes SHA256 {hashlib.sha256(out).hexdigest()}')
        if a.reference:print('PINNED MODEM: ZERO BYTE DIFFERENCES')
        return 0
    except (OSError,ValueError) as exc:p.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
