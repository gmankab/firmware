#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Python construction of Novatek's small structured ILM tables.

All vendor-observed seed masks, origin addresses, high-angle samples and
calibration profile values are explicitly documented metadata, not claimed to
have been invented from firmware source. This generator does not import the
Rust implementation or read reference firmware; optionally compare its result
against a pinned reference using the main verification tool.
"""
import hashlib
import math
import struct

LOOKUPS_START, LOOKUPS_END = 0x196F8, 0x198DC
PROFILES_START, PROFILES_END = 0x19BBC, 0x19BF0
MASK20 = (1 << 20) - 1
DIGESTS = {
    LOOKUPS_START: 'ce5d00f0e32f44773f87376cda87b5bd40c84f6788482d1e41d8e7c28f726b5a',
    PROFILES_START: '3411d81186eb7802dee4c92f041dea90726a8d2e0b6a75612520a5080b6ebe09',
}


def tangent_curve():
    points = []
    for index in range(28):
        points.append(100 if index == 15 else math.floor(100 * math.tan(math.radians(index * 3))))
    points.extend([953, 1913, 5730])  # observed terminal samples, no vendor formula inferred
    assert len(points) == 31 and all(a < b for a, b in zip(points, points[1:]))
    return points


def allocation_addresses():
    result = [origin + i * 0x190 for origin in (0x39284, 0x2affa, 0x38f64, 0x2acda) for i in range(2)]
    result += [origin + i * 0xc80 for origin in (0x33310, 0x1b290) for i in range(4)]
    result += [0x1e4a0 + i * 0x640 for i in range(4)]
    result += [0x373e8, 0x274f8, 0x30fa0, 0x38068]
    assert len(result) == 24 and all(0 <= x < 0x40000 for x in result)
    return result


def mask_sequence():
    seed = [0x06cf5,0x1b3d4,0x21b3d,0x286cf,0x33d43,0x4f50d,0x6a1b3,0x6cf50]
    order = [5,4,1,7,6,3,0,2]
    matrix = [[1,4,6,2,0,3,5,7], [5,7,0,3,6,2,1,4],
              [2,6,4,1,7,5,3,0], [3,0,7,5,4,1,2,6]]
    result = [(seed[index] ^ MASK20) for index in order]
    result += [seed[index] for index in order]
    result += [seed[5], seed[0], seed[6] ^ MASK20, seed[1] ^ MASK20] + [0]*4
    result += [(seed[index] ^ MASK20) for index in [4,7,3,2]] + [0]*4
    result += [seed[index] ^ (MASK20 if column >= 4 else 0)
               for row in matrix for column,index in enumerate(row)]
    result += [seed[index] ^ MASK20 for index in order]
    assert len(result) == 72 and all(0 <= value <= MASK20 for value in result)
    return result


def emit_lookups():
    output = bytearray([9,8,8,9,8,8]); output.extend(struct.pack('<H',1))
    output.extend(struct.pack('<31H',*tangent_curve()))
    output.extend(bytes(x for edge in [(0,0),(0,1),(0,2),(1,3),(2,3)] for x in edge))
    addresses = allocation_addresses()
    selected = addresses[20:24] + addresses[16:20]
    selected += [0x28e20 + i*0x640 for i in range(4)]
    selected += addresses[8:12]
    output.extend(struct.pack('<16I',*selected))
    output.extend(bytes(row ^ column for row in (0,15,7,8) for column in (0,9,11,2)))
    output.extend(bytes(range(1,7))); output.extend((0,7))
    output.extend([8,0,0,0] + [6]*9 + [0]*3 + [4,5,6,7])
    output.extend(struct.pack('<72I',*mask_sequence()))
    output.extend([2,2,2,3,3,3,4,4])
    assert len(output) == LOOKUPS_END - LOOKUPS_START
    return bytes(output)


def emit_profiles():
    observed = [[109,111,112,114,115,117,118],
                [109,103,100,94,85,85,85],
                [0,5,10,19,25,40,60]]
    output = b''.join(struct.pack('<8H',*curve,0) for curve in observed) + bytes(4)
    assert len(output) == PROFILES_END - PROFILES_START
    return output


def independent_fragments():
    result = {LOOKUPS_START: emit_lookups(), PROFILES_START: emit_profiles()}
    for start,data in result.items():
        digest=hashlib.sha256(data).hexdigest()
        if digest != DIGESTS[start]:
            raise ValueError(f'revision-specific auxiliary table digest mismatch at {start:#x}: {digest}')
    return result


if __name__ == '__main__':
    for start,data in independent_fragments().items():
        print(f'0x{start:x}: {len(data)} bytes sha256 {hashlib.sha256(data).hexdigest()}')
