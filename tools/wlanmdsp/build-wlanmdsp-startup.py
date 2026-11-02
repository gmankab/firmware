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

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-startup.S"
HOOK_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-loader-hooks.S"
LOOKUP_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-config-lookup.S"
RECORD_PREFIX_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-record-consumer-prefix.S"
IMPORT_ALIASES_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-import-aliases.S"
POSTALIASES_COMPARE_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-postaliases-compare.S"
ARRAY_COMPARE_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-record-array-compare.S"
RECORD_WRAPPER_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-record-wrapper.S"
IRQ_STATE_GUARD_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-irq-state-guard.S"
RUNTIME_TABLE_INIT_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-runtime-table-init.S"
RECORD_CONSUMER_BODY_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-record-consumer-body.S"
RUNTIME_FORMAT_HANDLER_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-runtime-format-handler.S"
RUNTIME_SESSION_INIT_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-runtime-session-init.S"
RUNTIME_REPORTING_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-runtime-reporting.S"
RUNTIME_REGISTRATION_SOURCE = ROOT / "sources/xiaomi-nabu/wlanmdsp/wlan-runtime-registration.S"
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
CONFIG_LOOKUP_LENGTH = 0x9c
THROUGH_CONFIG_LOOKUP_LENGTH = THROUGH_LOADER_HOOKS_LENGTH + CONFIG_LOOKUP_LENGTH
RECORD_CONSUMER_PREFIX_LENGTH = 0x12c
RECORD_CONSUMER_BODY_LENGTH = 0x1fc
THROUGH_RECORD_PREFIX_LENGTH = THROUGH_CONFIG_LOOKUP_LENGTH + RECORD_CONSUMER_PREFIX_LENGTH
THROUGH_COMPLETE_RECORD_CONSUMER_LENGTH = THROUGH_RECORD_PREFIX_LENGTH + RECORD_CONSUMER_BODY_LENGTH
IMPORT_ALIASES_OFFSET = 0x1fa8
IMPORT_ALIASES_LENGTH = 0x18
POSTALIASES_COMPARE_LENGTH = 0x50
THROUGH_POSTALIASES_COMPARE_LENGTH = IMPORT_ALIASES_LENGTH + POSTALIASES_COMPARE_LENGTH
ARRAY_COMPARE_LENGTH = 0x128
THROUGH_ARRAY_COMPARE_LENGTH = THROUGH_POSTALIASES_COMPARE_LENGTH + ARRAY_COMPARE_LENGTH
RECORD_WRAPPER_LENGTH = 0x4c
IRQ_STATE_GUARD_LENGTH = 0x1c
RUNTIME_TABLE_INIT_LENGTH = 0x68
THROUGH_RUNTIME_HELPERS_LENGTH = (THROUGH_ARRAY_COMPARE_LENGTH + RECORD_WRAPPER_LENGTH
                                  + IRQ_STATE_GUARD_LENGTH + RUNTIME_TABLE_INIT_LENGTH)
RUNTIME_FORMAT_HANDLER_LENGTH = 0x138
THROUGH_RUNTIME_FORMAT_HANDLER_LENGTH = THROUGH_COMPLETE_RECORD_CONSUMER_LENGTH + THROUGH_RUNTIME_HELPERS_LENGTH + RUNTIME_FORMAT_HANDLER_LENGTH
RUNTIME_SESSION_INIT_LENGTH = 0x154
THROUGH_RUNTIME_SESSION_INIT_LENGTH = THROUGH_RUNTIME_FORMAT_HANDLER_LENGTH + RUNTIME_SESSION_INIT_LENGTH
RUNTIME_REPORTING_LENGTH = 0x10c
THROUGH_RUNTIME_REPORTING_LENGTH = THROUGH_RUNTIME_SESSION_INIT_LENGTH + RUNTIME_REPORTING_LENGTH
RUNTIME_REGISTRATION_LENGTH = 0x168
THROUGH_RUNTIME_REGISTRATION_LENGTH = THROUGH_RUNTIME_REPORTING_LENGTH + RUNTIME_REGISTRATION_LENGTH
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


def check_reference(fragment: bytes, firmware: bytes, runtime_offset: int = 0) -> None:
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
    if runtime_offset < 0:
        raise FragmentError("negative executable runtime offset")
    if len(candidates) != 1 or candidates[0][4] < runtime_offset + len(fragment):
        raise FragmentError("WLAN executable PT_LOAD does not cover requested fragment")
    expected = firmware[FILE_OFFSET + runtime_offset:FILE_OFFSET + runtime_offset + len(fragment)]
    if len(expected) != len(fragment):
        raise FragmentError("reference executable truncated")
    for at, (got, want) in enumerate(zip(fragment, expected)):
        if got != want:
            raise FragmentError(f"first mismatch at ELF file offset 0x{FILE_OFFSET+runtime_offset+at:x}, "
                                f"vaddr 0x{VADDR+runtime_offset+at:x}: built 0x{got:02x}, reference 0x{want:02x}")


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


