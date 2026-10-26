# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 32: four source-derived buffered window control functions."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('wlan_cp32',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late;SPEC.loader.exec_module(late)
class WindowHandlers(unittest.TestCase):
    expected=[('window-state-update',0x106e8c,0x106f8c),('window-record-lookup',0x106f8c,0x107060),('window-record-validate',0x107060,0x1071a8),('window-slot-reset',0x1071a8,0x1071e0)]
    def test_coverage_and_owned_bounds(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=31)
        new=late.select_regions(late.load_registry(),through_checkpoint=32)
        self.assertEqual(len(new)-len(old),4)
        self.assertEqual(old[-1].file_end,0x106e8c)
        self.assertEqual([(r.name,r.file_start,r.file_end) for r in new[-4:]],self.expected)
        self.assertEqual(sum(r.size_bytes for r in new[-4:]),852)
        self.assertEqual(sum(r.size_bytes for r in new),28700)
        self.assertEqual(new[-1].file_end,0x1071e0)
    def test_no_raw_oem_instruction_directives(self):
        for n,a,b in self.expected:
            with self.subTest(n=n):
                s=(ROOT/'sources/xiaomi-nabu/wlanmdsp'/f'agent14-late-{n}.S').read_text()
                for instr in ('.word','.long','.byte','.incbin','.inst','.fill'):
                    self.assertNotIn(instr,s)
                self.assertIn('agent14_late_'+n.replace('-','_')+':',s)
    def test_source_order_preserves_original_packets(self):
        a=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-window-record-lookup.S').read_text()
        b=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-window-record-validate.S').read_text()
        self.assertIn('{ r18 = r0; r0 = ##0x6820; r20 = memw(r1+#0x0) }',a)
        self.assertIn('{ r0 = ##0x6838; r18 = r0; r17 = r1 }',b)
if __name__=='__main__':unittest.main()
