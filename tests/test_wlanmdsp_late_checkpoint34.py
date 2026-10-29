# SPDX-License-Identifier: GPL-3.0-only
"""Checkpoint 34: five typed V66 WLAN slot/window helpers."""
from pathlib import Path
import importlib.util, sys, unittest
ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location('wlan_cp34', ROOT/'tools/build-wlanmdsp-late.py')
assert SPEC and SPEC.loader
late = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = late
SPEC.loader.exec_module(late)

class WindowSlotStartup(unittest.TestCase):
    expected = [
        ('window-slot-bootstrap',0x107568,0x107598),
        ('window-slot-summary',0x107598,0x1075c4),
        ('session-window-flush',0x1075c4,0x107634),
        ('session-window-packet',0x107634,0x1076e4),
        ('window-slot-reinit',0x1076e4,0x107710),
    ]
    def test_contiguous_coverage_and_previous_boundary(self):
        previous=late.select_regions(late.load_registry(),through_checkpoint=33)
        current=late.select_regions(late.load_registry(),through_checkpoint=34)
        self.assertEqual(previous[-1].file_end,0x107568)
        self.assertEqual([(r.name,r.file_start,r.file_end) for r in current[-5:]], self.expected)
        self.assertEqual(len(current)-len(previous),5)
        self.assertEqual(sum(r.size_bytes for r in current[-5:]),424)
        self.assertEqual(sum(r.size_bytes for r in current),30028)
        self.assertEqual(current[-1].file_end,0x107710)
    def test_assembly_mnemonics_only(self):
        for name, start, end in self.expected:
            with self.subTest(name=name):
                p=ROOT/'sources/xiaomi-nabu/wlanmdsp'/f'agent14-late-{name}.S'
                source=p.read_text()
                self.assertIn('agent14_late_'+name.replace('-','_')+':',source)
                for prohibited in ('.word','.byte','.long','.incbin','.fill','.inst'):
                    self.assertNotIn(prohibited,source)
    def test_packet_preservation(self):
        source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-window-slot-bootstrap.S').read_text()
        self.assertIn('r4 = add(r29,#0x4); r5 = add(r29,#0x10)',source)
        source=(ROOT/'sources/xiaomi-nabu/wlanmdsp/agent14-late-session-window-packet.S').read_text()
        self.assertIn(':endloop0',source)
        self.assertIn('r24 -= mpyi(r0,#0xff)',source)
        self.assertIn('loop0(pkt_004c,r22)',source)

if __name__=='__main__':unittest.main()
