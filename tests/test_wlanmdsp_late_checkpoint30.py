# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 30: paired session-mode and session-packet operations."""
from pathlib import Path
import importlib.util
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('wlan_cp30', ROOT/'tools/wlanmdsp/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)
class SessionModeCallbacks(unittest.TestCase):
    expected=[
        ('session-mode-toggle',0x106694,0x10671c),
        ('session-bit-dispatch',0x10671c,0x1067d4),
        ('session-events-update',0x1067d4,0x1068bc),
        ('session-queue-create',0x1068bc,0x10691c),
        ('session-queue-init',0x10691c,0x1069c4),
        ('session-missing-thunk',0x1069c4,0x1069d0),
        ('session-packet-dispatch',0x1069d0,0x106ab4),
    ]
    def test_exact_file_boundaries_and_count(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=29)
        new=late.select_regions(late.load_registry(),through_checkpoint=30)
        self.assertEqual(len(new),len(old)+7)
        self.assertEqual(old[-1].file_end,0x106694)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in new[-7:]],self.expected)
        self.assertEqual(sum(x.size_bytes for x in new[-7:]),1056)
        self.assertEqual(sum(x.size_bytes for x in new),26864)
        self.assertEqual(new[-1].file_end,0x106ab4)
    def test_source_mnemonics_only(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                src=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late'/f'{name}.S').read_text()
                for forbidden in ('.word','.long','.byte','.incbin','.inst','.fill'):
                    self.assertNotIn(forbidden,src)
                self.assertIn(f'agent14_late_{name.replace("-","_")}:',src)
                self.assertRegex(src,r'(?m)^\{[^\n]+\}(?:\s*:mem_noshuf|\s+:endloop0)?$')
    def test_packet_order_fixed_by_source(self):
        p=ROOT/'sources/xiaomi-nabu/wlanmdsp/late'
        a=(p/'session-mode-toggle.S').read_text()
        b=(p/'session-events-update.S').read_text()
        self.assertIn('{ call -1068; r1 = #0x3; r2 = #0x4; r0 = memub(r16+#0x0) }',a)
        self.assertIn('{ call -1480; r1 = r18; r2 = #0x4; r0 = memub(r17+#0x3) }',b)
if __name__=='__main__': unittest.main()
