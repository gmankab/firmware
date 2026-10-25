# SPDX-License-Identifier: GPL-3.0-only
"""Source-only guardrails for exact A640 external-authentication workflow."""
from pathlib import Path
import importlib.util
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "tools" / "assemble-a640-exact.py"
SPEC = importlib.util.spec_from_file_location("a640_exact", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class TestA640ExactAssembly(unittest.TestCase):
    def test_pinned_revision_cannot_be_replaced_by_a_different_payload(self):
        self.assertEqual(len(MODULE.REFERENCE_SHA256), 64)
        self.assertEqual(len(MODULE.REFERENCE_EMBEDDED_ROOT_SHA256), 64)
        self.assertEqual(MODULE.EXTERNAL_AUTH_SIZE, 6400)
        self.assertEqual(MODULE.OUTPUT_SIZE, 14256)
        with self.assertRaisesRegex(ValueError, "exactly|expected 6400"):
            MODULE.validate_external_authentication(b"test-only")
        MODULE.validate_external_authentication(bytes(6400))  # only the length is validated here
        with self.assertRaisesRegex(ValueError, "output length"):
            MODULE.validate_exact_rebuilt_image(bytes(14255))
        with self.assertRaisesRegex(ValueError, "not the exact original A640 revision"):
            MODULE.validate_exact_rebuilt_image(bytes(14256))

    def test_rejects_existing_destination_without_clobbering(self):
        with tempfile.TemporaryDirectory() as directory:
            original = Path(directory) / "original.auth"
            original.write_bytes(bytes(6400))  # deliberately invalid OEM signature
            output = Path(directory) / "existing.mbn"
            output.write_bytes(b"keep")
            with self.assertRaises(SystemExit) as raised:
                MODULE.main([str(original), str(output), "--firmware-tools", str(SCRIPT)])
            self.assertEqual(raised.exception.code, 1)
            self.assertEqual(output.read_bytes(), b"keep")


if __name__ == "__main__":
    unittest.main()
