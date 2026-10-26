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
import struct
from pathlib import Path
import runpy
import subprocess
import tempfile

FILE_START = 0x38108
FILE_END = 0x38280
VIRTUAL_START = 0xc0840108
PHYSICAL_START = 0x8e040108
PINNED_SHA256 = '6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'


def normalize_llvm6_locked_packets(data: bytes) -> bytes:
    """Convert LLVM6's legal packet schedule to the source-ordered ISA form.

    Hexagon's LLVM6 scheduler moves a .new compare's branch ahead of its
    predicate, and positions a conditional branch ahead of the extender and
    conditionally defined register. The original sequence is also a legal
    instruction packet. Reorder only the two independently assembled packet
    groups after validating instruction classes, operands and marker bits.
    The locked word-store uses checked architectural fields; LLVM6 does not
    accept it in the same packet as an extender. Normalize its producer word
    after explicitly checking the source-generated register and imm marker.
    No original reference bytes are read by this function.
    """
    result=bytearray(data)
    def read(address):return struct.unpack_from('<I',result,address-FILE_START)[0]
    def write(address,word):struct.pack_into('<I',result,address-FILE_START,word)
    expected_classes={
        0x3820c:0x5cff6afe,  # if(!p2.new) jump:nt acquire retry
        0x38210:0x750d4002,  # p2 = cmp.eq(r13,#0)
        0x3821c:0x7800c08c,  # LLVM6 extended immediate into r12, own packet
        0x38220:0xa0accd02,  # checked locked store(r12,p2) = r13
        0x38224:0x5cff62f2,  # if(!p2) retry
        0x38228:0x0c085004,  # constant extender for conditional r12
        0x3822c:0x7ec0408c,  # if(!p2) r12 = ##constant
        0x38230:0x418cd00d,  # if(p2) r13 = memw(r12+#0)
    }
    for address,expected in expected_classes.items():
        if read(address)!=expected:
            raise ValueError(f'unrecognized Hexagon LLVM6 locked packet at 0x{address:x}')
    a,b=read(0x3820c),read(0x38210)
    write(0x3820c,b)
    write(0x38210,a)
    # LLVM6 emits standalone r12=#imm. Move it to the locked-store packet
    # and remove only the private packet-isolation bit used by this class.
    write(0x3821c,0x78000000 | (1<<14) | 12)
    branch,ext,mov=read(0x38224),read(0x38228),read(0x3822c)
    write(0x38224,ext)
    write(0x38228,mov)
    write(0x3822c,branch)
    return bytes(result)


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
        if len(data) >= 0x38234-FILE_START:
            data = normalize_llvm6_locked_packets(data)
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
        if word(0x38274) != 0xf18c4d0d:
            raise ValueError('Controller status mask must be sourced from checked AND-NOT operation')
        if (word(0x3827c) & (3<<14)) != (3<<14):
            raise ValueError('Status-mask conditional branch must end its packet')
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
