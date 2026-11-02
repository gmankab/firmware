# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 26: record dispatch and boundary audit."""
import importlib.util
from pathlib import Path
import sys
import unittest
ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp26',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC);sys.modules[SPEC.name]=late;SPEC.loader.exec_module(late)

class RecordDispatchTests(unittest.TestCase):
    def test_chained_complete_function(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=25)
        current=late.select_regions(late.load_registry(),through_checkpoint=26)
        self.assertEqual(len(current),len(old)+1)
        self.assertEqual(old[-1].file_end,0x105bac)
        r=current[-1]
        self.assertEqual((r.file_start,r.file_end,r.size_bytes,r.vaddr),
                         (0x105bac,0x105e70,708,0xb00c5bac))
        self.assertEqual(sum(x.size_bytes for x in current),23724)
    def test_no_oem_opcode_arrays(self):
        s=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/record-validate-dispatch.S').read_text()
        self.assertIn('agent14_late_record_validate_dispatch:',s)
        for directive in ('.word','.long','.byte','.incbin','.inst','.fill'):
            self.assertNotIn(directive,s)
        self.assertIn('pkt_0264:',s)
        self.assertNotIn('checkpoint50',s.lower())

if __name__=='__main__':unittest.main()
