# SPDX-License-Identifier: GPL-3.0-only
"""Synthetic independent Python checks; no original proprietary payloads."""
from __future__ import annotations

import random
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools/halo"))
import pm_transport as pm


class TestPmTransport(unittest.TestCase):
    def test_five_bytes_four_words_five_host_registers(self):
        values = (0x0102030405, 0x060708090A, 0x0B0C0D0E0F, 0x1011121314)
        wire = bytes(range(1, 21))
        self.assertEqual(pm.words_to_wire(values), wire)
        self.assertEqual(pm.wire_to_words(wire), values)
        self.assertEqual(pm.wire_to_host_registers(wire),
                         (0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F10, 0x11121314))
        self.assertEqual(pm.host_registers_to_wire(pm.wire_to_host_registers(wire)), wire)

    def test_160_single_bit_probes(self):
        for bit in range(160):
            wire = bytearray(20)
            wire[bit // 8] |= 1 << (7 - bit % 8)
            words = pm.wire_to_words(bytes(wire))
            self.assertEqual(pm.words_to_wire(words), bytes(wire))
            self.assertEqual(pm.host_registers_to_wire(pm.wire_to_host_registers(bytes(wire))), bytes(wire))

    def test_wide_word_and_host_register_rejected(self):
        with self.assertRaises(ValueError):
            pm.words_to_wire((1 << 40, 0, 0, 0))
        with self.assertRaises(ValueError):
            pm.host_registers_to_wire((1 << 32, 0, 0, 0, 0))
        with self.assertRaises(ValueError):
            pm.wire_to_host_registers(bytes(19))
        with self.assertRaises(ValueError):
            pm.wire_to_words(bytes(21))

    def test_word_byte_lane_crossing_and_limits(self):
        for i, (first, lane, last) in enumerate(((0, 0, 4), (4, 1, 8), (8, 2, 12), (12, 3, 16))):
            v = pm.byte_address_for_pm_word(i)
            self.assertEqual(v, (pm.BASE + first, lane, pm.BASE + last))
        self.assertEqual(pm.byte_address_for_pm_word(pm.WORDS - 1),
                         (pm.PM_REGISTER_END - 8, 3, pm.PM_REGISTER_END - 4))
        for invalid in (-1, pm.WORDS, 2**256):
            with self.assertRaises(ValueError):
                pm.byte_address_for_pm_word(invalid)

    def test_random_roundtrip_blocks(self):
        rand = random.Random(7654321)
        for _ in range(350):
            wire = rand.randbytes(20)
            self.assertEqual(pm.words_to_wire(pm.wire_to_words(wire)), wire)
            self.assertEqual(pm.host_registers_to_wire(pm.wire_to_host_registers(wire)), wire)

    def test_wmfw_rejects_bad_header_or_layout(self):
        for bad in (b"", b"WMFW", bytes(34236)):
            with self.assertRaises(ValueError):
                pm.parse_wmfw_header(bad)
        data = bytearray(pm.WMFW_LENGTH)
        data[:4] = b"WMFW"
        data[4:8] = (40).to_bytes(4, "little")
        data[10:12] = bytes((4, 3))
        with self.assertRaises(ValueError):
            pm.parse_wmfw_header(data)

    def test_full_audit_rejects_nonreference_no_hidden_reuse(self):
        with self.assertRaises(ValueError):
            pm.audit_blob(bytes(pm.WMFW_LENGTH))


if __name__ == "__main__":
    unittest.main()
