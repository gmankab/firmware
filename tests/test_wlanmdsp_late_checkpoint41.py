# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 41: controller event processing and status query helpers."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('wlan_cp41',ROOT/'tools/build-wlanmdsp-late.py')
assert spec and spec.loader
late=importlib.util.module_from_spec(spec);sys.modules[spec.name]=late;spec.loader.exec_module(late)
class ControllerEventsAndFeatureQueries(unittest.TestCase):
    expected=[('controller-event-pipeline',0x108bd4,0x10904c),
              ('session-diag-routed-call',0x10904c,0x109078),
              ('session-record-slot-array',0x109078,0x1090dc),
              ('session-flag-query',0x1090dc,0x1090f4),
              ('session-feature-query',0x1090f4,0x109114)]
    def test_contiguous_region_ownership(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=40)
        now=late.select_regions(late.load_registry(),through_checkpoint=41)
        self.assertEqual(old[-1].file_end,0x108bd4)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in now[-5:]],self.expected)
        self.assertEqual(len(now)-len(old),5)
        self.assertEqual(sum(x.size_bytes for x in now[-5:]),1344)
        self.assertEqual(sum(x.size_bytes for x in now),36688)
    def test_source_only_and_packet_controls(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late'/f'{name}.S').read_text()
                self.assertIn('agent14_late_'+name.replace('-','_')+':',source)
                for invalid in ('.word','.long','.byte','.fill','.incbin','.inst'):
                    self.assertNotIn(invalid,source)
        ev=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/controller-event-pipeline.S').read_text()
        self.assertIn('if (p0) call ##-745548',ev)
        self.assertIn('p0 = cmp.eq(r2,r4); r4 = add(r3,#1); r5 = r3',ev)
        arr=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/session-record-slot-array.S').read_text()
        self.assertIn('loop0(pkt_0040,r17)',arr)
        self.assertIn('}  :endloop0',arr)
        self.assertIn('jump ##1951620',arr)
if __name__=='__main__':unittest.main()
