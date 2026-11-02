#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent checked construction of the 24-entry controller allocation table.

The four base groups and final singletons were decoded from the pinned
firmware. These are observed metadata constants, not independently discovered
vendor source symbols. The repeated entries are constructed by strides, not
copied from a raw OEM byte slice. This does not recover irregular calibration.
"""
import hashlib
import struct

FILE_OFFSET=0x185c0
FILE_END=0x18620
WORD_COUNT=24
EXPECTED_SHA256='74f24e4c3557fc128a16f3e76414fd1856327262f3dd49e937c54edf6a755637'

def addresses() -> list[int]:
    pair_origins=[0x39284,0x2affa,0x38f64,0x2acda]
    quad_origins=[0x33310,0x1b290]
    plane_origin=0x1e4a0
    specials=[0x373e8,0x274f8,0x30fa0,0x38068]
    words=[origin+i*0x190 for origin in pair_origins for i in range(2)]
    words += [origin+i*0xc80 for origin in quad_origins for i in range(4)]
    words += [plane_origin+i*0x640 for i in range(4)]
    words += specials
    assert len(words)==WORD_COUNT and all(0<=x<0x40000 and x%2==0 for x in words)
    return words

def serialize() -> bytes:
    result=struct.pack('<24I',*addresses())
    assert len(result)==FILE_END-FILE_OFFSET
    return result

if __name__=='__main__':
    d=serialize()
    print(f'file {FILE_OFFSET:#x}-{FILE_END:#x}: {len(d)} reference-decoded metadata bytes, SHA256={hashlib.sha256(d).hexdigest()}')