def emit_with_config_lookup(prefix: bytes, llvm_mc: str, cpu: str) -> bytes:
    if len(prefix) != THROUGH_LOADER_HOOKS_LENGTH:
        raise FragmentError("config lookup requires the complete preceding loader hooks")
    lookup = assemble(LOOKUP_SOURCE, llvm_mc, cpu, expected_length=CONFIG_LOOKUP_LENGTH)
    return prefix + lookup


def emit_with_record_prefix(prefix: bytes, llvm_mc: str, cpu: str) -> bytes:
    if len(prefix) != THROUGH_CONFIG_LOOKUP_LENGTH:
        raise FragmentError("record prefix requires preceding WLAN lookup routine")
    packetized = assemble(RECORD_PREFIX_SOURCE, llvm_mc, cpu,
                          expected_length=RECORD_CONSUMER_PREFIX_LENGTH)
    return prefix + packetized


def emit_with_complete_record_consumer(prefix: bytes, llvm_mc_v66: str) -> bytes:
    """Complete the preceding Hexagon record consumer using V66 assembly.

    The 508-byte body includes a multiply-subtract instruction unavailable
    in the older LLVM6 Hexagon encoder. A separate newer assembler must
    explicitly be supplied; never copy source OEM bytes as fallback.
    """
    if len(prefix) != THROUGH_RECORD_PREFIX_LENGTH:
        raise FragmentError("complete consumer requires the exact preceding 7596-byte prefix")
    body = assemble(RECORD_CONSUMER_BODY_SOURCE, llvm_mc_v66, cpu="hexagonv66",
                    expected_length=RECORD_CONSUMER_BODY_LENGTH)
    return prefix + body


def emit_with_full_runtime_helpers(prefix: bytes, llvm_mc: str, cpu: str) -> bytes:
    """Append checked aliases/record comparators after closing the gap."""
    if len(prefix) != THROUGH_COMPLETE_RECORD_CONSUMER_LENGTH:
        raise FragmentError("runtime helpers require closed record consumer gap")
    following = assemble_import_aliases_through_runtime_helpers(llvm_mc, cpu)
    if len(following) != THROUGH_RUNTIME_HELPERS_LENGTH:
        raise FragmentError("runtime helper block has unexpected length")
    return prefix + following


def emit_with_runtime_format_handler(prefix: bytes, llvm_mc_v66: str) -> bytes:
    """Append the independent WLAN record-state selector and tail trampolines.

    Includes six genuine executable Hexagon NOP instructions, which are
    unrelated to the 0x154–0x1000 linker alignment pad.
    """
    if len(prefix) != 0x2208:
        raise FragmentError("runtime format handler requires contiguous WLAN prefix through 0x2208")
    routine = assemble(RUNTIME_FORMAT_HANDLER_SOURCE, llvm_mc_v66, "hexagonv66",
                       expected_length=RUNTIME_FORMAT_HANDLER_LENGTH)
    output = prefix + routine
    if len(output) != THROUGH_RUNTIME_FORMAT_HANDLER_LENGTH:
        raise FragmentError("runtime format handler has a hole or overlaps preceding WLAN code")
    return output


def emit_with_runtime_session_init(prefix: bytes, llvm_mc_v66: str) -> bytes:
    """Append initialized WLAN codec runtime table and error reporting paths.

    Uses `:mem_noshuf` to preserve one critical Hexagon packet's store/load
    ordering. No proprietary opcode arrays are embedded in the source.
    """
    if len(prefix) != THROUGH_RUNTIME_FORMAT_HANDLER_LENGTH:
        raise FragmentError("WLAN session initialization requires prefix through 0x2340")
    assembled = assemble(RUNTIME_SESSION_INIT_SOURCE, llvm_mc_v66, "hexagonv66",
                         expected_length=RUNTIME_SESSION_INIT_LENGTH)
    output = prefix + assembled
    if len(output) != THROUGH_RUNTIME_SESSION_INIT_LENGTH:
        raise FragmentError("WLAN session initialization overlaps adjacent routines")
    return output


