# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 29: seven exact WLAN session/descriptor helper routines."""
from pathlib import Path
import importlib.util
import sys
import unittest

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location('wlan_cp29', ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = late
SPEC.loader.exec_module(late)

class SessionDescriptorHelpers(unittest.TestCase):
    expected = [
        ('interface-record-apply',0x1063f4,0x10656c),
        ('status-logging-thunk',0x10656c,0x106578),
        ('session-list-find',0x106578,0x106598),
        ('descriptor-event-thunk',0x106598,0x1065a8),
        ('session-attach',0x1065a8,0x106638),
        ('session-next-thunk',0x106638,0x106644),
        ('session-seek',0x106644,0x106694),
    ]
    def test_exact_file_boundaries_and_count(self):
        old = late.select_regions(late.load_registry(),through_checkpoint=28)
        new = late.select_regions(late.load_registry(),through_checkpoint=29)
        self.assertEqual(len(new),len(old)+7)
        self.assertEqual(old[-1].file_end,0x1063f4)
        self.assertEqual([(r.name,r.file_start,r.file_end) for r in new[-7:]],self.expected)
        self.assertEqual(sum(r.size_bytes for r in new[-7:]),672)
        self.assertEqual(sum(r.size_bytes for r in new),25808)
        self.assertTrue(all(r.checkpoint==29 for r in new[-7:]))
        self.assertEqual(new[-1].file_end,0x106694)
    def test_source_is_named_instructions_only(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                text = (ROOT/'sources/xiaomi-nabu/wlanmdsp/late'/f'{name}.S').read_text()
                for prohibited in ('.word','.long','.byte','.incbin','.inst','.fill'):
                    self.assertNotIn(prohibited,text)
                self.assertIn(f'agent14_late_{name.replace("-","_")}:',text)
                self.assertRegex(text,r'(?m)^\{[^\n]+\}(?:\s*:mem_noshuf|\s+:endloop0)?$')
    def test_attach_packet_order_is_explicit(self):
        text=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/session-attach.S').read_text()
        self.assertIn('{ memw(r16+#0x84) = r2; r17:16 = memd(r29+#0x10) }:mem_noshuf',text)

if __name__=='__main__':
    unittest.main()
