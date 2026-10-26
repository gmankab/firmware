# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 27: four exact Hexagon runtime session helpers."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('wlan_cp27',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late;SPEC.loader.exec_module(late)
class SessionHelperTests(unittest.TestCase):
    expected=[
        ('record-setup',0x105e70,0x105f48),
        ('session-record-publish',0x105f48,0x106044),
        ('session-record-release',0x106044,0x1060e0),
        ('session-status-guard',0x1060e0,0x106128),
    ]
    def test_adjacent_correct_count(self):
        previous=late.select_regions(late.load_registry(),through_checkpoint=26)
        current=late.select_regions(late.load_registry(),through_checkpoint=27)
        self.assertEqual(len(current)-len(previous),4)
        self.assertEqual(previous[-1].file_end,0x105e70)
        self.assertEqual([(r.name,r.file_start,r.file_end) for r in current[-4:]],self.expected)
        self.assertEqual(sum(x.size_bytes for x in current),24420)
        self.assertEqual(sum(x.size_bytes for x in current[-4:]),696)
    def test_signed_source_mnemonics_without_oem_words(self):
        for name,a,b in self.expected:
            with self.subTest(name=name):
                src=(ROOT/'sources/xiaomi-nabu/wlanmdsp'/f'agent14-late-{name}.S').read_text()
                for forbidden in ('.word','.byte','.long','.incbin','.fill','.inst'):
                    self.assertNotIn(forbidden,src)
                self.assertIn(f'agent14_late_{name.replace("-","_")}:',src)
                self.assertRegex(src,r'(?m)^\{[^\n]+\}(?:\s*:endloop0)?$')
if __name__=='__main__':unittest.main()
