# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 33: five window-record helpers, no source-gap or OEM bytes."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('agent14_cp33',ROOT/'tools/build-wlanmdsp-late.py')
assert spec and spec.loader
late=importlib.util.module_from_spec(spec);sys.modules[spec.name]=late;spec.loader.exec_module(late)
class WindowRecordHelpers(unittest.TestCase):
    expected=[('window-record-resolve',0x1071e0,0x1072ec),('window-record-commit',0x1072ec,0x1073a0),('window-record-validate-mode',0x1073a0,0x1073f4),('window-descriptor-prepare',0x1073f4,0x107438),('window-data-lookup',0x107438,0x107568)]
    def test_coverage(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=32)
        current=late.select_regions(late.load_registry(),through_checkpoint=33)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in current[-5:]],self.expected)
        self.assertEqual(old[-1].file_end,0x1071e0)
        self.assertEqual(sum(x.size_bytes for x in current[-5:]),904)
        self.assertEqual(sum(x.size_bytes for x in current),29604)
        self.assertEqual(len(current)-len(old),5)
    def test_source_no_original_bytes(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late'/f'{name}.S').read_text()
                self.assertIn('agent14_late_'+name.replace('-','_')+':',source)
                for forbidden in ('.word','.long','.byte','.incbin','.inst','.fill'):
                    self.assertNotIn(forbidden,source)
    def test_preserved_hexagon_packets(self):
        root=ROOT/'sources/xiaomi-nabu/wlanmdsp/late'
        a=(root/'window-record-resolve.S').read_text()
        b=(root/'window-record-commit.S').read_text()
        c=(root/'window-record-validate-mode.S').read_text()
        d=(root/'window-data-lookup.S').read_text()
        self.assertIn(':mem_noshuf',a)
        self.assertEqual(b.count(':mem_noshuf'),3)
        self.assertIn('{ r2 = r0; r5 = #0x0; r1:0 = combine(#0x48,r18); r4 = memw(r17+#0x0) }',c)
        self.assertIn('{ call -739004; r17 = #0x1; r0 = #0x13; r1 = and(r16,#255) }',d)
if __name__=='__main__':unittest.main()
