#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Derive two WLAN dispatch tables from authored assembly labels.

This emits only the adjacent 496 DATA bytes at file [0x2f6cd0, 0x2f6ec0).
The source functions at file 0x100b5c and 0x100eb0 are dependencies, not
additional output. No intervening bytes, executable bytes, or MBN container
are emitted. An optional OEM reference is used only after generation, for
exact-revision comparison. The build itself does not require that reference.

Every target is a defined .text symbol at a structurally valid packet start
within its registered function. ELF32 symbols are parsed directly in Python;
no disassembly text or target word array supplies the generated addresses.
ELF symbol layout: https://gabi.xinuos.com/elf/05-symtab.html
Packet framing is provided by wlan_late_packets.py and its primary sources.
"""
from __future__ import annotations

import argparse
from dataclasses import dataclass
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import struct
import sys
import tempfile


ROOT = Path(__file__).resolve().parents[2]
_SPEC = importlib.util.spec_from_file_location(
    "agent14_wlan_late_for_tables", ROOT / "tools/wlanmdsp/build-wlanmdsp-late.py")
assert _SPEC is not None and _SPEC.loader is not None
late = importlib.util.module_from_spec(_SPEC)
sys.modules[_SPEC.name] = late
_SPEC.loader.exec_module(late)
BuildError = late.BuildError

TABLE_FILE_START = 0x2F6CD0
TABLE_FILE_END = 0x2F6EC0
TABLE_LENGTH = TABLE_FILE_END - TABLE_FILE_START
TABLE_SHA256 = "f33f4dae122e666e9eefcfd539eb2405bc967d4928e96a72b3897907144f7390"


@dataclass(frozen=True)
class TableSpec:
    name: str
    file_start: int
    first_type: int
    entry_count: int
    source: str
    entry_symbol: str
    function_file_start: int
    function_file_end: int
    case_types: tuple[int, ...]
    sha256: str
    default_label: str = "restore_status"

    @property
    def size_bytes(self) -> int:
        return 4 * self.entry_count

    @property
    def file_end(self) -> int:
        return self.file_start + self.size_bytes

    @property
    def function_vaddr(self) -> int:
        return late.TEXT_VADDR + self.function_file_start - late.TEXT_FILE_START


# These sets describe the authored handler labels, not firmware address words.
# All other in-range types target restore_status. Both dispatchers initialize
# their status register before jumping, so the default skips return_success's
# separate status assignment. The complete mappings are emitted in the manifest.
TABLES = (
    TableSpec(
        name="short-type-dispatch-table", file_start=0x2F6CD0,
        first_type=0, entry_count=19,
        source="sources/xiaomi-nabu/wlanmdsp/late/short-type-dispatch.S",
        entry_symbol="wlan_late_short_type_dispatch",
        function_file_start=0x100B5C, function_file_end=0x100D78,
        case_types=(0x0000, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006,
                    0x0009, 0x000B, 0x000C, 0x000D, 0x000E, 0x000F,
                    0x0011, 0x0012),
        sha256="7c375e48709b69e965a818eb8fe680cae93f2fbb0283192c640a38669afe646c"),
    TableSpec(
        name="wide-type-dispatch-table", file_start=0x2F6D1C,
        first_type=0x1002, entry_count=105,
        source="sources/xiaomi-nabu/wlanmdsp/late/wide-type-dispatch.S",
        entry_symbol="wlan_late_wide_type_dispatch",
        function_file_start=0x100EB0, function_file_end=0x1012A0,
        case_types=(
            0x1002, 0x1003, 0x1004, 0x1005, 0x1008, 0x1009, 0x100D, 0x100E,
            0x100F, 0x1010, 0x1011, 0x1012, 0x1014, 0x1015, 0x1016, 0x1017,
            0x1018, 0x101A, 0x101E, 0x1020, 0x1021, 0x1022, 0x1023, 0x1024,
            0x1026, 0x1027, 0x1028, 0x102C, 0x102D, 0x1032, 0x1039, 0x103A,
            0x103B, 0x103C, 0x103D, 0x103E, 0x103F, 0x1040, 0x1041, 0x1042,
            0x1044, 0x1045, 0x1047, 0x1048, 0x104A, 0x104D, 0x1053, 0x1054,
            0x1057, 0x1059, 0x1061, 0x1062, 0x106A),
        sha256="33dc7257afa9baecbc797b149fe51e08ee071ed0b319c216682249593310f930"),
)


@dataclass(frozen=True)
class Symbol:
    offset: int
    section_index: int


def read_text_symbols(obj: bytes) -> tuple[dict[str, Symbol], int]:
    """Read a bounded ELF32 relocatable object's single static symbol table.

    Undefined and absolute symbols remain distinguishable from .text labels;
    derive_table rejects them if they are needed as table targets. The input
    must retain its symbol table, rather than being a stripped raw text blob.
    """
    try:
        sections = late.startup.elf32_sections(obj)
    except (late.startup.FragmentError, struct.error) as exc:
        raise BuildError(f"cannot read assembly symbols: {exc}") from exc
    if struct.unpack_from("<H", obj, 16)[0] != 1:
        raise BuildError("assembly symbols require a relocatable ELF object")
    shoff = struct.unpack_from("<I", obj, 32)[0]
    shentsize = struct.unpack_from("<H", obj, 46)[0]
    headers = [struct.unpack_from("<10I", obj, shoff + i * shentsize)
               for i in range(len(sections))]
    text_sections = [s for s in sections if s["name"] == ".text" and s["kind"] == 1]
    if len(text_sections) != 1:
        raise BuildError("symbols require exactly one .text section")
    text_index = int(text_sections[0]["index"])
    text_header = headers[text_index]
    if text_header[3] != 0 or text_header[2] & 6 != 6:
        raise BuildError(".text must be executable, allocated and section-relative")
    symtabs = [s for s in sections if s["kind"] == 2]
    if len(symtabs) != 1:
        raise BuildError("exactly one static symbol table is required")
    symtab = headers[int(symtabs[0]["index"])]
    offset, length, link, first_global, entsize = symtab[4], symtab[5], symtab[6], symtab[7], symtab[9]
    if entsize != 16 or length == 0 or length % 16:
        raise BuildError("malformed ELF32 symbol table entry size")
    if first_global > length // 16:
        raise BuildError("symbol table local/global boundary is out of range")
    if link >= len(headers) or headers[link][1] != 3:
        raise BuildError("symbol table must link to a string table")
    strings = headers[link]
    names = obj[strings[4]:strings[4] + strings[5]]
    if not names or names[0] != 0:
        raise BuildError("symbol string table must begin with an empty name")
    symbols = {}
    for at in range(offset, offset + length, 16):
        name_at, value, _size, _info, _other, section_index = struct.unpack_from("<IIIBBH", obj, at)
        if name_at >= len(names):
            raise BuildError("symbol name points outside its string table")
        name_end = names.find(b"\0", name_at)
        if name_end < 0:
            raise BuildError("unterminated symbol name")
        try:
            name = names[name_at:name_end].decode("ascii")
        except UnicodeDecodeError as exc:
            raise BuildError("non-ASCII symbol name") from exc
        if not name:
            continue
        if name in symbols:
            raise BuildError(f"duplicate symbol name: {name}")
        symbols[name] = Symbol(value, section_index)
    return symbols, text_index


def dependency_for(spec: TableSpec, regions: list) -> late.Region:
    """Require the source registry's exact executable dependency span."""
    matches = [r for r in regions if r.source == spec.source or r.symbol == spec.entry_symbol]
    if len(matches) != 1:
        raise BuildError(f"{spec.name}: missing or ambiguous registered source dependency")
    region = matches[0]
    if (region.source != spec.source or region.symbol != spec.entry_symbol or
            region.file_start != spec.function_file_start or
            region.file_end != spec.function_file_end or region.classification != "executable"):
        raise BuildError(f"{spec.name}: registered source dependency has unexpected bounds or identity")
    return region