def emit_with_runtime_reporting(prefix: bytes, llvm_mc_v66: str) -> bytes:
    """Append runtime status reports, import trampoline and branch aliases."""
    if len(prefix) != THROUGH_RUNTIME_SESSION_INIT_LENGTH:
        raise FragmentError("WLAN runtime reporting requires contiguous prefix through 0x2494")
    built = assemble(RUNTIME_REPORTING_SOURCE, llvm_mc_v66, "hexagonv66",
                     expected_length=RUNTIME_REPORTING_LENGTH)
    joined = prefix + built
    if len(joined) != THROUGH_RUNTIME_REPORTING_LENGTH:
        raise FragmentError("WLAN runtime reporting interval overlaps or has a hole")
    return joined


def emit_with_runtime_registration(prefix: bytes, llvm_mc_v66: str) -> bytes:
    """Append completed WLAN registration, guarded teardown and table seeding.

    Version 66 packetization preserves memory transactions through
    `:mem_noshuf` and exact duplex register ordering.
    """
    if len(prefix) != THROUGH_RUNTIME_REPORTING_LENGTH:
        raise FragmentError("WLAN runtime registration requires prefix through 0x25a0")
    block = assemble(RUNTIME_REGISTRATION_SOURCE, llvm_mc_v66, "hexagonv66",
                     expected_length=RUNTIME_REGISTRATION_LENGTH)
    output = prefix + block
    if len(output) != THROUGH_RUNTIME_REGISTRATION_LENGTH:
        raise FragmentError("WLAN runtime registration is not contiguous")
    return output


def assemble_import_aliases_through_runtime_helpers(llvm_mc: str, cpu: str) -> bytes:
    """Reconstruct a disconnected group through record-wrapper/runtime setup.

    The final packet has a mandatory `mem_noshuf` directive, which LLVM6
    rejects on Hexagon v62. LLVM6 v65 is used for this newer assembly source.
    """
    prefix = assemble_import_aliases_through_array_compare(llvm_mc, cpu)
    wrapper = assemble(RECORD_WRAPPER_SOURCE, llvm_mc, cpu,
                       expected_length=RECORD_WRAPPER_LENGTH)
    guard = assemble(IRQ_STATE_GUARD_SOURCE, llvm_mc, cpu,
                     expected_length=IRQ_STATE_GUARD_LENGTH)
    init = assemble(RUNTIME_TABLE_INIT_SOURCE, llvm_mc, cpu,
                    expected_length=RUNTIME_TABLE_INIT_LENGTH)
    return prefix + wrapper + guard + init


def assemble_import_aliases_through_array_compare(llvm_mc: str, cpu: str) -> bytes:
    """Append a standalone fieldwise-array comparison function to the aliases.

    The interval is disjoint from the earlier partial record consumer; the
    return and 2 hardware loops are independently encoded with LLVM6.
    """
    earlier = assemble_import_aliases_through_compare(llvm_mc, cpu)
    arrays = assemble(ARRAY_COMPARE_SOURCE, llvm_mc, cpu,
                      expected_length=ARRAY_COMPARE_LENGTH)
    return earlier + arrays


