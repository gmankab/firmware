# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 12: bounded counted-frame preparation and packet encoding."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp12',ROOT/'tools/wlanmdsp/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC);sys.modules[SPEC.name]=late;SPEC.loader.exec_module(late)

class Agent14Checkpoint12(unittest.TestCase):
    def test_next_function_is_exactly_adjacent(self):
        reg=late.load_registry()
        old=late.select_regions(reg,through_checkpoint=11)
        new=late.select_regions(reg,through_checkpoint=12)
        self.assertEqual((old[0].file_start,old[-1].file_end),(0x1001c4,0x102aa8))
        self.assertEqual(len(new),len(old)+1)
        self.assertEqual((new[-1].file_start,new[-1].file_end),(0x102aa8,0x102cd0))
        self.assertEqual(new[-1].size_bytes,552)
        self.assertEqual(sum(r.size_bytes for r in new),11020)
    def test_far_branch_and_nonshuffled_packet_size(self):
        p=ROOT/'sources/xiaomi-nabu/wlanmdsp/late/counted-frame-prepare.S'
        s=p.read_text()
        self.assertIn('jump ##1977576',s)
        self.assertIn('r21 = #0x6; r2 = #0x6; r1 = r21',s)
        self.assertNotIn('.incbin',s)
        self.assertNotIn('.word ',s)
        self.assertTrue(s.count('label_')>=10)

if __name__=='__main__':unittest.main()
