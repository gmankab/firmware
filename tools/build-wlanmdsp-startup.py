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
HOOK_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-loader-hooks.S"
FILE_OFFSET = 0x40000
VADDR = 0xB0000000
TEXT_LENGTH = 0x154
PLT_START = 0x1000
PLT_FIRST_ENTRY = 0x1030
PLT_ENTRY_BYTES = 16
PLT_COUNT = 181
PLT_GOT = 0xB0340018
PLT_LENGTH = PLT_FIRST_ENTRY + PLT_COUNT * PLT_ENTRY_BYTES - PLT_START
CODE_AND_PLT_LENGTH = PLT_START + PLT_LENGTH
LOADER_HOOKS_LENGTH = 0x64
THROUGH_LOADER_HOOKS_LENGTH = CODE_AND_PLT_LENGTH + LOADER_HOOKS_LENGTH
# Identified uniform gap immediately following _init up to the linker PLT.
# Treat this as layout fill, NEVER as independently reconstructed code.
LINKER_GAP_WORD = 0x00c0007f
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


def read_resolved_text(obj: bytes, expected_length: int = TEXT_LENGTH) -> bytes:
    sections = elf32_sections(obj)
    matches = [s for s in sections if s["name"] == ".text" and s["kind"] == 1]
    if len(matches) != 1:
        raise FragmentError("exactly one executable .text section required")
    text = matches[0]
    for section in sections:
        if section["kind"] in (4, 9) and section["info"] == text["index"] and section["size"]:
            raise FragmentError("unresolved text relocation: fragment is not byte-reproducible")
    offset, length = int(text["offset"]), int(text["size"])
    if length != expected_length:
        raise FragmentError(f"expected {expected_length} encoded bytes, got {length}")
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


def plt_assembly() -> str:
    """Generate authentic Hexagon PLT semantics from its linker-derived layout.

    PLTGOT=0xb0340018; the first lazy binding slot is PLTGOT+0x10. The
    dynamic table advertises 0x87c JMPREL bytes / 12 bytes per RELA=181
    imported symbols. We encode the resolver and one dispatch stub for each
    slot, rather than importing original instruction bytes.
    """
    lines = [
        "// SPDX-License-Identifier: GPL-3.0-only",
        "// Reconstructed Hexagon ELF PLT from GOT layout and RELA count.",
        ".text", ".globl wlan_plt", "wlan_plt:",
        "{ r28 = add(pc,##3403800) }",  # PC-relative resolver GOT offset
        "{ r14 -= add(r28,#16); r15 = memw(r28+#8); r28 = memw(r28+#4) }",
        "{ r14 = asr(r14,#2); jumpr r28 }",
        ".space 20,0",  # resolver is aligned to 0x30 before imports
    ]
    for idx in range(PLT_COUNT):
        thunk_address = PLT_FIRST_ENTRY + PLT_ENTRY_BYTES * idx
        slot_address = (PLT_GOT - VADDR) + 0x10 + 4 * idx
        displacement = slot_address - thunk_address
        lines.extend((f"wlan_plt_{idx:03}:",
                      f"{{ r14 = add(pc,##{displacement}) }}",
                      "{ r28 = memw(r14+#0) }", "{ jumpr r28 }"))
    return "\n".join(lines) + "\n"


def assemble(source: Path, llvm_mc: str, cpu: str = "hexagonv65",
             expected_length: int = TEXT_LENGTH) -> bytes:
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
        return read_resolved_text(obj_path.read_bytes(), expected_length)


def assemble_plt(llvm_mc: str, cpu: str) -> bytes:
    with tempfile.TemporaryDirectory(prefix="wlan-plt-") as work:
        source = Path(work) / "derived-plt.S"
        source.write_text(plt_assembly())
        return assemble(source, llvm_mc, cpu, expected_length=PLT_LENGTH)


def emit_with_plt(startup: bytes, llvm_mc: str, cpu: str) -> bytes:
    if len(startup) != TEXT_LENGTH:
        raise FragmentError("startup fragment is not 0x154 bytes")
    linker_gap = LINKER_GAP_WORD.to_bytes(4, "little") * ((PLT_START-TEXT_LENGTH)//4)
    plt = assemble_plt(llvm_mc, cpu)
    if len(startup) + len(linker_gap) + len(plt) != CODE_AND_PLT_LENGTH:
        raise FragmentError("PLT layout has a hole or overlaps executable code")
    return startup + linker_gap + plt


def emit_with_loader_hooks(prefix: bytes, llvm_mc: str, cpu: str) -> bytes:
    if len(prefix) != CODE_AND_PLT_LENGTH:
        raise FragmentError("loader hooks require the complete preceding PLT")
    hooks = assemble(HOOK_SOURCE, llvm_mc, cpu, expected_length=LOADER_HOOKS_LENGTH)
    return prefix + hooks


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("output", type=Path, help="independently rebuilt raw executable bytes")
    ap.add_argument("--llvm-mc", default="llvm-mc", help="LLVM Hexagon-capable llvm-mc binary")
    ap.add_argument("--cpu", default="hexagonv65", help="assembler codegen CPU (v66 not supported by supplied LLVM6)")
    ap.add_argument("--source", default=SOURCE, type=Path)
    ap.add_argument("--reference", type=Path, help="optional WLAN MBN for strict revision comparison")
    ap.add_argument("--manifest", type=Path, help="optional JSON containing rebuild digest/coverage")
    ap.add_argument("--with-plt", action="store_true",
                    help="emit startup, isolated 0x154–0x1000 linker fill, and 181-symbol PLT")
    ap.add_argument("--with-loader-hooks", action="store_true",
                    help="include 100-byte independently assembled post-PLT entry and GOT hooks")
    ap.add_argument("--emit-plt-assembly", type=Path,
                    help="write independently generated PLT source for inspection")
    args = ap.parse_args()
    fragment = assemble(args.source, args.llvm_mc, args.cpu)
    if args.emit_plt_assembly is not None:
        args.emit_plt_assembly.parent.mkdir(parents=True, exist_ok=True)
        args.emit_plt_assembly.write_text(plt_assembly())
    if args.with_plt or args.with_loader_hooks:
        fragment = emit_with_plt(fragment, args.llvm_mc, args.cpu)
    if args.with_loader_hooks:
        fragment = emit_with_loader_hooks(fragment, args.llvm_mc, args.cpu)
    if args.reference is not None:
        check_reference(fragment, args.reference.read_bytes())
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(fragment)
    digest = hashlib.sha256(fragment).hexdigest()
    report = {"firmware": "wlanmdsp.mbn", "file_offset": FILE_OFFSET,
              "vaddr": VADDR, "verified_bytes": len(fragment) if args.reference else None,
              "generated_bytes": len(fragment), "sha256": digest,
              "complete_firmware": False,
              "independently_encoded_startup_bytes": TEXT_LENGTH,
              "independently_encoded_plt_instruction_bytes": (PLT_LENGTH - 20) if (args.with_plt or args.with_loader_hooks) else 0,
              "linker_layout_fill_bytes": (PLT_START - TEXT_LENGTH + 20) if (args.with_plt or args.with_loader_hooks) else 0,
              "independently_encoded_loader_hook_bytes": LOADER_HOOKS_LENGTH if args.with_loader_hooks else 0,
              "provenance": "independently assembled Hexagon source; no OEM instruction bytes embedded"}
    if args.manifest is not None:
        args.manifest.parent.mkdir(parents=True, exist_ok=True)
        args.manifest.write_text(json.dumps(report, sort_keys=True, indent=2) + "\n")
    print(f"WLAN Hexagon partial executable: {len(fragment)} layout bytes "
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
