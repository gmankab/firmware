# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 39: session controller record handling and owner-pointer repair."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('wlan_cp39',ROOT/'tools/build-wlanmdsp-late.py')
assert spec and spec.loader
late=importlib.util.module_from_spec(spec);sys.modules[spec.name]=late;spec.loader.exec_module(late)
class ControllerRecordHandler(unittest.TestCase):
    expected=[('session-controller-record-handler',0x108724,0x108a14),
              ('session-owner-relink',0x108a14,0x108a80)]
    def test_contiguous_bounds(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=38)
        new=late.select_regions(late.load_registry(),through_checkpoint=39)
        self.assertEqual(old[-1].file_end,0x108724)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in new[-2:]],self.expected)
        self.assertEqual(len(new)-len(old),2)
        self.assertEqual(sum(x.size_bytes for x in new[-2:]),860)
        self.assertEqual(sum(x.size_bytes for x in new),35004)
    def test_mnemonics_only(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                src=(ROOT/'sources/xiaomi-nabu/wlanmdsp'/f'agent14-late-{name}.S').read_text()
                self.assertIn('agent14_late_'+name.replace('-','_')+':',src)
                for banned in ('.word','.long','.byte','.inst','.incbin','.fill'):
                    self.assertNotIn(banned,src)
    def test_far_branch_and_duplex_packet_controls(self):
        src=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-session-controller-record-handler.S').read_text()
        self.assertIn('jump ##1953856',src)
        self.assertIn('r17 = r1; r21 = r0; r23:22 = combine',src)
        self.assertIn('r3 = add(r29,#0x14); r2 = r22; r1 = r23',src)
if __name__=='__main__':unittest.main()
