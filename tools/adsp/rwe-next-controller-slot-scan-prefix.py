#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild the first 96 bytes of ADSP controller-slot table scanning.

LLVM6 emits the decoded instructions but cannot preserve the old packetizer's
chosen triple of standalone p0 compare, r0 assignment and two-op duplex.
Three distinct source-assembled packets are joined using precisely the two
checked architectural Hexagon packet-continuation fields. No OEM instruction
bits are read during emission; reference bytes are used for verification only.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import struct
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
if str(T) not in sys.path:
    sys.path.insert(0, str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import packet_continuation

START = 0x121c0
END = 0x12220
SIZE = END - START
PHYSICAL = 0x8be0f1c0
SOURCE = T / 'rwe-next-controller-slot-scan-prefix.S'
REFERENCE_SHA = '4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
# LLVM6 generates the original operations with packet-end markers; their
# tested, source-assembled operand bits are unchanged by this transformation.
MERGED_PACKET_WORDS = (0x121cc, 0x121d0)


def render(llvm_mc: str) -> bytes:
    spec = importlib.util.spec_from_file_location('adsp_emit', T / 'emit-adsp-entry.py')
    if not spec or not spec.loader:
        raise RuntimeError('Missing standalone LLVM6 source assembler')
    emitter = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(emitter)
    data = bytearray(emitter.assemble(llvm_mc, SOURCE, SIZE))
    for address in MERGED_PACKET_WORDS:
        at = address - START
        original = struct.unpack_from('<I', data, at)[0]
        encoded = packet_continuation(original)
        if encoded & ~0xc000 != original & ~0xc000:
            raise emitter.AdspError('Packet adjustment changed an instruction operand')
        struct.pack_into('<I', data, at, encoded)
    return bytes(data)


def verify(reference: bytes, built: bytes) -> None:
    if hashlib.sha256(reference).hexdigest() != REFERENCE_SHA:
        raise ValueError('Unexpected pinned ADSP reference revision')
    if len(built) != SIZE or built != reference[START:END]:
        raise ValueError('Controller-slot scanner prefix differs from original reference')
    if reference[START - 4:START] != bytes(4):
        raise ValueError('Expected four-byte linker alignment before scanner')


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--llvm-mc', required=True)
    parser.add_argument('--reference', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    data = render(args.llvm_mc)
    if args.reference:
        verify(args.reference.read_bytes(), data)
    args.output.write_bytes(data)
    print(f'ADSP scanner prefix: {SIZE} exact executable bytes, '
          f'file [0x{START:x},0x{END:x}) / physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}), '
          f'sha256={hashlib.sha256(data).hexdigest()}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
