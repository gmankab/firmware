#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Novatek DLM narrow-bank relocation validation.

The lower bank is unknown externally supplied firmware data; this checks a
conditional copy/relocation, not newly authored firmware source bytes.
"""
from __future__ import annotations
import argparse, hashlib, json
from pathlib import Path

PINNED = '8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5'
BANK = 0x4000
DIRECTORY = (0x25220, 16)
NARROW_WINDOW = (0x25460, 0x2574c - 0x25460)
RECORD_START = 0x254a0
STRIDE = 0x28
COUNT = 18
EXEMPT = (8, 9)


def reconstruct_lower_span(start: int, source: bytes) -> bytes:
    if (start,len(source)) not in (DIRECTORY, NARROW_WINDOW):
        raise ValueError('narrow bank range or length mismatch')
    result = bytearray(source)
    if start == DIRECTORY[0]:
        for offset in range(0,16,4):
            entry=int.from_bytes(result[offset:offset+4],'little')
            index=entry & 0x7fff
            if index+0x1000>0x7fff:
                raise ValueError('15-bit directory index relocation overflow')
            result[offset:offset+4]=((entry & ~0x7fff) | (index+0x1000)).to_bytes(4,'little')
    else:
        for record in range(COUNT):
            offset=RECORD_START-start+record*STRIDE
            if offset+4>len(result):raise ValueError('record exceeds narrow span')
            if record in EXEMPT:continue
            for at in (offset, offset+2):
                value=int.from_bytes(result[at:at+2], 'little')
                if value+0x10>0xffff:raise ValueError('record index relocation overflow')
                result[at:at+2]=(value+0x10).to_bytes(2,'little')
    return bytes(result)


def verify(image: bytes) -> dict:
    if len(image)!=196608 or hashlib.sha256(image).hexdigest()!=PINNED:
        raise ValueError('input does not match pinned postmarketOS Novatek firmware')
    checked=0
    for start,length in (DIRECTORY,NARROW_WINDOW):
        rebuilt=reconstruct_lower_span(start,image[start:start+length])
        expected=image[start+BANK:start+BANK+length]
        if rebuilt!=expected:
            at=next(i for i,(x,y) in enumerate(zip(rebuilt,expected)) if x!=y)
            raise ValueError(f'narrow DLM relocation mismatch at file 0x{start+BANK+at:x}')
        checked+=length
    expected_start=tuple(0x25230+index*0x8c for index in range(4))
    actual_start=tuple(0x22000+4*int.from_bytes(image[0x25220+4*j:0x25222+4*j],'little')
                       for j in range(4))
    if actual_start!=expected_start:
        raise ValueError('narrow descriptor directory does not point to four records')
    return dict(schema='agent20-19-dlm-narrow-bank-v1',reference_sha256=PINNED,
                copied_or_relocated_bytes=checked,source_bytes_added=0,
                external_seed_reduction=checked,directory_record_start=expected_start[0],
                index_record_pairs=COUNT-len(EXEMPT),exempt_records=list(EXEMPT),
                mismatched_bytes=0)


def main() -> None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('firmware',type=Path)
    args=p.parse_args()
    print(json.dumps(verify(args.firmware.read_bytes()),indent=2))


if __name__=='__main__':main()
