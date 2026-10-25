# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 09: full WLAN command/status parser source and exclusive bounds."""
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parent.parent
REGISTRY = ROOT / 'sources/xiaomi-nabu/wlanmdsp/agent14-late-regions.json'
ASM = ROOT / 'sources/xiaomi-nabu/wlanmdsp/agent14-late-command-parser.S'
S = importlib.util.spec_from_file_location('agent14_checkpoint09_late', ROOT / 'tools/build-wlanmdsp-late.py')
assert S and S.loader
import sys
late = importlib.util.module_from_spec(S)
sys.modules[S.name] = late
S.loader.exec_module(late)


class CommandParserCheckpoint09Tests(unittest.TestCase):
    def test_contiguous_parser_after_checkpoint_08(self):
        r = late.load_registry()
        prior = late.select_regions(r, through_checkpoint=8)
        current = late.select_regions(r, through_checkpoint=9)
        self.assertEqual(prior[-1].file_end, 0x102368)
        self.assertEqual((current[-1].file_start, current[-1].file_end),
                         (0x102368, 0x1026BC))
        self.assertEqual(len(current) - len(prior), 1)
        self.assertEqual(current[-1].size_bytes, 852)
        self.assertEqual(sum(x.size_bytes for x in current), 9464)
        self.assertLess(current[-1].file_end, late.OWNED_FILE_END)
        self.assertEqual(current[-1].vaddr, 0xB00C2368)

    def test_three_distant_jumps_preserve_explicit_constant_extension(self):
        source = ASM.read_text()
        self.assertIn('jump ##1979164', source)
        self.assertIn('jump ##1979100', source)
        self.assertIn('jump ##-719072', source)
        self.assertNotIn('.incbin', source)
        self.assertNotIn('.word ', source)
        self.assertNotIn('.long ', source)

    def test_old_value_packet_dependencies_remain_explicit(self):
        body = ASM.read_text()
        self.assertIn('r0 = #0x2; r21 = ##0xffff; r3 = memub(r20+#0x0)', body)
        self.assertIn('r1:0 = combine(r19,#0); r3:2 = combine(#0x0,#0x0);', body)
        self.assertIn('r5 = memub(r22+#0x3)', body)
        self.assertIn('memub(r1++#0x4)', body)

    def test_code_ends_before_next_function(self):
        record = late.select_regions(late.load_registry(),through_checkpoint=9)[-1]
        self.assertEqual(record.symbol, 'agent14_late_command_parser')
        self.assertEqual(record.sha256, 'fd4d53401e3e295ac4be69fc5eec69761fbb9adc641d719d57ab461c835ddbc7')
        self.assertIn('jumpr r31', ASM.read_text())
        self.assertIn('jump 311232', ASM.read_text())


if __name__ == '__main__':
    unittest.main()
