# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 28: five exact V66 codec session and resource helpers."""
import importlib.util,sys,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('wlan_cp28',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC);sys.modules[SPEC.name]=late;SPEC.loader.exec_module(late)
class CodecModeHelpers(unittest.TestCase):
    names=['codec-mode-router','codec-mode-allocate','codec-mode-notify','codec-mode-counter','codec-mode-access']
    bounds=[(0x106128,0x1061ec),(0x1061ec,0x1062a8),(0x1062a8,0x106318),(0x106318,0x106360),(0x106360,0x1063f4)]
    def test_exact_boundaries(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=27)
        new=late.select_regions(late.load_registry(),through_checkpoint=28)
        self.assertEqual(len(new),len(old)+5)
        self.assertEqual(old[-1].file_end,0x106128)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in new[-5:]],[(name,*b) for name,b in zip(self.names,self.bounds)])
        self.assertEqual(sum(x.size_bytes for x in new),25136)
        self.assertEqual(sum(x.size_bytes for x in new[-5:]),716)
    def test_name_only_instructions_no_reference_data(self):
        for name in self.names:
            with self.subTest(name=name):
                s=(ROOT/'sources/xiaomi-nabu/wlanmdsp'/f'agent14-late-{name}.S').read_text()
                for prohibited in ('.word','.long','.byte','.incbin','.inst','.fill'):
                    self.assertNotIn(prohibited,s)
                self.assertIn('agent14_late_'+name.replace('-','_')+':',s)
                self.assertRegex(s,r'(?m)^\{[^\n]+\}(?::mem_noshuf|\s+:endloop0)?$')
    def test_mixed_memory_packet_uses_explicit_ordering(self):
        s=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-codec-mode-access.S').read_text()
        self.assertIn('memw(r18+#0x10) = #0x1; r2 = memw(r16+#0x18) }:mem_noshuf',s)
if __name__=='__main__':unittest.main()
