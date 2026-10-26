# SPDX-License-Identifier: GPL-3.0-only
"""Exact source and packet checks for class-index bitmap masks."""
from __future__ import annotations
import os,importlib.util,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('.','_').replace('-','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-slot-bitmap-mask.py')
pub=load('adsp-agent20-rwe-next-controller-slot-bitmap-publish.py')
class BitmapMask(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_packet_contract(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11f18,0x11f58,64))
        self.assertEqual(pub.END,m.START)
        s=m.SOURCE.read_text()
        for snippet in ('r2=and(r16,#31)','r16=and(##0x1ffffffc,lsr(r16,#3))',
                        'r3=memw(r16<<#0+##0xf020a188)','p0=!tstbit(r3,r2)',
                        'if(!p0.new) r0=##0x10000'):
            self.assertIn(snippet,s)
        self.assertNotIn('.incbin',s)
    def test_source_exact_and_contiguous(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();new=m.render(self.mc)
        m.verify(ref,new)
        self.assertEqual(len(new),16*4)
        self.assertEqual(pub.render(self.mc)+new,ref[pub.START:m.END])
    def test_rejects_corruption_and_wrong_revision(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();data=m.render(self.mc);invalid=bytearray(data);invalid[5]^=0x40
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,invalid)
        invalid_ref=bytearray(ref);invalid_ref[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(invalid_ref,data)
if __name__=='__main__':unittest.main()
