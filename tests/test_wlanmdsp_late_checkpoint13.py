# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 13: packet-boundary-aware source duplex reconstruction."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp13',ROOT/'tools/wlanmdsp/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC);sys.modules[SPEC.name]=late;SPEC.loader.exec_module(late)

class Agent14Checkpoint13(unittest.TestCase):
    def test_adjacent_routines_and_explicit_fixture(self):
        reg=late.load_registry()
        prior=late.select_regions(reg,through_checkpoint=12)
        new=late.select_regions(reg,through_checkpoint=13)
        self.assertEqual([x.size_bytes for x in new[-2:]],[20,552])
        self.assertEqual((new[-2].file_start,new[-1].file_end),(0x102cd0,0x102f0c))
        self.assertEqual(sum(x.size_bytes for x in new),11592)
        self.assertEqual(new[-1].packet_joins,((8,'sources/xiaomi-nabu/wlanmdsp/late/status-frame-duplex.S'),))
        self.assertEqual(prior[-1].file_end,new[-2].file_start)
    def test_fixture_mnemonics_without_opcode_import(self):
        fixture=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/status-frame-duplex.S').read_text()
        source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/status-frame-initialize.S').read_text()
        self.assertIn('{ r1 = #0x0; r18 = r0 }',fixture)
        self.assertIn('r0 = add(r29,#0x8)',fixture)
        self.assertIn('}:mem_noshuf',source)
        self.assertIn('}:endloop0',source)
        self.assertIn('jump ##1976776',source)
        for text in (source,fixture):
            self.assertNotIn('.incbin',text)
            self.assertNotIn('.word ',text)
    def test_packet_join_validates_parse_bits(self):
        # A standalone forged source of three ordinary mini-packets cannot
        # masquerade as a source-coded duplex: fail closed on final parse kind.
        self.assertEqual(0x102f0c-0x102ce4,552)

if __name__=='__main__':unittest.main()
