#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble an independently written Hexagon startup fragment.

No original firmware is required for the build. A provided original WLAN MBN
is read only for opt-in, exact-revision byte verification. The emitted file is
an unsigned fragment, never a bootable or signed MBN container.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import struct
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-startup.S"
FILE_OFFSET = 0x40000
VADDR = 0xB0000000
TEXT_LENGTH = 0x154
FIRMWARE_SHA256 = "87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b"


class FragmentError(ValueError):
    """An unsafe, incomplete or nonmatching reconstruction."""


def elf32_sections(image: bytes) -> list[dict[str, int | str]]:
    """Read bounded ELF32 section headers, without a target-specific parser."""
    if len(image) < 52 or image[:6] != b"\x7fELF\x01\x01":
        raise FragmentError("assembler output is not little-endian ELF32")
    if struct.unpack_from("<H", image, 18)[0] != 164:
        raise FragmentError("assembler output is not Hexagon (machine=164)")
    off = struct.unpack_from("<I", image, 32)[0]
    entsize, count, names_idx = struct.unpack_from("<HHH", image, 46)
    if entsize < 40 or count == 0 or names_idx >= count:
        raise FragmentError("malformed section-header table")
    if off + count * entsize > len(image):
        raise FragmentError("section-header table exceeds object")
    sections = [struct.unpack_from("<10I", image, off + i * entsize)
                for i in range(count)]
    names_section = sections[names_idx]
    string_start, string_size = names_section[4:6]
    if string_start + string_size > len(image):
        raise FragmentError("section-name table exceeds object")
    string_table = image[string_start:string_start + string_size]
    out: list[dict[str, int | str]] = []
    for index, sh in enumerate(sections):
        name_idx = sh[0]
        if name_idx >= len(string_table):
            raise FragmentError("section name points outside string table")
        end = string_table.find(b"\0", name_idx)
        if end < 0:
            raise FragmentError("unterminated section name")
        try:
            name = string_table[name_idx:end].decode("ascii")
        except UnicodeDecodeError as exc:
            raise FragmentError("non-ASCII section name") from exc
        section = {"name": name, "index": index, "kind": sh[1],
                   "offset": sh[4], "size": sh[5], "info": sh[7]}
        if sh[1] != 8 and sh[4] + sh[5] > len(image):
            raise FragmentError(f"section {name} exceeds assembled object")
        out.append(section)
    return out


def read_resolved_text(obj: bytes) -> bytes:
    sections = elf32_sections(obj)
    matches = [s for s in sections if s["name"] == ".text" and s["kind"] == 1]
    if len(matches) != 1:
        raise FragmentError("exactly one executable .text section required")
    text = matches[0]
    for section in sections:
        if section["kind"] in (4, 9) and section["info"] == text["index"] and section["size"]:
            raise FragmentError("unresolved text relocation: fragment is not byte-reproducible")
    offset, length = int(text["offset"]), int(text["size"])
    if length != TEXT_LENGTH:
        raise FragmentError(f"expected {TEXT_LENGTH} encoded bytes, got {length}")
    return obj[offset:offset + length]


def check_reference(fragment: bytes, firmware: bytes) -> None:
    if hashlib.sha256(firmware).hexdigest() != FIRMWARE_SHA256:
        raise FragmentError("wrong WLAN firmware revision (SHA-256 differs)")
    if firmware[:6] != b"\x7fELF\x01\x01" or len(firmware) < 52:
        raise FragmentError("reference is not little-endian ELF32")
    if struct.unpack_from("<H", firmware, 18)[0] != 164:
        raise FragmentError("reference is not Hexagon ELF")
    if struct.unpack_from("<I", firmware, 24)[0] != VADDR:
        raise FragmentError("unexpected WLAN ELF entry")
    phoff = struct.unpack_from("<I", firmware, 28)[0]
    entsize, count = struct.unpack_from("<HH", firmware, 42)
    if entsize < 32 or phoff + entsize * count > len(firmware):
        raise FragmentError("invalid ELF program-header table")
    programs = [struct.unpack_from("<8I", firmware, phoff + i * entsize)
                for i in range(count)]
    candidates = [p for p in programs if p[0] == 1 and p[1] == FILE_OFFSET
                  and p[2] == VADDR and (p[6] & 5) == 5]
    if len(candidates) != 1 or candidates[0][4] < len(fragment):
        raise FragmentError("WLAN executable PT_LOAD not found at file 0x40000")
    expected = firmware[FILE_OFFSET:FILE_OFFSET + len(fragment)]
    if len(expected) != len(fragment):
        raise FragmentError("reference executable truncated")
    for at, (got, want) in enumerate(zip(fragment, expected)):
        if got != want:
            raise FragmentError(f"first mismatch at ELF file offset 0x{FILE_OFFSET+at:x}, "
                                f"vaddr 0x{VADDR+at:x}: built 0x{got:02x}, reference 0x{want:02x}")


