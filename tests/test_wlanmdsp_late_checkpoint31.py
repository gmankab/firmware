# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 31: session controller command and stream window processing."""
from pathlib import Path
import importlib.util
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('wlan_cp31',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)
class SessionCommandWindow(unittest.TestCase):
    expected=[
        ('session-command-check',0x106ab4,0x106b74),
        ('session-command-update',0x106b74,0x106bc8),
        ('session-command-validate',0x106bc8,0x106cfc),
        ('session-slot-index',0x106cfc,0x106d48),
        ('session-slot-thunk',0x106d48,0x106d50),
        ('session-link-report',0x106d50,0x106d84),
        ('session-read-window',0x106d84,0x106e8c),
    ]
    def test_registry_extends_contiguously(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=30)
        new=late.select_regions(late.load_registry(),through_checkpoint=31)
        self.assertEqual(len(new)-len(old),len(self.expected))
        self.assertEqual(old[-1].file_end,0x106ab4)
        self.assertEqual([(r.name,r.file_start,r.file_end) for r in new[-7:]],self.expected)
        self.assertEqual(sum(x.size_bytes for x in new[-7:]),984)
        self.assertEqual(sum(x.size_bytes for x in new),27848)
    def test_no_raw_firmware_instruction_bytes(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late'/f'{name}.S').read_text()
                for directive in ('.word','.long','.byte','.incbin','.inst','.fill'):
                    self.assertNotIn(directive,source)
                self.assertIn('agent14_late_'+name.replace('-','_')+':',source)
    def test_controller_packet_ordering(self):
        root=ROOT/'sources/xiaomi-nabu/wlanmdsp/late'
        a=(root/'session-command-validate.S').read_text()
        b=(root/'session-link-report.S').read_text()
        c=(root/'session-read-window.S').read_text()
        self.assertIn('{ call -2572; r1 = #0x4; r2 = #0x1; r0 = memub(r16+#0x0) }',a)
        self.assertIn('{ r2 = r0; r5 = #0x0; r1:0 = combine(#0x47,r19); r4 = memw(r18+#0x0) }',b)
        self.assertIn('{ memw(r16+#0x48) = r3; r17:16 = memd(r29+#0x8) }:mem_noshuf',c)
if __name__=='__main__':unittest.main()
