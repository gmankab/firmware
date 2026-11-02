# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 25: word-aligned script interpreter, packet framing, and ownership."""
import importlib.util
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('wlan_cp25',ROOT/'tools/wlanmdsp/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)

class InterpreterTests(unittest.TestCase):
    def test_contiguous_complete_function_and_prior_checkpoint(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=24)
        current=late.select_regions(late.load_registry(),through_checkpoint=25)
        self.assertEqual((current[-1].name,current[-1].file_start,current[-1].file_end),
                         ('script-operand-interpreter',0x105744,0x105bac))
        self.assertEqual(len(current),len(old)+1)
        self.assertEqual(old[-1].file_end,current[-1].file_start)
        self.assertEqual(sum(r.size_bytes for r in current),23016)
        self.assertEqual(current[-1].vaddr,0xb00c5744)
        self.assertEqual(current[-1].size_bytes,1128)
    def test_named_branch_tables_and_no_reference_code_embedding(self):
        text=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/script-operand-interpreter.S').read_text()
        for directive in ('.word','.byte','.long','.incbin','.fill','.org','.inst'):
            self.assertNotIn(directive,text)
        self.assertIn('loop0(pkt_00b0,r2)',text)
        self.assertIn('pkt_0458:',text)
        self.assertIn('pkt_0460:',text)
        self.assertRegex(text,r'(?m)^\{[^\n]*\}(?:\s*:endloop0)?$')
        self.assertNotIn('checkpoint50',text.lower())
    def test_other_agent_range_is_unchanged(self):
        region=late.select_regions(late.load_registry(),through_checkpoint=25)[-1]
        self.assertGreaterEqual(region.file_start,0x1001c4)
        self.assertLessEqual(region.file_end,0x353d38)

if __name__=='__main__':unittest.main()
