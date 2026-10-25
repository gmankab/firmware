# SPDX-License-Identifier: GPL-3.0-only
"""Regression tests for the independently authored ADSP bootstrap fragment."""
from __future__ import annotations

import importlib.util
import os
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "tools" / "emit-adsp-entry.py"
ASM = ROOT / "tools" / "adsp-boot-vectors.S"

spec = importlib.util.spec_from_file_location("adsp_boot", SCRIPT)
assert spec is not None and spec.loader is not None
adsp = importlib.util.module_from_spec(spec)
sys.modules["adsp_boot"] = adsp
spec.loader.exec_module(adsp)

LLVM_MC = os.environ.get("HEXAGON_LLVM_MC") or shutil.which("llvm-mc")
REFERENCE = os.environ.get("NABU_ADSP_REFERENCE")


class ADSPBootstrapTests(unittest.TestCase):
    def test_reject_wrong_machine_and_endianness(self) -> None:
        elf = bytearray(52)
        elf[:6] = b"\x7fELF\x01\x01"
        struct.pack_into("<HHI", elf, 16, 1, 164, 1)
        struct.pack_into("<H", elf, 40, 52)
        self.assertEqual(adsp.elf32_header(elf)["machine"], 164)
        struct.pack_into("<H", elf, 18, 40)
        with self.assertRaisesRegex(adsp.AdspError, "Hexagon"):
            adsp.elf32_header(elf)
        elf[5] = 2
        with self.assertRaisesRegex(adsp.AdspError, "little-endian"):
            adsp.elf32_header(elf)

    def test_checked_range_rejects_overlap_beyond_file(self) -> None:
        for off, length in ((-1, 4), (12, -1), (12, 5), (17, 0)):
            with self.subTest(off=off, length=length):
                with self.assertRaises(adsp.AdspError):
                    adsp.checked_interval(16, off, length, "test")
        adsp.checked_interval(16, 0, 16, "test")

    def test_source_is_meaningful_assembly(self) -> None:
        source = ASM.read_text()
        self.assertIn("bootstrap_start:", source)
        self.assertIn("jump trap_vector_1", source)
        self.assertIn("r0 = #0", source)
        self.assertNotIn(".incbin", source)
        self.assertNotIn(".word", source)
        self.assertNotIn(".byte", source)

    @unittest.skipUnless(LLVM_MC, "Hexagon-capable llvm-mc unavailable")
    def test_compiler_emits_nine_original_instructions(self) -> None:
        generated = adsp.assemble(LLVM_MC)
        self.assertEqual(len(generated), 9 * 4)
        # Assert packet structure and the six self-loop trap slots. These are
        # structural checks, not a fallback emitter of vendor opcodes.
        self.assertEqual(generated[4:8], generated[8:12])
        self.assertEqual(generated[4:8], generated[24:28])

    def test_controller_source_uses_packets_and_predicated_branches(self) -> None:
        source = adsp.CONTROLLER_SOURCE.read_text()
        self.assertIn("poll_timer:", source)
        self.assertIn("if (!p0.new) jump:t poll_timer", source)
        self.assertIn("secondary_mode:", source)
        self.assertIn("primary_mode:", source)
        self.assertIn("fallback_mode:", source)
        self.assertIn("memw(r0+#0) = ##0x20485699", source)
        self.assertIn("dcinva(r7)", source)
        self.assertIn("poll_legacy_controller:", source)
        self.assertIn("sp = add(pc,##0x20458)", source)
        self.assertIn("r2 = ##0x700", source)
        self.assertIn("barrier", source)
        self.assertIn("adsp_interrupt_vector_init:", source)
        self.assertNotIn(".word", source)
        self.assertNotIn(".incbin", source)

    @unittest.skipUnless(LLVM_MC, "Hexagon-capable llvm-mc unavailable")
    def test_controller_compiles_without_unresolved_relocations(self) -> None:
        generated = adsp.assemble(LLVM_MC, adsp.CONTROLLER_SOURCE, 0x4c0)
        self.assertEqual(len(generated), 0x4c0)
        self.assertEqual(generated[0x6c:0x70], bytes.fromhex("00c08d52"))
        self.assertNotEqual(generated[0xac:0xb0], generated[0x158:0x15c])

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         "Hexagon assembler and exact Nabu ADSP reference required")
    def test_controller_is_byte_identical_to_original(self) -> None:
        generated = (adsp.assemble(LLVM_MC, adsp.CONTROLLER_SOURCE, 0x4c0) +
                     adsp.assemble_syscfg_handoff(LLVM_MC) +
                     adsp.assemble_tlb_map(LLVM_MC) +
                     adsp.assemble_tlb_flush(LLVM_MC) +
                     adsp.assemble(LLVM_MC, adsp.CACHELINE_COPY_SOURCE, 0x78) +
                     adsp.assemble(LLVM_MC, adsp.MEMORY_COPY_SOURCE, 0x40) +
                     adsp.assemble_tlb_reconcile(LLVM_MC))
        self.assertEqual(len(generated), 0x7a0)
        original = Path(REFERENCE).read_bytes()
        adsp.verify_reference(generated, original, 0x41e0)
        mutated = bytearray(generated)
        mutated[15] ^= 0x80
        with self.assertRaisesRegex(adsp.AdspError, "mismatch"):
            adsp.verify_reference(bytes(mutated), original, 0x41e0)

    @unittest.skipUnless(LLVM_MC, "Hexagon-capable llvm-mc unavailable")
    def test_syscfg_handoff_uses_typed_and_source_assembly(self) -> None:
        handoff = adsp.assemble_syscfg_handoff(LLVM_MC)
        self.assertEqual(len(handoff), 84)
        self.assertEqual(handoff[:4], struct.pack("<I", 0x6e92c00f))
        self.assertEqual(handoff[8:12], struct.pack("<I", 0x670fc012))
        self.assertEqual(handoff[20:24], struct.pack("<I", 0x57c0c002))
        # Instruction packets compiled independently by LLVM6 (one
        # 12-byte extender+dual-operation packet at file 0x46cc).
        self.assertEqual(
            handoff[0x2c:0x38],
            bytes.fromhex("5b40000f1e4400781fd91ff3"))
        self.assertEqual(handoff[-4:], struct.pack("<I", 0x529fc000))

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         "Hexagon assembler and exact Nabu ADSP reference required")
    def test_syscfg_handoff_matches_reference_and_rejects_wrong_revision(self) -> None:
        handoff = adsp.assemble_syscfg_handoff(LLVM_MC)
        ref = Path(REFERENCE).read_bytes()
        adsp.verify_reference(handoff, ref, 0x46a0)
        bad = bytearray(ref)
        bad[0x46b0] ^= 1
        with self.assertRaisesRegex(adsp.AdspError, "Wrong Xiaomi Nabu ADSP revision"):
            adsp.verify_reference(handoff, bytes(bad), 0x46a0)

    def test_tlb_source_has_three_checked_assembler_sentinels(self) -> None:
        source = adsp.TLB_MAP_SOURCE.read_text()
        self.assertEqual(source.count("{ nop }"), 3)
        self.assertIn("tlb_loop:", source)
        self.assertIn("adsp_tlb_map_7f:", source)
        self.assertIn("adsp_tlb_map_7e:", source)
        self.assertNotIn(".word", source)
        self.assertNotIn(".incbin", source)

    @unittest.skipUnless(LLVM_MC, "Hexagon-capable llvm-mc unavailable")
    def test_tlb_placeholder_replacement_is_exact_and_guarded(self) -> None:
        generated = adsp.assemble_tlb_map(LLVM_MC)
        self.assertEqual(len(generated), 0x94)
        special = struct.pack("<I", 0x6c02c000)
        for offset in (0x20, 0x5c, 0x88):
            self.assertEqual(generated[offset:offset+4], special)
        self.assertNotIn(struct.pack("<I", 0x7f00c000), generated)
        with tempfile.TemporaryDirectory() as td:
            tampered = Path(td) / "mismatched-tlb-source.S"
            tampered.write_text(adsp.TLB_MAP_SOURCE.read_text().replace(
                "{ nop }", "{ isync }", 1))
            with self.assertRaisesRegex(adsp.AdspError, "Expected TLB placeholder"):
                adsp.assemble_tlb_map(LLVM_MC, tampered)

    @unittest.skipUnless(LLVM_MC and REFERENCE, "Exact ADSP reference required")
    def test_complete_tlb_region_matches_reference(self) -> None:
        generated = adsp.assemble_tlb_map(LLVM_MC)
        adsp.verify_reference(generated, Path(REFERENCE).read_bytes(), 0x46f4)

    def test_tlb_flush_source_has_five_source_level_sentinels(self) -> None:
        source = adsp.TLB_FLUSH_SOURCE.read_text()
        self.assertEqual(source.count("{ nop }"), 5)
        self.assertIn("adsp_tlb_clear_one:", source)
        self.assertIn("walk_table1:", source)
        self.assertIn("walk_table2:", source)
        self.assertNotIn(".word", source)
        self.assertNotIn(".incbin", source)

    @unittest.skipUnless(LLVM_MC, "Hexagon-capable llvm-mc unavailable")
    def test_tlb_flush_sentinels_are_replaced_and_guarded(self) -> None:
        generated = adsp.assemble_tlb_flush(LLVM_MC)
        self.assertEqual(len(generated), 0x90)
        special = struct.pack("<I", 0x6c02c000)
        for offset in (0x04, 0x20, 0x48, 0x64, 0x80):
            self.assertEqual(generated[offset:offset + 4], special)
        with tempfile.TemporaryDirectory() as td:
            tampered = Path(td) / "bad-tlb-flush.S"
            tampered.write_text(adsp.TLB_FLUSH_SOURCE.read_text().replace(
                "{ nop }", "{ isync }", 1))
            with self.assertRaisesRegex(adsp.AdspError, "Expected TLB placeholder"):
                adsp.assemble_tlb_flush(LLVM_MC, tampered)

    @unittest.skipUnless(LLVM_MC and REFERENCE, "Exact ADSP reference required")
    def test_independently_assembled_tlb_flush_matches_original(self) -> None:
        out = adsp.assemble_tlb_flush(LLVM_MC)
        adsp.verify_reference(out, Path(REFERENCE).read_bytes(), 0x4788)

    def test_memory_copy_source_contains_typed_hardware_loop(self) -> None:
        source = adsp.MEMORY_COPY_SOURCE.read_text()
        self.assertIn("adsp_memory_copy:", source)
        self.assertIn("loop0(copy_loop,r2)", source)
        self.assertIn("memd(r1++#8)", source)
        self.assertIn("memd(r0++#8)", source)
        self.assertIn(":endloop0", source)
        self.assertNotIn(".word", source)
        self.assertNotIn(".incbin", source)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         "Exact ADSP reference and Hexagon assembler required")
    def test_memory_copy_source_matches_reference_and_loop_packet(self) -> None:
        generated = adsp.assemble(LLVM_MC, adsp.MEMORY_COPY_SOURCE, 0x40)
        adsp.verify_reference(generated, Path(REFERENCE).read_bytes(), 0x4890)
        # The last loop packet is a NOP + postincremented memd store with
        # endloop0 encoding; instruction packets remain source-generated.
        self.assertEqual(generated[0x34:0x3c], bytes.fromhex("0080007f08c4c0ab"))
        self.assertEqual(generated[-4:], struct.pack("<I", 0x529fc000))

    def test_nested_cacheline_hardware_loop_source(self) -> None:
        source = adsp.CACHELINE_COPY_SOURCE.read_text()
        self.assertIn("loop1(outer_loop,r6)", source)
        self.assertIn("loop0(inner_loop,r3)", source)
        self.assertIn("dczeroa(r1)", source)
        self.assertIn("l2fetch(r1,r7)", source)
        self.assertIn("}:endloop0:endloop1", source)
        self.assertEqual(source.count("{ nop }"), 3)
        self.assertNotIn(".incbin", source)
        self.assertNotIn(".word", source)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         "Exact ADSP reference and Hexagon assembler required")
    def test_nested_cacheline_copy_assembles_identically(self) -> None:
        generated = adsp.assemble(LLVM_MC, adsp.CACHELINE_COPY_SOURCE, 0x78)
        adsp.verify_reference(generated, Path(REFERENCE).read_bytes(), 0x4818)
        # Both the loop packets and exact aligned NOPs come from source.
        self.assertEqual(generated[-12:],
                         struct.pack("<III", 0x7F00C000, 0x7F00C000, 0x7F00C000))

    def test_tlb_reconciliation_source_and_loop_packet(self) -> None:
        source = adsp.TLB_RECONCILE_SOURCE.read_text()
        self.assertIn("adsp_tlb_reconcile:", source)
        self.assertIn("loop0(recon_loop,#127)", source)
        self.assertIn("}:endloop0", source)
        self.assertIn("source_loop:", source)
        self.assertEqual(source.count("{ nop }"), 7)
        self.assertNotIn(".word", source)
        self.assertNotIn(".incbin", source)

    @unittest.skipUnless(LLVM_MC, "Hexagon-capable llvm-mc unavailable")
    def test_tlb_reconciliation_replaces_exactly_seven_privileged_slots(self) -> None:
        generated = adsp.assemble_tlb_reconcile(LLVM_MC)
        self.assertEqual(len(generated), 0xB0)
        expected = {
            0x30: 0x6c43c004, 0x58: 0x6c20c020,
            0x5c: 0x6c06c300, 0x60: 0x6c20c040,
            0x8c: 0x6c20c020, 0x90: 0x6c00c300,
            0x94: 0x6c20c040,
        }
        for offset, word in expected.items():
            with self.subTest(at=hex(offset)):
                self.assertEqual(
                    generated[offset:offset + 4], struct.pack("<I", word))
        self.assertEqual(generated[0x68:0x6c], struct.pack("<I", 0x7f00c000))
        with tempfile.TemporaryDirectory() as td:
            tampered = Path(td) / "wrong-tlb-reconcile.S"
            tampered.write_text(adsp.TLB_RECONCILE_SOURCE.read_text().replace(
                "{ nop }", "{ isync }", 1))
            with self.assertRaisesRegex(adsp.AdspError, "Expected privileged TLB placeholder"):
                adsp.assemble_tlb_reconcile(LLVM_MC, tampered)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         "Exact Nabu ADSP reference and assembler required")
    def test_tlb_reconciliation_matches_exact_adsp_reference(self) -> None:
        adsp.verify_reference(adsp.assemble_tlb_reconcile(LLVM_MC),
                              Path(REFERENCE).read_bytes(), 0x48d0)

    @unittest.skipUnless(LLVM_MC, "Hexagon-capable llvm-mc unavailable")
    def test_reject_relocations_and_malformed_section_offsets(self) -> None:
        with tempfile.TemporaryDirectory() as t:
            output = Path(t) / "boot.o"
            subprocess.run(
                [LLVM_MC, "-triple=hexagon", "-mcpu=hexagonv65",
                 "-filetype=obj", str(ASM), "-o", str(output)],
                check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
            )
            original = bytearray(output.read_bytes())
            hdr = adsp.elf32_header(original)
            # Force section headers to reference past end of file.
            struct.pack_into("<I", original, 32, len(original) - 4)
            with self.assertRaisesRegex(adsp.AdspError, "section headers"):
                adsp.read_text_section(original)
            self.assertEqual(len(adsp.read_text_section(output.read_bytes())), 36)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         "Hexagon assembler and exact Nabu ADSP reference required")
    def test_combined_v66_boot_prefix(self) -> None:
        # The first nine instructions are compiled by LLVM; the following
        # system/cache operations use checked, typed V66 encoders.
        sys.path.insert(0, str(ROOT / "tools"))
        try:
            from adsp_hexagon_v66 import boot_special_code
            generated = adsp.assemble(LLVM_MC) + boot_special_code()
        finally:
            sys.path.pop(0)
        self.assertEqual(len(generated), 68)
        adsp.verify_reference(generated, Path(REFERENCE).read_bytes(), 0x3000)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         "Hexagon assembler and exact Nabu ADSP reference required")
    def test_match_original_and_reject_altered_entry(self) -> None:
        expected = adsp.assemble(LLVM_MC)
        original = Path(REFERENCE).read_bytes()
        adsp.verify_reference(expected, original)
        mutated = bytearray(expected)
        mutated[-1] ^= 1
        with self.assertRaisesRegex(adsp.AdspError, "mismatch"):
            adsp.verify_reference(bytes(mutated), original)


if __name__ == "__main__":
    unittest.main()
