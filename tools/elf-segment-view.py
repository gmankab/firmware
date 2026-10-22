#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build an analysis-only ELF32 file with section headers for sectionless firmware.

This does not recover instructions, alter the firmware payload, or create
redistributable firmware. The input firmware is proprietary and must stay local.
"""
from __future__ import annotations
import argparse
import struct
from pathlib import Path

ELF32_HDR = '<16sHHIIIIIHHHHHH'
PHDR = '<IIIIIIII'
SHDR = '<IIIIIIIIII'


def sectionize(original: bytes, only_executable: bool = True) -> tuple[bytes, list[dict]]:
    if len(original) < 52 or original[:6] != b'\x7fELF\x01\x01':
        raise ValueError('expected little-endian ELF32')
    fields = list(struct.unpack_from(ELF32_HDR, original))
    phoff, phentsize, phnum = fields[5], fields[9], fields[10]
    if phentsize != 32 or phoff + phentsize * phnum > len(original):
        raise ValueError('invalid program header table')
    shstr = bytearray(b'\0')
    sections = [(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)]
    infos = []
    for i in range(phnum):
        p_type, off, vaddr, paddr, filesz, memsz, flags, align = struct.unpack_from(PHDR, original, phoff+i*32)
        if p_type != 1 or (only_executable and not flags & 1) or not filesz:
            continue
        if off + filesz > len(original):
            raise ValueError('segment extends beyond file')
        name = f'.segment{i}.text' if flags & 1 else f'.segment{i}.data'
        shname = len(shstr)
        shstr.extend(name.encode('ascii') + b'\0')
        shaddr = vaddr or paddr
        # Sections may be discontiguous. Preserve actual original offsets, not
        # original ELF metadata. Only append a section table to the file.
        sections.append((shname, 1, 2 | (4 if flags & 1 else 0), shaddr, off, filesz, 0, 0, 4, 0))
        infos.append(dict(name=name, file_offset=off, address=shaddr, size=filesz))
    shstr_name = len(shstr)
    shstr.extend(b'.shstrtab\0')
    out = bytearray(original)
    shstr_off = len(out)
    out.extend(shstr)
    while len(out) % 4:
        out.append(0)
    sht_off = len(out)
    sections.append((shstr_name, 3, 0, 0, shstr_off, len(shstr), 0, 0, 1, 0))
    for s in sections:
        out.extend(struct.pack(SHDR, *s))
    fields[6] = sht_off
    fields[11] = 40
    fields[12] = len(sections)
    fields[13] = len(sections)-1
    struct.pack_into(ELF32_HDR, out, 0, *fields)
    if out[52:len(original)] != original[52:]:
        raise AssertionError('modified original payload beyond ELF metadata')
    return bytes(out), infos


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('firmware', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    view, infos = sectionize(args.firmware.read_bytes())
    args.output.write_bytes(view)
    for i in infos:
        print(f"{i['name']}: file=0x{i['file_offset']:x} vm=0x{i['address']:x} bytes=0x{i['size']:x}")


if __name__ == '__main__':
    main()
