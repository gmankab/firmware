# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 35: session diagnostics, record update and bounded dispatch."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('wlan_cp35',ROOT/'tools/build-wlanmdsp-late.py')
assert spec and spec.loader
late=importlib.util.module_from_spec(spec);sys.modules[spec.name]=late;spec.loader.exec_module(late)

class SessionControllerHelpers(unittest.TestCase):
    expected=[('session-window-diagnostics',0x107710,0x1077e8),
              ('session-window-update',0x1077e8,0x1078d0),
              ('session-type-dispatch',0x1078d0,0x107970),
              ('session-invalid-mode-log',0x107970,0x107998),
              ('session-controller-create',0x107998,0x107b60)]
    def test_bounds_and_contiguous_coverage(self):
        prev=late.select_regions(late.load_registry(),through_checkpoint=34)
        actual=late.select_regions(late.load_registry(),through_checkpoint=35)
        self.assertEqual(prev[-1].file_end,0x107710)
        self.assertEqual([(x.name,x.file_start,x.file_end) for x in actual[-5:]],self.expected)
        self.assertEqual(len(actual)-len(prev),5)
        self.assertEqual(sum(x.size_bytes for x in actual[-5:]),1104)
        self.assertEqual(sum(x.size_bytes for x in actual),31132)
    def test_only_mnemonics_and_symbols(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                data=(ROOT/'sources/xiaomi-nabu/wlanmdsp'/f'agent14-late-{name}.S').read_text()
                self.assertIn('agent14_late_'+name.replace('-','_')+':',data)
                for forbidden in ('.byte','.word','.long','.incbin','.fill','.inst'):
                    self.assertNotIn(forbidden,data)
    def test_dispatch_extender_and_controller_source_order(self):
        a=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-session-type-dispatch.S').read_text()
        self.assertIn('jump ##-912',a)
        b=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-session-controller-create.S').read_text()
        self.assertIn('r4 = #-0x1; r5 = add(r29,#0x8)',b)
        self.assertIn('callr r5',b)

if __name__=='__main__':unittest.main()
