# SPDX-License-Identifier: GPL-3.0-only
"""Reference-hash, packet and linker-padding tests for slot deactivation."""
from __future__ import annotations
import os,importlib.util,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    p=T/name;sp=importlib.util.spec_from_file_location(name.replace('.','_').replace('-','_'),p)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-slot-deactivate.py')
prev=load('adsp-agent20-rwe-next-controller-slot-reserve.py')
class Deactivate(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_bounds_and_named_source(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x12098,0x12114,124))
        self.assertEqual(m.START,prev.END)
        src=m.SOURCE.read_text()
        for phrase in ('r3=setbit(r16,#16)', 'memw(r1+#0)=#0',
                       'memw(r1+#4)=#0', 'r2=memw(##0xf00250c4)',
                       'if(p0) call trigger_slot_callback','dealloc_return'):
            self.assertIn(phrase,src)
        self.assertNotIn('.incbin',src)
    def test_exact_bytes_and_joined_pair(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();data=m.render(self.mc);m.verify(ref,data)
        self.assertEqual(data+bytes(4),ref[m.START:m.END+4])
        self.assertEqual(prev.render(self.mc)+data,ref[prev.START:m.END])
    def test_mismatched_source_and_corruption_rejected(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();data=m.render(self.mc)
        wrong=bytearray(data);wrong[12]^=0x10
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,wrong)
        wrongref=bytearray(ref);wrongref[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrongref,data)
if __name__=='__main__':unittest.main()
