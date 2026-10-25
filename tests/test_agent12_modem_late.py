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
        self.assertEqual((agent12.CONTROL_START, agent12.CONTROL_END), (0x7C040, 0x7C0A0))
        self.assertEqual((agent12.AUX_START, agent12.AUX_END), (0x446000, 0x4460D8))
        self.assertEqual(agent12.ATOMIC_END - agent12.ATOMIC_START, 64)
        self.assertEqual(agent12.CONTROL_END - agent12.CONTROL_START, 96)
        self.assertEqual(agent12.AUX_END - agent12.AUX_START, 216)
        self.assertEqual(agent12.ATOMIC_END, agent12.CONTROL_START)
        self.assertLess(agent12.CONTROL_END, agent12.AUX_START)
        self.assertLessEqual(agent12.AUX_END, 0x446000 + 0x11DA4)
        self.assertEqual(agent12.AUX_INIT_START,0x446200)
        self.assertEqual(agent12.AUX_INIT_END,0x446254)
        self.assertEqual(agent12.AUX_GUARD_START,0x446260)
        self.assertEqual(agent12.AUX_GUARD_END,0x446278)
        self.assertEqual(agent12.AUX_END,0x4460D8)
        self.assertLess(agent12.AUX_END,agent12.AUX_INIT_START)
        self.assertLess(agent12.AUX_INIT_END,agent12.AUX_GUARD_START)

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

    def test_controller_reorder_validates_hexagon_instruction_classes(self):
        # Synthetic packet categories: branch, constant extender, source move.
        words = [0x7800C000] * 24
        words[5:8] = [0x10004000, 0x0C004000, 0x7800C000]
        words[11:14] = [0x0C004000, 0x7800C000, 0x48000000]  # duplex has 00 parse field
        words[19:22] = [0x10004000, 0x0C004000, 0x7E00C000]
        src = struct.pack('<24I', *words)
        built = agent12.restore_controller_packet_order(src)
        got = list(struct.unpack('<24I', built))
        self.assertEqual([x >> 24 for x in got[5:8]], [0x0C,0x78,0x10])
        self.assertEqual([x & agent12.MARKER_MASK for x in got[5:8]],
                         [agent12.CONTINUE_MARKER,agent12.CONTINUE_MARKER,agent12.END_MARKER])
        self.assertEqual([x >> 24 for x in got[19:22]], [0x0C,0x7E,0x10])
        self.assertEqual(got[12] & agent12.MARKER_MASK, agent12.CONTINUE_MARKER)
        self.assertEqual(got[13] & agent12.MARKER_MASK, 0)  # duplex parse format
        with self.assertRaisesRegex(ValueError, 'classes changed'):
            bad=words.copy();bad[19] = 0x78004000
            agent12.restore_controller_packet_order(struct.pack('<24I',*bad))
        with self.assertRaisesRegex(ValueError, 'packet markers'):
            bad=words.copy();bad[13] |= agent12.END_MARKER
            agent12.restore_controller_packet_order(struct.pack('<24I',*bad))

    def test_aux_initializer_packet_rewrite_is_checked(self):
        words = [0x7800C000] * 21
        words[6:8] = [0x7800C002,0x28000000]
        data=struct.pack('<21I',*words)
        rewritten=agent12.restore_aux_init_packet(data)
        rewritten_words=list(struct.unpack('<21I',rewritten))
        self.assertEqual(rewritten_words[6]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(rewritten_words[7]&agent12.MARKER_MASK,0)
        self.assertEqual(rewritten_words[6]>>24,0x78)
        with self.assertRaisesRegex(ValueError,'scalar/duplex'):
            bad=words.copy();bad[7] = 0x7A000000
            agent12.restore_aux_init_packet(struct.pack('<21I',*bad))
        with self.assertRaisesRegex(ValueError,'packet marker'):
            bad=words.copy();bad[6] = 0x78004002
            agent12.restore_aux_init_packet(struct.pack('<21I',*bad))
        with self.assertRaisesRegex(ValueError,'84 bytes'):
            agent12.restore_aux_init_packet(data[:-4])

    def test_sources_use_packets_and_real_control_register_semantics(self):
        a = agent12.ATOMIC_SOURCE.read_text()
        b = agent12.AUX_SOURCE.read_text()
        c = agent12.CONTROL_SOURCE.read_text()
        d = agent12.AUX_INIT_SOURCE.read_text()
        e = agent12.AUX_GUARD_SOURCE.read_text()
        self.assertIn("memw_locked(r0)", a)
        self.assertIn("if (!p0.new) r3 = add", a)
        self.assertIn("memb(##3249533708) = r2.new", a)
        self.assertIn("framelimit", b)
        self.assertIn("HEX_WRITE_FRAMELIMIT 0", b)
        self.assertIn("r29 = add(pc,##82752)", b)
        self.assertIn("memd(r0++#8) = r5:4", b)
        self.assertIn(".Lbyte_loop", b)
        self.assertIn(".Lword_loop", b)
        self.assertIn('r5:4 = combine(##260,#0)', c)
        self.assertIn('if (!p0.new) jump:nt', c)
        self.assertIn('conditional_cleanup_callback', c)
        self.assertIn('.Lconstructor_loop',d)
        self.assertIn('r28.h',d)
        self.assertIn('memw(r29+#0) = r27',d)
        self.assertIn('jumpr:nt',e)
        self.assertIn('aux_guard_handoff',e)

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
        self.assertEqual({k: len(v) for k, v in a.items()},
                         {0x7C000:160,0x446000:216,0x446200:84,0x446260:24})
        if os.getenv("NABU_MODEM_REFERENCE"):
            agent12.verify_reference(Path(os.environ["NABU_MODEM_REFERENCE"]).read_bytes(), a)


if __name__ == "__main__":
    unittest.main()
