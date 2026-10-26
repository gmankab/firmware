# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 21: exact Hexagon V66 interface-record dispatch and packet rules."""
import importlib.util
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location('agent14_cp21', ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = late
SPEC.loader.exec_module(late)

class InterfaceRecordDispatcherTests(unittest.TestCase):
    def test_complete_function_is_adjacent_and_bounded(self):
        old = late.select_regions(late.load_registry(), through_checkpoint=20)
        new = late.select_regions(late.load_registry(), through_checkpoint=21)
        self.assertEqual(len(new),len(old)+1)
        self.assertEqual((new[-1].file_start,new[-1].file_end),(0x1045e8,0x1050d8))
        self.assertEqual(new[-1].size_bytes,2800)
        self.assertEqual(sum(x.size_bytes for x in new),20244)
        self.assertEqual(new[-1].vaddr,0xb00c45e8)
        self.assertEqual(new[-1].vaddr+new[-1].size_bytes,0xb00c50d8)
        self.assertLess(new[-1].file_end,late.OWNED_FILE_END)

    def test_typed_hexagon_packet_source_preserves_extenders_and_order(self):
        src=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-interface-record-dispatch.S').read_text()
        self.assertIn('if (p0) call ##-542440',src)
        self.assertIn('if (p0) call ##-542668',src)
        self.assertIn('{ memw(r2+#0x10) = r3; r2 = memw(r18+#0x0) }:mem_noshuf',src)
        self.assertIn('{ memw(r2+#0x10) = r3; r2 = memw(r22+#0x0) }:mem_noshuf',src)
        self.assertIn('label_00d4:',src)
        self.assertIn('label_09ac:',src)
        self.assertEqual(src.count('// offset 0x'),338)
        for token in ('.word','.long','.byte','.incbin','.fill','.org'):
            self.assertNotIn(token,src)

    def test_registration_rejects_overlapping_or_out_of_range_fragments(self):
        import copy
        info=late.load_registry()
        self.assertEqual(info[-1].sha256,'edb482bed18bae21b5da93724800ea18b9e9c7054ed45cbcf838b176287b619a')
        payload=__import__('json').loads((ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-regions.json').read_text())
        bad=copy.deepcopy(payload)
        bad['regions'][-1]['file_start']-=4
        with self.assertRaises(late.BuildError):late.validate_registry(bad)
        bad=copy.deepcopy(payload)
        bad['regions'][-1]['file_end']=late.OWNED_FILE_END+4
        with self.assertRaises(late.BuildError):late.validate_registry(bad)

if __name__=='__main__':unittest.main()
