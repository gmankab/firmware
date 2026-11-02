#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent bitfield construction of the 22 upper-owned bytes of a function.

NDS32 wide instructions are serialized big-endian; compressed instructions
are two-byte big-endian. Only two MOVI bytes beginning at file 0x10000 belong
to Agent 18. The other two are Agent 17's separate ownership.
"""
import hashlib
import struct

START,END=0x10000,0x10016
EXPECTED_SHA256='c545577eae40ed57b6a66d428560eff20b9c3d99de081339612747cc93d4f0a4'

def emit() -> bytes:
    movi=(0x22<<25)|(6<<20)|0x20104
    add_shifted=(0x20<<25)|(6<<20)|(6<<15)|(0<<10)|(1<<5)
    signed_halfword_load=(0x11<<25)|(0<<20)|(6<<15)
    disp=0xffaa-0x1000a
    assert disp%2==0
    jal=(0x24<<25)|(1<<24)|((disp//2)&0xffffff)
    compact_halfword_load=0xa400|(0<<6)|(6<<3)|0
    signed_halfword_store_gp=(0x1e<<25)|(4<<17)|((-5494//2)&0x3ffff)
    pop=0xfc80
    return (struct.pack('>I',movi)[2:]+struct.pack('>I',add_shifted)
        +struct.pack('>I',signed_halfword_load)+struct.pack('>I',jal)
        +struct.pack('>H',compact_halfword_load)
        +struct.pack('>I',signed_halfword_store_gp)+struct.pack('>H',pop))

if __name__=='__main__':
    data=emit()
    print(f'cross-owner upper suffix {START:#x}-{END:#x}, {len(data)} bytes, SHA-256 {hashlib.sha256(data).hexdigest()}')
    assert len(data)==22 and hashlib.sha256(data).hexdigest()==EXPECTED_SHA256
