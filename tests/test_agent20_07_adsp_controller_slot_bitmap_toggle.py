# SPDX-License-Identifier: GPL-3.0-only
"""Validate independent V66 source and exact slot-bitmap toggle bytes."""
from __future__ import annotations
import os,importlib.util,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('.','_').replace('-','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-slot-bitmap-toggle.py')
prev=load('adsp-agent20-rwe-next-controller-slot-bitmap-mask.py')
class Toggle(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_packet_semantics_and_bounds(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11f58,0x11fd8,128))
        self.assertEqual(m.START,prev.END)
        s=m.SOURCE.read_text()
        for t in ('r2=asl(r17,#4)','r1:0=bitsplit(r17,#5)',
                  'r5=clrbit(r4,r2)','r4=setbit(r4,r2)',
                  'memw(r1<<#2+##0xf020a188)=r5',
                  'memw(r3+r1<<#2)=r2','dealloc_return'):
            self.assertIn(t,s)
        self.assertNotIn('.incbin',s)
    def test_original_bytes_and_contiguous_helpers(self):
        if not self.mc or not self.ref:self.skipTest('Hexagon LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(ref,b)
        self.assertEqual(len(b),32*4)
        self.assertEqual(prev.render(self.mc)+b,ref[prev.START:m.END])
    def test_wrong_revision_and_data_corruption_rejected(self):
        if not self.mc or not self.ref:self.skipTest('Hexagon LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        corrupt=bytearray(b);corrupt[2]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,corrupt)
        corrupt_ref=bytearray(ref);corrupt_ref[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(corrupt_ref,b)
if __name__=='__main__':unittest.main()
