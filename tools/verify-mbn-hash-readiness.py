#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify source-only per-program-header SHA-384 readiness against pinned MBNs.

Rust's `examples/mbn_hash_readiness.rs` emits a reference-independent JSONL
ledger. This optional checker verifies each ready digest using the *exact*
source-pinned original image and independently parses ELF32 program headers.
It never imports OEM bytes into the generated source or counts missing bytes
as reconstructed. Original signature consistency is checked separately by
`verify-mbn-signatures.py`, not by this digest-only script.
"""

import argparse
import hashlib
import json
from pathlib import Path
import struct
import sys

NAMES = ("a640_zap.mbn", "adsp.mbn", "cdsp.mbn", "modem.mbn", "venus.mbn", "wlanmdsp.mbn")
MAGIC = b"\x7fELF\x01\x01\x01"


class ReadinessError(ValueError):
    """Invalid source-hash ledger, input layout or pinned revision."""


def as_uint(value, description):
    if type(value) is not int or value < 0:
        raise ReadinessError(f"{description}: expected unsigned integer")
    return value


def check_range(value, total, label, allow_empty=False):
    if not isinstance(value, list) or len(value) != 2:
        raise ReadinessError(f"{label}: expected [start,end]")
    start, end = [as_uint(x, label) for x in value]
    if end > total or start > end or (not allow_empty and start == end):
        raise ReadinessError(f"{label}: invalid bounded range")
    return start, end


def read_elf_headers(image):
    if len(image) < 52 or image[:7] != MAGIC:
        raise ReadinessError("not supported little-endian ELF32")
    ehdr = struct.unpack_from("<16sHHIIIIIHHHHHH", image)
    if ehdr[3] != 1 or ehdr[8] != 52 or ehdr[9] != 32:
        raise ReadinessError("invalid ELF32 geometry")
    offset, count = ehdr[5], ehdr[10]
    if offset < 52 or not 1 <= count <= 256 or offset + count * 32 > len(image):
        raise ReadinessError("ELF32 program table out of bounds")
    headers = [struct.unpack_from("<8I", image, offset + i * 32) for i in range(count)]
    auth = [i for i, ph in enumerate(headers) if ph[6] & 0x0700_0000 == 0x0200_0000]
    if len(auth) != 1:
        raise ReadinessError("expected exactly one hash/auth segment")
    for i, ph in enumerate(headers):
        if ph[0] == 1 and ph[4] > ph[5]:
            raise ReadinessError(f"PT_LOAD {i}: p_filesz exceeds p_memsz")
        if ph[4] and ph[1] + ph[4] > len(image):
            raise ReadinessError(f"file-backed ELF segment {i} exceeds image")
    return headers, auth[0]


def verify_image_report(report, image, pinned_digest):
    if type(report) is not dict or report.get("image") not in NAMES:
        raise ReadinessError("invalid or unknown image name")
    name = report["image"]
    if hashlib.sha256(image).hexdigest() != pinned_digest:
        raise ReadinessError(f"{name}: wrong pinned SHA-256 reference revision")
    if as_uint(report.get("file_bytes"), "file_bytes") != len(image):
        raise ReadinessError(f"{name}: file size mismatch")
    headers, hash_index = read_elf_headers(image)
    rows = report.get("segments")
    if not isinstance(rows, list) or len(rows) != len(headers):
        raise ReadinessError(f"{name}: missing or duplicated program-header entry")
    ready = skipped = pending = total_ready_bytes = 0
    for i, (row, ph) in enumerate(zip(rows, headers)):
        if not isinstance(row, dict) or as_uint(row.get("index"), "segment index") != i:
            raise ReadinessError(f"{name}: altered or duplicate program-header index {i}")
        if row.get("kind") != ph[0]:
            raise ReadinessError(f"{name}: program-header kind mismatch at {i}")
        expected = [ph[1], ph[1] + ph[4]]
        if row.get("file_range") != expected:
            raise ReadinessError(f"{name}: segment geometry mismatch at {i}")
        status = row.get("status")
        placeholder = i == hash_index or ph[4] == 0
        if placeholder:
            if status != "placeholder" or "sha384" in row:
                raise ReadinessError(f"{name}: nonzero hash placeholder at {i}")
            skipped += 1
        elif status == "ready":
            digest = row.get("sha384")
            if not isinstance(digest, str) or len(digest) != 96 or any(x not in "0123456789abcdef" for x in digest):
                raise ReadinessError(f"{name}: malformed source SHA-384 at {i}")
            expected_digest = hashlib.sha384(image[ph[1]:ph[1] + ph[4]]).hexdigest()
            if digest != expected_digest:
                raise ReadinessError(f"{name}: source digest does not match pinned executable/data segment {i}")
            ready += 1
            total_ready_bytes += ph[4]
        elif status == "pending":
            first = check_range(row.get("first_missing"), len(image), f"{name} first missing segment {i}")
            if first[0] < ph[1] or first[1] > ph[1] + ph[4]:
                raise ReadinessError(f"{name}: pending hole outside segment {i}")
            missing = as_uint(row.get("missing_bytes"), "pending missing bytes")
            if missing < first[1] - first[0] or missing > ph[4]:
                raise ReadinessError(f"{name}: implausible missing byte count at {i}")
            pending += 1
        else:
            raise ReadinessError(f"{name}: unknown hash-readiness status at {i}")
    for attr, expected in (("ready_entries", ready), ("pending_entries", pending),
                           ("zero_placeholders", skipped), ("ready_hashed_bytes", total_ready_bytes)):
        if as_uint(report.get(attr), attr) != expected:
            raise ReadinessError(f"{name}: inconsistent total {attr}")
    if type(report.get("all_input_hashes_ready")) is not bool or report["all_input_hashes_ready"] != (pending == 0):
        raise ReadinessError(f"{name}: invalid all_input_hashes_ready flag")
    return {"image": name, "source_ready_segments": ready, "ready_hashed_file_bytes": total_ready_bytes,
            "pending_segments": pending, "zero_placeholders": skipped,
            "reference_sha256": pinned_digest, "all_ready_source_hashes_match": True,
            "reference_only_verification": True, "hardware_authentication": False}


def verify_reports(rows, reference_dir, pins):
    if not isinstance(rows, list) or len(rows) != len(NAMES):
        raise ReadinessError("report must contain all six images exactly once")
    reported = [x.get("image") if isinstance(x, dict) else None for x in rows]
    if reported != list(NAMES) or set(pins) != set(NAMES):
        raise ReadinessError("images must be in pinned order without duplicates")
    out = []
    for report in rows:
        image = (Path(reference_dir) / report["image"]).read_bytes()
        out.append(verify_image_report(report, image, pins[report["image"]]))
    return out


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_report", type=Path, help="six newline-delimited JSON source-readiness rows")
    parser.add_argument("reference_dir", type=Path, help="private exact original six-image directory")
    parser.add_argument("--pins", type=Path,
                        default=Path(__file__).resolve().parents[1] / "reference/xiaomi-nabu/firmware.sha256")
    args = parser.parse_args(argv)
    try:
        from importlib.util import spec_from_file_location, module_from_spec
        # The existing pinned-auditor validates the full SHA-256 manifest.
        spec = spec_from_file_location("mbn_revisions", Path(__file__).with_name("audit-nabu-mbn-sources.py"))
        auditor = module_from_spec(spec); spec.loader.exec_module(auditor)
        pins = auditor.pinned_sha256s(args.pins)
        rows = [json.loads(line) for line in args.source_report.read_text().splitlines() if line.strip()]
        results = verify_reports(rows, args.reference_dir, pins)
    except (OSError, ValueError, ReadinessError, json.JSONDecodeError) as err:
        parser.exit(1, f"FAIL: {err}\n")
    for result in results:
        print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    print("NOTICE: hashes compare generated source with ORIGINAL pinned ELF segments only; no new signed image or device trust is implied.", file=sys.stderr)


if __name__ == "__main__":
    main()
