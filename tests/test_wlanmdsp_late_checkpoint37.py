# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 37: source-authored controller session dispatch and frame pack."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('wlan_cp37',ROOT/'tools/build-wlanmdsp-late.py')
assert spec and spec.loader
late=importlib.util.module_from_spec(spec);sys.modules[spec.name]=late;spec.loader.exec_module(late)
class SessionControllerDispatch(unittest.TestCase):
    expected=[('session-controller-dispatch',0x107f7c,0x108288),('session-frame-pack',0x108288,0x108368)]
    def test_exclusive_bounds(self):
        prev=late.select_regions(late.load_registry(),through_checkpoint=36)
        current=late.select_regions(late.load_registry(),through_checkpoint=37)
        self.assertEqual(prev[-1].file_end,0x107f7c)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in current[-2:]],self.expected)
        self.assertEqual(len(current)-len(prev),2)
        self.assertEqual(sum(x.size_bytes for x in current[-2:]),1004)
        self.assertEqual(sum(x.size_bytes for x in current),33188)
    def test_source_code_only(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                src=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late'/f'{name}.S').read_text()
                self.assertIn('agent14_late_'+name.replace('-','_')+':',src)
                for x in ('.word','.long','.byte','.fill','.inst','.incbin'):self.assertNotIn(x,src)
    def test_packet_control(self):
        s=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/session-controller-dispatch.S').read_text()
        self.assertIn('jump ##1955196',s)
        f=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/session-frame-pack.S').read_text()
        self.assertEqual(f.count(':mem_noshuf'),2)
if __name__=='__main__':unittest.main()