def derive_table(spec: TableSpec, text: bytes, symbols: dict[str, Symbol],
                 text_section_index: int) -> tuple[bytes, list[dict]]:
    """Resolve symbolic case mappings to little-endian runtime addresses.

    Packet structure can validate a proposed label target, but cannot prove
    a function boundary; the registered source span supplies that boundary.
    No fall-through offset or firmware data is inferred for a missing symbol.
    """
    if len(text) != spec.function_file_end - spec.function_file_start:
        raise BuildError(f"{spec.name}: source function length differs from its required span")
    if spec.entry_count < 1 or spec.first_type < 0:
        raise BuildError(f"{spec.name}: invalid type range")
    cases = set(spec.case_types)
    if len(cases) != len(spec.case_types) or any(
            not spec.first_type <= tag < spec.first_type + spec.entry_count for tag in cases):
        raise BuildError(f"{spec.name}: invalid explicit case set")
    try:
        starts = {packet.offset for packet in late.packets.parse_packets(text, spec.function_vaddr)}
    except late.packets.ParseError as exc:
        raise BuildError(f"{spec.name}: {exc}") from exc

    def resolve(label: str) -> int:
        if label not in symbols:
            raise BuildError(f"{spec.name}: missing required symbol {label}")
        symbol = symbols[label]
        if symbol.section_index == 0:
            raise BuildError(f"{spec.name}: undefined required symbol {label}")
        if symbol.section_index != text_section_index:
            raise BuildError(f"{spec.name}: {label} is not defined in .text")
        if not 0 <= symbol.offset < len(text):
            raise BuildError(f"{spec.name}: {label} lies outside its source function")
        if symbol.offset not in starts:
            raise BuildError(f"{spec.name}: {label} does not point to a packet start")
        return symbol.offset

    if resolve(spec.entry_symbol) != 0:
        raise BuildError(f"{spec.name}: registered entry symbol must begin the source function")
    data = bytearray()
    mappings = []
    for tag in range(spec.first_type, spec.first_type + spec.entry_count):
        is_default = tag not in cases
        label = spec.default_label if is_default else f"type_{tag:04x}"
        target_offset = resolve(label)
        target_vaddr = spec.function_vaddr + target_offset
        data.extend(struct.pack("<I", target_vaddr))
        mappings.append({"type": tag, "label": label, "target_vaddr": target_vaddr,
                         "target_code_offset": target_offset, "is_default": is_default})
    return bytes(data), mappings


