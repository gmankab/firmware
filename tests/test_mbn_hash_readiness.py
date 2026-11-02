# SPDX-License-Identifier: GPL-3.0-only
"""Synthetic input tests for the independent source-hash readiness auditor."""
from copy import deepcopy
import hashlib
import importlib.util
from pathlib import Path
import struct
import unittest

S = importlib.util.spec_from_file_location(
    "mbn_hash_readiness", Path(__file__).resolve().parents[1] / "tools/verify/verify-mbn-hash-readiness.py")
checker = importlib.util.module_from_spec(S)
S.loader.exec_module(checker)


def fixture():
    image = bytearray(0x220)
    image[:7] = b"\x7fELF\x01\x01\x01"
    struct.pack_into("<H", image, 16, 2)
    struct.pack_into("<H", image, 18, 164)
    struct.pack_into("<I", image, 20, 1)
    struct.pack_into("<I", image, 28, 52)
    struct.pack_into("<H", image, 40, 52)
    struct.pack_into("<H", image, 42, 32)
    struct.pack_into("<H", image, 44, 3)
    # MBN original ELF first header, hash segment and executable source.
    struct.pack_into("<8I", image, 52, 0, 0, 0, 0, 148, 148, 0x07000000, 0x1000)
    struct.pack_into("<8I", image, 84, 0, 0x100, 0, 0, 0xc0, 0xc0, 0x02000000, 0x1000)
    struct.pack_into("<8I", image, 116, 1, 0x200, 0, 0, 0x20, 0x20, 5, 4)
    image[0x200:0x220] = bytes(range(32))
    report = {
        "image": "a640_zap.mbn", "file_bytes": len(image),
        "ready_entries": 1, "ready_hashed_bytes": 148,
        "pending_entries": 1, "zero_placeholders": 1,
        "all_input_hashes_ready": False,
        "segments": [
            {"index": 0, "kind": 0, "file_range": [0, 148], "status": "ready",
             "sha384": hashlib.sha384(image[:148]).hexdigest()},
            {"index": 1, "kind": 0, "file_range": [0x100, 0x1c0], "status": "placeholder"},
            {"index": 2, "kind": 1, "file_range": [0x200, 0x220], "status": "pending",
             "first_missing": [0x200, 0x210], "missing_bytes": 0x20},
        ],
    }
    return bytes(image), report


class SourceHashReadinessTests(unittest.TestCase):
    def test_accepts_verified_ready_headers_without_claiming_source_complete(self):
        image, r = fixture()
        result = checker.verify_image_report(r, image, hashlib.sha256(image).hexdigest())
        self.assertEqual(result["ready_hashed_file_bytes"], 148)
        self.assertEqual(result["pending_segments"], 1)
        self.assertFalse(result["hardware_authentication"])
        self.assertTrue(result["reference_only_verification"])

    def test_rejects_mutated_source_digest_and_reference(self):
        image, r = fixture()
        wrong = deepcopy(r)
        wrong["segments"][0]["sha384"] = "00" * 48
        with self.assertRaisesRegex(checker.ReadinessError, "does not match"):
            checker.verify_image_report(wrong, image, hashlib.sha256(image).hexdigest())
        with self.assertRaisesRegex(checker.ReadinessError, "pinned SHA-256"):
            checker.verify_image_report(r, image, "f" * 64)

    def test_rejects_invalid_program_header_indexes_geometry_and_labels(self):
        image, r = fixture()
        for transform in [
            lambda x: x["segments"][1].update(index=0),
            lambda x: x["segments"][2].update(file_range=[0x201, 0x220]),
            lambda x: x["segments"][1].update(status="ready", sha384="0" * 96),
            lambda x: x["segments"][2].update(first_missing=[0x1ff, 0x210]),
            lambda x: x["segments"][2].update(missing_bytes=1),
            lambda x: x.update(ready_hashed_bytes=4096),
            lambda x: x.update(all_input_hashes_ready=True),
        ]:
            altered = deepcopy(r)
            transform(altered)
            with self.subTest(altered=altered), self.assertRaises(checker.ReadinessError):
                checker.verify_image_report(altered, image, hashlib.sha256(image).hexdigest())

    def test_requires_six_pinned_images_in_canonical_order(self):
        image, r = fixture()
        with self.assertRaisesRegex(checker.ReadinessError, "six images"):
            checker.verify_reports([r], "/nonexistent", {"a640_zap.mbn": hashlib.sha256(image).hexdigest()})

    def test_rejects_changed_elf_header_or_p_filesz_memsz_violation(self):
        image, r = fixture()
        corrupt = bytearray(image)
        struct.pack_into("<I", corrupt, 116 + 20, 0x1f)
        with self.assertRaisesRegex(checker.ReadinessError, "p_filesz exceeds p_memsz"):
            checker.read_elf_headers(corrupt)
        corrupt = bytearray(image)
        corrupt[4] = 2  # ELF64
        with self.assertRaises(checker.ReadinessError):
            checker.read_elf_headers(corrupt)


if __name__ == "__main__":
    unittest.main()
