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

NAMES = ('a640_zap', 'adsp', 'cdsp', 'modem', 'venus', 'wlanmdsp')
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
    arg.add_argument('nabu_firmware_directory', type=pathlib.Path,
                     help='directory containing the six *.mbn files')
    arg.add_argument('output_rust', type=pathlib.Path)
    a = arg.parse_args()
    out = [
        '// SPDX-License-Identifier: GPL-3.0-only',
        '// GENERATED descriptive ELF header data. NO executable bytes or OEM signatures.',
        '// Rebuild: tools/generate-elf-specs.py FIRMWARE_DIR crates/nabu-firmware-research/src/elf_specs.rs',
        'use crate::elf32_prefix::{Elf32Program, Elf32Spec};',
        'pub const KNOWN_ELFS: &[Elf32Spec] = &[',
    ]
    for name in NAMES:
        out.extend(spec(a.nabu_firmware_directory / (name + '.mbn')))
    out.append('];')
    a.output_rust.write_text('\n'.join(out) + '\n')
    print(f'Wrote {a.output_rust} describing {len(NAMES)} header layouts; no code or authentication payload copied')

if __name__ == '__main__':
    main()
