# SPDX-License-Identifier: GPL-3.0-only
"""Agent 14 checkpoint 24: six source-assembled WLAN runtime helpers."""
import importlib.util
from pathlib import Path
import re
import sys
import unittest

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location('agent14_cp24',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = late
SPEC.loader.exec_module(late)

class StreamStatusHelperTests(unittest.TestCase):
    EXPECTED = [
        ('status-flag-update',0x1055f8,0x105638),
        ('status-flag-alloc',0x105638,0x105668),
        ('session-start-thunk',0x105668,0x10567c),
        ('buffer-slot-attach',0x10567c,0x1056b4),
        ('buffer-slot-detach',0x1056b4,0x1056e0),
        ('channel-count-reset',0x1056e0,0x105744),
    ]
    def test_bounded_contiguous_six_functions(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=23)
        all_=late.select_regions(late.load_registry(),through_checkpoint=24)
        self.assertEqual(len(all_),len(old)+6)
        self.assertEqual(old[-1].file_end,0x1055f8)
        self.assertEqual([(r.name,r.file_start,r.file_end) for r in all_[-6:]], self.EXPECTED)
        self.assertEqual(sum(r.size_bytes for r in all_),21888)
        self.assertEqual(sum(r.size_bytes for r in all_[-6:]),332)
        self.assertEqual((all_[-1].vaddr,all_[-1].vaddr+all_[-1].size_bytes),
                         (0xb00c56e0,0xb00c5744))
    def test_only_named_isa_mnemonics_no_vendor_opcode_blobs(self):
        for name,_,__ in self.EXPECTED:
            with self.subTest(name=name):
                src=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late'/f'{name}.S').read_text()
                for directive in ('.word','.long','.byte','.incbin','.fill','.org','.inst'):
                    self.assertNotIn(directive,src)
                self.assertIn('.globl agent14_late_'+name.replace('-','_'),src)
                self.assertRegex(src,r'(?m)^\{[^\n]+\}(?::mem_noshuf)?$')
    def test_boundaries_not_truncated_or_filled(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=23)
        latest=late.select_regions(late.load_registry(),through_checkpoint=24)
        self.assertEqual(old[0].file_start,0x1001c4)
        self.assertEqual(latest[-1].file_end,0x105744)
        self.assertEqual(len(latest[-6:]),6)

if __name__=='__main__': unittest.main()
