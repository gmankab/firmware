# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 40: session window binding and controller context allocation."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('wlan_cp40',ROOT/'tools/wlanmdsp/build-wlanmdsp-late.py')
assert spec and spec.loader
late=importlib.util.module_from_spec(spec);sys.modules[spec.name]=late;spec.loader.exec_module(late)
class WindowAndControllerInitialization(unittest.TestCase):
    expected=[('controller-window-bind',0x108a80,0x108b34),
              ('controller-context-allocate',0x108b34,0x108bd4)]
    def test_contiguous_coverage(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=39)
        new=late.select_regions(late.load_registry(),through_checkpoint=40)
        self.assertEqual(old[-1].file_end,0x108a80)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in new[-2:]],self.expected)
        self.assertEqual(len(new)-len(old),2)
        self.assertEqual(sum(x.size_bytes for x in new[-2:]),340)
        self.assertEqual(sum(x.size_bytes for x in new),35344)
    def test_only_mnemonics_and_packet_order(self):
        for name,_,_ in self.expected:
            source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late'/f'{name}.S').read_text()
            self.assertIn('agent14_late_'+name.replace('-','_')+':',source)
            for prohibited in ('.word','.long','.byte','.incbin','.fill','.inst'):
                self.assertNotIn(prohibited,source)
        src=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/controller-window-bind.S').read_text()
        self.assertEqual(src.count(':mem_noshuf'),5)
        self.assertIn('memw(r16+#0x10) = r2; r2 = memw(r17+#0xc0)',src)
if __name__=='__main__':unittest.main()
