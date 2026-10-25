# SPDX-License-Identifier: GPL-3.0-only
"""Reproducibility and provenance gate tests; fixtures contain no OEM bytes."""

import contextlib
import copy
import importlib.util
import io
import json
import os
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("mbn_reproducibility", ROOT / "tools" / "mbn-reproducibility.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def fixture_probe(folder):
    """Source-export fixture with expected geometry; bytes are synthetic."""
    records = []
    for image_index, name in enumerate(MODULE.IMAGES):
        total = MODULE.FILE_BYTES[name]
        segments = [
            {"index": 0, "type": 0, "start": 0, "file_bytes": 148,
             "memory_bytes": 0, "virtual_address": 0, "physical_address": 0, "flags": 0x07000000},
            {"index": 1, "type": 0, "start": 0x1000, "file_bytes": 0x1A38,
             "memory_bytes": 0x2000, "virtual_address": 0x6000, "physical_address": 0x6000,
             "flags": 0x02200000},
            {"index": 2, "type": 1, "start": 0x3000, "file_bytes": total - 0x3000,
             "memory_bytes": total - 0x3000 + (0 if image_index == 0 else 4096),
             "virtual_address": 0x5000, "physical_address": 0x5000, "flags": 7},
        ]
        spans = [[0, 0x1000], [0x1000, 0x1030], [0x1030, 0x10A8]]
        if name == "a640_zap.mbn":
            spans += [[0x10A8, 0x1138], [0x2A38, 0x3000], [0x3000, 0x37B0]]
        records.append({"image": name, "file_bytes": total,
                        "generated_bytes": sum(end - start for start, end in spans),
                        "missing_bytes": total - sum(end - start for start, end in spans),
                        "source_complete": False, "full_image_emission_refused": True,
                        "holes": MODULE.complement(spans, total), "regions": [], "segments": segments})
        provenance = [{"start": 0, "end": 148, "kind": "source_metadata"},
                      {"start": 148, "end": 0x1000, "kind": "explicit_layout_fill"},
                      {"start": 0x1000, "end": 0x10A8, "kind": "source_metadata"}]
        if name == "a640_zap.mbn":
            provenance += [{"start": 0x10A8, "end": 0x1138, "kind": "source_metadata"},
                           {"start": 0x2A38, "end": 0x3000, "kind": "explicit_layout_fill"},
                           {"start": 0x3000, "end": 0x3004, "kind": "source_metadata"},
                           {"start": 0x3004, "end": 0x3638, "kind": "source_code"},
                           {"start": 0x3638, "end": 0x3640, "kind": "explicit_layout_fill"},
                           {"start": 0x3640, "end": 0x37B0, "kind": "source_code"}]
        records[-1]["provenance_regions"] = provenance
        directory = Path(folder) / name
        directory.mkdir()
        for index, (start, end) in enumerate(spans):
            filename = f"region-{start:08x}-{end:08x}.bin"
            (directory / filename).write_bytes(bytes([index + 1]) * (end - start))
            records[-1]["regions"].append({"start": start, "end": end, "file": filename})
        inv = MODULE.segment_inventory(segments, total)
        records[-1]["strict_geometry"] = {
            **inv["disjoint_load_permission_bytes"],
            "unassigned_file_ranges": inv["unassigned_file_ranges"],
            "memory_only_bytes": inv["memory_only_pt_load_bytes_sum"],
            "authentication_file_range": [0x1138, 0x2A38],
            "hash_file_range": [0x10A8, 0x1138],
        }
    return records


class SourceOutputTests(unittest.TestCase):
    def test_source_spans_hashes_and_holes_partition_the_pinned_files(self):
        with tempfile.TemporaryDirectory() as directory:
            reports = MODULE.validate_probe_output(json.dumps(fixture_probe(directory)), directory)
        first = reports[0]
        self.assertEqual(first["generated_bytes"], 7856)
        self.assertEqual(first["missing_bytes"], 6400)
        self.assertEqual(first["holes"], [[0x1138, 0x2A38]])
        self.assertEqual(first["generated_byte_categories"], {
            "code": 1956, "data": 0, "metadata_and_hashes": 464,
            "explicit_layout_padding": 5436, "external_authentication": 0})
        self.assertFalse(first["executable_permissions_are_instruction_count"])
        self.assertFalse(first["semantics_fully_validated"])
        self.assertTrue(first["full_image_emission_refused"])
        self.assertTrue(all(not image["source_complete"] for image in reports))
        self.assertTrue(all(image["generated_bytes"] == 4264 for image in reports[1:]))
        self.assertTrue(all(len(region["sha256"]) == 64 for image in reports for region in image["source_regions"]))

    def test_missing_hole_and_forged_complete_flag_are_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            records = fixture_probe(directory)
            for field, value in (("holes", []), ("source_complete", True), ("full_image_emission_refused", False)):
                changed = copy.deepcopy(records)
                changed[0][field] = value
                with self.subTest(field=field), self.assertRaises(MODULE.ReproducibilityError):
                    MODULE.validate_probe_output(json.dumps(changed), directory)

    def test_duplicate_overlap_and_out_of_bounds_regions_are_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            records = fixture_probe(directory)
            changed = copy.deepcopy(records)
            changed[0]["regions"].insert(1, copy.deepcopy(changed[0]["regions"][0]))
            with self.assertRaisesRegex(MODULE.ReproducibilityError, "overlapping"):
                MODULE.validate_probe_output(json.dumps(changed), directory)
            changed = copy.deepcopy(records)
            changed[0]["regions"][-1]["end"] += 1
            with self.assertRaisesRegex(MODULE.ReproducibilityError, "extent"):
                MODULE.validate_probe_output(json.dumps(changed), directory)

    def test_region_filename_traversal_and_wrong_length_are_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            records = fixture_probe(directory)
            changed = copy.deepcopy(records)
            changed[0]["regions"][0]["file"] = "../outside.bin"
            with self.assertRaisesRegex(MODULE.ReproducibilityError, "noncanonical"):
                MODULE.validate_probe_output(json.dumps(changed), directory)
            path = Path(directory) / MODULE.IMAGES[0] / records[0]["regions"][0]["file"]
            path.write_bytes(b"short")
            with self.assertRaisesRegex(MODULE.ReproducibilityError, "byte length"):
                MODULE.validate_probe_output(json.dumps(records), directory)

    def test_cross_language_geometry_disagreement_is_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            records = fixture_probe(directory)
            records[0]["strict_geometry"]["executable_and_writable"] -= 1
            with self.assertRaisesRegex(MODULE.ReproducibilityError, "geometry disagree"):
                MODULE.validate_probe_output(json.dumps(records), directory)

    def test_provenance_must_cover_exact_source_bytes_without_external_claims(self):
        with tempfile.TemporaryDirectory() as directory:
            records = fixture_probe(directory)
            changed = copy.deepcopy(records)
            changed[0]["provenance_regions"][0]["end"] -= 1
            with self.assertRaisesRegex(MODULE.ReproducibilityError, "different bytes"):
                MODULE.validate_probe_output(json.dumps(changed), directory)
            changed = copy.deepcopy(records)
            changed[0]["provenance_regions"][0]["kind"] = "external_authentication"
            with self.assertRaisesRegex(MODULE.ReproducibilityError, "external"):
                MODULE.validate_probe_output(json.dumps(changed), directory)

    def test_malformed_records_are_reported_as_gate_failure(self):
        for value in (None, {}, [None], [], "invalid json"):
            with self.subTest(value=value), self.assertRaises(MODULE.ReproducibilityError):
                MODULE.validate_probe_output(json.dumps(value), Path("unused"))

    def test_bss_beyond_eof_is_memory_only_and_alias_permissions_are_disjoint(self):
        def segment(index, start, size, memory, flags):
            return {"index": index, "type": 1, "start": start, "file_bytes": size,
                    "memory_bytes": memory, "virtual_address": 0, "physical_address": 0, "flags": flags}
        report = MODULE.segment_inventory([
            segment(0, 0, 8, 8, 5), segment(1, 4, 8, 8, 6),
            segment(2, 999999, 0, 400, 6),
        ], 16)
        self.assertEqual(report["file_backed_unique_bytes"], 12)
        self.assertEqual(report["disjoint_load_permission_bytes"], {
            "executable_not_writable": 4, "writable_not_executable": 4,
            "executable_and_writable": 4, "neither_executable_nor_writable": 0})
        self.assertEqual(report["memory_only_pt_load_bytes_sum"], 400)
        self.assertEqual(report["unassigned_file_ranges"], [[12, 16]])
        self.assertFalse(report["bss_is_file_content"])

    def test_malformed_load_memory_and_real_file_overrun_are_refused(self):
        base = {"index": 0, "type": 1, "start": 0, "file_bytes": 10,
                "memory_bytes": 9, "virtual_address": 0, "physical_address": 0, "flags": 7}
        with self.assertRaisesRegex(MODULE.ReproducibilityError, "filesz exceeds"):
            MODULE.segment_inventory([base], 10)
        base["memory_bytes"] = 10
        with self.assertRaisesRegex(MODULE.ReproducibilityError, "exceeds pinned"):
            MODULE.segment_inventory([base], 9)


def minimal_readiness_fixture():
    return [{"image": name, "file_bytes": MODULE.FILE_BYTES[name],
             "ready_entries": 1, "ready_hashed_bytes": 148, "pending_entries": 0,
             "zero_placeholders": 0, "all_input_hashes_ready": True,
             "segments": [{"index": 0, "kind": 0, "file_range": [0, 148],
                           "status": "ready", "sha384": "a" * 96}]}
            for name in MODULE.IMAGES]


class ReproducibilityTests(unittest.TestCase):
    def test_firmware_difference_fails_but_host_difference_is_separately_reported(self):
        first = {"source_output": [{"sha256": "a" * 64}],
                 "hash_readiness": minimal_readiness_fixture(),
                 "host_artifacts": {"firmware-tools": "b" * 64}}
        second = copy.deepcopy(first)
        second["source_output"][0]["sha256"] = "c" * 64
        with self.assertRaisesRegex(MODULE.ReproducibilityError, "disagree"):
            MODULE.compare_builds(first, second)
        second = copy.deepcopy(first)
        second["host_artifacts"]["firmware-tools"] = "c" * 64
        report = MODULE.compare_builds(first, second)
        self.assertTrue(report["firmware_source_outputs_byte_identical"])
        self.assertFalse(report["host_tools_byte_identical"])
        self.assertTrue(report["per_segment_sha384_readiness_identical"])
        second = copy.deepcopy(first)
        second["hash_readiness"][0]["segments"][0]["sha384"] = "b" * 96
        with self.assertRaisesRegex(MODULE.ReproducibilityError, "per-segment SHA-384"):
            MODULE.compare_builds(first, second)
        second = copy.deepcopy(first)
        second.pop("hash_readiness")
        with self.assertRaisesRegex(MODULE.ReproducibilityError, "missing"):
            MODULE.compare_builds(first, second)


    def test_partial_hash_readiness_rejects_forged_complete_and_bad_geometry(self):
        original = minimal_readiness_fixture()
        self.assertEqual(MODULE.validate_readiness_rows(copy.deepcopy(original)), original)
        for edit in [
            lambda rows: rows[0].update(ready_hashed_bytes=999),
            lambda rows: rows[0]["segments"][0].update(file_range=[0, 999_999_999]),
            lambda rows: rows[0].update(all_input_hashes_ready=False),
            lambda rows: rows[0]["segments"][0].update(sha384="not a digest"),
            lambda rows: rows[0]["segments"][0].update(index=3),
            lambda rows: rows[0]["segments"][0].update(status="placeholder"),
            lambda rows: rows[0].update(image="different.mbn"),
        ]:
            bad = copy.deepcopy(original)
            edit(bad)
            with self.subTest(bad=bad), self.assertRaises(MODULE.ReproducibilityError):
                MODULE.validate_readiness_rows(bad)

    def test_canonical_output_does_not_depend_on_mapping_insertion_order(self):
        self.assertEqual(MODULE.canonical_json({"z": 1, "a": {"y": 2, "x": 3}}),
                         MODULE.canonical_json({"a": {"x": 3, "y": 2}, "z": 1}))
        with self.assertRaises(ValueError):
            MODULE.canonical_json({"invalid": float("nan")})

    def test_build_environments_isolate_cache_and_override_wrapper_injection(self):
        with mock.patch.dict(os.environ, {"RUSTC_WRAPPER": "other", "CARGO_ENCODED_RUSTFLAGS": "other",
                                         "CARGO_BUILD_TARGET": "other", "CARGO_PROFILE_RELEASE_OPT_LEVEL": "0",
                                         "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER": "other",
                                         "CC": "other", "CC_x86_64_unknown_linux_gnu": "other",
                                         "LDFLAGS": "other", "RUSTC_BOOTSTRAP": "1"}):
            env = MODULE.build_environment(Path("/private/one"), 123, "/rust/bin/rustc")
        self.assertEqual(env["CARGO_HOME"], "/private/one/cargo-home")
        self.assertEqual(env["CARGO_TARGET_DIR"], "/private/one/target")
        self.assertEqual(env["SOURCE_DATE_EPOCH"], "123")
        self.assertEqual(env["CARGO_INCREMENTAL"], "0")
        self.assertNotIn("RUSTC_WRAPPER", env)
        self.assertNotIn("CARGO_ENCODED_RUSTFLAGS", env)
        self.assertNotIn("CARGO_BUILD_TARGET", env)
        self.assertNotIn("CARGO_PROFILE_RELEASE_OPT_LEVEL", env)
        self.assertNotIn("CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER", env)
        self.assertNotIn("CC", env)
        self.assertNotIn("CC_x86_64_unknown_linux_gnu", env)
        self.assertNotIn("LDFLAGS", env)
        self.assertNotIn("RUSTC_BOOTSTRAP", env)

    def test_strict_source_mismatch_fails_even_when_legacy_audit_and_counts_match(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            records = fixture_probe(path)
            source = MODULE.validate_probe_output(json.dumps(records), path)[0]
            original = bytearray(MODULE.FILE_BYTES["a640_zap.mbn"])
            for region in records[0]["regions"]:
                original[region["start"]:region["end"]] = (
                    path / "a640_zap.mbn" / region["file"]).read_bytes()
            correct = MODULE.compare_source_with_reference(source, original)
            self.assertEqual(correct["compared_generated_bytes"], 7856)
            changed = copy.deepcopy(source)
            changed["source_regions"][0]["sha256"] = "0" * 64
            legacy_audit = [{"image": "a640_zap.mbn", "independently_generated_file_bytes": 7856,
                             "missing_file_bytes": 6400}]
            auditor = mock.Mock()
            auditor.pinned_sha256s.return_value = {"a640_zap.mbn": MODULE.sha256(original)}
            auditor.audit.return_value = legacy_audit
            verifier = mock.Mock()
            builds = [{"checkout": ROOT, "executable": path / "first", "source_output": [source]},
                      {"checkout": ROOT, "executable": path / "second", "source_output": [changed]}]
            with mock.patch.object(MODULE, "load_module", side_effect=[auditor, verifier]), \
                    mock.patch.object(MODULE, "pinned_references", return_value={"a640_zap.mbn": bytes(original)}):
                with self.assertRaisesRegex(MODULE.ReproducibilityError, "strict generated source differs"):
                    MODULE.verify_originals(path, builds, path, 123)
            self.assertEqual(auditor.audit.call_count, 2)
            verifier.verify_image.assert_not_called()

    def test_wrong_pinned_reference_is_rejected_before_byte_use(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "a640_zap.mbn"
            path.write_bytes(b"not the known firmware")
            with self.assertRaisesRegex(MODULE.ReproducibilityError, "wrong pinned"):
                MODULE.pinned_references(directory, {name: "0" * 64 for name in MODULE.IMAGES})

    def test_failure_does_not_publish_a_partial_report(self):
        out, err = io.StringIO(), io.StringIO()
        with mock.patch.object(MODULE, "reproduce", side_effect=MODULE.ReproducibilityError("second build failed")):
            with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err), self.assertRaises(SystemExit) as raised:
                MODULE.main([])
        self.assertEqual(raised.exception.code, 1)
        self.assertEqual(out.getvalue(), "")
        self.assertIn("second build failed", err.getvalue())

    def test_existing_manifest_is_not_overwritten_or_built_again(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "report.json"
            path.write_bytes(b"keep")
            with mock.patch.object(MODULE, "reproduce") as build:
                with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                    MODULE.main(["--output", str(path)])
                build.assert_not_called()
            self.assertEqual(path.read_bytes(), b"keep")

    def test_atomic_publication_refuses_a_destination_creation_race(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "report.json"
            actual_link = os.link
            def compete(source, destination):
                Path(destination).write_bytes(b"competitor")
                return actual_link(source, destination)
            with mock.patch.object(MODULE.os, "link", side_effect=compete):
                with self.assertRaisesRegex(MODULE.ReproducibilityError, "during publication"):
                    MODULE.publish_manifest(path, b"new report")
            self.assertEqual(path.read_bytes(), b"competitor")
            self.assertEqual([p.name for p in Path(directory).iterdir()], ["report.json"])

    def test_source_snapshot_rejects_traversal_and_links(self):
        for filename, kind in (("../escape", tarfile.REGTYPE), ("link", tarfile.SYMTYPE)):
            archive = io.BytesIO()
            with tarfile.open(fileobj=archive, mode="w") as output:
                member = tarfile.TarInfo(filename)
                member.type = kind
                member.linkname = "../escape" if kind == tarfile.SYMTYPE else ""
                output.addfile(member)
            with tempfile.TemporaryDirectory() as directory:
                with self.subTest(filename=filename), self.assertRaises(MODULE.ReproducibilityError):
                    MODULE.extract_snapshot(archive.getvalue(), Path(directory) / "source")


if __name__ == "__main__":
    unittest.main()
