# SPDX-License-Identifier: GPL-3.0-only
"""Agent 19 synthetic checks; real firmware and Rust binary are opt-in inputs."""
from decimal import Decimal
import importlib.util
import json
import os
from pathlib import Path
import struct
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("novatek_dlm_check", ROOT / "tools/novatek/verify-novatek-dlm.py")
dlm = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(dlm)


def repair_checksums(image):
    """Synthetic-fixture helper; write payload checks before dependent footers."""
    for base in (0, 0x100):
        for field, start, end in ((0x18, 0x200, 0x1ac80), (0x1c, 0x22000, 0x2e190)):
            struct.pack_into("<I", image, base + field, dlm.crc32_mpeg2(image[start:end]))
        for entry in range(base + 0x30, base + 0x60, 16):
            length, start = struct.unpack_from("<II", image, entry + 4)
            struct.pack_into("<I", image, entry + 12, dlm.crc32_mpeg2(image[start:start + length + 1]))
    for base in (0, 0x100):
        struct.pack_into("<I", image, base + 0xfc, dlm.crc32_mpeg2(image[base:base + 0xfc]))


def synthetic_envelope():
    image = bytearray(0x30000)
    for bank, offsets in enumerate(((0, 0xa00, 0x3200), (0, 0x4a00, 0x7200))):
        base = 0x100 * bank
        struct.pack_into("<6I", image, base, 0x200, 0, 0x1aa7f, 0x22000, 0x20000, 0xc18f)
        struct.pack_into("<II", image, base + 0x20, 3 if bank == 0 else 1,
                         0x538110e0 if bank == 0 else 0)
        for index, offset in enumerate(offsets):
            struct.pack_into("<3I", image, base + 0x30 + 16 * index, 0x20000 + offset, 0, 0x22000 + offset)
        struct.pack_into("<3I", image, base + 0xf0, 0x2c190 + base, 0xff, base)
    struct.pack_into("<I", image, 0x200, 0x01020304)  # synthetic controller scalar
    image[0x22000] = image[0x2f000] = 0x41
    image[-3:] = b"NVT"
    repair_checksums(image)
    return image


