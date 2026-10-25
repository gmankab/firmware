# SPDX-License-Identifier: GPL-3.0-only
"""Offline data regeneration, source provenance and numeric independence."""
from decimal import localcontext
import hashlib
import importlib.util
from pathlib import Path
import shutil
import struct
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
TOOL = ROOT / "tools/devices/xiaomi-nabu/verify_agent20_18_data.py"
SPEC = importlib.util.spec_from_file_location("verify_agent20_18_data", TOOL)
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)

EXPECTED_DIGESTS = {
    0x195B0: "3cdee06ec254a6f173f97a5a70cfa394db8d65b677a8e7b53eb6e903d430e497",
    0x19614: "842f27102e31bc49ad3c4aebdbdaf2cd18c523298726832a4ec39ddafa683b02",
    0x19678: "fa6b8f447b9551d59e380e59334b1c927d03cf20261e3f42e876e72c01dd271c",
    0x196B8: "ff948665735fc3c049522ee3101a4b0539dd28944b13c6352e03f5ae9da53456",
    0x198DC: "f337b24fa79ee840e8aeccbc54612020c752f9d413bd93d13f70579e4e2d77af",
    0x19994: "a0d2ba4689500f80d103fe23205290a43fd377be0487ab2ed0290c1538224290",
    0x19A4C: "e9d8dca41fb507bc3aeb028e5b7565061f2841cea43b794b402479a12215f0ef",
    0x19B04: "70cee0878647c90d51b8fcc5c476162b1fdef81fbc40807fcc0cb3b38d16de3f",
    0x19BF0: "c26af2ad9c57bf0b64d2d1b93b1e911170e63c39d7bf69bfe423b9e8770a8f4c",
}


def copy_public_sources(root):
    directory = root / VERIFY.REFERENCE_DIRECTORY
    directory.mkdir(parents=True)
    for name in ("arm-log_data.c", "arm-math_config.h"):
        shutil.copyfile(ROOT / VERIFY.REFERENCE_DIRECTORY / name, directory / name)
    return directory


class IndependentDataContract(unittest.TestCase):
    def test_offline_generation_needs_only_bundled_public_sources(self):
        # This isolated root contains no Rust, generated fragments, manifests,
        # firmware or toolchain; source parsing and arithmetic must suffice.
        with tempfile.TemporaryDirectory(prefix="agent18-data-test-") as temporary:
            root = Path(temporary)
            copy_public_sources(root)
            fragments = VERIFY.independent_fragments(root)
        self.assertEqual(len(fragments), 9)
        self.assertEqual(sum(map(len, fragments.values())), 5304)
        self.assertEqual({start: hashlib.sha256(data).hexdigest()
                          for start, data in fragments.items()}, EXPECTED_DIGESTS)
        self.assertEqual(len(fragments[0x19BF0]), 530 * 8)

    def test_public_constants_and_layout_sources_are_both_pinned(self):
        for name in ("arm-log_data.c", "arm-math_config.h"):
            with self.subTest(source=name), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                directory = copy_public_sources(root)
                path = directory / name
                path.write_bytes(path.read_bytes() + b"\n/* substituted version */\n")
                with self.assertRaisesRegex(ValueError, "public source hash mismatch"):
                    VERIFY.independent_fragments(root)

    def test_trig_rounding_is_stable_across_precision_and_caller_context(self):
        with localcontext() as context:
            context.prec = 6
            at_80 = VERIFY._mathematical_fragments(80)
            at_112 = VERIFY._mathematical_fragments(112)
        self.assertEqual(at_80, at_112)
        for start in (0x198DC, 0x19994, 0x19A4C, 0x19B04):
            self.assertEqual(len(at_80[start]), 184)
            self.assertEqual(at_80[start][182:], b"\x00\x00")
        sine = struct.unpack("<91H", at_80[0x198DC][:182])
        cosine = struct.unpack("<91H", at_80[0x19994][:182])
        tangent = struct.unpack("<91H", at_80[0x19A4C][:182])
        self.assertEqual(sine, tuple(reversed(cosine)))
        self.assertEqual((sine[0], sine[30], sine[90]), (0, 50, 100))
        self.assertEqual((tangent[0], tangent[45], tangent[90]), (0, 100, 10000))
        with self.assertRaisesRegex(ValueError, "at least 80"):
            VERIFY._mathematical_fragments(79)

    def test_optional_reference_comparison_rejects_unpinned_image(self):
        with self.assertRaisesRegex(ValueError, "wrong reference"):
            VERIFY.checked_reference(bytes(VERIFY.IMAGE_SIZE))


if __name__ == "__main__":
    unittest.main()
