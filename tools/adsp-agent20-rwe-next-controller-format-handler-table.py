#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Recreate the 37-entry ADSP codec-format *data* dispatch table.

Each entry is a virtual function pointer to a known selector handler in the
RWE load. It is not Hexagon code and contributes ZERO executable bytes.
No original OEM machine-code or payload slices are copied to form output.
"""
from __future__ import annotations
import argparse,hashlib,struct
from pathlib import Path
START=0x129bc
END=0x12a50
SIZE=END-START
RWE_FILE_OFFSET=0x6000
RWE_VADDR=0xf0003000
RWE_PADDR=0x8be03000
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
# These are file offsets to source-authored controller codec-format case blocks,
# not vendor encoding words. Repeated destinations are intentional: unsupported
# format IDs fall back to the same error branch.
FORMATS_TO_HANDLER=(
    0x126dc,0x126ec,0x126fc,0x12734,0x12744,0x12828,
    0x12774,0x12784,0x12828,0x127c8,0x12828,0x12828,
    0x12828,0x12810,0x12764,0x1289c,0x128ac,0x128bc,
    0x128cc,0x128dc,0x128ec,0x128fc,0x1290c,0x1291c,
    0x1292c,0x1293c,0x1294c,0x1295c,0x1296c,0x1297c,
    0x1297c,0x1297c,0x1298c,0x1299c,0x1297c,0x1297c,0x129ac,
)

def virtual_handler(file_offset:int)->int:
    if (not isinstance(file_offset,int) or not 0x126a8<=file_offset<START
            or file_offset%4):
        raise ValueError(f'Invalid codec handler address {file_offset!r}')
    return RWE_VADDR+(file_offset-RWE_FILE_OFFSET)

def render()->bytes:
    if len(FORMATS_TO_HANDLER)!=37:
        raise ValueError('Expected exactly 37 codec-format handlers')
    return struct.pack('<37I',*(virtual_handler(a) for a in FORMATS_TO_HANDLER))

def verify(reference:bytes,table:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong pinned ADSP firmware revision')
    if len(table)!=SIZE or table!=reference[START:END]:
        raise ValueError('ADSP codec handler-table data differ from original')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();b=render()
    if args.reference:verify(args.reference.read_bytes(),b)
    args.output.write_bytes(b)
    print(f'ADSP non-executable codec handler table: {SIZE} DATA bytes, '
          f'{len(FORMATS_TO_HANDLER)} runtime virtual pointers; '
          f'file [{START:#x},{END:#x}), SHA256={hashlib.sha256(b).hexdigest()}. '
          'Excluded from executable code totals.')
    return 0
if __name__=='__main__':raise SystemExit(main())
