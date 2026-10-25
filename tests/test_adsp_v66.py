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

    def test_architectural_restore_register_fields(self) -> None:
        self.assertEqual(v66.read_system_register(24,v66.SystemRegister.PMUEVTCFG),0x6eb4c018)
        self.assertEqual(v66.read_system_register(29,v66.SystemRegister.S47),0x6eafc01d)
        self.assertEqual(v66.read_system_register_pair(0,48),0x6f30c000)
        self.assertEqual(v66.read_system_register_pair(24,44),0x6f2cc018)
        self.assertEqual(v66.read_system_register_pair(30,50),0x6f32c01e)
        self.assertEqual(v66.write_predicate_group_from_gpr(24),0x6238c004)
        self.assertEqual(v66.start_thread_from_register(23),0x6477c020)
        for word in (
            v66.read_system_register_pair(0,48),v66.read_system_register_pair(24,44),
            v66.read_system_register_pair(30,50),v66.write_predicate_group_from_gpr(24),
            v66.start_thread_from_register(23)):
            self.assertEqual((word >> 14)&3,3)

    def test_htid_fields_and_packet_continuation(self) -> None:
        self.assertEqual(v66.read_system_register(23,v66.SystemRegister.HTID),0x6e88c017)
        self.assertEqual(v66.read_system_register(22,v66.SystemRegister.HTID),0x6e88c016)
        self.assertEqual(v66.packet_continuation(
            v66.read_system_register(22,v66.SystemRegister.HTID)),0x6e884016)
        self.assertEqual(v66.read_system_register(22,v66.SystemRegister.S54),0x6eb6c016)

    @unittest.skipUnless(REFERENCE,"Exact firmware reference required")
    def test_htid_register_reads_at_distinct_boot_sites(self) -> None:
        b=Path(REFERENCE).read_bytes()
        for offset,rd,cont in ((0x4e1c,23,False),(0x4e48,22,True),
                                (0x4e60,23,False),(0x4f10,23,False)):
            with self.subTest(offset=hex(offset)):
                value=v66.read_system_register(rd,v66.SystemRegister.HTID)
                if cont:value=v66.packet_continuation(value)
                self.assertEqual(struct.unpack_from("<I",b,offset)[0],value)

    def test_system_register_pair_rejects_invalid_register_width(self) -> None:
        for dst,sys in [(-1,48),(1,48),(31,44),(32,44),(0,-2),(0,45),(0,64),
                        (0,"S48"),("r0",44)]:
            with self.subTest(dst=dst,sys=sys):
                with self.assertRaises(v66.HexagonEncodingError):
                    v66.read_system_register_pair(dst,sys)
        for bad in [-1,32,"r23"]:
            with self.subTest(source=bad):
                with self.assertRaises(v66.HexagonEncodingError):
                    v66.write_predicate_group_from_gpr(bad)
                with self.assertRaises(v66.HexagonEncodingError):
                    v66.start_thread_from_register(bad)

    @unittest.skipUnless(REFERENCE,"Exact reference required")
    def test_privileged_restore_instructions_at_all_seven_offsets(self) -> None:
        ref=Path(REFERENCE).read_bytes()
        operands={
            0x4ce8:v66.read_system_register(24,v66.SystemRegister.PMUEVTCFG),
            0x4cec:v66.write_predicate_group_from_gpr(24),
            0x4cf0:v66.read_system_register_pair(0,48),
            0x4cf4:v66.read_system_register_pair(24,44),
            0x4cf8:v66.read_system_register(29,v66.SystemRegister.S47),
            0x4cfc:v66.read_system_register_pair(30,50),
            0x4d08:v66.start_thread_from_register(23),
        }
        for off,encoded in operands.items():
            with self.subTest(offset=hex(off)):
                self.assertEqual(struct.unpack_from("<I",ref,off)[0],encoded)

    def test_hexagon_control_bank_fields_and_pair_width(self) -> None:
        expected={
            v66.ControlRegister.P3_0:0x6a04c002,
            v66.ControlRegister.GP:0x6a0bc002,
        }
        for ctrl,word in expected.items():
            self.assertEqual(v66.read_control_register(2,ctrl),word)
        self.assertEqual(v66.read_control_register(3,v66.ControlRegister.USR),0x6a08c003)
        self.assertEqual(v66.read_control_register(3,v66.ControlRegister.UGP),0x6a0ac003)
        for ctr,word in ((v66.ControlRegisterPair.LC0_SA0,0x6800c002),
                         (v66.ControlRegisterPair.LC1_SA1,0x6802c002),
                         (v66.ControlRegisterPair.M1_0,0x6806c002),
                         (v66.ControlRegisterPair.CS1_0,0x680cc002)):
            self.assertEqual(v66.read_control_register_pair(2,ctr),word)
        for bad_dest in [-1,1,31,32,"R0"]:
            with self.assertRaises(v66.HexagonEncodingError):
                v66.read_control_register_pair(bad_dest,v66.ControlRegisterPair.M1_0)
        for bad in [3,"P3_0",v66.ControlRegisterPair.CS1_0]:
            with self.assertRaises(v66.HexagonEncodingError):
                v66.read_control_register(0,bad)
        for bad in [3,"M1_0",v66.ControlRegister.USR]:
            with self.assertRaises(v66.HexagonEncodingError):
                v66.read_control_register_pair(0,bad)

    @unittest.skipUnless(REFERENCE,"Exact Nabu reference required")
    def test_context_capture_register_fields_and_packet_positions(self) -> None:
        b=Path(REFERENCE).read_bytes()
        variants={
            0x4e9c:v66.packet_continuation(v66.read_control_register(2,v66.ControlRegister.P3_0)),
            0x4ea8:v66.packet_continuation(v66.read_control_register(3,v66.ControlRegister.USR)),
            0x4eb4:v66.packet_continuation(v66.read_control_register(2,v66.ControlRegister.GP)),
            0x4ec0:v66.packet_continuation(v66.read_control_register(3,v66.ControlRegister.UGP)),
            0x4ecc:v66.packet_continuation(v66.read_control_register_pair(2,v66.ControlRegisterPair.LC0_SA0)),
            0x4ed4:v66.packet_continuation(v66.read_control_register_pair(2,v66.ControlRegisterPair.LC1_SA1)),
            0x4edc:v66.packet_continuation(v66.read_control_register_pair(2,v66.ControlRegisterPair.M1_0)),
            0x4ee8:v66.read_control_register_pair(2,v66.ControlRegisterPair.CS1_0),
            0x4eec:v66.packet_continuation(v66.read_system_register(2,v66.SystemRegister.CCR)),
            0x4ef4:v66.packet_continuation(v66.read_system_register(3,v66.SystemRegister.SSR)),
            0x4efc:v66.read_system_register(2,v66.SystemRegister.ELR),
            0x4f00:v66.packet_continuation(v66.read_system_register(2,v66.SystemRegister.BADVA)),
            0x4f08:v66.read_system_register(3,v66.SystemRegister.STID),
        }
        for loc,ins in variants.items():
            with self.subTest(offset=hex(loc)):
                self.assertEqual(struct.unpack_from("<I",b,loc)[0],ins)

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

    def test_event_vector_base_system_register_fields(self) -> None:
        self.assertEqual(v66.write_system_register(v66.SystemRegister.EVB,1),0x6701c010)
        self.assertEqual(v66.write_system_register(v66.SystemRegister.EVB,0),0x6700c010)

    @unittest.skipUnless(REFERENCE,"Exact reference required")
    def test_evb_assignments_at_two_assembler_independent_sites(self) -> None:
        b=Path(REFERENCE).read_bytes()
        for offset,reg in ((0x3c4c,0),(0x4cc8,1)):
            with self.subTest(offset=hex(offset)):
                self.assertEqual(struct.unpack_from("<I",b,offset)[0],
                    v66.write_system_register(v66.SystemRegister.EVB,reg))

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
