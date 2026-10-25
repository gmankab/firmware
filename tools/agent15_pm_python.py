#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Python audit of CS35L41 HALO packed-program *transport*.

No PM instruction encoder/decoder is implemented. Output contains counts and
hashes only; OEM program words are never embedded in the project or reports.
"""
from __future__ import annotations

import argparse
import collections
import hashlib
import json
import struct
from pathlib import Path

WMFW_LENGTH = 34236
WMFW_SHA256 = "d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c"
PM_SHA256 = "0edacef35965ea2bd8c709f8cad73fd08714bb47922065d25e3b9ba1bf34c517"
PM_START = 0x278C
PM_END = 0x76C4
BASE = 0x03800000
PM_REGISTER_END = 0x03804F38
PM_BYTES = PM_END - PM_START
WORDS = PM_BYTES // 5
BLOCK_BYTES = 20


def words_to_wire(words: tuple[int, int, int, int]) -> bytes:
    if len(words) != 4 or any(not 0 <= w < (1 << 40) for w in words):
        raise ValueError("four bounded 40-bit storage words required")
    return b"".join(w.to_bytes(5, "big") for w in words)


def wire_to_words(block: bytes) -> tuple[int, int, int, int]:
    if len(block) != BLOCK_BYTES:
        raise ValueError("packed PM block must have exactly 20 bytes")
    return tuple(int.from_bytes(block[i:i + 5], "big") for i in range(0, 20, 5))


def wire_to_host_registers(block: bytes) -> tuple[int, int, int, int, int]:
    if len(block) != BLOCK_BYTES:
        raise ValueError("packed PM block must have exactly 20 bytes")
    return struct.unpack(">5I", block)


def host_registers_to_wire(regs: tuple[int, int, int, int, int]) -> bytes:
    if len(regs) != 5 or any(not 0 <= r < (1 << 32) for r in regs):
        raise ValueError("five bounded 32-bit host register values required")
    return struct.pack(">5I", *regs)


def byte_address_for_pm_word(word_index: int) -> tuple[int, int, int]:
    if not 0 <= word_index < WORDS:
        raise ValueError("program-memory word index out of assigned range")
    offset = word_index * 5
    first = BASE + (offset & ~3)
    last = BASE + ((offset + 4) & ~3)
    if last >= PM_REGISTER_END:
        raise ValueError("host register beyond end of assigned PM region")
    return first, offset % 4, last


def parse_wmfw_header(blob: bytes) -> list[tuple[int, int, int, int]]:
    if len(blob) != WMFW_LENGTH or blob[:4] != b"WMFW":
        raise ValueError("not the expected WMFW container")
    if struct.unpack_from("<I", blob, 4)[0] != 40 or blob[10:12] != b"\x04\x03":
        raise ValueError("expected HALO core 4, WMFW format 3, header 40")
    pos = 40
    records = []
    while pos < len(blob):
        if pos + 8 > len(blob):
            raise ValueError("truncated WMFW region descriptor")
        descriptor, length = struct.unpack_from("<II", blob, pos)
        nxt = pos + 8 + length
        if nxt > len(blob):
            raise ValueError("WMFW region exceeds image")
        records.append((pos, descriptor >> 24, descriptor & 0xFFFFFF, length))
        pos = nxt
    if len(records) != 11:
        raise ValueError("wrong number of WMFW image regions")
    pm = [(start, address, count) for start, kind, address, count in records if kind == 0x10]
    if pm != [(PM_START - 8, 0, PM_BYTES)]:
        raise ValueError("PM region location differs from pinned reference")
    return records


def audit_blob(blob: bytes) -> dict[str, object]:
    parse_wmfw_header(blob)
    digest = hashlib.sha256(blob).hexdigest()
    if digest != WMFW_SHA256:
        raise ValueError("full WMFW SHA-256 differs from pinned reference")
    pm = blob[PM_START:PM_END]
    pm_hash = hashlib.sha256(pm).hexdigest()
    if len(pm) != PM_BYTES or pm_hash != PM_SHA256:
        raise ValueError("program-memory region SHA-256 differs from pinned reference")
    reconstructed = bytearray()
    host_write_count = 0
    word_frequencies: collections.Counter[int] = collections.Counter()
    for i, part in enumerate(range(0, len(pm), BLOCK_BYTES)):
        block = pm[part:part + BLOCK_BYTES]
        words = wire_to_words(block)
        word_frequencies.update(words)
        if words_to_wire(words) != block:
            raise ValueError(f"five-byte word roundtrip mismatch in block {i}")
        regs = wire_to_host_registers(block)
        if host_registers_to_wire(regs) != block:
            raise ValueError(f"AHB register serialization mismatch in block {i}")
        for reg in regs:
            expected_reg_addr = BASE + host_write_count * 4
            if expected_reg_addr >= PM_REGISTER_END or not 0 <= reg <= 0xFFFFFFFF:
                raise ValueError("host register access outside window")
            reconstructed.extend(reg.to_bytes(4, "big"))
            host_write_count += 1
    if bytes(reconstructed) != pm or host_write_count != 5070:
        raise ValueError("PM transport comparison failed")
    if (byte_address_for_pm_word(WORDS - 1)[2] != PM_REGISTER_END - 4 or
            word_frequencies[0] != 29 or len(word_frequencies) != 3701):
        raise ValueError("pinned PM inventory differs")
    return {
        "firmware_sha256": digest,
        "pm_payload_sha256": pm_hash,
        "pm_file_start": "0x278c",
        "pm_file_end_exclusive": "0x76c4",
        "dsp_pm_address_start": 0,
        "pm_payload_bytes": PM_BYTES,
        "40bit_storage_words": WORDS,
        "20byte_transport_blocks": PM_BYTES // BLOCK_BYTES,
        "32bit_host_writes": host_write_count,
        "register_base": "0x03800000",
        "register_end_exclusive": "0x03804f38",
        "distinct_storage_words": len(word_frequencies),
        "all_zero_storage_words": word_frequencies[0],
        "transport_byte_mismatches": 0,
        "independently_rebuilt_executable_bytes": 0,
        "unresolved_instruction_isa": True,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("reference_wmfw", type=Path)
    parser.add_argument("--json-out", type=Path)
    args = parser.parse_args()
    if args.json_out is not None and args.json_out.resolve() == args.reference_wmfw.resolve():
        parser.error("refusing to overwrite the reference WMFW")
    report = audit_blob(args.reference_wmfw.read_bytes())
    printed = json.dumps(report, indent=2) + "\n"
    print(printed, end="")
    if args.json_out is not None:
        args.json_out.write_text(printed, encoding="utf-8")


if __name__ == "__main__":
    main()