class PackedScheduleTests(unittest.TestCase):
    def test_schedule_links_reproduce_only_consumer_proven_15bit_fields(self):
        fragments = dlm.generated_fragments()
        fields = [(start,name,data) for start,name,data in fragments if name=="wide-schedule-word-link"]
        self.assertEqual(len(fields),34)
        self.assertEqual(sum(len(data) for _,_,data in fields),68)
        for index,(start,name,data) in enumerate(fields):
            base = 0x22a64 if index < 17 else 0x26a64
            table = 0x24904 if index < 17 else 0x28904
            self.assertEqual(start,base+0x50+(index%17)*0x8c)
            self.assertEqual(struct.unpack("<H",data)[0],(table-0x22000)//4)
            # The upper halfword of each original u32 remains UNKNOWN.
            self.assertEqual(start+len(data),start+2)

    def test_narrow_schedule_byte_level_credit_excludes_unproven_bit15(self):
        parts=[(a,b,c) for a,b,c in dlm.generated_fragments()
               if b=="narrow-schedule-word-link"]
        self.assertEqual(len(parts),8)
        self.assertEqual(sum(len(data) for _,_,data in parts),12)
        for idx,(start,_,data) in enumerate(parts):
            base,table=((0x25230,0x25d4c) if idx<4 else (0x29230,0x29d4c))
            rec=idx%4
            self.assertEqual(start,base+rec*0x8c+0x50)
            self.assertEqual(data,struct.pack("<H",(table-0x22000)//4)[:2 if rec<2 else 1])
            self.assertEqual(data[-1],(0x0f if idx<4 else 0x1f) if rec<2 else 0x53)

    def test_descriptor_bank_relocation_is_flag_preserving_and_external_only(self):
        record=bytearray([0x2e]*0x8c)
        for index in range(9):
            struct.pack_into("<H",record,0x48+2*index,0x8d18 if index%2 else 0x0a41)
        transformed=dlm.transform_descriptor_bank(record)
        for index in range(9):
            self.assertEqual(struct.unpack_from("<H",transformed,0x48+2*index)[0],
                             0x9d18 if index%2 else 0x1a41)
        self.assertEqual(transformed[0x5a:0x5c],record[0x5a:0x5c])
        with self.assertRaises(ValueError):
            dlm.transform_descriptor_bank(b"x"*71)
        struct.pack_into("<H",record,0x48,0xf500)
        with self.assertRaises(ValueError):
            dlm.transform_descriptor_bank(record)

    def test_idle_mailboxes_have_same_controller_token(self):
        fragments = dlm.generated_fragments()
        mailbox = next((start,name,data) for start,name,data in fragments if name=="controller-idle-mailboxes")
        self.assertEqual(mailbox, (0x2e130,"controller-idle-mailboxes",struct.pack("<II",0x12345678,0x12345678)))

    def test_source_owned_reset_defaults_from_upper_ilm_gp_stores(self):
        f={start:(name,data) for start,name,data in dlm.generated_fragments()
           if name.endswith("initializer") or name=="channel-status-invalid-sentinel"}
        self.assertEqual(sorted(f),[0x2e17c,0x2e180,0x2e188,0x2e18c])
        self.assertEqual(f[0x2e17c][1],struct.pack("<i",-1))
        self.assertEqual(f[0x2e180][1],struct.pack("<i",-1))
        self.assertEqual(f[0x2e188][1],struct.pack("<I",1))
        self.assertEqual(f[0x2e18c][1],b"\xff")
        # Controller counter at 0x2e184 and bits 0x2e18d.. are excluded.
        self.assertNotIn(0x2e184, f)

    def test_high_bits_cross_group_boundaries_and_sentinel_is_not_zero(self):
        values = [0] * 50
        values[0], values[9], values[10], values[23] = 0x100, 0x700, 0x300, 0x4a5
        raw = dlm.pack_record(values)
        self.assertEqual(struct.unpack_from("<5I", raw, 52), (0x38000001, 3, 0x800, 0, 0))
        self.assertEqual(raw[23], 0xa5)
        self.assertEqual(dlm.unpack_record(raw), values)
        disabled = dlm.pack_record([dlm.SENTINEL] * 50)
        self.assertEqual(disabled[:50], bytes([255]) * 50)
        self.assertEqual(disabled[50:52], bytes(2))
        self.assertEqual(struct.unpack_from("<5I", disabled, 52), (0x3fffffff,) * 5)

    def test_invalid_widths_alignment_and_reserved_high_bits_fail(self):
        for values in ([0] * 49, [2048] * 50, [-1] * 50, [True] * 50):
            with self.assertRaises(ValueError):
                dlm.pack_record(values)
        for length in (0, 71, 73):
            with self.assertRaises(ValueError):
                dlm.unpack_record(bytes(length))
        for offset, value in ((50, 1), (55, 0x40), (71, 0x80)):
            malformed = bytearray(72)
            malformed[offset] = value
            with self.assertRaises(ValueError):
                dlm.unpack_record(malformed)

    def test_ring_walks_and_translations_preserve_explicit_disable_masks(self):
        self.assertEqual(dlm.wide_schedule(7)[:5], [99, 67, 35, 3, 1506])
        for row in range(24):
            for a, b in zip(dlm.wide_schedule(row), dlm.wide_schedule(row, True)):
                self.assertEqual(b, a + 16 if 7 <= row < 18 and a != dlm.SENTINEL else a)
        for row in range(3, 18):
            for a, b in zip(dlm.narrow_schedule(row), dlm.narrow_schedule(row, True)):
                self.assertEqual(b, a + 15 if a != dlm.SENTINEL else a)
        self.assertEqual([dlm.narrow_schedule(row).count(dlm.SENTINEL) for row in (14, 16, 17)], [12, 28, 38])
        self.assertEqual(dlm.wide_schedule(21), [dlm.SENTINEL] * 30 + [0] * 20)
        with self.assertRaises(ValueError):
            dlm.narrow_schedule(20)
        with self.assertRaises(ValueError):
            dlm.wide_schedule(-1)


class UncreditedCommandCandidatesTests(unittest.TestCase):
    def test_observed_40bit_recursion_has_32_phase_mirror_records(self):
        actual = dlm.mirrored_control_candidate()
        self.assertEqual(len(actual), 160)
        for i, chunk in enumerate(actual[j:j+5] for j in range(0,160,5)):
            raw = int.from_bytes(chunk, "little")
            row = i // 4
            selected = (16 + row) if i % 4 < 2 else (31 - row)
            self.assertEqual((raw >> 24) & 0x1f, selected)
            self.assertEqual((raw >> 23) & 1, i % 2)
            self.assertEqual(raw & ~((0x1f << 24) | (1 << 23)), 0x8000064800)

    def test_transfer_word_recursion_matches_permuted_stride_patterns(self):
        observed=dlm.transfer_word_candidate()
        self.assertEqual(len(observed),110)
        image=bytearray(0x2d4be)
        image[0x2d450:0x2d4be]=observed
        records=dlm.inspect_external_five_byte_transfers(image)
        self.assertEqual(len(records),22)
        self.assertEqual(records[0],(0x19,0x9004))
        for i,slot in enumerate((4,0,1,2,3,4),1):
            self.assertEqual(records[i],(0x32,0x80c8+slot*0x190))
        for i,slot in enumerate((4,0,1,2,3,4),16):
            self.assertEqual(records[i],(0x32,0x9068+slot*0x190))
        for i,slot in zip((7,8,11,12,15),(0,2,1,3,5)):
            self.assertEqual(records[i],(0x19,0x9068+slot*0x190))
        for i in (9,13):
            self.assertEqual(records[i:i+2],[(0x02,0x4bf4),(0x02,0x4c04)])

    def test_uncredited_40bit_and_external_records_do_not_enter_source_ledger(self):
        image = bytearray(0x2d4be)
        image[0x2d3b0:0x2d450] = dlm.mirrored_control_candidate()
        image[0x2d450:0x2d4be] = dlm.transfer_word_candidate()
        report = dlm.verify_uncredited_candidates(image)
        self.assertEqual(report["candidate_mathematically_matched_bytes"],160)
        self.assertEqual(report["external_transfer_structurally_validated_bytes"],110)
        self.assertEqual(report["candidate_transfer_mathematically_matched_bytes"],110)
        self.assertEqual(report["additional_source_owned_bytes"],0)
        self.assertFalse(report["source_generation_semantics_proven"])
        self.assertFalse(any(start <= 0x2d3b0 < start + len(data)
                             for start,_,data in dlm.generated_fragments()))
        image[0x2d3b1] ^= 1
        with self.assertRaisesRegex(ValueError,"recurrence"):
            dlm.verify_uncredited_candidates(image)
        image[0x2d3b1] ^= 1
        image[0x2d450] ^= 1
        with self.assertRaisesRegex(ValueError,"external transfer"):
            dlm.verify_uncredited_candidates(image)

    @unittest.skipUnless(os.environ.get("NOVATEK_REFERENCE"),"pinned Novatek reference not supplied")
    def test_pinned_reference_matches_only_uncredited_candidate(self):
        image = Path(os.environ["NOVATEK_REFERENCE"]).read_bytes()
        self.assertEqual(__import__("hashlib").sha256(image).hexdigest(),dlm.REFERENCE_SHA256)
        result = dlm.verify_uncredited_candidates(image)
        self.assertEqual(result["candidate_mathematically_matched_bytes"],160)
        self.assertEqual(result["candidate_transfer_mathematically_matched_bytes"],110)
        self.assertEqual(result["additional_source_owned_bytes"],0)


class GaussianAndCrcTests(unittest.TestCase):
    def test_machin_pi_decimal_density_and_rounding_margin(self):
        self.assertTrue(str(dlm.decimal_pi()).startswith("3.14159265358979323846264338327950288419716939937510"))
        raw, margin, x = dlm.gaussian_kernel(400)
        samples = struct.unpack("<81H", raw)
        self.assertEqual(samples[-1], 3989)
        self.assertEqual(list(samples), sorted(samples))
        self.assertEqual(x, 72)
        self.assertGreater(margin, Decimal("0.001583"))
        self.assertLess(margin, Decimal("0.001584"))
        self.assertEqual(struct.unpack("<81H", dlm.gaussian_kernel(800)[0])[-1], 1995)

    def test_invalid_gaussian_width_and_u16_overflow_are_rejected(self):
        for sigma in (0, -1, 1, 24, 65536, 25.2, True):
            with self.assertRaises(ValueError):
                dlm.gaussian_kernel(sigma)
        self.assertEqual(len(dlm.gaussian_kernel(25)[0]), 162)

    def test_table_crc_public_vectors_and_zero_residue(self):
        self.assertEqual(dlm.crc32_mpeg2(b""), 0xffffffff)
        self.assertEqual(dlm.crc32_mpeg2(b"123456789"), 0x0376e6e7)
        payload = bytes(range(256)) + b"synthetic CRC residue"
        checksum = dlm.crc32_mpeg2(payload)
        self.assertEqual(dlm.crc32_mpeg2(payload + struct.pack(">I", checksum)), 0)

    def test_fragments_are_disjoint_data_with_frozen_owned_byte_total(self):
        fragments = dlm.generated_fragments()
        self.assertEqual(len(fragments), 69)
        self.assertEqual(sum(len(data) for _, _, data in fragments), 9353)
        self.assertEqual(sum(len(data) for _, name, data in fragments if name == "gaussian-half-kernel"), 2916)
        self.assertTrue(all(a + len(data) <= b for (a, _, data), (b, _, _) in zip(fragments, fragments[1:])))
        self.assertEqual(sum(max(0, min(start + len(data), 0x22900) - max(start, 0x22000))
                             for start, _, data in fragments), 648)


class EnvelopeTests(unittest.TestCase):
    def test_all_twelve_physical_crc_fields_and_external_mirror(self):
        image = synthetic_envelope()
        checks = dlm.check_envelope(image)
        self.assertEqual(len(checks), 12)
        self.assertEqual(len({offset for offset, _ in checks}), 12)
        image[0x2f001] ^= 1
        with self.assertRaisesRegex(ValueError, "mirror"):
            dlm.check_envelope(image)

    def test_malformed_descriptor_and_vector_rejected_despite_valid_checksums(self):
        for offset, value, error in ((0x38, 0x22001, "profile scalar"), (0x204, 0xdeadbeef, "vector")):
            image = synthetic_envelope()
            struct.pack_into("<I", image, offset, value)
            repair_checksums(image)
            with self.assertRaisesRegex(ValueError, error):
                dlm.check_envelope(image)

    def test_payload_corruption_marker_padding_and_truncation_fail(self):
        image = synthetic_envelope()
        for offset in (0x700, 0x2e000, 0x1ac80, 0x2fffe):
            mutated = bytearray(image)
            mutated[offset] ^= 1
            with self.assertRaises(ValueError):
                dlm.check_envelope(mutated)
        for length in (0, 511, 196607):
            with self.assertRaises(ValueError):
                dlm.check_envelope(image[:length])
        with self.assertRaisesRegex(ValueError, "SHA-256"):
            dlm.verify_reference(image, dlm.generated_fragments())


class ManifestTests(unittest.TestCase):
    def write_emitted_fixture(self, directory):
        fragments = dlm.generated_fragments()
        manifest = {"schema": "agent20-19-novatek-dlm-v1", "profile": "postmarketos-nabu",
                    "reference_sha256": dlm.REFERENCE_SHA256, "reference_verified": False,
                    "flashable": False, "mismatched_source_bytes": None, **dlm.EXPECTED_COUNTS,
                    "fragments": [dlm.fragment_metadata(fragment) for fragment in fragments],
                    "regions": [dict(row, evidence="synthetic fixture") for row in dlm.expected_regions(fragments)]}
        for fragment, metadata in zip(fragments, manifest["fragments"]):
            (directory / metadata["file"]).write_bytes(fragment[2])
        (directory / "manifest.json").write_text(json.dumps(manifest))
        return fragments, manifest

    def test_changed_output_bytes_cannot_pass_on_manifest_hash_alone(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            fragments, manifest = self.write_emitted_fixture(directory)
            dlm.check_manifest(directory, fragments)
            path = directory / manifest["fragments"][0]["file"]
            changed = bytearray(path.read_bytes())
            changed[-1] ^= 1
            path.write_bytes(changed)
            with self.assertRaisesRegex(ValueError, "byte mismatch"):
                dlm.check_manifest(directory, fragments)

    def test_manifest_extent_path_and_provenance_changes_are_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            fragments, manifest = self.write_emitted_fixture(directory)
            for key, value in (("file_end", manifest["fragments"][0]["file_end"] + 1),
                               ("file", "../outside.bin"), ("sha256", "0" * 64)):
                changed = json.loads(json.dumps(manifest))
                changed["fragments"][0][key] = value
                (directory / "manifest.json").write_text(json.dumps(changed))
                with self.assertRaisesRegex(ValueError, "extent/hash"):
                    dlm.check_manifest(directory, fragments)
            manifest["new_executable_bytes"] = 9353
            (directory / "manifest.json").write_text(json.dumps(manifest))
            with self.assertRaisesRegex(ValueError, "accounting"):
                dlm.check_manifest(directory, fragments)

    def test_unchanged_totals_cannot_hide_false_region_credit_overlap_or_mirror_dependency(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            fragments, manifest = self.write_emitted_fixture(directory)
            dlm.check_manifest(directory, fragments)
            unknown = next(i for i, row in enumerate(manifest["regions"]) if row["kind"] == "unknown_dlm")
            mirror = next(i for i, row in enumerate(manifest["regions"]) if row["kind"] == "derived_mirror")
            for index, key, value in ((unknown, "kind", "source_dlm_data"),
                                      (unknown, "file_start", manifest["regions"][unknown]["file_start"] - 1),
                                      (mirror, "source_independent", True)):
                changed = json.loads(json.dumps(manifest))
                changed["regions"][index][key] = value
                (directory / "manifest.json").write_text(json.dumps(changed))
                with self.assertRaisesRegex(ValueError, "regional partition/provenance"):
                    dlm.check_manifest(directory, fragments)


class OptionalReferenceTests(unittest.TestCase):
    @unittest.skipUnless(os.environ.get("NOVATEK_REFERENCE"), "NOVATEK_REFERENCE not supplied")
    def test_pinned_reference_matches_independent_recipes_and_crc(self):
        image = Path(os.environ["NOVATEK_REFERENCE"]).read_bytes()
        self.assertEqual(len(dlm.verify_reference(image, dlm.generated_fragments())), 12)

    @unittest.skipUnless(os.environ.get("NOVATEK_REFERENCE") and os.environ.get("NOVATEK_DLM_BIN"),
                         "NOVATEK_REFERENCE and NOVATEK_DLM_BIN not both supplied")
    def test_rust_emission_matches_decimal_verifier_and_pinned_reference(self):
        with tempfile.TemporaryDirectory() as tmp:
            subprocess.run([os.environ["NOVATEK_DLM_BIN"], "emit", tmp], check=True, capture_output=True, text=True)
            report = dlm.verification_report(os.environ["NOVATEK_REFERENCE"], tmp)
        self.assertEqual(report["independent_data_bytes"], 9353)
        self.assertEqual(report["reference_assisted_descriptor_bytes"], 2940)
        self.assertEqual(report["mismatched_source_bytes"], 0)
        self.assertTrue(report["rust_output_matches"])


if __name__ == "__main__":
    unittest.main()