def assemble_import_aliases_through_compare(llvm_mc: str, cpu: str) -> bytes:
    """Reconstruct a separate branch/record-comparator region after the PLT.

    The earlier packed-record consumer has a 0x1fc0-0x1dac gap, so these
    bytes must not be appended to the contiguous startup layout.
    """
    aliases = assemble(IMPORT_ALIASES_SOURCE, llvm_mc, cpu,
                       expected_length=IMPORT_ALIASES_LENGTH)
    compare = assemble(POSTALIASES_COMPARE_SOURCE, llvm_mc, cpu,
                       expected_length=POSTALIASES_COMPARE_LENGTH)
    return aliases + compare


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
    ap.add_argument("--with-config-lookup", action="store_true",
                    help="include the 156-byte table lookup and descriptor extractor")
    ap.add_argument("--with-record-prefix", action="store_true",
                    help="append the 300-byte record-consumer entry and initial checks; partial function")
    ap.add_argument("--llvm-mc-v66", default=None,
                    help="Hexagon v66 llvm-mc (LLVM22), mandatory to rebuild the record-consumer gap")
    ap.add_argument("--with-complete-record-consumer", action="store_true",
                    help="reconstruct 508-byte V66 consumer body joining runtime prefix to import aliases")
    ap.add_argument("--with-full-initial-region", action="store_true",
                    help="emit contiguous WLAN startup, PLT, consumer and helper functions through 0x2208")
    ap.add_argument("--with-runtime-format-handler", action="store_true",
                    help="append the 312-byte WLAN format-state handler, tail stubs and NOPs through 0x2340")
    ap.add_argument("--with-runtime-session-init", action="store_true",
                    help="append 340-byte codec runtime table initialization through offset 0x2494")
    ap.add_argument("--with-runtime-reporting", action="store_true",
                    help="append 268-byte WLAN logging helpers and tail-branch stubs through 0x25a0")
    ap.add_argument("--with-runtime-registration", action="store_true",
                    help="append 360-byte WLAN registration, teardown and table initialization through 0x2708")
    ap.add_argument("--import-aliases-only", action="store_true",
                    help="emit three separately verified Hexagon tail aliases at runtime offset 0x1fa8")
    ap.add_argument("--import-aliases-with-compare", action="store_true",
                    help="emit import aliases and 80-byte record comparison at runtime 0x1fa8")
    ap.add_argument("--import-aliases-with-array-compare", action="store_true",
                    help="emit aliases, the 80-byte comparator, and 296-byte array comparator")
    ap.add_argument("--import-aliases-with-runtime-helpers", action="store_true",
                    help="emit all post-import record comparators and runtime helpers to offset 0x2208")
    ap.add_argument("--emit-plt-assembly", type=Path,
                    help="write independently generated PLT source for inspection")
    args = ap.parse_args()
    if args.with_runtime_registration:
        args.with_runtime_reporting = True
    if args.with_runtime_reporting:
        args.with_runtime_session_init = True
    if args.with_runtime_session_init:
        args.with_runtime_format_handler = True
    if args.with_runtime_format_handler:
        args.with_full_initial_region = True
    if args.with_full_initial_region:
        args.with_complete_record_consumer = True
    if args.with_complete_record_consumer:
        args.with_record_prefix = True
        if not args.llvm_mc_v66:
            raise FragmentError("closing WLAN consumer gap requires --llvm-mc-v66")
    disjoint_aliases = (args.import_aliases_only or args.import_aliases_with_compare
                        or args.import_aliases_with_array_compare or args.import_aliases_with_runtime_helpers)
    if sum(int(x) for x in (args.import_aliases_only, args.import_aliases_with_compare,
                            args.import_aliases_with_array_compare, args.import_aliases_with_runtime_helpers)) > 1:
        raise FragmentError("choose exactly one disjoint import region mode")
    if disjoint_aliases and (args.with_plt or args.with_loader_hooks
                             or args.with_config_lookup or args.with_record_prefix):
        raise FragmentError("disjoint import region cannot be combined with contiguous prefix")
    runtime_offset = IMPORT_ALIASES_OFFSET if disjoint_aliases else 0
    if args.import_aliases_with_runtime_helpers:
        fragment = assemble_import_aliases_through_runtime_helpers(args.llvm_mc, args.cpu)
    elif args.import_aliases_with_array_compare:
        fragment = assemble_import_aliases_through_array_compare(args.llvm_mc, args.cpu)
    elif args.import_aliases_with_compare:
        fragment = assemble_import_aliases_through_compare(args.llvm_mc, args.cpu)
    elif args.import_aliases_only:
        fragment = assemble(IMPORT_ALIASES_SOURCE, args.llvm_mc, args.cpu,
                            expected_length=IMPORT_ALIASES_LENGTH)
    else:
        fragment = assemble(args.source, args.llvm_mc, args.cpu)
    if args.emit_plt_assembly is not None:
        args.emit_plt_assembly.parent.mkdir(parents=True, exist_ok=True)
        args.emit_plt_assembly.write_text(plt_assembly())
    if args.with_plt or args.with_loader_hooks or args.with_config_lookup or args.with_record_prefix:
        fragment = emit_with_plt(fragment, args.llvm_mc, args.cpu)
    if args.with_loader_hooks or args.with_config_lookup or args.with_record_prefix:
        fragment = emit_with_loader_hooks(fragment, args.llvm_mc, args.cpu)
    if args.with_config_lookup or args.with_record_prefix:
        fragment = emit_with_config_lookup(fragment, args.llvm_mc, args.cpu)
    if args.with_record_prefix:
        fragment = emit_with_record_prefix(fragment, args.llvm_mc, args.cpu)
    if args.with_complete_record_consumer:
        fragment = emit_with_complete_record_consumer(fragment, args.llvm_mc_v66)
    if args.with_full_initial_region:
        fragment = emit_with_full_runtime_helpers(fragment, args.llvm_mc, args.cpu)
    if args.with_runtime_format_handler:
        fragment = emit_with_runtime_format_handler(fragment, args.llvm_mc_v66)
    if args.with_runtime_session_init:
        fragment = emit_with_runtime_session_init(fragment, args.llvm_mc_v66)
    if args.with_runtime_reporting:
        fragment = emit_with_runtime_reporting(fragment, args.llvm_mc_v66)
    if args.with_runtime_registration:
        fragment = emit_with_runtime_registration(fragment, args.llvm_mc_v66)
    if args.reference is not None:
        check_reference(fragment, args.reference.read_bytes(), runtime_offset=runtime_offset)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(fragment)
    digest = hashlib.sha256(fragment).hexdigest()
    report = {"firmware": "wlanmdsp.mbn", "file_offset": FILE_OFFSET+runtime_offset,
              "vaddr": VADDR+runtime_offset, "verified_bytes": len(fragment) if args.reference else None,
              "generated_bytes": len(fragment), "sha256": digest,
              "complete_firmware": False,
              "independently_encoded_startup_bytes": TEXT_LENGTH,
              "independently_encoded_plt_instruction_bytes": (PLT_LENGTH - 20) if (args.with_plt or args.with_loader_hooks or args.with_config_lookup or args.with_record_prefix) else 0,
              "linker_layout_fill_bytes": (PLT_START - TEXT_LENGTH + 20) if (args.with_plt or args.with_loader_hooks or args.with_config_lookup or args.with_record_prefix) else 0,
              "independently_encoded_loader_hook_bytes": LOADER_HOOKS_LENGTH if (args.with_loader_hooks or args.with_config_lookup or args.with_record_prefix) else 0,
              "independently_encoded_config_lookup_bytes": CONFIG_LOOKUP_LENGTH if (args.with_config_lookup or args.with_record_prefix) else 0,
              "independently_encoded_record_prefix_bytes": RECORD_CONSUMER_PREFIX_LENGTH if args.with_record_prefix else 0,
              "independently_encoded_record_body_bytes": RECORD_CONSUMER_BODY_LENGTH if args.with_complete_record_consumer else 0,
              "independently_encoded_import_alias_bytes": IMPORT_ALIASES_LENGTH if (disjoint_aliases or args.with_full_initial_region) else 0,
              "independently_encoded_record_comparison_bytes": POSTALIASES_COMPARE_LENGTH if (args.import_aliases_with_compare or args.import_aliases_with_array_compare or args.import_aliases_with_runtime_helpers or args.with_full_initial_region) else 0,
              "independently_encoded_array_comparison_bytes": ARRAY_COMPARE_LENGTH if (args.import_aliases_with_array_compare or args.import_aliases_with_runtime_helpers or args.with_full_initial_region) else 0,
              "independently_encoded_runtime_wrapper_bytes": RECORD_WRAPPER_LENGTH if (args.import_aliases_with_runtime_helpers or args.with_full_initial_region) else 0,
              "independently_encoded_irq_guard_bytes": IRQ_STATE_GUARD_LENGTH if (args.import_aliases_with_runtime_helpers or args.with_full_initial_region) else 0,
              "independently_encoded_runtime_init_bytes": RUNTIME_TABLE_INIT_LENGTH if (args.import_aliases_with_runtime_helpers or args.with_full_initial_region) else 0,
              "independently_encoded_runtime_format_handler_bytes": RUNTIME_FORMAT_HANDLER_LENGTH if args.with_runtime_format_handler else 0,
              "independently_encoded_runtime_session_init_bytes": RUNTIME_SESSION_INIT_LENGTH if args.with_runtime_session_init else 0,
              "independently_encoded_runtime_reporting_bytes": RUNTIME_REPORTING_LENGTH if args.with_runtime_reporting else 0,
              "independently_encoded_runtime_registration_bytes": RUNTIME_REGISTRATION_LENGTH if args.with_runtime_registration else 0,
              "separate_disconnected_executable_region": disjoint_aliases,
              "provenance": "independently assembled Hexagon source; no OEM instruction bytes embedded"}
    if args.manifest is not None:
        args.manifest.parent.mkdir(parents=True, exist_ok=True)
        args.manifest.write_text(json.dumps(report, sort_keys=True, indent=2) + "\n")
    print(f"WLAN Hexagon partial executable: {len(fragment)} layout bytes "
          f"(file 0x{FILE_OFFSET+runtime_offset:x}–0x{FILE_OFFSET + runtime_offset + len(fragment):x}; "
          f"vaddr 0x{VADDR+runtime_offset:x}–0x{VADDR+runtime_offset+len(fragment):x})")
    if args.reference is not None:
        print("Exact byte identity against SHA-256-verified reference.")
    print("Partial executable fragment only; NOT a complete, authenticated firmware image.")


if __name__ == "__main__":
    try:
        main()
    except FragmentError as exc:
        raise SystemExit(f"ERROR: {exc}") from exc
