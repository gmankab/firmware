# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 23: complete 1208-byte WLAN interface status/query path."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp23',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)

class InterfaceStatusQueryTests(unittest.TestCase):
    def test_bounded_adjacent_full_function(self):
        prior=late.select_regions(late.load_registry(),through_checkpoint=22)
        now=late.select_regions(late.load_registry(),through_checkpoint=23)
        self.assertEqual(len(now),len(prior)+1)
        self.assertEqual((now[-1].file_start,now[-1].file_end),(0x105140,0x1055f8))
        self.assertEqual(now[-1].size_bytes,1208)
        self.assertEqual(sum(x.size_bytes for x in now),21556)
        self.assertEqual((now[-1].vaddr,now[-1].vaddr+now[-1].size_bytes),
                         (0xb00c5140,0xb00c55f8))
    def test_packet_source_never_embeds_reference_code(self):
        text=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-interface-status-query.S').read_text()
        self.assertEqual(text.count('// offset 0x'),147)
        self.assertIn('{ if (!p0) call ##1402252 }',text)
        self.assertIn(':mem_noshuf',text)
        self.assertIn('label_00e0:',text)
        self.assertIn('label_019c:',text)
        for illegal in ('.word','.long','.byte','.incbin','.fill','.org'):
            self.assertNotIn(illegal,text)

if __name__=='__main__':unittest.main()
