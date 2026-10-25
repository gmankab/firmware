# SPDX-License-Identifier: GPL-3.0-only
"""Typed Hexagon V66 control-register encoder regression tests."""
from __future__ import annotations

import importlib.util
import os
from pathlib import Path
import struct
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
ENCODER = ROOT / "tools" / "adsp_hexagon_v66.py"
spec = importlib.util.spec_from_file_location("adsp_v66", ENCODER)
assert spec and spec.loader
v66 = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = v66
spec.loader.exec_module(v66)
REFERENCE = os.environ.get("NABU_ADSP_REFERENCE")


class V66BootstrapTests(unittest.TestCase):
    def test_register_encodings_distinguish_source_and_destination(self) -> None:
        syscfg = v66.SystemRegister.SYSCFG
        self.assertEqual(v66.read_system_register(0, syscfg), 0x6E92C000)
        self.assertEqual(v66.read_system_register(15, syscfg), 0x6E92C00F)
        self.assertEqual(v66.write_system_register(syscfg, 0), 0x6700C012)
        self.assertEqual(v66.write_system_register(syscfg, 15), 0x670FC012)

    def test_disallow_bad_reg_and_bit_ranges(self) -> None:
        for invalid in (-1, 32, 999, "r0"):
            with self.subTest(register=invalid):
                with self.assertRaises(v66.HexagonEncodingError):
                    v66.read_system_register(invalid, v66.SystemRegister.SSR)
        for invalid in (-1, 32, 2.5):
            with self.subTest(bit=invalid):
                with self.assertRaises(v66.HexagonEncodingError):
                    v66.set_bit(0, 0, invalid)

    def test_near_jump_alignment_and_range(self) -> None:
        self.assertEqual(v66.jump_near(0x40, 0x63c), 0x5800C2FE)
        with self.assertRaises(v66.HexagonEncodingError):
            v66.jump_near(0x40, 0x643)
        with self.assertRaises(v66.HexagonEncodingError):
            v66.jump_near(0x800, 0x200)
        with self.assertRaises(v66.HexagonEncodingError):
            v66.jump_near(0, 0x2000)

    def test_all_v66_boot_ops_terminate_hexagon_packets(self) -> None:
        code = v66.boot_special_code()
        self.assertEqual(len(code), 32)
        # 0b11 parse bits are a single-instruction terminating packet.
        for word in struct.unpack("<8I", code):
            self.assertEqual((word >> 14) & 3, 3)

    def test_fixed_operand_hexagon_tlb_write_encoding(self) -> None:
        self.assertEqual(v66.tlb_write_pair_r3_r2_index_r0(), 0x6c02c000)
        self.assertEqual((v66.tlb_write_pair_r3_r2_index_r0() >> 14) & 3, 3)

    @unittest.skipUnless(REFERENCE, "ADSP reference required")
    def test_fixed_tlb_encoding_repeats_at_independent_call_sites(self) -> None:
        data = Path(REFERENCE).read_bytes()
        encoded = struct.pack("<I", v66.tlb_write_pair_r3_r2_index_r0())
        for location in (0x3ebc, 0x4714, 0x4750, 0x477c, 0x478c):
            with self.subTest(location=hex(location)):
                self.assertEqual(data[location:location + 4], encoded)

    def test_syscfg_handoff_steps_use_explicit_source_and_operand_boundaries(self) -> None:
        steps = v66.syscfg_handoff_steps()
        self.assertEqual(sum(size for _, _, size in steps), 84)
        self.assertEqual(sum(kind == "isa" for kind, _, _ in steps), 6)
        self.assertTrue(all(kind in ("isa", "asm") for kind, _, _ in steps))
        for kind, operation, size in steps:
            with self.subTest(operation=operation):
                self.assertIn(size, (4, 12))
                if kind == "isa":
                    self.assertIsInstance(operation, int)
                    self.assertEqual((operation >> 14) & 3, 3)
                else:
                    self.assertIsInstance(operation, str)
                    self.assertIn("{", operation)
        self.assertEqual(steps[-1][1], "{ jumpr lr }")

    @unittest.skipUnless(REFERENCE, "ADSP reference required")
    def test_instructions_match_two_independent_adsp_call_sites(self) -> None:
        original = Path(REFERENCE).read_bytes()
        self.assertEqual(
            v66.boot_special_code(), original[0x3024:0x3044]
        )
        self.assertEqual(
            v66.read_system_register(15, v66.SystemRegister.SYSCFG),
            struct.unpack_from("<I", original, 0x46A0)[0]
        )
        self.assertEqual(
            v66.write_system_register(v66.SystemRegister.SYSCFG, 15),
            struct.unpack_from("<I", original, 0x46A8)[0]
        )


if __name__ == "__main__":
    unittest.main()
