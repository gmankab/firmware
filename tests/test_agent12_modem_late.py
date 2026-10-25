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
        self.assertEqual((agent12.SERVICE_START,agent12.SERVICE_END),(0x7C2EC,0x7C324))
        self.assertLess(agent12.CONTROL_END,agent12.SERVICE_START)
        self.assertEqual((agent12.SERVICE_MMIO_START,agent12.SERVICE_MMIO_END),
                         (0x7C324,0x7C3A8))
        self.assertEqual((agent12.SERVICE_FLAGS_START,agent12.SERVICE_FLAGS_END),
                         (0x7C3A8,0x7C40C))
        self.assertEqual(agent12.SERVICE_END, agent12.SERVICE_MMIO_START)
        self.assertEqual(agent12.SERVICE_MMIO_END,agent12.SERVICE_FLAGS_START)
        self.assertEqual(agent12.SERVICE_FLAGS_END,agent12.SERVICE_ALLOC_START)
        self.assertEqual((agent12.SERVICE_ALLOC_START,agent12.SERVICE_ALLOC_END),
                         (0x7C40C,0x7C4B0))
        self.assertLess(agent12.SERVICE_ALLOC_END,agent12.AUX_START)
        self.assertLessEqual(agent12.AUX_END, 0x446000 + 0x11DA4)
        self.assertEqual(agent12.AUX_INIT_START,0x446200)
        self.assertEqual(agent12.AUX_INIT_END,0x446254)
        self.assertEqual(agent12.AUX_GUARD_START,0x446260)
        self.assertEqual(agent12.AUX_GUARD_END,0x446278)
        self.assertEqual(agent12.AUX_END,0x4460D8)
        self.assertLess(agent12.AUX_END,agent12.AUX_INIT_START)
        self.assertLess(agent12.AUX_INIT_END,agent12.AUX_GUARD_START)
        self.assertEqual((agent12.AUX_STAGE_START,agent12.AUX_STAGE_END), (0x446278,0x4462E0))
        self.assertEqual((agent12.AUX_MMIO_START,agent12.AUX_MMIO_END), (0x4462E0,0x4462F8))
        self.assertEqual(agent12.AUX_GUARD_END,agent12.AUX_STAGE_START)
        self.assertEqual(agent12.AUX_STAGE_END,agent12.AUX_MMIO_START)
        self.assertEqual((agent12.AUX_HEAP_START,agent12.AUX_HEAP_END),(0x4462F8,0x4463B4))
        self.assertEqual(agent12.AUX_MMIO_END,agent12.AUX_HEAP_START)
        self.assertEqual(agent12.AUX_HEAP_END,agent12.AUX_WORKER_START)
        self.assertEqual((agent12.AUX_WORKER_START,agent12.AUX_WORKER_END),
                         (0x4463B4,0x446420))
        self.assertEqual((agent12.AUX_LOOKUP_START,agent12.AUX_LOOKUP_END),
                         (0x446420,0x446488))
        self.assertEqual(agent12.AUX_WORKER_END,agent12.AUX_LOOKUP_START)

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

    def test_auxiliary_stage_packet_rewrite_is_checked(self):
        words=[0x7800C000]*26
        words[7]=0xB000C000
        words[8]=0x28000000  # following Hexagon duplex
        words[10]=0x3C00C000
        words[17]=0x5A00C000
        words[23]=0x5A004000
        words[24]=0x7800C000
        packed=struct.pack('<26I',*words)
        built=agent12.restore_aux_stage_packets(packed)
        got=struct.unpack('<26I',built)
        for off in [0x1c,0x28,0x44,0x60]:
            self.assertEqual(got[off//4]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(got[0x5c//4]&agent12.MARKER_MASK,agent12.END_MARKER)
        self.assertEqual(got[8]&agent12.MARKER_MASK,0)
        bad=words.copy();bad[10]=0x7800C000
        with self.assertRaisesRegex(ValueError,'instruction class'):
            agent12.restore_aux_stage_packets(struct.pack('<26I',*bad))
        bad=words.copy();bad[8]=0x2800C000
        with self.assertRaisesRegex(ValueError,'final duplex'):
            agent12.restore_aux_stage_packets(struct.pack('<26I',*bad))
        with self.assertRaisesRegex(ValueError,'104 bytes'):
            agent12.restore_aux_stage_packets(packed[:-4])

    def test_service_probe_reorder_validates_isa_and_lengths(self):
        words=[0x7800C000]*14
        words[5]=0x5A00C000
        words[6]=0x0C004000
        words[9]=0x7800C000
        words[10]=0x0C004000
        synthetic=struct.pack('<14I',*words)
        actual=agent12.restore_service_probe_packets(synthetic)
        decoded=struct.unpack('<14I',actual)
        self.assertEqual(decoded[5]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(decoded[9]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        bad=words.copy();bad[5]=0x7800C000
        with self.assertRaisesRegex(ValueError,'instruction classes'):
            agent12.restore_service_probe_packets(struct.pack('<14I',*bad))
        bad=words.copy();bad[9]=0x78004000
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_service_probe_packets(struct.pack('<14I',*bad))
        with self.assertRaisesRegex(ValueError,'56 compiled bytes'):
            agent12.restore_service_probe_packets(synthetic[:-4])

    def test_service_mmio_packet_adjustment_rejects_unrelated_opcode_classes(self):
        words=[0x7800C000]*33
        words[5]=0x5A00C000
        words[6]=0x0C004000
        words[8]=0x5B00C000
        words[9]=0x0C004000
        source=struct.pack('<33I',*words)
        decoded=struct.unpack('<33I',agent12.restore_service_mmio_packets(source))
        for i in (5,8):
            self.assertEqual(decoded[i]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
            self.assertEqual(decoded[i+1],words[i+1])
        self.assertEqual(decoded[0:5],tuple(words[0:5]))
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            bad=words.copy();bad[5]=0x7800C000
            agent12.restore_service_mmio_packets(struct.pack('<33I',*bad))
        with self.assertRaisesRegex(ValueError,'packet markers'):
            bad=words.copy();bad[8]=0x5B004000
            agent12.restore_service_mmio_packets(struct.pack('<33I',*bad))
        with self.assertRaisesRegex(ValueError,'132 bytes'):
            agent12.restore_service_mmio_packets(source[:-4])

    def test_service_flag_packet_reorder_checks_source_semantics(self):
        # Synthetic input: cmp/branch moved before its extender/move.
        words=[0x7800C000]*25
        i=(agent12.SERVICE_FLAGS_START + 0x40 - agent12.SERVICE_FLAGS_START)//4
        words[i:i+3]=[0x10004010,0x0eb1410f,0x7e80c702]
        decoded=list(struct.unpack('<25I',agent12.restore_service_flag_packets(
            struct.pack('<25I',*words))))
        self.assertEqual([x>>24 for x in decoded[i:i+3]],[0x0e,0x7e,0x10])
        self.assertEqual([x&agent12.MARKER_MASK for x in decoded[i:i+3]],
                         [agent12.CONTINUE_MARKER,agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[i+1]=0x78004000
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_service_flag_packets(struct.pack('<25I',*bad))
        bad=words.copy();bad[i]=0x1000C010
        with self.assertRaisesRegex(ValueError,'packet parse markers'):
            agent12.restore_service_flag_packets(struct.pack('<25I',*bad))
        with self.assertRaisesRegex(ValueError,'100 source bytes'):
            agent12.restore_service_flag_packets(bytes(96))

    def test_allocator_return_packet_fixup_rejects_other_opcodes(self):
        words=[0x7800C000]*41
        idx=0x98//4
        words[idx:idx+3]=[0x7070C000,0x3e0c0005,0x961ec01e]
        encoded=struct.pack('<41I',*words)
        fixed=struct.unpack('<41I',agent12.restore_service_alloc_return_packet(encoded))
        self.assertEqual(fixed[idx]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(fixed[idx] & ~agent12.MARKER_MASK,
                         words[idx] & ~agent12.MARKER_MASK)
        bad=words.copy();bad[idx]=0x7800C000
        with self.assertRaisesRegex(ValueError,'instruction classes'):
            agent12.restore_service_alloc_return_packet(struct.pack('<41I',*bad))
        bad=words.copy();bad[idx+1]=0x3e0c4005
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_service_alloc_return_packet(struct.pack('<41I',*bad))
        with self.assertRaisesRegex(ValueError,'164 source bytes'):
            agent12.restore_service_alloc_return_packet(encoded[:-4])

    def test_aux_heap_producer_branch_reorder_is_checked(self):
        words=[0x7800C000]*47
        i=0x6c//4
        words[i:i+2]=[0x10426024,0x7800C002]
        repaired=struct.unpack('<47I',agent12.restore_aux_heap_predicate_packet(
            struct.pack('<47I',*words)))
        self.assertEqual([x>>24 for x in repaired[i:i+2]],[0x78,0x10])
        self.assertEqual([x&agent12.MARKER_MASK for x in repaired[i:i+2]],
                         [agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[i]=0x78004024
        with self.assertRaisesRegex(ValueError,'opcode class'):
            agent12.restore_aux_heap_predicate_packet(struct.pack('<47I',*bad))
        bad=words.copy();bad[i+1]=0x78004002
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_heap_predicate_packet(struct.pack('<47I',*bad))
        with self.assertRaisesRegex(ValueError,'188 bytes'):
            agent12.restore_aux_heap_predicate_packet(bytes(184))

    def test_aux_worker_checked_branch_and_packet_adjustments(self):
        words=[0x7800C000]*27
        i=0x24//4
        words[i:i+2]=[0x78dffff0,0x5800c01e]
        j=0x3c//4
        words[j:j+2]=[0x5a00c050,0x73306020]
        rebuilt=struct.unpack('<27I',agent12.restore_aux_worker_packets(
            struct.pack('<27I',*words)))
        self.assertEqual(rebuilt[i]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(rebuilt[i+1]&0x3f,0x20)
        self.assertEqual(rebuilt[j]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        bad=words.copy();bad[i+1]=0x7800c01e
        with self.assertRaisesRegex(ValueError,'ISA classes'):
            agent12.restore_aux_worker_packets(struct.pack('<27I',*bad))
        bad=words.copy();bad[i+1]=0x5800c022
        with self.assertRaisesRegex(ValueError,'PC-relative'):
            agent12.restore_aux_worker_packets(struct.pack('<27I',*bad))
        bad=words.copy();bad[j+1]=0x7330c020
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_worker_packets(struct.pack('<27I',*bad))
        with self.assertRaisesRegex(ValueError,'108 bytes'):
            agent12.restore_aux_worker_packets(bytes(104))

    def test_aux_lookup_epilogue_fixup_rejects_invalid_packet(self):
        words=[0x7800C000]*26
        i=0x60//4
        words[i:i+2]=[0x7070C000,0x3e0c1f40]
        out=struct.unpack('<26I',agent12.restore_aux_lookup_return_packet(
            struct.pack('<26I',*words)))
        self.assertEqual(out[i]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(out[i]&~agent12.MARKER_MASK,words[i]&~agent12.MARKER_MASK)
        broken=words.copy();broken[i]=0x7800C000
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_aux_lookup_return_packet(struct.pack('<26I',*broken))
        broken=words.copy();broken[i+1]|=agent12.CONTINUE_MARKER
        with self.assertRaisesRegex(ValueError,'packet marker'):
            agent12.restore_aux_lookup_return_packet(struct.pack('<26I',*broken))
        with self.assertRaisesRegex(ValueError,'104 source bytes'):
            agent12.restore_aux_lookup_return_packet(bytes(100))

    def test_sources_use_packets_and_real_control_register_semantics(self):
        a = agent12.ATOMIC_SOURCE.read_text()
        b = agent12.AUX_SOURCE.read_text()
        c = agent12.CONTROL_SOURCE.read_text()
        d = agent12.AUX_INIT_SOURCE.read_text()
        e = agent12.AUX_GUARD_SOURCE.read_text()
        f = agent12.AUX_STAGE_SOURCE.read_text()
        g = agent12.AUX_MMIO_SOURCE.read_text()
        h = agent12.SERVICE_SOURCE.read_text()
        j = agent12.SERVICE_MMIO_SOURCE.read_text()
        k = agent12.SERVICE_FLAGS_SOURCE.read_text()
        l = agent12.SERVICE_ALLOC_SOURCE.read_text()
        m = agent12.AUX_HEAP_SOURCE.read_text()
        n = agent12.AUX_WORKER_SOURCE.read_text()
        o = agent12.AUX_LOOKUP_SOURCE.read_text()
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
        self.assertIn('memw(r16+#40)',f)
        self.assertIn('aux_run_worker',f)
        self.assertIn('HEX_WRITE_FRAMELIMIT',b)
        self.assertIn('memw(r2+#2052)',g)
        self.assertIn('service_load',h)
        self.assertIn('service_flush',h)
        self.assertIn('combine(#0,##3249533728)',h)
        self.assertIn('memw(r2+#-1012)',j)
        self.assertIn('setbit(r3,#2)',j)
        self.assertIn('jumpr r31',j)
        self.assertIn('memw(r3+#0) &= r2',k)
        self.assertIn('if (!p0.new) r2',k)
        self.assertIn('platform_alloc_probe',l)
        self.assertIn('memw(r16+#32)',l)
        self.assertIn('mpyi(r2,r4)',m)
        self.assertIn('worker_register',n)
        self.assertIn('r2 = memw(r2+#0)',o)

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
                         {0x7C000:160,0x7C2EC:452,0x446000:216,0x446200:84,0x446260:552})
        if os.getenv("NABU_MODEM_REFERENCE"):
            agent12.verify_reference(Path(os.environ["NABU_MODEM_REFERENCE"]).read_bytes(), a)


if __name__ == "__main__":
    unittest.main()
