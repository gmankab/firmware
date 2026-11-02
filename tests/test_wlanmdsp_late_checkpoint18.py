# SPDX-License-Identifier: GPL-3.0-only
"""Agent 14 checkpoint 18: counted record processing and packet layout."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp18',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)
class RecordPipelineTests(unittest.TestCase):
    def test_full_adjacent_packet_region(self):
        p=late.select_regions(late.load_registry(),through_checkpoint=17)
        n=late.select_regions(late.load_registry(),through_checkpoint=18)
        self.assertEqual(len(n),len(p)+1)
        self.assertEqual((n[-1].file_start,n[-1].file_end),(0x103660,0x103b4c))
        self.assertEqual(n[-1].size_bytes,1260)
        self.assertEqual(sum(x.size_bytes for x in n),14728)
    def test_source_preserves_loop_and_packet_memory_order(self):
        data=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/interface-record-pipeline.S').read_text()
        self.assertIn('loop0(0xc,r2)',data)
        self.assertIn('}:endloop0',data)
        self.assertIn('jump ##1973488',data)
        self.assertGreaterEqual(data.count(':mem_noshuf'),2)
        for directive in ('.incbin','.byte','.word','.long','.fill'):
            self.assertNotIn(directive,data)
if __name__=='__main__':unittest.main()
