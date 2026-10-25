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
import struct
import runpy
from pathlib import Path
import subprocess
import tempfile
import sys

FILE_START=0xcb00
FILE_END=0xcc1c
ELF_VADDR=0xc0814b00
PHYSICAL_START=0x8e014b00
PINNED_REFERENCE_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'



def preserve_original_legal_packet_order(generated:bytes)->bytes:
    """Normalize LLVM6's legal packet scheduling to the source packet order.

    LLVM6 moves the .new conditional branch ahead of its producer in two
    2-word packets. The valid target uses compare first, branch last. Move
    *only* LLVM-authored operations after checking classes, operands and
    start/end parse markers; never import bytes from the reference.
    """
    data=bytearray(generated)
    packet_bits=3<<14
    for address,cmp_reg,cmp_prefix,end_marker in (
        (0xcb78,8,0xf2000000,3),
        (0xcb98,9,0x75000000,1),
    ):
        off=address-FILE_START
        branch,comparison=struct.unpack_from('<II',data,off)
        if (branch & 0xff000000)!=0x5c000000 or (branch & packet_bits)!=(1<<14):
            raise ValueError(f'unrecognized LLVM6 modem RWE branch at {address:#x}')
        if ((comparison & 0xff000000)!=cmp_prefix or
            (comparison>>16)&31 != cmp_reg or
            (comparison & packet_bits)!=(end_marker<<14)):
            raise ValueError(f'unrecognized LLVM6 RWE predicate at {address+4:#x}')
        compare=(comparison & ~packet_bits)|(1<<14)
        branch=(branch & ~packet_bits)|(end_marker<<14)
        struct.pack_into('<II',data,off,compare,branch)
    # The compact compare-and-branch at 0xcbc8 is written by the checked
    # macro. The preceding r17 compare must CONTINUE the same 3-word packet,
    # but LLVM6 has no knowledge of the manually encoded compact operation.
    off=0xcbc4-FILE_START
    predicate=struct.unpack_from('<I',data,off)[0]
    if ((predicate&0xff000000)!=0x75000000 or
         ((predicate>>16)&31)!=17 or
         (predicate&packet_bits)!=(3<<14)):
        raise ValueError('unexpected standalone r17 predicate before compact branch')
    struct.pack_into('<I',data,off,(predicate&~packet_bits)|(1<<14))
    # Architecturally: comparison, branch, byte load must have
    # CONTINUE, CONTINUE, END packet flags respectively.
    for addr,wanted in ((0xcb78,1),(0xcb7c,3),(0xcb98,1),(0xcb9c,1),
                        (0xcbc4,1),(0xcbc8,1),(0xcbcc,3)):
        got=struct.unpack_from('<I',data,addr-FILE_START)[0]
        if (got & packet_bits)!=(wanted<<14):
            raise ValueError(f'incorrect packet parse field at {addr:#x}')
    return bytes(data)

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
        return preserve_original_legal_packet_order(blob)


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