def assemble(source: Path, llvm_mc: str, cpu: str = "hexagonv65") -> bytes:
    asm = source.read_text(encoding="utf-8")
    forbidden = (".incbin", ".byte", ".short", ".word", ".long", ".quad")
    if any(token in asm for token in forbidden):
        raise FragmentError("startup assembly must not embed original opcode/data words")
    with tempfile.TemporaryDirectory(prefix="wlanmdsp-hexagon-") as scratch:
        obj_path = Path(scratch) / "startup.o"
        args = [llvm_mc, "-triple=hexagon", f"-mcpu={cpu}", "-filetype=obj",
                "-o", str(obj_path), str(source)]
        try:
            proc = subprocess.run(args, capture_output=True, text=True, check=False, timeout=45)
        except FileNotFoundError as exc:
            raise FragmentError(f"Hexagon assembler unavailable: {llvm_mc}") from exc
        if proc.returncode != 0:
            raise FragmentError("Hexagon assembly failed: " + proc.stderr[:1000])
        return read_resolved_text(obj_path.read_bytes())


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("output", type=Path, help="independently rebuilt raw executable bytes")
    ap.add_argument("--llvm-mc", default="llvm-mc", help="LLVM Hexagon-capable llvm-mc binary")
    ap.add_argument("--cpu", default="hexagonv65", help="assembler codegen CPU (v66 not supported by supplied LLVM6)")
    ap.add_argument("--source", default=SOURCE, type=Path)
    ap.add_argument("--reference", type=Path, help="optional WLAN MBN for strict revision comparison")
    ap.add_argument("--manifest", type=Path, help="optional JSON containing rebuild digest/coverage")
    args = ap.parse_args()
    fragment = assemble(args.source, args.llvm_mc, args.cpu)
    if args.reference is not None:
        check_reference(fragment, args.reference.read_bytes())
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(fragment)
    digest = hashlib.sha256(fragment).hexdigest()
    report = {"firmware": "wlanmdsp.mbn", "file_offset": FILE_OFFSET,
              "vaddr": VADDR, "verified_bytes": len(fragment) if args.reference else None,
              "generated_bytes": len(fragment), "sha256": digest,
              "complete_firmware": False,
              "provenance": "independently assembled Hexagon source; no OEM instruction bytes embedded"}
    if args.manifest is not None:
        args.manifest.parent.mkdir(parents=True, exist_ok=True)
        args.manifest.write_text(json.dumps(report, sort_keys=True, indent=2) + "\n")
    print(f"WLAN Hexagon startup: {len(fragment)} independently encoded bytes "
          f"(file 0x{FILE_OFFSET:x}–0x{FILE_OFFSET + len(fragment):x}; "
          f"vaddr 0x{VADDR:x}–0x{VADDR+len(fragment):x})")
    if args.reference is not None:
        print("Exact byte identity against SHA-256-verified reference.")
    print("Partial executable fragment only; NOT a complete, authenticated firmware image.")


if __name__ == "__main__":
    try:
        main()
    except FragmentError as exc:
        raise SystemExit(f"ERROR: {exc}") from exc
