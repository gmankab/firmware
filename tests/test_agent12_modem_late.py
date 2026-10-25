# SPDX-License-Identifier: GPL-3.0-only
"""Source-only regression tests for isolated later modem Hexagon routines."""
import importlib.util
import os
from pathlib import Path
import struct
import unittest

ROOT = Path(__file__).resolve().parents[1]
TOOL = ROOT / "tools" / "assemble-modem-late.py"
spec = importlib.util.spec_from_file_location("assemble_modem_late", TOOL)
assert spec and spec.loader
agent12 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(agent12)


class ModemLateTests(unittest.TestCase):
    def test_regions_are_disjoint_and_bounded_by_executable_segments(self):
        self.assertEqual((agent12.ATOMIC_START, agent12.ATOMIC_END), (0x7C000, 0x7C040))
        self.assertEqual((agent12.AUX_START, agent12.AUX_END), (0x446000, 0x4460D8))
        self.assertEqual(agent12.ATOMIC_END - agent12.ATOMIC_START, 64)
        self.assertEqual(agent12.AUX_END - agent12.AUX_START, 216)
        self.assertLess(agent12.ATOMIC_END, agent12.AUX_START)
        self.assertLessEqual(agent12.AUX_END, 0x446000 + 0x11DA4)

    def test_atomic_packet_reorder_checks_opcode_class_and_parse_bits(self):
        words = [0x7800C000] * 16  # synthetic A2_tfrsi instruction for surrounding words
        # Architecturally typed synthetic Hexagon instruction classes, not OEM reference data.
        words[4] = 0x10004000  # J4_predicated test+branch: continuation marker
        words[5] = 0x7400C000  # A2_predicated arithmetic: end-of-packet marker
        first = struct.pack("<16I", *words)
        out = agent12.restore_atomic_packet_order(first)
        decoded = list(struct.unpack("<16I", out))
        self.assertEqual(decoded[4] & agent12.MARKER_MASK, agent12.CONTINUE_MARKER)
        self.assertEqual(decoded[5] & agent12.MARKER_MASK, agent12.END_MARKER)
        self.assertEqual(decoded[4] >> 24, 0x74)
        self.assertEqual(decoded[5] >> 24, 0x10)
        self.assertEqual(decoded[0:4], words[0:4])
        self.assertEqual(decoded[6:], words[6:])
        with self.assertRaisesRegex(ValueError, "opcode|comparison-branch"):
            agent12.restore_atomic_packet_order(first[:16] + struct.pack("<I", 0x78004000) + first[20:])
        with self.assertRaisesRegex(ValueError, "parse markers"):
            agent12.restore_atomic_packet_order(first[:16] + struct.pack("<I", 0x1000C000) + first[20:])
        with self.assertRaisesRegex(ValueError, "64 bytes"):
            agent12.restore_atomic_packet_order(first[:60])

    def test_sources_use_packets_and_real_control_register_semantics(self):
        a = agent12.ATOMIC_SOURCE.read_text()
        b = agent12.AUX_SOURCE.read_text()
        self.assertIn("memw_locked(r0)", a)
        self.assertIn("if (!p0.new) r3 = add", a)
        self.assertIn("memb(##3249533708) = r2.new", a)
        self.assertIn("framelimit", b)
        self.assertIn("HEX_WRITE_FRAMELIMIT 0", b)
        self.assertIn("r29 = add(pc,##82752)", b)
        self.assertIn("memd(r0++#8) = r5:4", b)
        self.assertIn(".Lbyte_loop", b)
        self.assertIn(".Lword_loop", b)

    def test_rejects_bad_executable_elf_magic_and_isa(self):
        with self.assertRaisesRegex(ValueError, "ELF32"):
            agent12.executable_segments(b"not an ELF")
        with self.assertRaisesRegex(ValueError, "Hexagon"):
            obj = bytearray(52)
            obj[:6] = b"\x7fELF\x01\x01"
            struct.pack_into("<H", obj, 18, 40)  # ARM EABI, not Hexagon
            agent12.executable_segments(bytes(obj))

    def test_rejects_unpinned_or_truncated_reference(self):
        with self.assertRaisesRegex(ValueError, "wrong modem firmware revision"):
            agent12.verify_reference(b"short", {agent12.ATOMIC_START: bytes(64)})

    @unittest.skipUnless(os.getenv("NABU_HEXAGON_LLVM_MC"), "set LLVM 6 Hexagon MC path")
    def test_optional_llvm6_exact_source_lengths(self):
        a = agent12.output_fragments(Path(os.environ["NABU_HEXAGON_LLVM_MC"]))
        self.assertEqual({k: len(v) for k, v in a.items()}, {0x7C000: 64, 0x446000: 216})
        if os.getenv("NABU_MODEM_REFERENCE"):
            agent12.verify_reference(Path(os.environ["NABU_MODEM_REFERENCE"]).read_bytes(), a)


if __name__ == "__main__":
    unittest.main()
