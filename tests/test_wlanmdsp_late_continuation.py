# SPDX-License-Identifier: GPL-3.0-only
"""Agent20-14 checkpoint 05: verified Hexagon status-packet and helper source.

This test suite is source-level and reference-free by default. An independent
optional byte-level comparison is performed by build-wlanmdsp-late.py.
"""
import importlib.util
import json
from pathlib import Path
import struct
import sys
import unittest

ROOT = Path(__file__).resolve().parent.parent
REGISTRY = ROOT / "sources/xiaomi-nabu/wlanmdsp/late/regions.json"
REGIONS = ROOT / "tools/wlanmdsp/build-wlanmdsp-late.py"
_spec = importlib.util.spec_from_file_location("agent14_wlan_late_05", REGIONS)
assert _spec and _spec.loader
late = importlib.util.module_from_spec(_spec)
sys.modules[_spec.name] = late
_spec.loader.exec_module(late)


class WlanHexagonStatusPacketSourceTests(unittest.TestCase):
    def setUp(self):
        self.records = late.load_registry()

    def test_previous_checkpoint_four_has_identical_bounds(self):
        prior = late.select_regions(self.records, through_checkpoint=4)
        self.assertEqual((prior[0].file_start, prior[-1].file_end),
                         (0x1001C4, 0x101724))
        self.assertEqual(sum(r.size_bytes for r in prior), 5472)

    def test_checkpoint_five_is_contiguous_and_stays_in_own_range(self):
        current = late.select_regions(self.records, through_checkpoint=5)
        self.assertEqual(len(current), 29)
        self.assertEqual((current[-3].file_start, current[-1].file_end),
                         (0x101724, 0x101870))
        self.assertEqual([r.size_bytes for r in current[-3:]], [284, 8, 40])
        self.assertEqual(sum(r.size_bytes for r in current), 5804)
        self.assertEqual(current[-1].file_end, 0x101870)
        self.assertLessEqual(current[-1].file_end, 0x353D38)

    def test_far_jump_has_explicit_constant_extension(self):
        body = (ROOT / "sources/xiaomi-nabu/wlanmdsp/late/"
                "status-packet-epilogue.S").read_text()
        self.assertIn("jump ##0x1e3ff0", body)
        self.assertIn("r0 = r18", body)
        self.assertNotIn(".word ", body)
        self.assertNotIn(".long ", body)

    def test_packet_body_has_legal_nonshuffled_memory_packet(self):
        body = (ROOT / "sources/xiaomi-nabu/wlanmdsp/late/"
                "status-packet-body.S").read_text()
        self.assertIn("memb(r16+#6) = r2", body)
        self.assertIn(":mem_noshuf", body)
        self.assertIn("jump 0x28", body)
        self.assertNotIn(".incbin", body)

    def test_status_word_helper_returns_by_dealloc_return(self):
        src = (ROOT / "sources/xiaomi-nabu/wlanmdsp/late/"
               "status-word-available.S").read_text()
        self.assertIn("dealloc_return", src)
        self.assertIn("r17:16 = memd(r29+#0)", src)
        self.assertIn("call 0x15556c", src)

    def test_checkpoint_six_extends_contiguously_by_six_functions(self):
        prior = late.select_regions(self.records, through_checkpoint=5)
        current = late.select_regions(self.records, through_checkpoint=6)
        self.assertEqual(len(prior), 29)
        self.assertEqual(len(current), 35)
        self.assertEqual((current[-6].file_start, current[-1].file_end),
                         (0x101870, 0x101BE4))
        self.assertEqual([r.size_bytes for r in current[-6:]],
                         [460, 40, 188, 88, 88, 20])
        self.assertEqual(sum(r.size_bytes for r in current), 6688)
        self.assertEqual(current[-1].file_end, 0x101BE4)

    def test_status_packet_followup_preserves_extended_call_and_nonshuffled_data(self):
        path = ROOT / "sources/xiaomi-nabu/wlanmdsp/late/buffer-service-post.S"
        text = path.read_text()
        self.assertIn("jump ##0x1e3dfc", text)
        self.assertIn("r17 = r0; r0 = r20", text)
        self.assertNotIn(".incbin", text)
        self.assertNotIn(".word ", text)

    def test_status_warning_thunk_not_assumed_to_return(self):
        path = ROOT / "sources/xiaomi-nabu/wlanmdsp/late/status-warning-thunk.S"
        text = path.read_text()
        self.assertIn("call 0x15b3e4", text)
        self.assertNotIn("dealloc_return", text)

    def test_all_three_fragments_match_pinned_source_identity(self):
        raw = json.loads(REGISTRY.read_text())
        self.assertEqual(raw["reference_sha256"],
                         "87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b")
        self.assertEqual([r.checkpoint for r in late.select_regions(self.records, through_checkpoint=5)[-3:]], [5,5,5])
        self.assertTrue(all(r.classification == "executable" for r in late.select_regions(self.records, through_checkpoint=5)[-3:]))


