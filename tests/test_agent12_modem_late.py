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
    def test_insert_controller_source_and_guarded_packet_reordering(self):
        self.assertEqual((agent12.AUX_INSERT_CONTROLLER_START,
                          agent12.AUX_INSERT_CONTROLLER_END),(0x447234,0x4472d0))
        source=agent12.AUX_INSERT_CONTROLLER_SOURCE.read_text()
        self.assertIn('memw(r3+#0)',source)
        self.assertIn('dealloc_return',source)
        self.assertNotIn('.word',source)
        words=[0x7800c000]*39
        first=(0x44723c-agent12.AUX_INSERT_CONTROLLER_START)//4
        branch=(0x4472a4-agent12.AUX_INSERT_CONTROLLER_START)//4
        lo=(0x4472b0-agent12.AUX_INSERT_CONTROLLER_START)//4
        last=(0x4472c4-agent12.AUX_INSERT_CONTROLLER_START)//4
        words[first]=0xa1ddd200
        words[branch:branch+2]=[0x5cdf78fa,0xf204c310]
        words[lo:lo+2]=[0x91854084,0xa190c528]
        words[last]=0xa183c200
        assembled=struct.pack('<39I',*words)
        reordered=struct.unpack('<39I',agent12.restore_aux_insert_controller_packets(assembled))
        self.assertEqual(reordered[first],0xa1dd5200)
        self.assertEqual(reordered[branch:branch+2],(0xf2044310,0x5cdff8fa))
        self.assertEqual(reordered[lo:lo+2],(0xa1904528,0x9185c084))
        self.assertEqual(reordered[last],0xa1834200)
        for idx,wrong,pattern in [
            (first,0x7800c000,'instruction semantics'),
            (first,0xa1dd5200,'packet marker'),
            (branch,0x7800c000,'opcode classes'),
            (lo+1,0x7800c000,'opcode classes'),
            (last,0xa1834200,'packet marker')
        ]:
            bad=words.copy();bad[idx]=wrong
            with self.assertRaisesRegex(ValueError,pattern):
                agent12.restore_aux_insert_controller_packets(struct.pack('<39I',*bad))
        with self.assertRaisesRegex(ValueError,'156 compiled source bytes'):
            agent12.restore_aux_insert_controller_packets(assembled[:-4])

    def test_remove_controller_reorders_only_checked_hexagon_ops(self):
        self.assertEqual((agent12.AUX_REMOVE_CONTROLLER_START,
                          agent12.AUX_REMOVE_CONTROLLER_END),(0x4472d0,0x447324))
        source=agent12.AUX_REMOVE_CONTROLLER_SOURCE.read_text()
        self.assertIn('dealloc_return',source)
        self.assertNotIn('.word',source)
        words=[0x7800c000]*21
        first=(0x447300-agent12.AUX_REMOVE_CONTROLLER_START)//4
        second=(0x447310-agent12.AUX_REMOVE_CONTROLLER_START)//4
        words[first:first+2]=[0x3c42c280,0x91ddc010]
        words[second:second+2]=[0x479044e2,0xa183c204]
        packed=struct.pack('<21I',*words)
        out=struct.unpack('<21I',agent12.restore_aux_remove_controller_packets(packed))
        self.assertEqual(out[first],0x3c424280)
        self.assertEqual(out[second:second+2],(0xa1834204,0x4790c4e2))
        for index,bad_word,pattern in [
            (first,0x7800c000,'opcode classes'),
            (first,0x3c424280,'packet markers'),
            (second,0x48004000,'opcode classes'),
            (second+1,0xa1834204,'packet markers')
        ]:
            invalid=words.copy();invalid[index]=bad_word
            with self.assertRaisesRegex(ValueError,pattern):
                agent12.restore_aux_remove_controller_packets(struct.pack('<21I',*invalid))
        with self.assertRaisesRegex(ValueError,'84 source bytes'):
            agent12.restore_aux_remove_controller_packets(packed[:-4])

    def test_late_callback_and_controller_scan_bounds(self):
        self.assertEqual((agent12.AUX_CALLBACK_START,agent12.AUX_CALLBACK_END),
                         (0x447324,0x447388))
        self.assertEqual((agent12.AUX_FIND_CONTROLLER_START,
                          agent12.AUX_FIND_CONTROLLER_END),(0x447388,0x447410))
        self.assertEqual((agent12.AUX_RELEASE_REGISTRY_START,
                          agent12.AUX_RELEASE_REGISTRY_END),(0x447410,0x447448))
        sources=[agent12.AUX_CALLBACK_SOURCE,agent12.AUX_FIND_CONTROLLER_SOURCE,
                 agent12.AUX_RELEASE_REGISTRY_SOURCE]
        for path in sources:
            text=path.read_text()
            self.assertNotIn('.word',text)
            self.assertIn('dealloc_return',text)
        self.assertIn('loop0',agent12.AUX_FIND_CONTROLLER_SOURCE.read_text())

    def test_callback_extender_duplex_rewrite_rejects_unrelated_source(self):
        words=[0x7800c000]*25
        i=(0x447340-agent12.AUX_CALLBACK_START)//4
        words[i-1:i+2]=[0x01e44630,0x7800c086,0x48050124]
        packed=struct.pack('<25I',*words)
        built=struct.unpack('<25I',agent12.restore_aux_callback_extender_packet(packed))
        self.assertEqual(built[i],0x78004086)
        self.assertEqual(built[:i],tuple(words[:i]))
        self.assertEqual(built[i+1:],tuple(words[i+1:]))
        for change,expected in [
            ((i-1,0x7e004000),'opcode classes'),
            ((i,0x7800c087),'operand'),
            ((i+1,0x4805c124),'packet markers'),
        ]:
            bad=words.copy();bad[change[0]]=change[1]
            with self.assertRaisesRegex(ValueError,expected):
                agent12.restore_aux_callback_extender_packet(struct.pack('<25I',*bad))
        with self.assertRaisesRegex(ValueError,'100 source-assembled bytes'):
            agent12.restore_aux_callback_extender_packet(packed[:-4])

    def test_aux_table_scanners_are_independent_source_functions(self):
        find=agent12.AUX_FIND_FREE_SOURCE.read_text()
        resolve=agent12.AUX_RESOLVE_SOURCE.read_text()
        self.assertIn('loop0',find)
        self.assertIn('jumpr r31',find)
        self.assertIn('loop0',resolve)
        self.assertIn('dealloc_return',resolve)
        self.assertNotIn('.word',find)
        self.assertNotIn('.word',resolve)
        self.assertEqual(agent12.AUX_RESOLVE_END-agent12.AUX_FIND_FREE_START,216)

    def test_aux_session_prefix_explicitly_stops_before_framekey(self):
        self.assertEqual(agent12.AUX_SESSION_PREFIX_END,0x446EE8)
        source=agent12.AUX_SESSION_PREFIX_SOURCE.read_text()
        self.assertNotIn('.word',source)
        self.assertNotIn('framekey',source.split('.text',1)[1])
        self.assertIn('bitsplit',source)
        self.assertEqual(agent12.AUX_SESSION_PREFIX_END-agent12.AUX_SESSION_PREFIX_START,88)
        self.assertEqual(agent12.AUX_DIAGNOSTIC_HELPERS_END-agent12.AUX_DIAGNOSTIC_HELPERS_START,116)

    def test_aux_registration_packet_change_rejects_unrelated_ops(self):
        words=[0x7800c000]*129
        i=(0x446D04-agent12.AUX_REGISTRATION_START)//4
        words[i:i+2]=[0x11b040f6,0xfd11f180]
        rebuilt=struct.unpack('<129I',agent12.restore_aux_registration_new_value_packet(
            struct.pack('<129I',*words)))
        self.assertEqual([x>>24 for x in rebuilt[i:i+2]],[0xfd,0x11])
        self.assertEqual([x&agent12.MARKER_MASK for x in rebuilt[i:i+2]],
                         [agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[i]=0x780040f6
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_aux_registration_new_value_packet(struct.pack('<129I',*bad))
        bad=words.copy();bad[i+1]=0xfd117180
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_registration_new_value_packet(struct.pack('<129I',*bad))
        with self.assertRaisesRegex(ValueError,'516 source-assembled bytes'):
            agent12.restore_aux_registration_new_value_packet(bytes(512))

    def test_aux_duplex_compaction_rejects_nonmatching_semantics(self):
        words=[0x7f00c000]*16
        a=0x24//4
        words[a:a+3]=[0x0fff7ff9,0x58004050,0x7800c003]
        b=0x38//4  # second pair in 64-byte assembler version
        words[b:b+2]=[0x59ff7f2c,0x7c00c002]
        output=agent12.compact_aux_controller_veneer_duplex(struct.pack('<16I',*words))
        outwords=struct.unpack('<15I',output)
        self.assertEqual(outwords[a+1],0x1603c050)
        self.assertEqual(outwords[0x34//4],0x7c004002)
        self.assertEqual(outwords[0x38//4],0x59ffff2c)
        bad=words.copy();bad[a+2]=0x7800c004
        with self.assertRaisesRegex(ValueError,'not r3=#0'):
            agent12.compact_aux_controller_veneer_duplex(struct.pack('<16I',*bad))
        bad=words.copy();bad[a+1]=0x78004050
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.compact_aux_controller_veneer_duplex(struct.pack('<16I',*bad))
        bad=words.copy();bad[b]=0x59ffff2c
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.compact_aux_controller_veneer_duplex(struct.pack('<16I',*bad))
        with self.assertRaisesRegex(ValueError,'64 source-assembled bytes'):
            agent12.compact_aux_controller_veneer_duplex(bytes(60))

    def test_aux_runtime_predicate_and_store_rewrites_fail_closed(self):
        words=[0x7800c000]*93
        first=(0x446a84-agent12.AUX_RUNTIME_INIT_START)//4
        second=(0x446ab8-agent12.AUX_RUNTIME_INIT_START)//4
        tail=(0x446b4c-agent12.AUX_RUNTIME_INIT_START)//4
        words[first:first+2]=[0x5c0058a4,0x7e00e032]
        words[second:second+2]=[0x10406188,0x7800c012]
        words[tail:tail+3]=[0x91934000,0x01e4467c,0x4800d228]
        packed=struct.pack('<93I',*words)
        rewritten=struct.unpack('<93I',agent12.restore_aux_runtime_init_packets(packed))
        self.assertEqual([x>>24 for x in rewritten[first:first+2]],[0x7e,0x5c])
        self.assertEqual([x>>24 for x in rewritten[second:second+2]],[0x78,0x10])
        self.assertEqual([x>>24 for x in rewritten[tail:tail+3]],[0x01,0x48,0x91])
        self.assertEqual([x&agent12.MARKER_MASK for x in rewritten[tail:tail+3]],
                         [agent12.CONTINUE_MARKER,agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[first]=0x78004000
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_aux_runtime_init_packets(struct.pack('<93I',*bad))
        bad=words.copy();bad[tail+1]=0x01e4c67c
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_runtime_init_packets(struct.pack('<93I',*bad))
        with self.assertRaisesRegex(ValueError,'372 source bytes'):
            agent12.restore_aux_runtime_init_packets(bytes(368))

    def test_aux_validator_predicate_packet_guard(self):
        words=[0x7800c000]*153
        i=(0x4468f0-agent12.AUX_VALIDATOR_START)//4
        j=(0x446994-agent12.AUX_VALIDATOR_START)//4
        words[i:i+2]=[0x5c205852,0x75424720]
        words[j:j+2]=[0x5c20585a,0x7542c4e0]
        result=struct.unpack('<153I',agent12.restore_aux_validator_predicate_packets(
            struct.pack('<153I',*words)))
        self.assertEqual([x>>24 for x in result[i:i+2]],[0x75,0x5c])
        self.assertEqual([x>>24 for x in result[j:j+2]],[0x75,0x5c])
        self.assertEqual([x & agent12.MARKER_MASK for x in result[j:j+2]],
                         [agent12.CONTINUE_MARKER,agent12.END_MARKER])
        malformed=words.copy();malformed[i]=0x78004000
        with self.assertRaisesRegex(ValueError,'instruction classes'):
            agent12.restore_aux_validator_predicate_packets(struct.pack('<153I',*malformed))
        malformed=words.copy();malformed[j+1]=0x754240e0
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_validator_predicate_packets(struct.pack('<153I',*malformed))
        with self.assertRaisesRegex(ValueError,'612 source bytes'):
            agent12.restore_aux_validator_predicate_packets(bytes(608))

    def test_aux_entry_is_source_encoded_not_oem_bytes(self):
        self.assertTrue(agent12.AUX_ENTRY_SOURCE.is_file())
        source=agent12.AUX_ENTRY_SOURCE.read_text()
        self.assertIn("call controller_call",source)
        self.assertIn("deallocframe",source)
        self.assertNotIn(".word",source)
        self.assertEqual(agent12.AUX_ENTRY_END-agent12.AUX_ENTRY_START,20)

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
        self.assertEqual((agent12.AUX_DISPATCH_START,agent12.AUX_DISPATCH_END),
                         (0x446650,0x446694))
        self.assertLess(agent12.AUX_LOOKUP_END,agent12.AUX_DISPATCH_START)
        self.assertEqual((agent12.AUX_TYPE_START,agent12.AUX_TYPE_END),
                         (0x446694,0x4467A0))
        self.assertEqual(agent12.AUX_DISPATCH_END,agent12.AUX_TYPE_START)
        self.assertEqual((agent12.AUX_NOTIFY_START,agent12.AUX_NOTIFY_END),
                         (0x4467A0,0x4467F0))
        self.assertEqual(agent12.AUX_TYPE_END,agent12.AUX_NOTIFY_START)
        self.assertEqual((agent12.AUX_ENTRY_START,agent12.AUX_ENTRY_END),(0x4467F0,0x446804))
        self.assertEqual(agent12.AUX_NOTIFY_END,agent12.AUX_ENTRY_START)
        self.assertEqual((agent12.AUX_VALIDATOR_START,agent12.AUX_VALIDATOR_END),
                         (0x446804,0x446A68))
        self.assertEqual(agent12.AUX_ENTRY_END,agent12.AUX_VALIDATOR_START)
        self.assertEqual((agent12.AUX_RUNTIME_INIT_START,agent12.AUX_RUNTIME_INIT_END),
                         (0x446A68,0x446BDC))
        self.assertEqual(agent12.AUX_VALIDATOR_END,agent12.AUX_RUNTIME_INIT_START)
        self.assertEqual((agent12.AUX_VENEERS_START,agent12.AUX_VENEERS_END),
                         (0x446BDC,0x446C18))
        self.assertEqual(agent12.AUX_RUNTIME_INIT_END,agent12.AUX_VENEERS_START)
        self.assertEqual((agent12.AUX_REGISTRATION_START,agent12.AUX_REGISTRATION_END),
                         (0x446C18,0x446E1C))
        self.assertEqual(agent12.AUX_VENEERS_END,agent12.AUX_REGISTRATION_START)
        self.assertEqual((agent12.AUX_DIAGNOSTIC_HELPERS_START,agent12.AUX_DIAGNOSTIC_HELPERS_END),
                         (0x446E1C,0x446E90))
        self.assertEqual(agent12.AUX_REGISTRATION_END,agent12.AUX_DIAGNOSTIC_HELPERS_START)
        self.assertEqual((agent12.AUX_SESSION_PREFIX_START,agent12.AUX_SESSION_PREFIX_END),
                         (0x446E90,0x446EE8))
        self.assertEqual(agent12.AUX_DIAGNOSTIC_HELPERS_END,agent12.AUX_SESSION_PREFIX_START)
        self.assertEqual((agent12.AUX_FIND_FREE_START,agent12.AUX_FIND_FREE_END),
                         (0x447028,0x447078))
        self.assertEqual((agent12.AUX_RESOLVE_START,agent12.AUX_RESOLVE_END),
                         (0x447078,0x447100))
        self.assertEqual(agent12.AUX_FIND_FREE_END,agent12.AUX_RESOLVE_START)
        self.assertLess(agent12.AUX_SESSION_PREFIX_END,agent12.AUX_FIND_FREE_START)

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

    def test_aux_codec_type_early_return_packet_checks(self):
        words=[0x7800C000]*67
        for i in (0x34//4,0x80//4):
            words[i:i+2]=[0x78dfffe0,0x3e041f40]
        encoded=struct.pack('<67I',*words)
        actual=struct.unpack('<67I',agent12.restore_aux_type_return_packets(encoded))
        for i in (0x34//4,0x80//4):
            self.assertEqual(actual[i]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
            self.assertEqual(actual[i]&~agent12.MARKER_MASK,words[i]&~agent12.MARKER_MASK)
        changed=words.copy();changed[0x80//4]=0x5a00c000
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_aux_type_return_packets(struct.pack('<67I',*changed))
        changed=words.copy();changed[0x34//4]=0x78df7fe0
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_type_return_packets(struct.pack('<67I',*changed))
        with self.assertRaisesRegex(ValueError,'268 bytes'):
            agent12.restore_aux_type_return_packets(bytes(264))

    def test_controller_notification_packet_reorder_is_checked(self):
        words=[0x7800C000]*20
        i=0x10//4
        words[i:i+3]=[0x5c00581c,0xf2034210,0x7060c002]
        j=0x38//4
        words[j:j+3]=[0x91dd4010,0x01e4467e,0x4880d13c]
        packed=struct.pack('<20I',*words)
        out=struct.unpack('<20I',agent12.restore_aux_controller_notify_packets(packed))
        self.assertEqual([x>>24 for x in out[i:i+3]],[0xf2,0x70,0x5c])
        self.assertEqual([x>>24 for x in out[j:j+3]],[0x01,0x48,0x91])
        for offset in (i,j):
            self.assertEqual([x&agent12.MARKER_MASK for x in out[offset:offset+3]],
               [agent12.CONTINUE_MARKER,agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[i]=0x7800401c
        with self.assertRaisesRegex(ValueError,'packet classes'):
            agent12.restore_aux_controller_notify_packets(struct.pack('<20I',*bad))
        bad=words.copy();bad[j+1]=0x01e4c67e
        with self.assertRaisesRegex(ValueError,'packet marker'):
            agent12.restore_aux_controller_notify_packets(struct.pack('<20I',*bad))
        with self.assertRaisesRegex(ValueError,'80 bytes'):
            agent12.restore_aux_controller_notify_packets(bytes(76))

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
        p = agent12.AUX_DISPATCH_SOURCE.read_text()
        q = agent12.AUX_TYPE_SOURCE.read_text()
        t = agent12.AUX_NOTIFY_SOURCE.read_text()
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
        self.assertIn('callr r17',p)
        self.assertIn('bitsplit(r0,#8)',q)
        self.assertIn('r0 = #-1',q)
        self.assertIn('controller_callback',t)

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
                         {0x7C000:160,0x7C2EC:452,0x446000:216,0x446200:84,0x446260:552,0x446650:416})
        if os.getenv("NABU_MODEM_REFERENCE"):
            agent12.verify_reference(Path(os.environ["NABU_MODEM_REFERENCE"]).read_bytes(), a)


if __name__ == "__main__":
    unittest.main()
