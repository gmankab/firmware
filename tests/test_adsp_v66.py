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

    def test_fixed_hexagon_tlb_read_and_lock_control_words(self) -> None:
        self.assertEqual(v66.tlb_read_pair_r5_r4_index_r3(), 0x6c43c004)
        self.assertEqual(v66.tlb_write_pair_r7_r6_index_r3(), 0x6c06c300)
        self.assertEqual(v66.tlb_write_pair_r1_r0_index_r3(), 0x6c00c300)
        self.assertEqual(int(v66.TlbControl.LOCK), 0x6c20c020)
        self.assertEqual(int(v66.TlbControl.UNLOCK), 0x6c20c040)
        for word in [v66.tlb_read_pair_r5_r4_index_r3(),
                     v66.tlb_write_pair_r7_r6_index_r3(),
                     v66.tlb_write_pair_r1_r0_index_r3(),
                     int(v66.TlbControl.LOCK),
                     int(v66.TlbControl.UNLOCK)]:
            self.assertEqual((word >> 14) & 3, 3)

    @unittest.skipUnless(REFERENCE, "ADSP reference required")
    def test_privileged_tlb_ops_match_original_distinct_positions(self) -> None:
        original = Path(REFERENCE).read_bytes()
        expected = {
            0x4900: v66.tlb_read_pair_r5_r4_index_r3(),
            0x4928: int(v66.TlbControl.LOCK),
            0x492c: v66.tlb_write_pair_r7_r6_index_r3(),
            0x4930: int(v66.TlbControl.UNLOCK),
            0x495c: int(v66.TlbControl.LOCK),
            0x4960: v66.tlb_write_pair_r1_r0_index_r3(),
            0x4964: int(v66.TlbControl.UNLOCK),
        }
        for offset, word in expected.items():
            with self.subTest(offset=hex(offset)):
                self.assertEqual(struct.unpack_from("<I", original, offset)[0], word)

    def test_hexagon_v66_physical_word_load_register_fields(self) -> None:
        self.assertEqual(v66.physical_word_load(5,0,4),0x9200E405)
        self.assertEqual(v66.physical_word_load(5,3,4),0x9203E405)
        self.assertEqual(v66.physical_word_load(2,0,1),0x9200E102)
        for reg in (-1,32,99,"r5"):
            with self.assertRaises(v66.HexagonEncodingError):
                v66.physical_word_load(reg,0,4)

    @unittest.skipUnless(REFERENCE,"Exact ADSP reference required")
    def test_three_physical_load_sites_match_reference(self) -> None:
        data=Path(REFERENCE).read_bytes()
        for offset,regs in ((0x4a44,(5,0,4)),(0x4ac0,(5,3,4)),(0x4af4,(2,0,1))):
            with self.subTest(offset=hex(offset)):
                self.assertEqual(struct.unpack_from("<I",data,offset)[0],
                                 v66.physical_word_load(*regs))

    def test_cfgbase_system_register_three_call_sites(self) -> None:
        self.assertEqual(v66.read_system_register(1,v66.SystemRegister.CFGBASE),0x6e9bc001)
        self.assertEqual(v66.read_system_register(0,v66.SystemRegister.CFGBASE),0x6e9bc000)

    @unittest.skipUnless(REFERENCE,"ADSP reference required")
    def test_cfgbase_decoder_agrees_across_four_sites(self) -> None:
        data=Path(REFERENCE).read_bytes()
        for offset,reg in [(0x3928,1),(0x3948,1),(0x40d4,0),(0x4ae4,1)]:
            with self.subTest(offset=hex(offset)):
                self.assertEqual(struct.unpack_from("<I",data,offset)[0],
                                 v66.read_system_register(reg,v66.SystemRegister.CFGBASE))

    def test_pmu_and_controller_s54_register_fields(self) -> None:
        self.assertEqual(v66.read_system_register(23,v66.SystemRegister.SYSCFG),0x6e92c017)
        self.assertEqual(v66.write_system_register(v66.SystemRegister.PMUCFG,23),0x6717c035)
        self.assertEqual(v66.read_system_register(23,v66.SystemRegister.S54),0x6eb6c017)

    @unittest.skipUnless(REFERENCE,"Original ADSP required")
    def test_v66_privileged_controller_registers_match_reference(self) -> None:
        data=Path(REFERENCE).read_bytes()
        checks={
            0x4c00: v66.read_system_register(23,v66.SystemRegister.SYSCFG),
            0x4c04: v66.write_system_register(v66.SystemRegister.PMUCFG,23),
            0x4c0c: v66.write_system_register(v66.SystemRegister.SYSCFG,0),
            0x4c14: v66.read_system_register(23,v66.SystemRegister.S54),
        }
        for off,word in checks.items():
            with self.subTest(offset=hex(off)):
                self.assertEqual(struct.unpack_from("<I",data,off)[0],word)

    def test_isdben_and_debug_break_fields(self) -> None:
        self.assertEqual(v66.read_system_register(1,v66.SystemRegister.ISDBEN),0x6eaac001)
        self.assertEqual(v66.write_system_register(v66.SystemRegister.ISDBEN,1),0x6701c02a)
        self.assertEqual(int(v66.DebugCommand.BREAKPOINT),0x6c20c000)
        self.assertEqual(v66.packet_continuation(
            v66.read_system_register(1,v66.SystemRegister.CFGBASE)),0x6e9b4001)
        with self.assertRaises(v66.HexagonEncodingError):
            v66.packet_continuation(0x6e9b4001)
        with self.assertRaises(v66.HexagonEncodingError):
            v66.packet_continuation(-1)

    @unittest.skipUnless(REFERENCE,"Original reference required")
    def test_debug_control_encodings_match_multiple_calls(self) -> None:
        b=Path(REFERENCE).read_bytes()
        checks={0x4c68:v66.read_system_register(1,v66.SystemRegister.ISDBEN),
                0x39b4:v66.write_system_register(v66.SystemRegister.ISDBEN,1),
                0x4cb4:v66.write_system_register(v66.SystemRegister.ISDBEN,1),
                0x39bc:int(v66.DebugCommand.BREAKPOINT),
                0x3e28:int(v66.DebugCommand.BREAKPOINT),
                0x4cbc:int(v66.DebugCommand.BREAKPOINT),
                0x4c74:v66.packet_continuation(v66.read_system_register(1,v66.SystemRegister.CFGBASE))}
        for off,expected in checks.items():
            with self.subTest(offset=hex(off)):
                self.assertEqual(struct.unpack_from("<I",b,off)[0],expected)

    def test_fixed_operand_hexagon_tlb_write_encoding(self) -> None:
        self.assertEqual(v66.tlb_write_pair_r3_r2_index_r0(), 0x6c02c000)
        self.assertEqual((v66.tlb_write_pair_r3_r2_index_r0() >> 14) & 3, 3)

    @unittest.skipUnless(REFERENCE, "ADSP reference required")
    def test_fixed_tlb_encoding_repeats_at_independent_call_sites(self) -> None:
        data = Path(REFERENCE).read_bytes()
        encoded = struct.pack("<I", v66.tlb_write_pair_r3_r2_index_r0())
        for location in (0x3ebc, 0x4714, 0x4750, 0x477c, 0x478c, 0x47a8, 0x47d0, 0x47ec, 0x4808):
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
