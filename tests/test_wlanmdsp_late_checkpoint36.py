# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 36: five source-authored codec session command routines."""
from pathlib import Path
import importlib.util,sys,unittest
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('wlan_cp36',ROOT/'tools/build-wlanmdsp-late.py')
assert spec and spec.loader
late=importlib.util.module_from_spec(spec);sys.modules[spec.name]=late;spec.loader.exec_module(late)

class CodecSessionCommands(unittest.TestCase):
    expected=[('session-command-register',0x107b60,0x107c34),
              ('session-command-status',0x107c34,0x107c8c),
              ('session-record-revalidate',0x107c8c,0x107d30),
              ('session-record-command',0x107d30,0x107f38),
              ('session-short-status',0x107f38,0x107f7c)]
    def test_contiguous_mnemonic_coverage(self):
        prev=late.select_regions(late.load_registry(),through_checkpoint=35)
        now=late.select_regions(late.load_registry(),through_checkpoint=36)
        self.assertEqual(prev[-1].file_end,0x107b60)
        self.assertEqual([(r.name,r.file_start,r.file_end) for r in now[-5:]],self.expected)
        self.assertEqual(len(now)-len(prev),5)
        self.assertEqual(sum(r.size_bytes for r in now[-5:]),1052)
        self.assertEqual(sum(r.size_bytes for r in now),32184)
    def test_no_embedded_reference_opcodes(self):
        for name,_,_ in self.expected:
            with self.subTest(name=name):
                s=(ROOT/'sources/xiaomi-nabu/wlanmdsp'/f'agent14-late-{name}.S').read_text()
                self.assertIn('agent14_late_'+name.replace('-','_')+':',s)
                for forbidden in ('.word','.long','.byte','.incbin','.fill','.inst'):
                    self.assertNotIn(forbidden,s)
    def test_conditional_call_extender_is_explicit(self):
        s=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-session-record-command.S').read_text()
        self.assertIn('if (!p0) call ##-888',s)
        self.assertIn('r1 = #0x0; r16 = r1',s)

if __name__=='__main__':unittest.main()