class WlanHexagonCheckpointSevenSourceTests(unittest.TestCase):
    def setUp(self):
        self.records = late.load_registry()

    def test_four_contiguous_controller_routines(self):
        old = late.select_regions(self.records, through_checkpoint=6)
        current = late.select_regions(self.records, through_checkpoint=7)
        self.assertEqual((old[-1].file_end, current[-1].file_end),
                         (0x101BE4, 0x102014))
        self.assertEqual(len(current) - len(old), 4)
        self.assertEqual([r.size_bytes for r in current[-4:]], [452, 124, 64, 432])
        self.assertEqual(sum(r.size_bytes for r in current), 7760)
        self.assertEqual(current[-1].file_end, 0x102014)
        self.assertLessEqual(current[-1].file_end, 0x353D38)

    def test_packet_order_and_far_jump_extension_are_explicit(self):
        base = ROOT / "sources/xiaomi-nabu/wlanmdsp/late/"
        first = (base / "interface-query-report.S").read_text()
        self.assertIn("call 0x13f040; memb(r18+#49) = #1; r0 = memw(r16+#44) }:mem_noshuf", first)
        fourth = (base / "session-aux-dispatch.S").read_text()
        self.assertIn("memb(r24+#49) = #1; r0 = memw(r3+#556) }:mem_noshuf", fourth)
        self.assertIn("jump ##0x1e380c", fourth)
        third = (base / "byte-counter-adjust.S").read_text()
        self.assertIn("jump ##0x154f58", third)
        for text in (first, fourth, third):
            self.assertNotIn(".word ", text)
            self.assertNotIn(".incbin", text)


class WlanHexagonCheckpointEightSourceTests(unittest.TestCase):
    def setUp(self):
        self.records = late.load_registry()

    def test_two_full_contiguous_interface_routines(self):
        before=late.select_regions(self.records,through_checkpoint=7)
        after=late.select_regions(self.records,through_checkpoint=8)
        self.assertEqual(len(after)-len(before),2)
        self.assertEqual([r.size_bytes for r in after[-2:]],[620,232])
        self.assertEqual((after[-2].file_start,after[-1].file_end),
                         (0x102014,0x102368))
        self.assertEqual(sum(r.size_bytes for r in after),8612)
        self.assertEqual(after[-1].file_end,0x102368)

    def test_memory_packet_order_and_extender_source_is_explicit(self):
        source=ROOT/"sources/xiaomi-nabu/wlanmdsp/late"
        event=(source/"interface-event-handler.S").read_text()
        reply=(source/"status-reply.S").read_text()
        self.assertIn("memb(r24+#49) = #1; r0 = memw(r16+#44) }:mem_noshuf",event)
        self.assertIn("{ r0 = ##25992; r18 = r0; r16 = r1 }",reply)
        for s in (event,reply):
            self.assertNotIn(".word ",s)
            self.assertNotIn(".incbin",s)


if __name__ == "__main__":
    unittest.main()
