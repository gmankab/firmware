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
        self.assertIn("r2 = ##0x700", source)
        self.assertIn("barrier", source)
        self.assertNotIn(".word", source)
        self.assertNotIn(".incbin", source)

    @unittest.skipUnless(LLVM_MC, "Hexagon-capable llvm-mc unavailable")
    def test_controller_compiles_without_unresolved_relocations(self) -> None:
        generated = adsp.assemble(LLVM_MC, adsp.CONTROLLER_SOURCE, 0x448)
        self.assertEqual(len(generated), 0x448)
        self.assertEqual(generated[0x34:0x38], bytes.fromhex("00c08d52"))
        self.assertNotEqual(generated[0x74:0x78], generated[0x120:0x124])

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         "Hexagon assembler and exact Nabu ADSP reference required")
    def test_controller_is_byte_identical_to_original(self) -> None:
        generated = adsp.assemble(LLVM_MC, adsp.CONTROLLER_SOURCE, 0x448)
        original = Path(REFERENCE).read_bytes()
        adsp.verify_reference(generated, original, 0x4218)
        mutated = bytearray(generated)
        mutated[15] ^= 0x80
        with self.assertRaisesRegex(adsp.AdspError, "mismatch"):
            adsp.verify_reference(bytes(mutated), original, 0x4218)

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
