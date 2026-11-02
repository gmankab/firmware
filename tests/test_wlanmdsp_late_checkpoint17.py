# SPDX-License-Identifier: GPL-3.0-only
"""Agent 14 checkpoint 17: auxiliary response and controller reset."""
import importlib.util
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parent.parent
SPEC=importlib.util.spec_from_file_location('agent14_cp17',ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=late
SPEC.loader.exec_module(late)

class ResponseAndResetTests(unittest.TestCase):
    def test_two_complete_adjacent_routines(self):
        old=late.select_regions(late.load_registry(),through_checkpoint=16)
        new=late.select_regions(late.load_registry(),through_checkpoint=17)
        self.assertEqual(len(new),len(old)+2)
        self.assertEqual([(r.file_start,r.file_end) for r in new[-2:]],[(0x103560,0x1035f8),(0x1035f8,0x103660)])
        self.assertEqual(sum(r.size_bytes for r in new),13468)
        self.assertEqual([r.checkpoint for r in new[-2:]],[17,17])
    def test_source_is_typed_and_has_no_oem_word_directives(self):
        for stem in ('interface-aux-reply','controller-io-reset'):
            source=(ROOT/f'sources/xiaomi-nabu/wlanmdsp/late/{stem}.S').read_text()
            for directive in ('.word','.long','.byte','.incbin','.fill'):
                self.assertNotIn(directive,source)
        source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/interface-aux-reply.S').read_text()
        self.assertIn('r17 |= asl(r7,#0x10)',source)
        self.assertIn('jump 1974888',source)
        reset=(ROOT/'sources/xiaomi-nabu/wlanmdsp/late/controller-io-reset.S').read_text()
        self.assertIn('dealloc_return',reset)
        self.assertIn('##0x6618',reset)

if __name__=='__main__':unittest.main()
