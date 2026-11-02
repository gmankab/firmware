# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 11: two exact WLAN record-state / packed-reply routines."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp11', ROOT/'tools/wlanmdsp/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)

class Agent14Checkpoint11Tests(unittest.TestCase):
    def test_exact_adjacent_bounds_and_prior_unchanged(self):
        reg=late.load_registry()
        before=late.select_regions(reg,through_checkpoint=10)
        current=late.select_regions(reg,through_checkpoint=11)
        self.assertEqual((before[0].file_start,before[-1].file_end),(0x1001c4,0x102820))
        self.assertEqual((current[-2].file_start,current[-1].file_end),(0x102820,0x102aa8))
        self.assertEqual([r.size_bytes for r in current[-2:]], [420,228])
        self.assertEqual(sum(r.size_bytes for r in current),10468)
    def test_jump_extenders_and_shuffled_memory_packets(self):
        root=ROOT/'sources/xiaomi-nabu/wlanmdsp/late'
        first=(root/'frame-status-parse.S').read_text()
        second=(root/'packed-record-reply.S').read_text()
        self.assertIn('jump ##1978280',first)
        self.assertIn('r1 = r22; r2 = #0x6;',first)
        self.assertIn('jump ##-793032',second)
        self.assertIn('jump ##-793132',second)
        self.assertEqual(second.count('}:mem_noshuf'),2)
        for s in (first,second):
            self.assertNotIn('.incbin',s)
            self.assertNotIn('.word ',s)

if __name__=='__main__': unittest.main()
