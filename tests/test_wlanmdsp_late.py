# SPDX-License-Identifier: GPL-3.0-only
"""WLAN late-region provenance, interval and optional real-toolchain checks."""
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parent.parent
TOOL = ROOT / "tools/build-wlanmdsp-late.py"
REGISTRY = ROOT / "sources/xiaomi-nabu/wlanmdsp/late/regions.json"
SPEC = importlib.util.spec_from_file_location("build_wlanmdsp_late", TOOL)
assert SPEC is not None and SPEC.loader is not None
late = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = late
SPEC.loader.exec_module(late)


class WlanLateRegistryTests(unittest.TestCase):
    def setUp(self):
        self.payload = json.loads(REGISTRY.read_text())

    def test_real_registry_stays_within_exclusive_executable_scope(self):
        regions = late.validate_registry(self.payload)
        self.assertGreaterEqual(len(regions), 4)
        self.assertEqual(self.payload["owned_file_start"], 0x100000)
        self.assertEqual(self.payload["owned_file_end"], 0x353D38)
        self.assertTrue(all(0x100000 <= r.file_start < r.file_end <= 0x353D38
                            for r in regions))
        checkpoint1 = late.select_regions(regions, through_checkpoint=1)
        self.assertEqual([r.file_end - r.file_start for r in checkpoint1],
                         [124, 464, 216, 64])

    def test_schema_and_identity_cannot_redefine_the_pinned_scope(self):
        for key, bad_value in (("schema_version", 2),
                               ("reference_sha256", "0" * 64),
                               ("owned_file_start", 0x40000),
                               ("owned_file_end", 0x380000)):
            with self.subTest(key=key):
                changed = copy.deepcopy(self.payload)
                changed[key] = bad_value
                with self.assertRaises(late.BuildError):
                    late.validate_registry(changed)

    def test_missing_registry_fields_are_rejected(self):
        for key in ("schema_version", "reference_sha256", "owned_file_start",
                    "owned_file_end", "regions"):
            with self.subTest(key=key):
                changed = copy.deepcopy(self.payload)
                del changed[key]
                with self.assertRaises(late.BuildError):
                    late.validate_registry(changed)

    def test_empty_or_nonlist_region_collection_is_rejected(self):
        for value in ([], {}, None, "regions"):
            with self.subTest(value=value):
                changed = copy.deepcopy(self.payload)
                changed["regions"] = value
                with self.assertRaises(late.BuildError):
                    late.validate_registry(changed)

    def test_out_of_scope_and_misaligned_intervals_are_rejected(self):
        for field, value in (("file_start", 0xFFFFC),
                             ("file_end", 0x353D3C),
                             ("file_start", 0x1001C5),
                             ("file_end", 0x10023F),
                             ("file_end", 0x1001C4)):
            with self.subTest(field=field, value=value):
                changed = copy.deepcopy(self.payload)
                changed["regions"][0][field] = value
                with self.assertRaises(late.BuildError):
                    late.validate_registry(changed)

    def test_overlap_and_reordered_intervals_are_rejected(self):
        changed = copy.deepcopy(self.payload)
        changed["regions"][1]["file_start"] -= 4
        with self.assertRaises(late.BuildError):
            late.validate_registry(changed)
        changed = copy.deepcopy(self.payload)
        changed["regions"].reverse()
        with self.assertRaises(late.BuildError):
            late.validate_registry(changed)

    def test_gaps_remain_explicit_and_cannot_be_concatenated(self):
        changed = copy.deepcopy(self.payload)
        changed["regions"][0]["file_end"] -= 4
        regions = late.validate_registry(changed)
        with self.assertRaises(late.BuildError):
            late.select_regions(regions, through_checkpoint=1)

    def test_duplicate_names_and_symbols_are_rejected(self):
        for field in ("name", "symbol"):
            with self.subTest(field=field):
                changed = copy.deepcopy(self.payload)
                changed["regions"][1][field] = changed["regions"][0][field]
                with self.assertRaises(late.BuildError):
                    late.validate_registry(changed)

    def test_source_paths_must_remain_in_the_owned_source_directory(self):
        for source in ("../outside.S", "/tmp/outside.S",
                       "tools/build-wlanmdsp-startup.py"):
            with self.subTest(source=source):
                changed = copy.deepcopy(self.payload)
                changed["regions"][0]["source"] = source
                with self.assertRaises(late.BuildError):
                    late.validate_registry(changed)

    def test_region_digest_classification_and_checkpoint_are_validated(self):
        for field, value in (("sha256", "z" * 64), ("sha256", "0" * 63),
                             ("classification", "layout-fill"),
                             ("checkpoint", 0), ("checkpoint", -1)):
            with self.subTest(field=field, value=value):
                changed = copy.deepcopy(self.payload)
                changed["regions"][0][field] = value
                with self.assertRaises(late.BuildError):
                    late.validate_registry(changed)

    def test_checkpoint_selection_filters_without_inventing_gap_fill(self):
        changed = copy.deepcopy(self.payload)
        changed["regions"] = changed["regions"][:4]
        for region in changed["regions"][2:]:
            region["checkpoint"] = 2
        regions = late.validate_registry(changed)
        selected = late.select_regions(regions, through_checkpoint=1)
        self.assertEqual(len(selected), 2)
        self.assertEqual((selected[0].file_start, selected[-1].file_end),
                         (0x1001C4, 0x100410))
        self.assertEqual(len(late.select_regions(regions)), 4)
        with self.assertRaises(late.BuildError):
            late.select_regions(regions, through_checkpoint=0)

    def test_invalid_json_load_has_build_error(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "invalid.json"
            path.write_text("{ definitely not valid JSON")
            with self.assertRaises(late.BuildError):
                late.load_registry(path)


@unittest.skipUnless(os.environ.get("WLAN_LATE_LLVM_MC"),
                     "set WLAN_LATE_LLVM_MC to a Hexagon v66-capable llvm-mc")
class WlanLateAssemblyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc = os.environ["WLAN_LATE_LLVM_MC"]
        cls.independent_mc = os.environ.get("WLAN_LATE_SECOND_LLVM_MC",
                                            os.environ.get("WLAN_LATE_INDEPENDENT_LLVM_MC"))
        cls.objdump = os.environ.get("WLAN_LATE_LLVM_OBJDUMP")
        cls.fragment, cls.manifest = late.build(
            cls.mc, through_checkpoint=1,
            independent_llvm_mc=cls.independent_mc,
            llvm_objdump=cls.objdump)

    def test_checkpoint1_exact_length_and_address_regression(self):
        self.assertEqual(len(self.fragment), 868)
        self.assertEqual(self.manifest["generated_bytes"], 868)
        self.assertEqual((self.manifest["file_start"], self.manifest["file_end"]),
                         (0x1001C4, 0x100528))
        self.assertEqual((self.manifest["vaddr_start"], self.manifest["vaddr_end"]),
                         (0xB00C01C4, 0xB00C0528))
        self.assertEqual(self.manifest["sha256"],
                         hashlib.sha256(self.fragment).hexdigest())

    def test_region_bytes_match_individual_pinned_digests(self):
        source_registry = late.select_regions(late.load_registry(), through_checkpoint=1)
        cursor = 0
        for region, report in zip(source_registry, self.manifest["regions"]):
            size = region.file_end - region.file_start
            section = self.fragment[cursor:cursor + size]
            self.assertEqual(hashlib.sha256(section).hexdigest(), region.sha256)
            self.assertEqual(report["sha256"], region.sha256)
            self.assertEqual(report["source_sha256"],
                             hashlib.sha256((ROOT / region.source).read_bytes()).hexdigest())
            self.assertGreater(report["packet_count"], 0)
            cursor += size
        self.assertEqual(cursor, len(self.fragment))
        self.assertEqual(len(self.manifest["regions"]), 4)

    def test_coverage_seam_and_provenance_counts_are_honest(self):
        self.assertEqual(self.manifest["source_derived_executable_bytes"], 868)
        for field in ("data_bytes", "layout_fill_bytes", "external_reference_bytes_in_output"):
            self.assertEqual(self.manifest[field], 0)
        gaps = [(gap["file_start"], gap["file_end"])
                for gap in self.manifest["scope_gaps"]]
        self.assertEqual(gaps, [(0x100000, 0x1001C4), (0x100528, 0x353D38)])
        self.assertEqual(self.manifest["next_uncovered_file_offset"], 0x100000)
        self.assertEqual(self.manifest["next_continuation_file_offset"], 0x100528)

    def test_unexecuted_checks_have_no_success_mismatch_count(self):
        self.assertFalse(self.manifest["reference_checked"])
        self.assertIsNone(self.manifest["reference_byte_mismatches"])
        self.assertEqual(self.manifest["independent_assembler_checked"], bool(self.independent_mc))
        self.assertEqual(self.manifest["independent_assembler_byte_mismatches"],
                         0 if self.independent_mc else None)
        self.assertEqual(self.manifest["decoder_checked"], bool(self.objdump))

    def test_repeat_build_has_identical_bytes(self):
        repeated, _ = late.build(self.mc, through_checkpoint=1)
        self.assertEqual(repeated, self.fragment)

    def test_latest_source_keeps_the_verified_prefix_and_matches_reference(self):
        reference_path = os.environ.get("WLANMDSP_REFERENCE")
        latest, report = late.build(
            self.mc, reference=Path(reference_path) if reference_path else None,
            independent_llvm_mc=self.independent_mc, llvm_objdump=self.objdump)
        self.assertTrue(latest.startswith(self.fragment))
        self.assertGreaterEqual(len(latest), len(self.fragment))
        if reference_path:
            expected = Path(reference_path).read_bytes()[report["file_start"]:report["file_end"]]
            self.assertEqual(latest, expected)
            self.assertEqual(report["reference_byte_mismatches"], 0)

    def test_wrong_region_digest_rejects_build(self):
        payload = json.loads(REGISTRY.read_text())
        payload["regions"][0]["sha256"] = "0" * 64
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "wrong-digest.json"
            path.write_text(json.dumps(payload))
            with self.assertRaises(late.BuildError):
                late.build(self.mc, through_checkpoint=1, registry_path=path)

    def test_wrong_region_size_rejects_build(self):
        payload = json.loads(REGISTRY.read_text())
        payload["regions"] = payload["regions"][:1]
        payload["regions"][0]["file_end"] -= 4
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "wrong-size.json"
            path.write_text(json.dumps(payload))
            with self.assertRaises(late.BuildError):
                late.build(self.mc, through_checkpoint=1, registry_path=path)

    @unittest.skipUnless(os.environ.get("WLANMDSP_REFERENCE"),
                         "set WLANMDSP_REFERENCE to the pinned private OEM MBN")
    def test_pinned_reference_byte_comparison(self):
        reference = Path(os.environ["WLANMDSP_REFERENCE"])
        fragment, manifest = late.build(self.mc, reference=reference, through_checkpoint=1)
        self.assertEqual(fragment, self.fragment)
        self.assertTrue(manifest["reference_checked"])
        self.assertEqual(manifest["reference_byte_mismatches"], 0)
        self.assertEqual(fragment, reference.read_bytes()[0x1001C4:0x100528])

    @unittest.skipUnless(os.environ.get("WLANMDSP_REFERENCE"),
                         "set WLANMDSP_REFERENCE for reference-mutation rejection")
    def test_reference_mutation_outside_new_span_is_rejected(self):
        reference = bytearray(Path(os.environ["WLANMDSP_REFERENCE"]).read_bytes())
        # Revision identity must cover the whole file, not just the new span.
        reference[-1] ^= 1
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "modified-reference.mbn"
            path.write_bytes(reference)
            with self.assertRaises(late.BuildError):
                late.build(self.mc, reference=path, through_checkpoint=1)

    def test_cli_emits_only_actual_bytes_and_manifest(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp) / "late.bin"
            manifest = Path(temp) / "late.json"
            subprocess.run([sys.executable, str(TOOL), str(output),
                            "--llvm-mc", self.mc, "--through-checkpoint", "1",
                            "--manifest", str(manifest)], check=True,
                           capture_output=True, text=True)
            self.assertEqual(output.read_bytes(), self.fragment)
            info = json.loads(manifest.read_text())
            self.assertEqual(info["file_start"], 0x1001C4)
            self.assertEqual(info["generated_bytes"], 868)
            self.assertEqual(info["layout_fill_bytes"], 0)


if __name__ == "__main__":
    unittest.main()
