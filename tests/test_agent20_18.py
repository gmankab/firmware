# SPDX-License-Identifier: GPL-3.0-only
"""Guard against silently accepting a different firmware or a truncated span."""
import importlib.util
from pathlib import Path
import unittest

PATH = Path(__file__).resolve().parents[1] / "tools/devices/xiaomi-nabu/verify-agent20-18.py"
SPEC = importlib.util.spec_from_file_location("verify_agent20_18", PATH)
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)


class VerificationContract(unittest.TestCase):
    def test_unpinned_same_length_reference_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "wrong reference"):
            VERIFY.checked_reference(bytes(VERIFY.IMAGE_SIZE))

    def test_equal_prefix_with_missing_tail_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "length"):
            VERIFY.compare(b"\x10\x20", b"\x10", "fixture")

    def test_mismatch_reports_absolute_file_offset(self):
        with self.assertRaisesRegex(ValueError, "0x10017"):
            VERIFY.compare(b"\x10\x20", b"\x10\x21", "fixture")
