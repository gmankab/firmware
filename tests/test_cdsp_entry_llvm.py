# SPDX-License-Identifier: GPL-3.0-only
"""Pure-Python CDSP verifier tests; do not require a private firmware file."""
import importlib.util
from pathlib import Path
import struct
import unittest

REPO = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("cdsp_hexagon_roundtrip", REPO / "tools" / "verify-cdsp-hexagon-entry.py")
cdsp = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(cdsp)


class HexagonCdspEntryTests(unittest.TestCase):
    def test_has_eight_jumps_and_one_register_init(self):
        self.assertEqual(len(cdsp.PACKETS), 9)
        self.assertEqual(cdsp.PACKETS[0], "jump 0x1c")
        self.assertEqual(cdsp.PACKETS[-1], "r0 = #0")
        self.assertEqual(cdsp.make_asm().count("{ "), 9)

    def test_rejects_non_elf_and_short_headers(self):
        for data in (b"", b"\x7fELF", bytes(100)):
            with self.assertRaises(ValueError):
                cdsp.parse_elf32_text(data)

    def test_rejects_truncated_section_records(self):
        data = bytearray(52)
        data[:6] = b"\x7fELF\x01\x01"
        struct.pack_into("<I", data, 32, 50)
        struct.pack_into("<H", data, 46, 40)
        struct.pack_into("<H", data, 48, 1)
        with self.assertRaises(ValueError):
            cdsp.parse_elf32_text(bytes(data))


if __name__ == "__main__":
    unittest.main()
