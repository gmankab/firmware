# SPDX-License-Identifier: GPL-3.0-only
"""Controller-slot scanner instruction source, packet markers and revision."""
from __future__ import annotations
import importlib.util
import os
from pathlib import Path
import struct
import unittest

T = Path(__file__).resolve().parents[1] / 'tools'
def load(name):
    s = importlib.util.spec_from_file_location(name.replace('-', '_').replace('.', '_'), T / name)
    assert s and s.loader
    mod = importlib.util.module_from_spec(s)
    s.loader.exec_module(mod)
    return mod

m = load('adsp-agent20-rwe-next-controller-slot-scan-prefix.py')
prior = load('adsp-agent20-rwe-next-controller-slot-leaves.py')

class SlotScanPrefix(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc = os.environ.get('HEXAGON_LLVM_MC')
        cls.ref = os.environ.get('NABU_ADSP_REFERENCE')

    def test_start_and_source(self):
        self.assertEqual((m.START,m.END,m.SIZE), (0x121c0,0x12220,96))
        self.assertEqual(prior.END+4, m.START)
        source = m.SOURCE.read_text()
        for part in ('allocframe(#56)', 'loop0(scan_slots,r2)',
                     'if(cmp.eq(r2.new,r16))', 'memd(sp+#8)=r27:26'):
            self.assertIn(part, source)
        self.assertNotIn('.incbin', source)

    def test_exact_assembly_and_packet_order(self):
        if not self.mc or not self.ref:
            self.skipTest('LLVM6 and pinned reference required')
        b=m.render(self.mc)
        ref=Path(self.ref).read_bytes()
        m.verify(ref,b)
        self.assertEqual(len(b),96)
        self.assertEqual((struct.unpack_from('<I',b,0x121cc-m.START)[0]>>14)&3,1)
        self.assertEqual((struct.unpack_from('<I',b,0x121d0-m.START)[0]>>14)&3,1)
        self.assertEqual((struct.unpack_from('<I',b,0x121d4-m.START)[0]>>14)&3,0)
        self.assertEqual(prior.render(self.mc)+bytes(4)+b,ref[prior.START:m.END])

    def test_fail_closed_on_corruption_and_revision(self):
        if not self.mc or not self.ref:
            self.skipTest('LLVM6 and pinned reference required')
        orig=Path(self.ref).read_bytes()
        b=m.render(self.mc)
        corrupt=bytearray(b);corrupt[4]^=1
        with self.assertRaisesRegex(ValueError,'differs'):
            m.verify(orig,corrupt)
        wrong=bytearray(orig);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):
            m.verify(wrong,b)

if __name__=='__main__':unittest.main()
