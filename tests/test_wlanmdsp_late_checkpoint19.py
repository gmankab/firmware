# SPDX-License-Identifier: GPL-3.0-only
"""Agent 14 checkpoint 19: controller-command pipeline and V66 packet lengths."""
import importlib.util
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp19',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)

class CommandPipelineTests(unittest.TestCase):
    def test_complete_following_function_is_adjacent(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=18)
        now=late.select_regions(late.load_registry(),through_checkpoint=19)
        self.assertEqual(len(now),len(old)+1)
        self.assertEqual((now[-1].file_start,now[-1].file_end),(0x103b4c,0x104398))
        self.assertEqual(now[-1].size_bytes,2124)
        self.assertEqual(sum(r.size_bytes for r in now),16852)
    def test_source_forces_extenders_and_command_packet_order(self):
        text=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/interface-command-pipeline.S').read_text()
        self.assertIn('jump:nt ##label_0450',text)
        self.assertIn('jump ##1972320',text)
        self.assertIn(':mem_noshuf',text)
        self.assertIn('{ callr r5; r2 = #0x0; r3 = add(r29,#0x3c); r4 = r22 }',text)
        self.assertIn('{ r4 = ##0xb0345730; r0 = add(r29,#0x3c); r2 = memw(r21+#0x0) }',text)
        for directive in ('.word','.long','.byte','.incbin','.fill','.org'):
            self.assertNotIn(directive,text)
    def test_no_region_claim_beyond_complete_function(self):
        current=late.select_regions(late.load_registry(),through_checkpoint=19)
        self.assertEqual(current[-1].file_end,0x104398)
        self.assertLess(current[-1].file_end,late.OWNED_FILE_END)
        self.assertEqual(late.TEXT_VADDR+current[-1].file_end-late.TEXT_FILE_START,0xb00c4398)

if __name__=='__main__':
    unittest.main()
