# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 14 source boundaries and decoded Hexagon checksum packet."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp14',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC);sys.modules[SPEC.name]=late;SPEC.loader.exec_module(late)

class Agent14Checkpoint14(unittest.TestCase):
    def test_aggregate_has_exact_adjacent_region(self):
        reg=late.load_registry()
        prior=late.select_regions(reg,through_checkpoint=13)
        after=late.select_regions(reg,through_checkpoint=14)
        self.assertEqual(prior[-1].file_end,0x102f0c)
        self.assertEqual((after[-1].file_start,after[-1].file_end),(0x102f0c,0x103110))
        self.assertEqual(after[-1].size_bytes,516)
        self.assertEqual(sum(r.size_bytes for r in after),12108)
        self.assertEqual(len(after)-len(prior),1)
    def test_hardware_checksum_and_extended_error_jump(self):
        text=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-interface-status-construct.S').read_text()
        self.assertIn('call -884',text)
        self.assertIn('memd(r29+#0x8) = r5:4',text)
        self.assertIn('jump ##1976456',text)
        self.assertNotIn('.word ',text)
        self.assertNotIn('.incbin',text)

if __name__=='__main__':unittest.main()
