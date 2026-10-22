#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Extract machine-readable *descriptive ELF metadata*, never payload code.

The generated Rust source contains only ELF32 numeric fields, PHDR values,
and the standard ELF magic. No vendor instruction, hash, or signature bytes.
The original firmware is used ONLY to acquire metadata at generation time.
"""
from __future__ import annotations
import argparse
import pathlib
import struct

PREFIX_LEN = 4096


def spec(path: pathlib.Path) -> list[str]:
    data = path.read_bytes()
    if data[:16] != b'\x7fELF\x01\x01\x01' + bytes(9):
        raise ValueError(f'{path}: unexpected ELF identification; aborting')
    (_, elf_type, mach, vers, entry, phoff, shoff, flags, ehsize, phentsize,
     phnum, shentsize, shnum, shstr) = struct.unpack_from('<16sHHIIIIIHHHHHH', data)
    if (elf_type, vers, phoff, shoff, ehsize, phentsize, shnum, shstr) != (2, 1, 52, 0, 52, 32, 0, 0):
        raise ValueError(f'{path}: unexpected ELF header model; extend generator')
    if phoff + 32*phnum > PREFIX_LEN or any(data[i] for i in range(phoff+32*phnum, PREFIX_LEN)):
        raise ValueError(f'{path}: extra bytes in prefix: cannot claim generated reproduction')
    fields = []
    for i in range(phnum):
        rec = struct.unpack_from('<IIIIIIII', data, phoff + i*32)
        fields.append('        Elf32Program { kind: 0x%x, offset: 0x%x, vaddr: 0x%x, paddr: 0x%x, '
                      'filesz: 0x%x, memsz: 0x%x, flags: 0x%x, align: 0x%x },' % rec)
    return ([f'    Elf32Spec {{ name: "{path.name}", machine: {mach}, entry: 0x{entry:x}, '
             f'flags: 0x{flags:x}, section_header_size: {shentsize}, headers: &[']
            + fields + ['    ] },'])


def main() -> None:
    arg = argparse.ArgumentParser()
    arg.add_argument('firmware_directory', type=pathlib.Path,
                     help='directory containing the ELF32 firmware files')
    arg.add_argument('output_rust', type=pathlib.Path)
    arg.add_argument('--names', nargs='+', metavar='FILENAME',
                     help='explicit filenames; default: discover ELF32 *.mbn and *.elf files')
    a = arg.parse_args()
    directory = a.firmware_directory
    if a.names:
        names = [pathlib.Path(name).name for name in a.names]
    else:
        names = sorted(p.name for p in directory.iterdir() if p.is_file()
                       and p.suffix in ('.mbn', '.elf')
                       and p.open('rb').read(6) == b'\x7fELF\x01\x01')
    if not names:
        arg.error('no candidate little-endian ELF32 files found')
    out = [
        '// SPDX-License-Identifier: GPL-3.0-only',
        '// Generated typed ELF header metadata; excludes executable and authentication payloads.',
        '// Generated with tools/generate-elf-specs.py; verify every descriptor before use.',
        'use crate::elf32_prefix::{Elf32Program, Elf32Spec};',
        'pub const KNOWN_ELFS: &[Elf32Spec] = &[',
    ]
    for name in names:
        out.extend(spec(directory / name))
    out.append('];')
    a.output_rust.parent.mkdir(parents=True, exist_ok=True)
    a.output_rust.write_text('\n'.join(out) + '\n')
    print(f'Wrote {a.output_rust} describing {len(names)} ELF metadata prefixes; excludes code and signatures')

if __name__ == '__main__':
    main()
