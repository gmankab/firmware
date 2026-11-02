# SPDX-License-Identifier: GPL-3.0-only
"""Regression checks for modem Hexagon post-TLB system initialization."""
import importlib.util
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "tools/assemble-modem-system.py"
SPEC = importlib.util.spec_from_file_location("modem_system_assembler", SCRIPT)
modem_system = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(modem_system)


class ModemSystemBuildTests(unittest.TestCase):
    def test_instruction_region_is_128_bytes(self):
        self.assertEqual(modem_system.PROGRAM_LENGTH, 128)
        self.assertEqual(modem_system.FILE_START, 0x35c4)
        self.assertEqual(modem_system.FILE_END, 0x3644)

    def test_prefix_length_is_contiguous(self):
        self.assertEqual(modem_system.PREFIX_LENGTH, 1476)
        self.assertEqual(modem_system.PROGRAM_LENGTH+modem_system.PREFIX_LENGTH, 1604)

    def test_hash_check_rejects_wrong_firmware_revision(self):
        with self.assertRaisesRegex(ValueError, 'SHA-256 mismatch'):
            modem_system.validate_reference(b'wrong modem revision', b'abc', 0x35c4)

    def test_source_uses_typed_system_pair_fields(self):
        source = (ROOT/'sources/xiaomi-nabu/modem/modem_sys_boot.s').read_text()
        self.assertIn('HEX_SYSTEM_PAIR_WRITE 30,2',source)
        self.assertIn('HEX_SYSTEM_PAIR_WRITE 4,2',source)
        self.assertIn('r3:2 = combine(#0,#0)', source)
        self.assertIn('call modem_followup',source)
        self.assertNotIn('.incbin',source)


if __name__ == '__main__': unittest.main()
