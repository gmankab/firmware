#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent ARM A32 asm round-trip for selected Venus executable ranges.

Unlike the typed Rust emitter, this tool uses the private reference only in
an explicit validation run to produce *textual assembler instructions*. It
rejects lengths/gaps and does not place original opcode words into Git.
"""
import argparse
import re
import subprocess
import tempfile
from pathlib import Path


def parse_addr(s: str) -> int:
    return int(s, 0)


def run() -> None:
    p = argparse.ArgumentParser()
    p.add_argument('reference', type=Path)
    p.add_argument('--file-offset', required=True, type=parse_addr)
    p.add_argument('--vma', required=True, type=parse_addr)
    p.add_argument('--length', required=True, type=parse_addr)
    p.add_argument('--data-range', action='append', default=[])
    p.add_argument('--binary-out', type=Path)
    args = p.parse_args()
    if args.file_offset % 4 or args.vma % 4 or args.length % 4 or args.length <= 0:
        raise ValueError('range must be nonempty and four-byte aligned')
    intervals: list[tuple[int, int]] = []
    for arg in args.data_range:
        left, right = arg.split(':', 1)
        a, b = int(left, 0), int(right, 0)
        if not (args.vma <= a < b <= args.vma + args.length) or (a | b) & 3:
            raise ValueError(f'invalid data range: {arg}')
        if intervals and intervals[-1][1] > a:
            raise ValueError('overlapping or unordered data intervals')
        intervals.append((a, b))
    image = args.reference.read_bytes()
    reference = image[args.file_offset:args.file_offset + args.length]
    if len(reference) != args.length:
        raise ValueError('truncated reference')
    pat = re.compile(r'^\s*([0-9a-f]+):\s+([0-9a-f]{8})\s+(\S+)(?:\s+(.*))?$', re.I)
    with tempfile.TemporaryDirectory() as directory:
        wd = Path(directory)
        (wd / 'source.bin').write_bytes(reference)
        (wd / 'source.S').write_text(
            '.syntax unified\n.arm\n.text\n.global entry\nentry:\n'
            f'.incbin "{wd / "source.bin"}"\n'
        )
        subprocess.run(['clang', '--target=armv7a-none-eabi', '-c', str(wd / 'source.S'),
                        '-o', str(wd / 'source.o')], check=True)
        listing = subprocess.check_output(
            ['llvm-objdump', '-d', f'--adjust-vma={args.vma}', str(wd / 'source.o')],
            text=True)
        code: dict[int, tuple[str, str]] = {}
        for line in listing.splitlines():
            m = pat.match(line)
            if m:
                code[int(m[1], 16)] = (m[3], (m[4] or '').split('@')[0].strip())
        lines = ['.syntax unified', '.arm', '.text', '.global entry', 'entry:']
        data_count = 0
        for address in range(args.vma, args.vma + args.length, 4):
            i = address - args.vma
            if any(a <= address < b for a, b in intervals):
                value = int.from_bytes(reference[i:i + 4], 'little')
                lines.append(f'  .word 0x{value:08x}')
                data_count += 1
                continue
            if address not in code:
                raise ValueError(f'disassembler omitted instruction at 0x{address:x}')
            op, operand = code[address]
            operand = re.sub(r'\s+<[^>]*>.*$', '', operand)
            conditions = ('eq', 'ne', 'lt', 'le', 'gt', 'ge', 'cs', 'cc',
                          'hs', 'lo', 'hi', 'ls', 'mi', 'pl', 'vs', 'vc')
            if (op in ('bl', 'blx', 'b') or
                    (op.startswith('bl') and op[2:] in conditions) or
                    (op.startswith('b') and op[1:] in conditions)):
                # Conditional BL (e.g., blne) is PC-relative just like B.
                # Its absolute objdump target must not become an absolute
                # assembler operand. Conditional BLS is disambiguated by
                # testing the complete condition suffix above.
                target = int(operand.split()[0], 16)
                delta = target - address
                operand = f'. {delta:+d}'
                operand = operand.replace('. +', '. + ').replace('. -', '. - ')
            lines.append(f'  {op} {operand}'.rstrip())
        (wd / 'rebuilt.S').write_text('\n'.join(lines) + '\n')
        subprocess.run(['clang', '--target=armv7a-none-eabi', '-c', str(wd / 'rebuilt.S'),
                        '-o', str(wd / 'rebuilt.o')], check=True)
        subprocess.run(['llvm-objcopy', '-O', 'binary', str(wd / 'rebuilt.o'),
                        str(wd / 'rebuilt.bin')], check=True)
        rebuilt = (wd / 'rebuilt.bin').read_bytes()
    if len(rebuilt) != len(reference) or rebuilt != reference:
        pos = next((i for i, (a, b) in enumerate(zip(rebuilt, reference)) if a != b),
                   min(len(reference), len(rebuilt)))
        raise ValueError(f'round-trip mismatch at byte {pos} / file 0x{args.file_offset + pos:x} ({rebuilt[pos:pos+4].hex()} vs {reference[pos:pos+4].hex()})')
    if args.binary_out is not None:
        args.binary_out.write_bytes(rebuilt)
    print(f'Independently reassembled {len(reference)} bytes: '
          f'{len(reference)//4 - data_count} A32 instructions, {data_count} data words, exact match')


if __name__ == '__main__':
    run()
