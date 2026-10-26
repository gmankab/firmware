#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build Agent11's independent Hexagon RWE exception handler fragments.

Unlike a disassemble/reassemble operation on OEM bytes, all opcodes are
emitted from source-level Hexagon assembly, checked architecture macros and
explicitly named register/branch operands. The original modem.mbn is used
only for optional reference verification (SHA-256 locked).
"""
from __future__ import annotations
import argparse
import hashlib
from pathlib import Path
import runpy
import subprocess
import tempfile

FILE_START = 0x38108
FILE_END = 0x38200
VIRTUAL_START = 0xc0840108
PHYSICAL_START = 0x8e040108
PINNED_SHA256 = '6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'


def assemble(llvm_mc: Path, source: Path) -> bytes:
    with tempfile.TemporaryDirectory(prefix='modem-rwe-exception-') as tmp:
        obj = Path(tmp) / 'exception.o'
        proc = subprocess.run([str(llvm_mc), '-triple=hexagon', '-mcpu=hexagonv60',
                               '-filetype=obj', str(source), '-o', str(obj)],
                              capture_output=True, text=True, check=False)
        if proc.returncode:
            raise ValueError(f'Hexagon V60 assembler failed: {proc.stderr}')
        text_from_object = runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        data = text_from_object(obj.read_bytes())
        expected = FILE_END - FILE_START
        if len(data) != expected:
            raise ValueError(f'Unexpected exception-entry size {len(data)}, expected {expected}')
        if data[:4] != bytes.fromhex('00c00058'):
            raise ValueError('Exception trap must start with a self-loop standalone branch')
        def word(address: int) -> int:
            at=address-FILE_START
            return int.from_bytes(data[at:at+4], 'little')
        if word(0x381b8) != 0x57e0c000:
            raise ValueError('Expected architectural standalone return-from-exception opcode')
        if word(0x381bc) != 0x529fc000:
            raise ValueError('Expected standalone link-register return after RTE')
        expected_nops=[0x7f004000, 0x7f00c000,
                       0x7f004000, 0x7f00c000,
                       0x7f004000, 0x7f004000, 0x7f004000, 0x7f00c000]
        if [word(at) for at in range(0x381e0,0x38200,4)] != expected_nops:
            raise ValueError('Exception NOP packets have incorrect grouping')
        if word(0x381dc)&(3<<14)!=(1<<14):
            raise ValueError('Last branch must continue into its NOP packet')
        return data


def verify_pinned(reference: bytes, result: bytes) -> None:
    digest = hashlib.sha256(reference).hexdigest()
    if digest != PINNED_SHA256:
        raise ValueError(f'Wrong modem firmware revision: {digest}')
    expected = reference[FILE_START:FILE_END]
    if len(expected) != len(result):
        raise ValueError('Truncated modem exception reference')
    if expected != result:
        offset = next(i for i, (x, y) in enumerate(zip(result, expected)) if x != y)
        raise ValueError(f'First exception mismatch file 0x{FILE_START+offset:x}: generated {result[offset]:02x} reference {expected[offset]:02x}')


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    root = Path(__file__).resolve().parents[1]
    parser.add_argument('--llvm-mc', required=True, type=Path)
    parser.add_argument('--source', type=Path, default=root / 'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_dispatch.s')
    parser.add_argument('--reference', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    try:
        data = assemble(args.llvm_mc, args.source)
        if args.reference:
            verify_pinned(args.reference.read_bytes(), data)
        args.output.write_bytes(data)
    except (OSError, ValueError) as exc:
        parser.exit(1, f'ERROR: {exc}\n')
    print(f'Modem exception entry: file [0x{FILE_START:x},0x{FILE_END:x}), physical 0x{PHYSICAL_START:x}, {len(data)} typed Hexagon bytes, SHA256={hashlib.sha256(data).hexdigest()}')
    if args.reference:
        print('Pinned original reference matches: 0 mismatching bytes')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
