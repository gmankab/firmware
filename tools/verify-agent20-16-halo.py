#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify Agent16 against private firmware and the pinned, unmodified Cirrus converter.

Does not fetch inputs or save proprietary payloads. Pass a separately obtained
vendor converter file. JSON output contains hashes/counts, not firmware data.
"""
import argparse
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import struct
import tempfile

REFERENCE_SHA256 = "d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c"
CONVERTER_SHA256 = "921b84b0c860b5b3cbc5e6596a9bc236e0d9ee40d92ae39de10cebdfcd1264cd"
CONVERTER_COMMIT = "a05b2878be911ae9ffaa440840cde17af98ba48e"


def verify(reference, binary, converter):
    raw = reference.read_bytes()
    if hashlib.sha256(raw).hexdigest() != REFERENCE_SHA256:
        raise ValueError("reference SHA-256 mismatch")
    if hashlib.sha256(converter.read_bytes()).hexdigest() != CONVERTER_SHA256:
        raise ValueError("untrusted or modified Cirrus converter")
    spec = importlib.util.spec_from_file_location("agent16_pinned_cirrus_converter", converter)
    vendor = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(vendor)
    report = json.loads(subprocess.check_output([str(binary), "verify", str(reference)], text=True))
    decoded = json.loads(subprocess.check_output([str(binary), "decode-xy", str(reference)], text=True))
    # Derive descriptors independently from the actual WMFW file, rather than
    # trusting offsets or lengths reported by the implementation under test.
    expected_regions = []
    cursor = struct.unpack_from("<I", raw, 4)[0]
    while cursor < len(raw):
        descriptor, length = struct.unpack_from("<II", raw, cursor)
        kind, address = descriptor >> 24, descriptor & 0xffffff
        payload = cursor + 8
        if payload + length > len(raw):
            raise ValueError("WMFW descriptor exceeds pinned file")
        if kind in (0x11, 0x12):
            if length % 12 or address % 4:
                raise ValueError("WMFW packed-data alignment mismatch")
            expected_regions.append((kind, payload, address, length // 3))
        cursor = payload + length
    actual_regions = [(r["kind"], r["file_offset"], r["dsp_word_offset"], len(r["words"]))
                      for r in decoded["regions"]]
    if actual_regions != expected_regions or cursor != len(raw):
        raise ValueError("Rust XM/YM descriptors or coverage disagree with WMFW")
    total_words = 0
    total_bytes = 0
    for region in decoded["regions"]:
        at = region["file_offset"]
        count = len(region["words"])
        payload = raw[at:at + count * 3]
        official = vendor.unpack_memory(io.BytesIO(payload), True, "p32", "u24")
        if official != region["words"]:
            raise ValueError(f"independent Cirrus decode mismatch at {at:#x}")
        total_words += count
        total_bytes += len(payload)
    if (len(decoded["regions"]), total_words, total_bytes) != (6, 3324, 9972):
        raise ValueError("unexpected complete XM/YM coverage")
    directory = vendor.halo_firmware_id_block("p32")
    directory.parse(io.BytesIO(raw[0x68:0xb0]), 72, io.BytesIO())
    if directory.fields["firmware_id"] != 0x400a4 or directory.fields["number_of_algorithms"] != 2:
        raise ValueError("vendor static directory interpretation mismatch")
    with tempfile.TemporaryDirectory(prefix="agent20-16-source-") as temporary:
        generated = Path(temporary) / "static-directory.bin"
        subprocess.run([str(binary), "emit-static-directory", str(generated)], check=True)
        source_bytes = generated.read_bytes()
        if source_bytes != raw[0x68:0xb0]:
            raise ValueError("static directory byte mismatch")
        # A forged reference must fail even if its reconstructed metadata agrees.
        changed = bytearray(raw)
        changed[0xb8] ^= 1
        bad = Path(temporary) / "invalid-private-reference.wmfw"
        bad.write_bytes(changed)
        rejected = subprocess.run([str(binary), "verify", str(bad)], capture_output=True)
        if rejected.returncode == 0 or b"SHA-256" not in rejected.stderr:
            raise ValueError("modified opaque input was accepted")
    report.update({
        "independent_converter_commit": CONVERTER_COMMIT,
        "independent_converter_sha256": CONVERTER_SHA256,
        "independently_decoded_xy_regions": 6,
        "independently_decoded_xy_words": total_words,
        "independently_decoded_xy_transport_bytes": total_bytes,
        "decode_disagreements": 0,
        "descriptor_address_disagreements": 0,
        "modified_reference_rejected": True,
        "decoded_external_bytes_are_source_reconstruction": False,
    })
    return report


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("reference", type=Path)
    p.add_argument("binary", type=Path)
    p.add_argument("cirrus_converter", type=Path)
    a = p.parse_args()
    print(json.dumps(verify(a.reference.resolve(), a.binary.resolve(), a.cirrus_converter.resolve()), indent=2))


if __name__ == "__main__":
    main()
