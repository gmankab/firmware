#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble modem's separately mapped RWE control descriptor code.

This span is not contiguous with the earlier PT_LOAD and the ROM-supplied
reference never enters source. A decoded match does not imply modem execution
or an OEM-accepted signature.
"""
from __future__ import annotations
import argparse
import hashlib
import runpy
from pathlib import Path
import subprocess
import tempfile
import sys

FILE_START=0xcb00
FILE_END=0xcb64
ELF_VADDR=0xc0814b00
PHYSICAL_START=0x8e014b00
PINNED_REFERENCE_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'


def assemble(llvm_mc:Path, source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='modem-rwe-') as tmp:
        obj=Path(tmp)/'rwe.o'
        p=subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:
            raise ValueError(f'Hexagon LLVM6 assembler failed: {p.stderr}')
        # Existing ELF32-aware standalone parser, no reference firmware read.
        text_from_object=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        blob=text_from_object(obj.read_bytes())
        if len(blob)!=FILE_END-FILE_START:
            raise ValueError(f'bad independent RWE span: {len(blob)} bytes expected {FILE_END-FILE_START}')
        # Check three distinct packet-start/end boundaries and the validated
        # architectural pair register. Do not overwrite any assembler bytes.
        words=[int.from_bytes(blob[k:k+4],'little') for k in range(0,len(blob),4)]
        if words[0] != (0x6e800000|(8<<16)|(1<<14)|12):
            raise ValueError('Hexagon HTID/S8 source operand mismatch')
        expected_gp_ext=(0x0c000000|(((0xc0815840>>20)&0x3ff)<<16)|
                         ((0xc0815840>>6)&0x3fff)|(1<<14))
        if words[1] != expected_gp_ext:
            raise ValueError('Hexagon extended GP-relative address was not preserved')
        if words[2]!=(0x49c00000|(3<<14)|14):
            raise ValueError('Hexagon long GP-relative paired load was shortened')
        if words[12]!=(0x6f000000|(56<<16)|(1<<14)|14):
            raise ValueError('Hexagon saved S57:56 source pairing mismatch')
        return blob


def verify_pinned(reference:bytes, produced:bytes)->None:
    found=hashlib.sha256(reference).hexdigest()
    if found!=PINNED_REFERENCE_SHA256:
        raise ValueError(f'wrong modem.mbn reference revision: {found}')
    slice=reference[FILE_START:FILE_START+len(produced)]
    if len(slice)!=len(produced):raise ValueError('original modem.mbn truncated')
    if slice!=produced:
        bad=next(i for i,(x,y) in enumerate(zip(slice,produced)) if x!=y)
        raise ValueError(f'first RWE mismatch file offset 0x{FILE_START+bad:x}: reference 0x{slice[bad]:02x}, generated 0x{produced[bad]:02x}')


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    r=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--source',type=Path,default=r/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_early_controller.s')
    p.add_argument('--output',required=True,type=Path)
    p.add_argument('--reference',type=Path,help='private pinned original modem.mbn')
    a=p.parse_args()
    try:
        produced=assemble(a.llvm_mc,a.source)
        if a.reference:verify_pinned(a.reference.read_bytes(),produced)
        a.output.write_bytes(produced)
        print(f'Hexagon V60 RWE file [0x{FILE_START:x},0x{FILE_END:x}) source-generated {len(produced)} bytes; '+
              f'VMA=0x{ELF_VADDR:x}; physical=0x{PHYSICAL_START:x}; sha256={hashlib.sha256(produced).hexdigest()}')
        if a.reference:print('Exact pinned modem.mbn span, 0 byte differences')
        print('Disjoint from earlier PT_LOAD; no OEM-authenticated or flashable modem image generated')
        return 0
    except (ValueError,OSError) as exc:
        print('ERROR',exc,file=sys.stderr)
        return 1
if __name__=='__main__':sys.exit(main())
