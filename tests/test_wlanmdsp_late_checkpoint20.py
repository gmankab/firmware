# SPDX-License-Identifier: GPL-3.0-only
"""Agent 14 checkpoint 20: four complete next interface routines."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp20',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)

class FourInterfaceRoutines(unittest.TestCase):
    def test_four_adjacent_functions(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=19)
        new=late.select_regions(late.load_registry(),through_checkpoint=20)
        self.assertEqual(len(new),len(old)+4)
        self.assertEqual([(x.file_start,x.file_end) for x in new[-4:]],
                         [(0x104398,0x104488),(0x104488,0x1044f0),
                          (0x1044f0,0x104524),(0x104524,0x1045e8)])
        self.assertEqual([x.size_bytes for x in new[-4:]],[240,104,52,196])
        self.assertEqual(sum(x.size_bytes for x in new),17444)
    def test_mnemonic_only_and_packet_constraints(self):
        sources=late.select_regions(late.load_registry(),through_checkpoint=20)[-4:]
        for item in sources:
            src=(ROOT/item.source).read_text()
            for token in ('.word','.long','.byte','.incbin','.org','.fill'):
                self.assertNotIn(token,src)
        handler=(ROOT/sources[-1].source).read_text()
        self.assertIn('p0 = cmp.eq(r18,#0x0); r1:0 = combine(#0x0,r17)',handler)
        self.assertIn('label_0064:',handler)
        self.assertIn('jump ##1970876',handler)
    def test_next_function_is_not_implicitly_added(self):
        # Bound this historical checkpoint test even after future regions append.
        r=late.select_regions(late.load_registry(), through_checkpoint=20)
        self.assertEqual(r[-1].file_end,0x1045e8)
        self.assertEqual(r[-1].vaddr + r[-1].size_bytes,0xb00c45e8)

if __name__=='__main__':unittest.main()
