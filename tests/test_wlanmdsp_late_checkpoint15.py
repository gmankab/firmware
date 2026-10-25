# SPDX-License-Identifier: GPL-3.0-only
"""Agent 20-14 checkpoint 15: status command packet reconstruction."""
from pathlib import Path
import importlib.util
import sys
import unittest

ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp15',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)
SRC=ROOT/'sources/xiaomi-nabu/wlanmdsp'

class ControllerFrameCommandTests(unittest.TestCase):
    def test_checkpoint_15_is_exactly_one_adjacent_routine(self):
        before=late.select_regions(late.load_registry(),through_checkpoint=14)
        after=late.select_regions(late.load_registry(),through_checkpoint=15)
        self.assertEqual(len(after)-len(before),1)
        self.assertEqual((before[-1].file_end,after[-1].file_start,after[-1].file_end),
                         (0x103110,0x103110,0x1033c4))
        self.assertEqual(after[-1].size_bytes,692)
        self.assertEqual(sum(r.size_bytes for r in after),12800)
        self.assertEqual(after[-1].packet_joins,(
            (0x4c,'sources/xiaomi-nabu/wlanmdsp/agent14-late-controller-frame-duplex-a.S'),
            (0x14c,'sources/xiaomi-nabu/wlanmdsp/agent14-late-controller-frame-duplex-b.S')))

    def test_instructions_and_duplexes_are_typed_source(self):
        body=(SRC/'agent14-late-controller-frame-command.S').read_text()
        first=(SRC/'agent14-late-controller-frame-duplex-a.S').read_text()
        second=(SRC/'agent14-late-controller-frame-duplex-b.S').read_text()
        self.assertIn('jump ##1975360',body)
        self.assertIn('r23 = add(r20,#0x28)',second)
        self.assertIn('r5:4 = combine(#0,#0x0)',first)
        self.assertIn('memb(r23+#0x3) = r26',body)
        for text in [body,first,second]:
            for directive in ('.word ','.long ','.incbin','.byte ','.fill '):
                self.assertNotIn(directive,text)

    def test_rejects_overlapping_packet_fixture_offsets(self):
        import json
        payload=json.loads((SRC/'agent14-late-regions.json').read_text())
        new=payload['regions'][-1]
        new['packet_joins'][1]['offset']=new['packet_joins'][0]['offset']+4
        with self.assertRaisesRegex(late.BuildError,'packet join overlaps'):
            late.validate_registry(payload)

if __name__=='__main__':unittest.main()
