#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build source-authored ADSP later executable fragments, never OEM payloads.

An exact byte comparison of a generated fragment against a private, pinned
reference is a verification input, never a source of output bytes.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
from pathlib import Path
import struct
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / 'tools'
REFERENCE_SHA256 = '4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
ELF_MACHINE_HEXAGON = 164
SEGMENTS = (
    (0x39000, 0xF0036000, 0x8BE36000, 0x1B97E8, 0x1BA000, 5),
    (0x364000, 0xB0000000, 0x8C500000, 0x94B790, 0x94C000, 5),
)
# File offset and exact executable-code length (not inferred BSS/alignment)
FRAGMENTS = {
    'queue-drain': (TOOLS / 'adsp-late-callback-drain.S', 0x39000, 0x54),
}

class LateAdspError(ValueError):
    """Format, architecture, revision or byte-identity failure."""


def checked_range(size: int, offset: int, length: int) -> None:
    if not (0 <= offset <= size and 0 <= length <= size - offset):
        raise LateAdspError(f'Invalid bounded interval at 0x{offset:x} len 0x{length:x}')


def validate_reference(data: bytes) -> None:
    if len(data) < 52 or data[:6] != b'\x7fELF\x01\x01':
        raise LateAdspError('Expected little-endian ELF32 reference')
    (
        kind, machine, version, entry, phoff, shoff, flags, ehsize,
        phentsize, phnum, shentsize, shnum, shstrndx
    ) = struct.unpack_from('<HHIIIIIHHHHHH', data, 16)
    if kind != 2 or machine != ELF_MACHINE_HEXAGON or version != 1 or flags != 0x66:
        raise LateAdspError('Wrong ADSP image ELF32/Hexagon-v66 identification')
    if entry != 0x8BE00000 or phentsize != 32 or phnum < 11 or ehsize < 52:
        raise LateAdspError('Wrong ADSP ELF32 entry/program-header layout')
    checked_range(len(data), phoff, phentsize * phnum)
    mappings = set()
    for i in range(phnum):
        kind, off, va, pa, fsz, msz, perms, align = struct.unpack_from(
            '<IIIIIIII', data, phoff + i * phentsize)
        if kind != 1:
            continue
        if fsz:
            checked_range(len(data), off, fsz)
        if msz < fsz:
            raise LateAdspError('ELF segment filesz exceeds memsz')
        if perms & 1:
            mappings.add((off, va, pa, fsz, msz, perms & 7))
    if not set(SEGMENTS).issubset(mappings):
        raise LateAdspError('ADSP later executable PT_LOAD mappings differ')
    digest = hashlib.sha256(data).hexdigest()
    if digest != REFERENCE_SHA256:
        raise LateAdspError(f'Wrong original ADSP revision: {digest}')


def extract_text(elf: bytes, length: int) -> bytes:
    # Agent04 preexisting parser requires exactly one executable .text,
    # rejects unresolved relocations and checks proper bounded ELF ranges.
    modfile = TOOLS / 'emit-adsp-entry.py'
    spec = importlib.util.spec_from_file_location('adsp_entry_parser', modfile)
    if spec is None or spec.loader is None:
        raise LateAdspError('ADSP ELF object parser unavailable')
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    try:
        return module.read_text_section(elf, expected_length=length)
    except Exception as e:
        raise LateAdspError(str(e)) from e


def compile_source(llvm_mc: str, source: Path, size: int) -> bytes:
    if not source.is_file():
        raise LateAdspError(f'Assembler source missing: {source}')
    asm = source.read_text()
    for forbidden in ('.word ', '.long ', '.byte ', '.incbin ', '.fill '):
        if forbidden in asm:
            raise LateAdspError(f'Private binary or opcode-word directive forbidden: {forbidden}')
    with tempfile.TemporaryDirectory(prefix='adsp-late-') as d:
        obj = Path(d) / 'program.o'
        result = subprocess.run(
            [llvm_mc, '-triple=hexagon', '-mcpu=hexagonv65', '-filetype=obj',
             str(source), '-o', str(obj)],
            text=True, capture_output=True, check=False)
        if result.returncode:
            raise LateAdspError(f'Hexagon LLVM6 assembler failed: {result.stderr.strip()}')
        return extract_text(obj.read_bytes(), size)


def verify_slice(code: bytes, data: bytes, offset: int) -> None:
    validate_reference(data)
    if not any(lo <= offset and offset + len(code) <= lo + size
               for lo, _va, _pa, size, _mem, _perms in SEGMENTS):
        raise LateAdspError(f'Generated fragment 0x{offset:x} outside ADSP executable segment')
    checked_range(len(data), offset, len(code))
    original = data[offset:offset + len(code)]
    if original != code:
        mismatch = next(i for i, (left, right) in enumerate(zip(code, original))
                        if left != right)
        raise LateAdspError(f'Mismatch at original file offset 0x{offset+mismatch:x}: '
                            f'generated={code[mismatch]:02x}, expected={original[mismatch]:02x}')


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--segment', choices=sorted(FRAGMENTS), default='queue-drain')
    p.add_argument('--llvm-mc', required=True)
    p.add_argument('--output', type=Path, required=True)
    p.add_argument('--reference', type=Path, help='private exact adsp.mbn (optional)')
    args = p.parse_args(argv)
    source, off, size = FRAGMENTS[args.segment]
    result = compile_source(args.llvm_mc, source, size)
    if args.reference is not None:
        verify_slice(result, args.reference.read_bytes(), off)
    args.output.write_bytes(result)
    print(f'{args.segment}: independently assembled {len(result)} executable bytes '
          f'at original file [0x{off:x},0x{off+size:x}); '
          f'sha256={hashlib.sha256(result).hexdigest()}; reference={args.reference is not None}')
    return 0

if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except LateAdspError as e:
        print(f'ERROR: {e}', file=sys.stderr)
        raise SystemExit(1) from e
