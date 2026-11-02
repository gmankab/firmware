#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-built ADSP V66 first-executable-segment tail; no copied OEM code.

Agent20-07 continues Agent04's contiguous `[0x41e0,0x51dc)` controller with
an independently assembled `[0x51dc,0x521c)` packet.  The OEM firmware is a
read-only optional verification oracle, and signature bytes are not emitted.
"""
from __future__ import annotations
import argparse
import hashlib
import importlib.util
from pathlib import Path
import struct
import sys

ADSP_ASM = Path(__file__).resolve().parents[2] / 'sources/xiaomi-nabu/adsp'
TOOLS = Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import (SystemRegister, read_system_register,
                              tlb_read_pair)

FILE_START = 0x51DC
FILE_END = 0x521C
PHYSICAL_START = 0x8BE021DC
SIZE = FILE_END - FILE_START
SHA256_REFERENCE = '4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE = ADSP_ASM / 'adsp-controller-tlb-snapshot.S'


def load_baseline_emitter():
    p = TOOLS / 'emit-adsp-entry.py'
    spec = importlib.util.spec_from_file_location('adsp_agent04_emitter', p)
    if spec is None or spec.loader is None:
        raise RuntimeError('unable to load checked Hexagon object decoder')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def render(llvm_mc: str) -> bytes:
    baseline = load_baseline_emitter()
    out = bytearray(baseline.assemble(llvm_mc, SOURCE, SIZE))
    sentinel = struct.pack('<I', 0x7F00C000)
    replacements = {
        0x00: read_system_register(23, SystemRegister.S54),
        0x1C: tlb_read_pair(2, 0),
    }
    for at, word in replacements.items():
        if out[at:at+4] != sentinel:
            raise baseline.AdspError(f'ADSP V66 privileged sentinel moved at +0x{at:x}')
        out[at:at+4] = struct.pack('<I', word)
    if any(out[i:i+4] == sentinel for i in range(0, len(out), 4)):
        raise baseline.AdspError('unresolved sentinel in ADSP TLB snapshot')
    return bytes(out)


def verify(reference: bytes, output: bytes) -> None:
    if hashlib.sha256(reference).hexdigest() != SHA256_REFERENCE:
        raise ValueError('wrong ADSP firmware revision: source SHA-256 mismatch')
    if len(output) != SIZE:
        raise ValueError('generated ADSP interval is wrong size')
    # Verify exact executable segment, address mapping and file bounds.
    base = load_baseline_emitter()
    base.verify_reference(output, reference, FILE_START)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--llvm-mc', required=True, help='independent LLVM6 Hexagon assembler')
    parser.add_argument('--reference', type=Path, help='private reference ADSP MBN; used for comparison only')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    bytes_out = render(args.llvm_mc)
    if args.reference:
        verify(args.reference.read_bytes(), bytes_out)
    args.output.write_bytes(bytes_out)
    print(f'ADSP source-built entry RX tail: {len(bytes_out)} exact bytes '
          f'at file 0x{FILE_START:x}..0x{FILE_END:x} '
          f'physical 0x{PHYSICAL_START:x}..0x{PHYSICAL_START+SIZE:x}; '
          f'SHA256={hashlib.sha256(bytes_out).hexdigest()}')
    return 0


if __name__ == '__main__':
    sys.exit(main())
