#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-rebuild ADSP V66 controller record snapshot [0x13390, 0x13478).

The sole privileged instruction is R2=SGP0, encoded by a checked V66 field
constructor at an LLVM6 NOP sentinel. One packet marker differs between the
LLVM6 scheduler and the OEM's valid arrangement, so its parse bits are
adjusted with a bounded ISA operation. No OEM instruction word is embedded.
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
from adsp_hexagon_v66 import SystemRegister, packet_continuation, read_system_register

START = 0x13390
END = 0x13478
SIZE = END - START
PHYSICAL = 0x8be10390
SGP0_FILE_OFFSET = 0x1339c
PACKET_CONTINUATION_AT = 0x133dc
SOURCE = T / 'rwe-next-controller-record-snapshot.S'
REFERENCE_SHA = '4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP = struct.pack('<I', 0x7f00c000)


def render(llvm_mc: str) -> bytes:
    spec = importlib.util.spec_from_file_location('hexagon_emitter', T/'emit-adsp-entry.py')
    if not spec or not spec.loader:
        raise RuntimeError('Missing independent LLVM6 source assembler')
    emitter = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(emitter)
    out = bytearray(emitter.assemble(llvm_mc, SOURCE, SIZE))
    at = SGP0_FILE_OFFSET - START
    if out[at:at + 4] != NOP:
        raise emitter.AdspError('Expected one-word NOP for checked V66 SGP0 read')
    struct.pack_into('<I', out, at, read_system_register(2, SystemRegister.SGP0))
    at = PACKET_CONTINUATION_AT - START
    asm_word = struct.unpack_from('<I', out, at)[0]
    merged_word = packet_continuation(asm_word)
    if (asm_word & ~0xc000) != (merged_word & ~0xc000):
        raise emitter.AdspError('Packet parse adjustment modified an operand')
    struct.pack_into('<I', out, at, merged_word)
    if any(out[i:i+4] == NOP for i in range(0, SIZE, 4)):
        raise emitter.AdspError('Unresolved privileged placeholder')
    return bytes(out)


def verify(reference: bytes, generated: bytes) -> None:
    if hashlib.sha256(reference).hexdigest() != REFERENCE_SHA:
        raise ValueError('Unexpected pinned ADSP firmware reference revision')
    if len(generated) != SIZE or generated != reference[START:END]:
        raise ValueError('ADSP controller record snapshot differs from reference')


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc', required=True)
    p.add_argument('--reference', type=Path)
    p.add_argument('--output', type=Path, required=True)
    args = p.parse_args()
    code = render(args.llvm_mc)
    if args.reference:
        verify(args.reference.read_bytes(), code)
    args.output.write_bytes(code)
    print(f'ADSP controller record snapshot: {SIZE} source-generated executable bytes '
          f'file [0x{START:x},0x{END:x}), physical [0x{PHYSICAL:x},'
          f'0x{PHYSICAL+SIZE:x}), 57 LLVM6 + 1 typed V66 word, '
          f'one checked packet-parse adjustment; sha256={hashlib.sha256(code).hexdigest()}')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
