#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build bounded Agent 20-14 WLAN code without reading OEM bytes as input.

Optional reference and second-toolchain checks are reported separately. A
single output is allowed only for adjacent registered regions; uncovered
leading/trailing ranges are recorded, never filled. No signed MBN is emitted.
"""
from __future__ import annotations

import argparse
from dataclasses import dataclass
import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import re
import struct
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parents[2]
REGISTRY = ROOT / "sources/xiaomi-nabu/wlanmdsp/late/regions.json"
OWNED_FILE_START = 0x100000
OWNED_FILE_END = 0x353D38
TEXT_FILE_START = 0x40000
TEXT_VADDR = 0xB0000000
TEXT_LENGTH = 0x313D38
REFERENCE_SHA256 = "87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b"
SOURCE_DIRECTORY = PurePosixPath("sources/xiaomi-nabu/wlanmdsp")


def _import(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


startup = _import("agent14_wlan_startup", ROOT / "tools/wlanmdsp/build-wlanmdsp-startup.py")
packets = _import("wlan_late_packets", ROOT / "tools/wlanmdsp/late_packets.py")


class BuildError(ValueError):
    """A registry, reconstruction or optional verification failed."""


@dataclass(frozen=True)
class Region:
    name: str
    file_start: int
    file_end: int
    source: str
    symbol: str
    sha256: str
    checkpoint: int
    classification: str
    # For rare packets where LLVM's shuffler pairs a different duplex,
    # explicitly join independently assembled mini-packets using ISA parse
    # bits, not private raw instructions. Each is (byte_offset, source path).
    packet_joins: tuple[tuple[int, str], ...] = ()

    @property
    def size_bytes(self) -> int:
        return self.file_end - self.file_start

    @property
    def vaddr(self) -> int:
        return TEXT_VADDR + self.file_start - TEXT_FILE_START


def _integer(value, label: str) -> int:
    if type(value) is not int:
        raise BuildError(f"{label} must be an integer")
    return value


def validate_registry(payload: dict, root: Path = ROOT) -> list[Region]:
    """Validate fixed ownership, revision binding, paths and nonoverlap.

    Gaps are valid registry facts. select_regions refuses to collapse them
    into a single output, because that would lose their file positions.
    """
    if not isinstance(payload, dict):
        raise BuildError("registry must be a JSON object")
    for key, expected in (("schema_version", 1),
                          ("owned_file_start", OWNED_FILE_START),
                          ("owned_file_end", OWNED_FILE_END)):
        if _integer(payload.get(key), key) != expected:
            raise BuildError(f"registry {key} must be {expected}")
    if payload.get("reference_sha256") != REFERENCE_SHA256:
        raise BuildError("registry reference_sha256 differs from the pinned WLAN revision")
    entries = payload.get("regions")
    if not isinstance(entries, list) or not entries:
        raise BuildError("registry regions must be a nonempty array")
    root = Path(root).resolve()
    source_directory = root.joinpath(*SOURCE_DIRECTORY.parts).resolve()
    regions = []
    names, symbols = set(), set()
    previous_end = OWNED_FILE_START
    for number, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise BuildError(f"region {number} must be an object")
        name, symbol = entry.get("name"), entry.get("symbol")
        if not isinstance(name, str) or not name or name in names:
            raise BuildError("region names must be nonempty and unique")
        if (not isinstance(symbol, str) or
                not re.fullmatch(r"[A-Za-z_.$][A-Za-z0-9_.$]*", symbol) or symbol in symbols):
            raise BuildError("region symbols must be valid and unique")
        start = _integer(entry.get("file_start"), f"{name}.file_start")
        end = _integer(entry.get("file_end"), f"{name}.file_end")
        if not OWNED_FILE_START <= start < end <= OWNED_FILE_END:
            raise BuildError(f"{name}: span is outside Agent 14 ownership or empty")
        if start % 4 or end % 4:
            raise BuildError(f"{name}: file boundaries must be four-byte aligned")
        if start < previous_end:
            raise BuildError(f"{name}: regions are unsorted or overlap")
        checkpoint = _integer(entry.get("checkpoint"), f"{name}.checkpoint")
        if checkpoint < 1:
            raise BuildError(f"{name}: checkpoint must be positive")
        if entry.get("classification") != "executable":
            raise BuildError(f"{name}: only executable source is supported")
        digest, source = entry.get("sha256"), entry.get("source")
        if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-f]{64}", digest):
            raise BuildError(f"{name}: sha256 must be 64 lowercase hexadecimal characters")
        if not isinstance(source, str) or "\\" in source:
            raise BuildError(f"{name}: source must be a repository-relative path")
        relative = PurePosixPath(source)
        if relative.is_absolute() or ".." in relative.parts or relative.suffix != ".S":
            raise BuildError(f"{name}: unsafe source path")
        resolved = root.joinpath(*relative.parts).resolve()
        if not resolved.is_relative_to(source_directory):
            raise BuildError(f"{name}: source is outside the WLAN source directory")
        joins = entry.get("packet_joins", [])
        if not isinstance(joins, list):
            raise BuildError(f"{name}: packet_joins must be a list")
        validated_joins = []
        prev_join_end = 0
        for join in joins:
            if not isinstance(join, dict) or set(join) != {"offset", "fixture"}:
                raise BuildError(f"{name}: packet_join must contain offset and fixture")
            offset = _integer(join["offset"], f"{name}.packet_join.offset")
            if offset < prev_join_end or offset % 4 or offset + 12 > end - start:
                raise BuildError(f"{name}: packet join overlaps or leaves the region")
            fixture = join["fixture"]
            if not isinstance(fixture, str) or "\\" in fixture:
                raise BuildError(f"{name}: fixture path must be relative")
            fp = PurePosixPath(fixture)
            if fp.is_absolute() or ".." in fp.parts or fp.suffix != ".S":
                raise BuildError(f"{name}: unsafe fixture path")
            if not root.joinpath(*fp.parts).resolve().is_relative_to(source_directory):
                raise BuildError(f"{name}: packet fixture leaves owned source directory")
            prev_join_end = offset + 12
            validated_joins.append((offset, fixture))
        regions.append(Region(name, start, end, source, symbol, digest,
                              checkpoint, "executable", tuple(validated_joins)))
        names.add(name)
        symbols.add(symbol)
        previous_end = end
    return regions


def _unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise BuildError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load_registry(path: Path = REGISTRY) -> list[Region]:
    try:
        payload = json.loads(Path(path).read_text(), object_pairs_hook=_unique_object)
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise BuildError(f"cannot read region registry: {exc}") from exc
    return validate_registry(payload)


def select_regions(regions: list[Region], through_checkpoint: int | None = None) -> list[Region]:
    if through_checkpoint is not None:
        if _integer(through_checkpoint, "through_checkpoint") < 1:
            raise BuildError("through_checkpoint must be positive")
        selected = [r for r in regions if r.checkpoint <= through_checkpoint]
    else:
        selected = list(regions)
    if not selected:
        raise BuildError("no regions selected")
    for left, right in zip(selected, selected[1:]):
        if left.file_end != right.file_start:
            raise BuildError("selected regions must be contiguous; a gap/overlap cannot be filled or concatenated")
    return selected


def _run(command: list[str]) -> subprocess.CompletedProcess:
    try:
        result = subprocess.run(command, check=False, capture_output=True, text=True, timeout=60)
    except (OSError, subprocess.TimeoutExpired) as exc:
        raise BuildError(f"tool failed: {command[0]}: {exc}") from exc
    if result.returncode:
        raise BuildError(f"tool failed: {command[0]}: {result.stderr[:2000]}")
    if re.search(r"not (?:a )?recognized|unrecognized (?:processor|cpu)|invalid (?:processor|cpu)",
                 result.stderr, re.IGNORECASE):
        raise BuildError(f"tool rejected the requested CPU: {result.stderr[:1000]}")
    return result


def _source_bytes(region: Region) -> bytes:
    try:
        source = (ROOT / region.source).read_bytes()
        text = source.decode("utf-8")
    except (OSError, UnicodeError) as exc:
        raise BuildError(f"{region.name}: cannot read source: {exc}") from exc
    # These code-only sources use no data, alignment, includes, macros, or raw
    # opcode directives. Reject other directives instead of hiding their bytes
    # under the executable count. Instruction mnemonics are not whitelisted.
    uncommented = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
    uncommented = re.sub(r"//[^\n]*", "", uncommented)
    allowed = {"text", "globl", "global", "type", "size", "equ", "equiv", "set"}
    for directive in re.findall(r"(?:^|;)\s*\.([A-Za-z0-9_]+)", uncommented, re.MULTILINE):
        if directive not in allowed:
            raise BuildError(f"{region.name}: non-code directive .{directive} is not permitted")
    if not re.search(r"^\s*" + re.escape(region.symbol) + r"\s*:", uncommented, re.MULTILINE):
        raise BuildError(f"{region.name}: registered entry symbol is missing from source")
    return source


def _typed_three_word_packet(fixture: Path, llvm_mc: str, cpu: str,
                             obj: Path) -> bytes:
    """Build 2 full A32-style Hexagon words plus a duplex from source.

    Hexagon packet parse flags encode boundaries in word bits 15:14:
    01 = continuation, 00 = final duplex, 11 = final ordinary. We assemble
    each *named mnemonic mini-packet*, require 11,11,00 and convert only
    their two independent packet terminators into continuation markers.
    This is a structural ISA operation, not an OEM opcode-byte override.
    """
    try:
        text = fixture.read_text()
    except (OSError, UnicodeError) as exc:
        raise BuildError(f"cannot read packet source fixture {fixture}: {exc}") from exc
    without_comments = re.sub(r"//[^\n]*", "", text)
    if re.search(r"(?:^|;)\s*\.(?:byte|word|long|short|incbin|fill|org)\b",
                 without_comments, re.MULTILINE):
        raise BuildError("packet fixture contains a prohibited raw-data directive")
    _run([str(llvm_mc), "-triple=hexagon", f"-mcpu={cpu}",
          "-filetype=obj", "-o", str(obj), str(fixture)])
    try:
        source_words = startup.read_resolved_text(obj.read_bytes(), 12)
    except (OSError, startup.FragmentError) as exc:
        raise BuildError(f"cannot assemble named packet fixture: {exc}") from exc
    words = struct.unpack("<3I", source_words)
    if [(w >> 14) & 3 for w in words] != [3, 3, 0]:
        raise BuildError("packet fixture must encode two ordinary minis and a duplex")
    return struct.pack("<3I", (words[0] & ~0xC000) | 0x4000,
                       (words[1] & ~0xC000) | 0x4000, words[2])


def _assemble(region: Region, source: Path, llvm_mc: str, cpu: str,
              obj: Path) -> bytes:
    _run([str(llvm_mc), "-triple=hexagon", f"-mcpu={cpu}", "-filetype=obj",
          "-o", str(obj), str(source)])
    try:
        assembled = startup.read_resolved_text(obj.read_bytes(), region.size_bytes)
    except (OSError, startup.FragmentError) as exc:
        raise BuildError(f"{region.name}: {exc}") from exc
    for number, (offset, fixture_name) in enumerate(region.packet_joins):
        fixture = ROOT.joinpath(*PurePosixPath(fixture_name).parts)
        mini_obj = obj.parent / f"{obj.stem}-join{number}.o"
        combined = _typed_three_word_packet(fixture, llvm_mc, cpu, mini_obj)
        # Prohibit a broken shuffler result that shifts later function bytes.
        if len(assembled[offset:offset+12]) != len(combined):
            raise BuildError(f"{region.name}: explicit packet width differs")
        assembled = assembled[:offset] + combined + assembled[offset+12:]
    return assembled


def _decode(obj: Path, llvm_objdump: str, length: int) -> int:
    result = _run([str(llvm_objdump), "--triple=hexagon", "--mcpu=hexagonv66",
                   "-d", "--section=.text", str(obj)])
    if re.search(r"<unknown>|<invalid>|(?:^|\s)\.(?:word|long)\b",
                 result.stdout, re.IGNORECASE | re.MULTILINE):
        raise BuildError("v66 decoder reported an unknown or invalid instruction")
    addresses = [int(match, 16) for match in re.findall(
        r"^\s*([0-9a-fA-F]+):\s+(?:[0-9a-fA-F]{2}\s+){3}[0-9a-fA-F]{2}\s+",
        result.stdout, re.MULTILINE)]
    if addresses != list(range(0, length, 4)):
        raise BuildError("v66 decoder did not account for every encoded word")
    return len(addresses)


def _check_reference(data: bytes, reference: bytes, file_start: int) -> None:
    try:
        startup.check_reference(data, reference, runtime_offset=file_start - TEXT_FILE_START)
        phoff = struct.unpack_from("<I", reference, 28)[0]
        entsize, count = struct.unpack_from("<HH", reference, 42)
        headers = [struct.unpack_from("<8I", reference, phoff + i * entsize) for i in range(count)]
        expected = (1, TEXT_FILE_START, TEXT_VADDR, TEXT_VADDR, TEXT_LENGTH, TEXT_LENGTH, 5, 0x40000)
        if sum(p == expected for p in headers) != 1:
            raise BuildError("reference has an unexpected WLAN executable PT_LOAD mapping")
    except (startup.FragmentError, struct.error) as exc:
        raise BuildError(str(exc)) from exc


def _stats(parsed) -> dict:
    return {"packet_count": len(parsed),
            "encoded_word_count": sum(p.word_count for p in parsed),
            "duplex_packet_count": sum(p.end_kind == "duplex" for p in parsed),
            "constant_extender_word_count": sum(p.extender_count for p in parsed),
            "loop0_end_packet_count": sum(p.loop0 for p in parsed),
            "loop1_end_packet_count": sum(p.loop1 for p in parsed)}


def build(llvm_mc: str, reference: Path | None = None,
          through_checkpoint: int | None = None,
          independent_llvm_mc: str | None = None, independent_cpu: str = "hexagonv65",
          llvm_objdump: str | None = None, registry_path: Path = REGISTRY,
          cpu: str = "hexagonv66") -> tuple[bytes, dict]:
    selected = select_regions(load_registry(registry_path), through_checkpoint)
    encoded = []
    details = []
    with tempfile.TemporaryDirectory(prefix="wlan-late-build-") as temporary:
        directory = Path(temporary)
        for index, region in enumerate(selected):
            source = _source_bytes(region)
            # Assemble a fixed snapshot: the source hash records precisely the
            # bytes passed to both toolchains, even if the working tree changes.
            snapshot = directory / f"region-{index}.S"
            snapshot.write_bytes(source)
            obj = directory / f"region-{index}.o"
            fragment = _assemble(region, snapshot, llvm_mc, cpu, obj)
            digest = hashlib.sha256(fragment).hexdigest()
            if digest != region.sha256:
                raise BuildError(f"{region.name}: generated SHA-256 differs from the registered digest")
            try:
                framing = packets.parse_packets(fragment, region.vaddr)
            except packets.ParseError as exc:
                raise BuildError(f"{region.name}: {exc}") from exc
            if independent_llvm_mc:
                other = _assemble(region, snapshot, independent_llvm_mc, independent_cpu,
                                  directory / f"independent-{index}.o")
                if other != fragment:
                    mismatch_count = sum(a != b for a, b in zip(other, fragment))
                    raise BuildError(f"{region.name}: independent assembler differs in {mismatch_count} bytes")
            decoded_words = _decode(obj, llvm_objdump, len(fragment)) if llvm_objdump else None
            details.append({"name": region.name, "file_start": region.file_start,
                            "file_end": region.file_end, "vaddr_start": region.vaddr,
                            "vaddr_end": region.vaddr + region.size_bytes,
                            "bytes": len(fragment), "sha256": digest,
                            "source": region.source, "symbol": region.symbol,
                            "source_sha256": hashlib.sha256(source).hexdigest(),
                            "packet_joins": [
                                {"offset": ofs, "fixture": fname,
                                 "fixture_sha256": hashlib.sha256(
                                     ROOT.joinpath(*PurePosixPath(fname).parts).read_bytes()
                                 ).hexdigest()}
                                for ofs, fname in region.packet_joins],
                            "checkpoint": region.checkpoint, "classification": "executable",
                            **_stats(framing), "decoder_word_count": decoded_words})
            encoded.append(fragment)
    data = b"".join(encoded)
    first, end = selected[0].file_start, selected[-1].file_end
    if reference is not None:
        try:
            reference_bytes = Path(reference).read_bytes()
        except OSError as exc:
            raise BuildError(f"cannot read optional reference: {exc}") from exc
        _check_reference(data, reference_bytes, first)
        expected = reference_bytes[first:end]
        if packets.packet_signature(packets.parse_packets(data)) != packets.packet_signature(
                packets.parse_packets(expected)):
            raise BuildError("reference packet framing differs")
    gaps = [{"file_start": a, "file_end": b, "bytes": b - a}
            for a, b in ((OWNED_FILE_START, first), (end, OWNED_FILE_END)) if a < b]
    parsed = packets.parse_packets(data, TEXT_VADDR + first - TEXT_FILE_START)
    manifest = {
        "schema_version": 1, "architecture": "hexagonv66",
        "assembler_cpu": cpu, "complete_firmware": False,
        "file_start": first, "file_end": end,
        "vaddr_start": TEXT_VADDR + first - TEXT_FILE_START,
        "vaddr_end": TEXT_VADDR + end - TEXT_FILE_START,
        "generated_bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(),
        "source_derived_executable_bytes": len(data), "data_bytes": 0,
        "layout_fill_bytes": 0, "external_reference_bytes_in_output": 0,
        "owned_file_start": OWNED_FILE_START, "owned_file_end": OWNED_FILE_END,
        "scope_gaps": gaps,
        "next_uncovered_file_offset": gaps[0]["file_start"] if gaps else None,
        "next_continuation_file_offset": end if end < OWNED_FILE_END else None,
        "reference_sha256": REFERENCE_SHA256, "reference_checked": reference is not None,
        "reference_byte_mismatches": 0 if reference is not None else None,
        "independent_assembler_checked": bool(independent_llvm_mc),
        "independent_assembler_cpu": independent_cpu if independent_llvm_mc else None,
        "independent_assembler_byte_mismatches": 0 if independent_llvm_mc else None,
        "decoder_checked": bool(llvm_objdump),
        "decoder_unknown_instruction_count": 0 if llvm_objdump else None,
        "verification_scope": "byte equality and packet structure; hardware behavior remains unverified",
        "through_checkpoint": through_checkpoint,
        **_stats(parsed), "regions": details,
    }
    return data, manifest


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    parser.add_argument("--llvm-mc", default=os.environ.get("WLAN_LATE_LLVM_MC", "llvm-mc"))
    parser.add_argument("--cpu", choices=("hexagonv65", "hexagonv66"), default="hexagonv66",
                        help="Assembler subtarget; the pinned firmware remains V66")
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--through-checkpoint", type=int)
    parser.add_argument("--independent-llvm-mc", default=os.environ.get("WLAN_LATE_SECOND_LLVM_MC", os.environ.get("WLAN_LATE_INDEPENDENT_LLVM_MC")))
    parser.add_argument("--independent-cpu", default="hexagonv65")
    parser.add_argument("--llvm-objdump", default=os.environ.get("WLAN_LATE_LLVM_OBJDUMP"))
    args = parser.parse_args()
    try:
        if args.manifest is not None and args.output.resolve() == args.manifest.resolve():
            raise BuildError("output and manifest must be different files")
        data, report = build(args.llvm_mc, args.reference, args.through_checkpoint,
                             args.independent_llvm_mc, args.independent_cpu, args.llvm_objdump,
                             cpu=args.cpu)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(data)
        if args.manifest:
            args.manifest.parent.mkdir(parents=True, exist_ok=True)
            args.manifest.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    except (BuildError, OSError) as exc:
        parser.exit(1, f"WLAN late build failed: {exc}\n")
    print(json.dumps({"generated_bytes": report["generated_bytes"],
                      "file_start": report["file_start"], "file_end": report["file_end"],
                      "sha256": report["sha256"], "reference_checked": report["reference_checked"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