def build(llvm_mc: str, independent_llvm_mc: str | None = None,
          reference: Path | None = None, registry_path: Path = late.REGISTRY) -> tuple[bytes, dict]:
    regions = late.load_registry(registry_path)
    encoded, reports = [], []
    cursor = TABLE_FILE_START
    with tempfile.TemporaryDirectory(prefix="wlan-late-tables-") as temporary:
        directory = Path(temporary)
        for number, spec in enumerate(TABLES):
            if (spec.file_start != cursor or spec.file_start % 4 or
                    not late.OWNED_FILE_START <= spec.file_start < spec.file_end <= late.OWNED_FILE_END):
                raise BuildError("table spans must be aligned, adjacent and inside Agent 14 ownership")
            region = dependency_for(spec, regions)
            source = late._source_bytes(region)
            snapshot = directory / f"dependency-{number}.S"
            snapshot.write_bytes(source)
            obj = directory / f"dependency-{number}.o"
            text = late._assemble(region, snapshot, llvm_mc, "hexagonv66", obj)
            text_digest = hashlib.sha256(text).hexdigest()
            if text_digest != region.sha256:
                raise BuildError(f"{spec.name}: source code SHA-256 differs from its registered digest")
            symbols, text_index = read_text_symbols(obj.read_bytes())
            data, mappings = derive_table(spec, text, symbols, text_index)
            digest = hashlib.sha256(data).hexdigest()
            if digest != spec.sha256:
                raise BuildError(f"{spec.name}: generated table SHA-256 differs from its pinned digest")
            if independent_llvm_mc:
                other_obj = directory / f"independent-{number}.o"
                other_text = late._assemble(region, snapshot, independent_llvm_mc, "hexagonv65", other_obj)
                if other_text != text:
                    raise BuildError(f"{spec.name}: independent assembler source bytes differ")
                other_symbols, other_index = read_text_symbols(other_obj.read_bytes())
                other_data, other_mappings = derive_table(spec, other_text, other_symbols, other_index)
                if other_data != data or other_mappings != mappings:
                    raise BuildError(f"{spec.name}: independent assembler symbolic targets differ")
            vaddr = late.TEXT_VADDR + spec.file_start - late.TEXT_FILE_START
            reports.append({
                "name": spec.name, "file_start": spec.file_start, "file_end": spec.file_end,
                "vaddr_start": vaddr, "vaddr_end": vaddr + len(data),
                "bytes": len(data), "sha256": digest, "classification": "data",
                "entry_width_bytes": 4, "entry_count": spec.entry_count,
                "first_type": spec.first_type, "last_type": spec.first_type + spec.entry_count - 1,
                "explicit_case_count": len(spec.case_types),
                "default_case_count": spec.entry_count - len(spec.case_types),
                "default_label": spec.default_label,
                "source": spec.source, "source_entry_symbol": spec.entry_symbol,
                "source_sha256": hashlib.sha256(source).hexdigest(),
                "source_code_sha256": text_digest,
                "source_registry_checkpoint": region.checkpoint,
                "source_function_file_start": region.file_start,
                "source_function_file_end": region.file_end,
                "source_function_vaddr": region.vaddr,
                "source_function_bytes": len(text),
                "target_packet_starts_checked": True,
                "unique_target_count": len({m["target_vaddr"] for m in mappings}),
                "independent_assembler_checked": bool(independent_llvm_mc),
                "independent_assembler_byte_mismatches": 0 if independent_llvm_mc else None,
                "reference_checked": reference is not None,
                "reference_entry_mismatches": 0 if reference is not None else None,
                "mappings": mappings,
            })
            encoded.append(data)
            cursor = spec.file_end
    data = b"".join(encoded)
    if cursor != TABLE_FILE_END or len(data) != TABLE_LENGTH:
        raise BuildError("generated tables do not cover exactly the required 496-byte data span")
    digest = hashlib.sha256(data).hexdigest()
    if digest != TABLE_SHA256:
        raise BuildError("combined table SHA-256 differs from the pinned digest")
    if reference is not None:
        try:
            reference_bytes = Path(reference).read_bytes()
        except OSError as exc:
            raise BuildError(f"cannot read optional reference: {exc}") from exc
        # This validates the whole pinned revision and exact PT_LOAD, then every
        # byte (therefore all 124 entries) of the independently generated tables.
        late._check_reference(data, reference_bytes, TABLE_FILE_START)
    vaddr = late.TEXT_VADDR + TABLE_FILE_START - late.TEXT_FILE_START
    report = {
        "schema_version": 1, "architecture": "hexagonv66", "complete_firmware": False,
        "artifact_kind": "separate-dispatch-table-data", "classification": "data",
        "file_start": TABLE_FILE_START, "file_end": TABLE_FILE_END,
        "vaddr_start": vaddr, "vaddr_end": vaddr + TABLE_LENGTH,
        "generated_bytes": len(data), "sha256": digest,
        "source_derived_executable_bytes": 0, "source_derived_data_bytes": len(data),
        "data_bytes": len(data), "layout_fill_bytes": 0,
        "external_reference_bytes_in_output": 0,
        "owned_file_start": late.OWNED_FILE_START, "owned_file_end": late.OWNED_FILE_END,
        "emitted_file_ranges": [{"file_start": TABLE_FILE_START, "file_end": TABLE_FILE_END,
                                 "bytes": len(data), "classification": "data"}],
        "coverage_scope": "only the table ranges; dependency code and intervening bytes are not emitted",
        "reference_sha256": late.REFERENCE_SHA256, "reference_checked": reference is not None,
        "reference_byte_mismatches": 0 if reference is not None else None,
        "reference_entry_mismatches": 0 if reference is not None else None,
        "independent_assembler_checked": bool(independent_llvm_mc),
        "independent_assembler_cpu": "hexagonv65" if independent_llvm_mc else None,
        "independent_assembler_byte_mismatches": 0 if independent_llvm_mc else None,
        "target_packet_starts_checked": True, "entry_count": sum(t.entry_count for t in TABLES),
        "verification_scope": "symbolic targets, registered source hashes, packet starts and optional byte equality; hardware behavior remains unverified",
        "tables": reports,
    }
    return data, report


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    parser.add_argument("--llvm-mc", default=os.environ.get("WLAN_LATE_LLVM_MC", "llvm-mc"))
    parser.add_argument("--independent-llvm-mc", default=os.environ.get(
        "WLAN_LATE_SECOND_LLVM_MC", os.environ.get("WLAN_LATE_INDEPENDENT_LLVM_MC")))
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--manifest", type=Path)
    args = parser.parse_args()
    try:
        if args.manifest is not None and args.output.resolve() == args.manifest.resolve():
            raise BuildError("output and manifest must be different files")
        data, report = build(args.llvm_mc, args.independent_llvm_mc, args.reference)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(data)
        if args.manifest:
            args.manifest.parent.mkdir(parents=True, exist_ok=True)
            args.manifest.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    except (BuildError, OSError) as exc:
        parser.exit(1, f"WLAN late table build failed: {exc}\n")
    print(json.dumps({"generated_bytes": report["generated_bytes"],
                      "file_start": report["file_start"], "file_end": report["file_end"],
                      "sha256": report["sha256"], "reference_checked": report["reference_checked"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
