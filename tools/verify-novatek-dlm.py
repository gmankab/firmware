#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent mathematical/CRC verifier for Agent 20-19 Novatek DLM data.

No Rust source or emitted payload is used to generate expected bytes. Schedules
are calculated by record/slot coordinates; Gaussian coefficients use 80-digit
Decimal arithmetic, Decimal.exp, and pi from Machin's arctangent identity.
CRC-32/MPEG-2 uses a byte lookup table, independently of Rust's bit-serial loop.
The precise reference is mandatory for CLI verification. Original firmware
remains an external comparison input and is never copied to generated source.
"""
from __future__ import annotations

import argparse
from decimal import Decimal, ROUND_FLOOR, ROUND_HALF_UP, localcontext
from functools import lru_cache
import hashlib
import json
from pathlib import Path
import struct
import sys

REFERENCE_SHA256 = "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5"
IMAGE_BYTES = 0x30000
DLM_START, DLM_END = 0x22000, 0x2e190
SENTINEL, SLOTS, RECORD_BYTES = 0x7ff, 50, 72
DECIMAL_PRECISION = 80
KERNEL_PROFILES = (
    (0x22488, 284), (0x2252a, 260), (0x225cc, 272), (0x2266e, 284),
    (0x2d7f4, 256), (0x2d896, 256), (0x2d938, 256), (0x2d9da, 260),
    (0x2da7c, 256), (0x2db1e, 268), (0x2dbc0, 252), (0x2dc62, 256),
    (0x2dd04, 252), (0x2dda6, 252), (0x2de48, 252), (0x2deea, 252),
    (0x2df98, 400), (0x2e03a, 800),
)


def pack_record(values):
    """50 low bytes and five base-eight high-bit groups, with reserved zeros."""
    if len(values) != SLOTS or any(type(v) is not int or not 0 <= v <= SENTINEL for v in values):
        raise ValueError("record requires 50 unsigned 11-bit integers")
    low = bytes(v % 256 for v in values) + bytes(2)
    high = b"".join(struct.pack("<I", sum((values[j + k] // 256) * 8**k
                                         for k in range(10)))
                    for j in range(0, SLOTS, 10))
    return low + high


def unpack_record(raw):
    if len(raw) != RECORD_BYTES or raw[50:52] != bytes(2):
        raise ValueError("record length or reserved alignment bytes are invalid")
    words = struct.unpack_from("<5I", raw, 52)
    if any(w >= 1 << 30 for w in words):
        raise ValueError("nonzero reserved high-word bits")
    return [raw[i] + ((words[i // 10] // 8**(i % 10)) % 8) * 256 for i in range(SLOTS)]


def wide_schedule(row, translated=False):
    """24 packed records; live 48*32-1 ring positions translate by 16."""
    if type(row) is not int or not 0 <= row < 24:
        raise ValueError("wide schedule row must be 0..23")
    if row in (0, 19):
        values = [49 - i for i in range(SLOTS)]
    elif row == 1:
        values = [74 - i for i in range(SLOTS)]
    elif row in (2, 3):
        values = [(49 - i) * 32 for i in range(SLOTS)]
    elif row == 4:
        values = [(49 - 2 * (i // 2)) * 32 + i % 2 for i in range(SLOTS)]
    elif row in (5, 6):
        values = [((48 if row == 5 else 49) - 2 * (i // 2)) * 32 + 1 - i % 2
                  for i in range(SLOTS)]
    elif row < 18:
        origin = 115 + 65 * ((row - 7) // 2) if row % 2 else 1340 - 65 * ((row - 8) // 2)
        values = [(origin - 32 * i) % 1535 for i in range(SLOTS)]
        for i in range(SLOTS):
            if (row == 16 and i >= 34) or (row == 17 and i < 14):
                values[i] = SENTINEL
            elif not translated:
                values[i] -= 16
    elif row == 18:
        values = [99 - i for i in range(SLOTS)]
    elif row == 20:
        values = [SENTINEL] * SLOTS
    elif row == 21:
        values = [SENTINEL] * 30 + [0] * 20
    elif row == 22:
        values = [(5 - i % 6) * 16 + 7 - i // 6 if i < 48 else SENTINEL for i in range(SLOTS)]
    else:
        values = [98 - 2 * i for i in range(SLOTS)]
    return values


def narrow_schedule(row, translated=False):
    """20 packed records; 48*30-1 ring positions and explicit disabled slots."""
    if type(row) is not int or not 0 <= row < 20:
        raise ValueError("narrow schedule row must be 0..19")
    if row == 0:
        return [74 - i for i in range(SLOTS)]
    if row == 1:
        return [49 - i for i in range(SLOTS)]
    if row == 2 or (translated and row >= 18):
        return [(49 - i) * 32 for i in range(SLOTS)]
    translation = 15 if translated else 0
    if row == 3:
        return [1410 - 30 * i + translation if i < 48 else SENTINEL for i in range(SLOTS)]
    if row < 18:
        lane = (row - 4) // 2
        origin = 1363 - 61 * lane if row % 2 == 0 else 32 + 61 * lane
        return [SENTINEL if ((row == 14 and 36 <= i < 48) or (row == 16 and i >= 22)
                             or (row == 17 and (i < 14 or i >= 26)))
                else (origin - 30 * i) % 1439 + translation for i in range(SLOTS)]
    return [(95 if row == 18 else 47) - i if i < 48 else SENTINEL for i in range(SLOTS)]


@lru_cache(maxsize=1)
def decimal_pi():
    """Machin: pi = 16 atan(1/5) - 4 atan(1/239), with ten guard digits."""
    with localcontext() as ctx:
        ctx.prec = DECIMAL_PRECISION + 10

        def atan_inverse(denominator):
            x = Decimal(1) / denominator
            term, total, index = x, x, 1
            while True:
                term *= -x * x
                updated = total + term / (2 * index + 1)
                if updated == total:
                    return total
                total = updated
                index += 1

        value = 16 * atan_inverse(5) - 4 * atan_inverse(239)
        ctx.prec = DECIMAL_PRECISION
        return +value


@lru_cache(maxsize=32)
def gaussian_kernel(sigma_tenths):
    """Return the 162 emitted bytes and their nearest rounding-boundary margin."""
    if type(sigma_tenths) is not int or not 1 <= sigma_tenths <= 65535:
        raise ValueError("sigma_tenths must be a positive u16")
    with localcontext() as ctx:
        ctx.prec = DECIMAL_PRECISION
        sigma = Decimal(sigma_tenths) / 10
        peak = Decimal(400000) / (sigma * (2 * decimal_pi()).sqrt())
        if peak.to_integral_value(rounding=ROUND_HALF_UP) > 65535:
            raise ValueError("Gaussian center coefficient exceeds u16")
        samples, minimum, closest_x = [], Decimal("Infinity"), None
        for x in range(80, -1, -1):
            value = peak * (-Decimal(x * x) / (2 * sigma * sigma)).exp()
            rounded = value.to_integral_value(rounding=ROUND_HALF_UP)
            distance = abs(value - value.to_integral_value(rounding=ROUND_FLOOR) - Decimal("0.5"))
            if distance < minimum:
                minimum, closest_x = distance, x
            samples.append(int(rounded))
        return struct.pack("<81H", *samples), minimum, closest_x


def generated_fragments():
    fragments = []
    for start, name, rows, generator, translated in (
        (0x24904, "initial-index-walks", 24, wide_schedule, False),
        (0x25d4c, "narrow-index-walks", 20, narrow_schedule, False),
        (0x28904, "translated-wide-index-walks", 24, wide_schedule, True),
        (0x29d4c, "translated-narrow-index-walks", 20, narrow_schedule, True),
    ):
        fragments.append((start, name, b"".join(pack_record(generator(row, translated)) for row in range(rows))))
    fragments.extend((start, "gaussian-half-kernel", gaussian_kernel(sigma)[0]) for start, sigma in KERNEL_PROFILES)
    fragments.append((0x2e130, "controller-idle-mailboxes", struct.pack("<II", 0x12345678, 0x12345678)))
    for descriptor_start, schedule_start in ((0x22a64, 0x24904), (0x26a64, 0x28904)):
        # NDS32 consumer masks record+0x50 to 15 bits and indexes DLM
        # as index*4+0x20000. Higher 16 bits are deliberately unclaimed.
        offset = schedule_start - DLM_START
        if offset < 0 or offset % 4 or offset//4 > 0x7fff:
            raise ValueError("invalid DLM schedule link")
        for record in range(17):
            fragments.append((descriptor_start+0x50+record*0x8c,
                             "wide-schedule-word-link",struct.pack("<H",offset//4)))
    fragments.sort()
    cursor = DLM_START
    for start, _, data in fragments:
        if start < cursor or not data or start + len(data) > DLM_END:
            raise ValueError("generated fragment overlap or invalid DLM extent")
        cursor = start + len(data)
    return fragments


def _crc_table():
    values = []
    for top_byte in range(256):
        word = top_byte << 24
        for _ in range(8):
            word = ((word << 1) ^ (0x04c11db7 if word & 0x80000000 else 0)) & 0xffffffff
        values.append(word)
    return tuple(values)


CRC_TABLE = _crc_table()


def crc32_mpeg2(data):
    crc = 0xffffffff
    for value in data:
        crc = ((crc << 8) & 0xffffffff) ^ CRC_TABLE[(crc >> 24) ^ value]
    return crc


def check_envelope(image):
    """Check transport structure and 12 CRC words; this does not decompile ILM."""
    if len(image) != IMAGE_BYTES:
        raise ValueError("incorrect Novatek image size")
    if image[-3:] != b"NVT":
        raise ValueError("missing NVT marker")
    word = lambda offset: struct.unpack_from("<I", image, offset)[0]
    checked = []

    def scalar(offset, expected):
        if word(offset) != expected:
            raise ValueError(f"profile scalar mismatch at 0x{offset:x}")

    def checksum(offset, start, length):
        if not 0 <= start < start + length <= len(image):
            raise ValueError("CRC range outside image")
        expected = crc32_mpeg2(image[start:start + length])
        if word(offset) != expected:
            raise ValueError(f"CRC mismatch at 0x{offset:x}")
        checked.append((offset, expected))

    for bank, offsets in enumerate(((0, 0xa00, 0x3200), (0, 0x4a00, 0x7200))):
        base = 0x100 * bank
        for index, expected in enumerate((0x200, 0, 0x1aa7f, 0x22000, 0x20000, 0xc18f)):
            scalar(base + 4 * index, expected)
        scalar(base + 0x20, 3 if bank == 0 else 1)
        scalar(base + 0x24, 0x538110e0 if bank == 0 else 0)
        for start, end in ((base + 0x28, base + 0x30), (base + 0x60, base + 0xf0)):
            if any(image[start:end]):
                raise ValueError("nonzero reserved metadata")
        checksum(base + 0x18, 0x200, 0x1aa80)
        checksum(base + 0x1c, DLM_START, DLM_END - DLM_START)
        for index, offset in enumerate(offsets):
            entry = base + 0x30 + 16 * index
            scalar(entry, 0x20000 + offset)
            scalar(entry + 4, 0)
            scalar(entry + 8, DLM_START + offset)
            checksum(entry + 12, DLM_START + offset, 1)
        scalar(base + 0xf0, 0x2c190 + base)
        scalar(base + 0xf4, 0xff)
        scalar(base + 0xf8, base)
        checksum(base + 0xfc, base, 0xfc)
    for offset in range(0x204, 0x400, 4):
        if word(offset) >= 0x1aa80:
            raise ValueError(f"candidate vector outside ILM at 0x{offset:x}")
    for start, end in ((0x400, 0x450), (0x1ac80, 0x22000), (0x2e190, 0x2f000), (0x2f900, 0x2fffd)):
        if any(image[start:end]):
            raise ValueError(f"nonzero reserved envelope at 0x{start:x}")
    if image[0x2f000:0x2f900] != image[0x22000:0x22900]:
        raise ValueError("DLM mirror mismatch")
    return checked


def fragment_metadata(fragment):
    start, name, data = fragment
    end = start + len(data)
    return {"name": name, "file": f"novatek-dlm-{start:06x}-{end:06x}.bin",
            "file_start": start, "file_end": end, "sram_start": start - 0x2000,
            "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}


EXPECTED_COUNTS = {
    "source_layout_bytes": 35136, "prior_source_executable_bytes": 538,
    "new_executable_bytes": 0, "source_dlm_data_bytes": 9328,
    "external_metadata_bytes": 1024, "unknown_ilm_bytes": 108054,
    "unknown_dlm_bytes": 40224, "derived_mirror_bytes": 2304,
    "mirror_from_source_bytes": 648, "mirror_from_external_bytes": 1656,
    "independent_unique_bytes": 45002, "total_bytes": 196608,
    "first_unknown_dlm": DLM_START, "crc_stored_fields_checked": 0,
}


def expected_regions(fragments):
    """Independent physical partition; mirrors retain their upstream category."""
    def row(start, end, kind):
        return {"file_start": start, "file_end": end, "kind": kind}

    regions = [row(0, 0x200, "external_metadata"), row(0x200, 0x400, "external_metadata"),
               row(0x400, 0x450, "source_layout"), row(0x450, 0x66a, "source_executable"),
               row(0x66a, 0x1ac80, "unknown_ilm"), row(0x1ac80, 0x22000, "source_layout")]
    cursor, data_regions = DLM_START, []
    for start, _, data in fragments:
        if not cursor <= start < start + len(data) <= DLM_END:
            raise ValueError("fragment order or DLM geometry invalid for regional ledger")
        if start > cursor:
            data_regions.append(row(cursor, start, "unknown_dlm"))
        data_regions.append(row(start, start + len(data), "source_dlm_data"))
        cursor = start + len(data)
    if cursor < DLM_END:
        data_regions.append(row(cursor, DLM_END, "unknown_dlm"))
    regions.extend(data_regions)
    regions.append(row(DLM_END, 0x2f000, "source_layout"))
    for source in data_regions:
        start, end = max(DLM_START, source["file_start"]), min(0x22900, source["file_end"])
        if start >= end:
            continue
        mirror = row(0x2f000 + start - DLM_START, 0x2f000 + end - DLM_START, "derived_mirror")
        mirror.update(source_file_start=start, source_file_end=end,
                      source_independent=source["kind"] == "source_dlm_data")
        regions.append(mirror)
    regions.extend((row(0x2f900, 0x2fffd, "source_layout"), row(0x2fffd, IMAGE_BYTES, "source_layout")))
    return regions


def check_manifest(directory, fragments):
    directory = Path(directory)
    manifest = json.loads((directory / "manifest.json").read_text())
    for key, expected in {"schema": "agent20-19-novatek-dlm-v1", "profile": "postmarketos-nabu",
                          "reference_sha256": REFERENCE_SHA256, "reference_verified": False,
                          "flashable": False, "mismatched_source_bytes": None, **EXPECTED_COUNTS}.items():
        if key not in manifest or type(manifest[key]) is not type(expected) or manifest[key] != expected:
            raise ValueError(f"manifest {key} disagrees with independent accounting")
    actual = manifest.get("fragments")
    if not isinstance(actual, list) or len(actual) != len(fragments):
        raise ValueError("manifest fragment count mismatch")
    for entry, fragment in zip(actual, fragments):
        expected = fragment_metadata(fragment)
        if not isinstance(entry, dict) or any(type(entry.get(key)) is not type(value)
                                               or entry.get(key) != value for key, value in expected.items()):
            raise ValueError(f"manifest fragment extent/hash mismatch at 0x{fragment[0]:x}")
        if (directory / expected["file"]).read_bytes() != fragment[2]:
            raise ValueError(f"emitted fragment byte mismatch at 0x{fragment[0]:x}")
    actual_regions, wanted_regions = manifest.get("regions"), expected_regions(fragments)
    if not isinstance(actual_regions, list) or len(actual_regions) != len(wanted_regions):
        raise ValueError("manifest regional partition count mismatch")
    mirror_fields = ("source_file_start", "source_file_end", "source_independent")
    for actual, wanted in zip(actual_regions, wanted_regions):
        if not isinstance(actual, dict) or any(type(actual.get(key)) is not type(value)
                                               or actual.get(key) != value for key, value in wanted.items()):
            raise ValueError(f"manifest regional partition/provenance mismatch at 0x{wanted['file_start']:x}")
        if wanted["kind"] != "derived_mirror" and any(key in actual for key in mirror_fields):
            raise ValueError("manifest non-mirror region declares a mirrored dependency")
        if not isinstance(actual.get("evidence"), str) or not actual["evidence"].strip():
            raise ValueError("manifest regional evidence is missing")


def verify_reference(reference, fragments):
    reference = bytes(reference)
    if hashlib.sha256(reference).hexdigest() != REFERENCE_SHA256:
        raise ValueError("reference SHA-256 mismatch; use the pinned postmarketOS revision")
    checks = check_envelope(reference)
    for start, _, data in fragments:
        if reference[start:start + len(data)] != data:
            mismatch = next(i for i, (left, right) in enumerate(zip(data, reference[start:])) if left != right)
            raise ValueError(f"independent recipe mismatch at file 0x{start + mismatch:x}")
    return checks


def verification_report(reference_path, emitted_directory):
    fragments = generated_fragments()
    checks = verify_reference(Path(reference_path).read_bytes(), fragments)
    check_manifest(emitted_directory, fragments)
    margin, sigma, x = min((gaussian_kernel(s)[1], s, gaussian_kernel(s)[2])
                           for s in {sigma for _, sigma in KERNEL_PROFILES})
    return {
        "reference_sha256": REFERENCE_SHA256, "reference_verified": True,
        "independent_data_bytes": sum(len(data) for _, _, data in fragments),
        "fragment_count": len(fragments), "schedule_records": 88,
        "gaussian_tables": len(KERNEL_PROFILES), "gaussian_coefficients": 1458,
        "decimal_precision": DECIMAL_PRECISION,
        "minimum_half_integer_distance": str(margin),
        "closest_sigma_tenths": sigma, "closest_sample_x": x,
        "crc_stored_fields_checked": len(checks), "mirror_bytes_checked": 2304,
        "mismatched_source_bytes": 0, "rust_output_matches": True,
        "concatenated_fragment_sha256": hashlib.sha256(b"".join(data for _, _, data in fragments)).hexdigest(),
        "fragments": [fragment_metadata(fragment) for fragment in fragments],
    }


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", type=Path, required=True)
    parser.add_argument("--emitted-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        print(json.dumps(verification_report(args.reference, args.emitted_dir), indent=2))
    except (OSError, ValueError, TypeError, KeyError) as exc:
        print(f"verify-novatek-dlm: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
