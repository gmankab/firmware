#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify named initializers, descriptor addresses and the complete provenance union.

Uses the unchanged, SHA-pinned Cirrus converter independently for memory
values, metadata names/offsets and the static directory. All original firmware
and temporary full repacks remain caller-supplied private inputs.
"""
import argparse
from collections import Counter
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import struct
import subprocess
import tempfile


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def verify(reference, binary, converter):
    previous = load_module("agent16_previous_audit", Path(__file__).with_name("verify-agent20-16-halo.py"))
    baseline = previous.verify(reference, binary, converter)
    raw = reference.read_bytes()
    vendor = load_module("agent16_vendor_controls", converter)
    metadata = {}
    metadata_bytes = set(range(struct.unpack_from("<I", raw, 4)[0]))
    locations = {"X": {}, "Y": {}}
    cursor = struct.unpack_from("<I", raw, 4)[0]
    while cursor < len(raw):
        descriptor, size = struct.unpack_from("<II", raw, cursor)
        kind, word_start = descriptor >> 24, descriptor & 0xffffff
        start = cursor + 8
        metadata_bytes.update(range(cursor, start))
        if kind in (0xff, 0xf2):
            metadata_bytes.update(range(start, start + size))
        if kind in (0x11, 0x12):
            memory = "X" if kind == 0x11 else "Y"
            words = vendor.unpack_memory(io.BytesIO(raw[start:start + size]), True, "p32", "u24")
            for i, value in enumerate(words):
                # Derive physical byte positions via register endianness,
                # independently of the Rust module's explicit lane table.
                offsets = [start + ((i * 3 + j) // 4) * 4 + 3 - (i * 3 + j) % 4 for j in range(3)]
                if word_start + i in locations[memory]:
                    raise ValueError("overlapping source memory regions")
                locations[memory][word_start + i] = (value, offsets)
        elif kind == 0xf2:
            stream = io.BytesIO(raw)
            stream.seek(cursor)
            block = vendor.wmfw_algorithm_information_data_block(stream, 3)
            block.parse(stream)
            for control in block.fields["coefficient_descriptors"]:
                f = control.fields
                identity = (block.fields["algorithm_id"], f["coefficient_name"])
                if identity in metadata:
                    raise ValueError("duplicate reference control")
                metadata[identity] = (f["type"], f["start_offset"], f["control_length"])
        cursor = start + size
    directory = vendor.halo_firmware_id_block("p32")
    directory.parse(io.BytesIO(raw[0x68:0xb0]), 72, io.BytesIO())
    d = directory.fields
    allocations = {d["firmware_id"]: d["sys_config_mem_offsets"]}
    allocations.update({a["algorithm_id"]: a["algorithm_offsets"] for a in d["algorithm_info"]})

    def invoke(*args):
        return json.loads(subprocess.check_output([str(binary), *args], text=True))

    controls = invoke("control-defaults")["controls"]
    actual = invoke("control-bytes")["bytes"]
    if len(controls) != 57 or len(metadata) != 57:
        raise ValueError("named control count mismatch")
    expected_bytes = {}
    seen = set()
    for control in controls:
        identity = (control["algorithm_id"], control["name"])
        if identity in seen:
            raise ValueError("duplicate generated named control")
        seen.add(identity)
        kind, relative, length = metadata[identity]
        memory = {5: "X", 6: "Y"}[kind]
        base = allocations[identity[0]][memory.lower() + "m_base"]
        count = allocations[identity[0]][memory.lower() + "m_size"]
        if (control["memory"], control["relative_word_offset"], len(control["values"]) * 4,
                control["static_dsp_word_address"]) != (memory, relative, length, base + relative):
            raise ValueError(f"metadata/static allocation disagreement for {identity}")
        if relative + length // 4 > count:
            raise ValueError("control exceeds static directory allocation")
        for i, value in enumerate(control["values"]):
            original, offsets = locations[memory][base + relative + i]
            if value != original:
                raise ValueError(f"named initializer mismatch for {identity}, word {i}")
            for j, offset in enumerate(offsets):
                if offset in expected_bytes:
                    raise ValueError("generated named controls overlap")
                expected_bytes[offset] = (value >> (j * 8)) & 255
    if seen != set(metadata) or actual != [[k, v] for k, v in sorted(expected_bytes.items())]:
        raise ValueError("generated file-byte mapping is incomplete or incorrect")
    if len(expected_bytes) != 792 or any(raw[k] != v for k, v in expected_bytes.items()):
        raise ValueError("generated initializer byte comparison failed")
    ranges = []
    for offset in sorted(expected_bytes):
        if ranges and ranges[-1][1] == offset:
            ranges[-1][1] += 1
        else:
            ranges.append([offset, offset + 1])
    overlap = sum(any(0xf90 + a <= k < 0xf90 + b for a, b in [(45, 348), (669, 6114)])
                  for k in expected_bytes)
    if overlap != 297:
        raise ValueError("old-zero overlap accounting mismatch")
    ledger = invoke("verify-memory", str(reference))
    expected_origins = ["external_opaque_dsp"] * len(raw)
    for offset in metadata_bytes:
        expected_origins[offset] = "source_authored_container_metadata"
    for offset in range(0x68, 0xb0):
        expected_origins[offset] = "reference_derived_static_directory"
    for offset in range(0xac, 0xaf):
        expected_origins[offset] = "explicit_directory_padding"
    for offset in expected_bytes:
        if expected_origins[offset] != "external_opaque_dsp":
            raise ValueError("named control overlaps a metadata/directory byte")
        expected_origins[offset] = "reference_observed_control_initialization"
    for a, b in [(45, 348), (669, 6114)]:
        for offset in range(0xf90 + a, 0xf90 + b):
            if raw[offset] != 0:
                raise ValueError("declared observed-zero byte is nonzero")
            if expected_origins[offset] == "external_opaque_dsp":
                expected_origins[offset] = "reference_observed_zero"
            elif expected_origins[offset] != "reference_observed_control_initialization":
                raise ValueError("unexpected zero/metadata overlap")
    expected_counts = dict(Counter(expected_origins))
    if expected_counts != {
        "source_authored_container_metadata": 3984, "reference_derived_static_directory": 69,
        "explicit_directory_padding": 3, "reference_observed_control_initialization": 792,
        "reference_observed_zero": 5451, "external_opaque_dsp": 23937,
    }:
        raise ValueError("independent byte-origin totals disagree")
    for origin, count in expected_counts.items():
        if ledger[origin + "_bytes"] != count:
            raise ValueError(f"provenance summary count disagrees for {origin}")
    expected_summary = {
        "total_bytes": len(raw), "known_bytes": 10299,
        "zero_initialization_overlap_bytes": overlap,
        "external_pm_bytes": 20280, "external_xy_bytes": 3657,
        "external_part_count": 17, "first_unreconstructed_file_offset": 0xb8,
        "independently_reconstructed_halo_instruction_bytes": 0,
        "live_runtime_dump_supplied": False, "hardware_register_addresses_asserted": False,
    }
    if any(ledger[key] != value for key, value in expected_summary.items()):
        raise ValueError("provenance summary disagrees with the independent audit")
    cursor = 0
    for span in ledger["spans"]:
        start, end, origin = span["start"], span["end"], span["origin"]
        if start != cursor or end <= start or end > len(raw):
            raise ValueError("provenance ledger contains a gap/overlap")
        if expected_origins[start:end] != [origin] * (end - start):
            raise ValueError(f"provenance byte categories disagree at {start:#x}")
        cursor = end
    if cursor != len(raw):
        raise ValueError("provenance ledger does not cover the complete file")
    with tempfile.TemporaryDirectory(prefix="agent20-16-memory-audit-") as temporary:
        root = Path(temporary)
        inputs, output = root / "inputs", root / "external-repacked.wmfw"
        subprocess.run([str(binary), "extract-memory-external", str(reference), str(inputs)], check=True)
        files = sorted(inputs.iterdir())
        external_bytes = sum(f.stat().st_size for f in files)
        if len(files) != 17 or external_bytes != 23937:
            raise ValueError("unexpected external source total")
        subprocess.run([str(binary), "assemble-memory-external", str(inputs), str(output)], check=True)
        if output.read_bytes() != raw:
            raise ValueError("full external repack mismatch")
        saved = files[0].read_bytes()
        corrupt = bytearray(saved)
        corrupt[0] ^= 1
        files[0].write_bytes(corrupt)
        bad_output = root / "must-not-exist.wmfw"
        rejected = subprocess.run([str(binary), "assemble-memory-external", str(inputs), str(bad_output)], capture_output=True)
        if rejected.returncode == 0 or bad_output.exists() or b"SHA-256" not in rejected.stderr:
            raise ValueError("corrupt same-size external input was not rejected")
        files[0].unlink()
        missing = subprocess.run([str(binary), "assemble-memory-external", str(inputs), str(bad_output)], capture_output=True)
        if missing.returncode == 0 or bad_output.exists():
            raise ValueError("missing external input was not rejected")
    return {
        "baseline_directory_and_transport_verification": baseline,
        "named_controls": 57, "logical_initializer_words": 264, "generated_initializer_bytes": 792,
        "generated_initializer_spans": ranges, "byte_mismatches": 0,
        "ordered_initializer_bytes_sha256": hashlib.sha256(bytes(v for _, v in sorted(expected_bytes.items()))).hexdigest(),
        "initializer_hash_is_contiguous_firmware_span": False,
        "independent_metadata_and_static_address_disagreements": 0,
        "overlap_with_previous_zero_bytes": overlap, "new_coverage_beyond_previous_zeros": 792 - overlap,
        "full_coverage_ledger": ledger,
        "independent_provenance_counts": expected_counts,
        "external_repack": {"files": len(files), "input_bytes": external_bytes, "output_bytes": len(raw),
                            "byte_mismatches": 0, "sha256": hashlib.sha256(raw).hexdigest()},
        "same_size_corrupt_external_fragment_rejected": True, "missing_external_fragment_rejected": True,
        "live_runtime_dump_supplied": False, "independently_reconstructed_halo_instruction_bytes": 0,
    }


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("reference", type=Path)
    p.add_argument("binary", type=Path)
    p.add_argument("cirrus_converter", type=Path)
    a = p.parse_args()
    print(json.dumps(verify(a.reference.resolve(), a.binary.resolve(), a.cirrus_converter.resolve()), indent=2))


if __name__ == "__main__":
    main()
