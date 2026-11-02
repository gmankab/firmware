# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 10: session activity and attribute record normalization."""
import importlib.util
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location('agent14_cp10', ROOT/'tools/wlanmdsp/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = late
SPEC.loader.exec_module(late)
SOURCE = ROOT/'sources/xiaomi-nabu/wlanmdsp/late'

class Agent14Checkpoint10Tests(unittest.TestCase):
    def test_three_adjacent_functions_and_exact_bytes(self):
        reg = late.load_registry()
        before = late.select_regions(reg, through_checkpoint=9)
        after = late.select_regions(reg, through_checkpoint=10)
        self.assertEqual(len(after)-len(before), 3)
        self.assertEqual([r.size_bytes for r in after[-3:]], [152,84,120])
        self.assertEqual((after[-3].file_start,after[-1].file_end),(0x1026bc,0x102820))
        self.assertEqual(after[-1].vaddr, 0xb00c27a8)
        self.assertEqual(sum(r.size_bytes for r in after), 9820)
        self.assertEqual(after[-1].file_end, 0x102820)
    def test_far_jump_and_no_shuffling_are_explicit(self):
        activity=(SOURCE/'session-activity-update.S').read_text()
        second=(SOURCE/'attribute-byte-snapshot.S').read_text()
        third=(SOURCE/'attribute-byte-select.S').read_text()
        self.assertIn('jump ##-792140',activity)
        self.assertIn(':mem_noshuf',second)
        self.assertEqual(third.count(':mem_noshuf'),2)
        for body in (activity,second,third):
            self.assertNotIn('.word ',body)
            self.assertNotIn('.incbin',body)
    def test_previous_checkpoint_09_is_unchanged(self):
        prior = late.select_regions(late.load_registry(),through_checkpoint=9)
        self.assertEqual((prior[0].file_start,prior[-1].file_end),(0x1001c4,0x1026bc))
        self.assertEqual(sum(r.size_bytes for r in prior),9464)

if __name__=='__main__':unittest.main()
