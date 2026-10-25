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
REGISTRY = ROOT / "sources/xiaomi-nabu/wlanmdsp/agent14-late-regions.json"
REGIONS = ROOT / "tools/build-wlanmdsp-late.py"
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
        body = (ROOT / "sources/xiaomi-nabu/wlanmdsp/"
                "agent14-late-status-packet-epilogue.S").read_text()
        self.assertIn("jump ##0x1e3ff0", body)
        self.assertIn("r0 = r18", body)
        self.assertNotIn(".word ", body)
        self.assertNotIn(".long ", body)

    def test_packet_body_has_legal_nonshuffled_memory_packet(self):
        body = (ROOT / "sources/xiaomi-nabu/wlanmdsp/"
                "agent14-late-status-packet-body.S").read_text()
        self.assertIn("memb(r16+#6) = r2", body)
        self.assertIn(":mem_noshuf", body)
        self.assertIn("jump 0x28", body)
        self.assertNotIn(".incbin", body)

    def test_status_word_helper_returns_by_dealloc_return(self):
        src = (ROOT / "sources/xiaomi-nabu/wlanmdsp/"
               "agent14-late-status-word-available.S").read_text()
        self.assertIn("dealloc_return", src)
        self.assertIn("r17:16 = memd(r29+#0)", src)
        self.assertIn("call 0x15556c", src)

    def test_all_three_fragments_match_pinned_source_identity(self):
        raw = json.loads(REGISTRY.read_text())
        self.assertEqual(raw["reference_sha256"],
                         "87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b")
        self.assertEqual([r.checkpoint for r in self.records[-3:]], [5,5,5])
        self.assertTrue(all(r.classification == "executable" for r in self.records[-3:]))


if __name__ == "__main__":
    unittest.main()
