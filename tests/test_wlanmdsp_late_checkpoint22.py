# SPDX-License-Identifier: GPL-3.0-only
"""Agent 14 checkpoint 22: original 104-byte WLAN acknowledgement helper."""
import importlib.util
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp22',ROOT/'tools/wlanmdsp/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)

class AckHelperTests(unittest.TestCase):
    def test_exact_bounded_helper(self):
        prev=late.select_regions(late.load_registry(),through_checkpoint=21)
        now=late.select_regions(late.load_registry(),through_checkpoint=22)
        self.assertEqual(len(now),len(prev)+1)
        self.assertEqual((now[-1].file_start,now[-1].file_end),(0x1050d8,0x105140))
        self.assertEqual(now[-1].size_bytes,104)
        self.assertEqual(sum(x.size_bytes for x in now),20348)
        self.assertEqual(now[-1].vaddr,0xb00c50d8)
        self.assertEqual(now[-1].vaddr+104,0xb00c5140)
    def test_all_instructions_are_explicit(self):
        source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/record-ack-thunk.S').read_text()
        self.assertEqual(source.count('// offset 0x'),10)
        self.assertIn('dealloc_return',source)
        self.assertIn('r1:0 = combine(#0x6,##0xc80)',source)
        for token in ('.word','.long','.byte','.incbin','.fill','.org'):
            self.assertNotIn(token,source)

if __name__=='__main__':unittest.main()
