# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 38: source-derived interface record inspection and type guard."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('wlan_cp38',ROOT/'tools/build-wlanmdsp-late.py')
assert spec and spec.loader
late=importlib.util.module_from_spec(spec);sys.modules[spec.name]=late;spec.loader.exec_module(late)
class InterfaceRecordInspection(unittest.TestCase):
    expected=[('session-record-codec-check',0x108368,0x1085e4),
              ('session-record-type-guard',0x1085e4,0x108724)]
    def test_adjacent_function_bounds(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=37)
        new=late.select_regions(late.load_registry(),through_checkpoint=38)
        self.assertEqual(old[-1].file_end,0x108368)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in new[-2:]],self.expected)
        self.assertEqual(len(new)-len(old),2)
        self.assertEqual(sum(x.size_bytes for x in new[-2:]),956)
        self.assertEqual(sum(x.size_bytes for x in new),34144)
    def test_named_opcode_source_only(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                src=(ROOT/'sources/xiaomi-nabu/wlanmdsp'/f'agent14-late-{name}.S').read_text()
                self.assertIn('agent14_late_'+name.replace('-','_')+':',src)
                for banned in ('.byte','.word','.long','.incbin','.fill','.inst'):
                    self.assertNotIn(banned,src)
    def test_packet_compatibility_controls(self):
        a=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-session-record-codec-check.S').read_text()
        self.assertIn('r0 = ##0xb02b7a7c; r2 = r18',a)
        self.assertIn('jump ##1954388',a)
        b=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-session-record-type-guard.S').read_text()
        self.assertIn('memb(r19+#0x0) = r2; r2 = memuh(r19+#0x0) } :mem_noshuf',b)
        self.assertIn('jump ##1954104',b)
if __name__=='__main__':unittest.main()
