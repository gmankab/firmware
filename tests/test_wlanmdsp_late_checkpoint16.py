# SPDX-License-Identifier: GPL-3.0-only
"""Agent 14 checkpoint 16: controller response frame serialization."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp16',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)

class ControllerResponseTests(unittest.TestCase):
    def test_is_new_adjacent_complete_function(self):
        before=late.select_regions(late.load_registry(),through_checkpoint=15)
        after=late.select_regions(late.load_registry(),through_checkpoint=16)
        self.assertEqual(len(after)-len(before),1)
        self.assertEqual((before[-1].file_end,after[-1].file_start,after[-1].file_end),(0x1033c4,0x1033c4,0x103560))
        self.assertEqual(after[-1].size_bytes,412)
        self.assertEqual(sum(x.size_bytes for x in after),13212)
        self.assertEqual(after[-1].checkpoint,16)
        self.assertFalse(after[-1].packet_joins)
    def test_only_source_mnemonics_no_reference_opcodes(self):
        p=ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-controller-response-pack.S'
        code=p.read_text()
        self.assertIn('r5:4 = combine(#0,#0x0)',code)
        self.assertIn('r3 = mux(p0,#0x60,#0x20)',code)
        self.assertIn('memb(r16+#0x2a) = r0',code)
        self.assertIn('}:mem_noshuf',code)
        self.assertIn('jump ##1975020',code)
        for forbidden in ('.word','.long','.incbin','.byte','.fill'):
            self.assertNotIn(forbidden,code)
    def test_source_digest_is_bound_to_registry(self):
        r=late.select_regions(late.load_registry(),through_checkpoint=16)[-1]
        self.assertRegex(r.sha256,r'^[0-9a-f]{64}$')
        self.assertEqual(r.source,'sources/xiaomi-nabu/wlanmdsp/agent14-late-controller-response-pack.S')

if __name__=='__main__':unittest.main()
