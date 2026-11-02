# SPDX-License-Identifier: GPL-3.0-only
"""ADSP controller slot publication packet correctness checks."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('rwe-next-controller-slot-bitmap-publish.py')
status=load('rwe-next-controller-slot-status.py')
class BitmapPublication(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_not_raw_firmware(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11ed0,0x11f18,72))
        self.assertEqual(status.END,m.START)
        src=m.SOURCE.read_text()
        for s in ('if(p0.new) jump:nt publish','if(p0.new) r2=memw(##0xf000eac0)',
                  'bitsplit(r16,#5)','call publish_slot_state','dealloc_return'):
            self.assertIn(s,src)
        self.assertNotIn('.incbin',src)
    def test_exact_llvm6_source_and_contiguity(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();out=m.render(self.mc);m.verify(ref,out)
        self.assertEqual(len(out),18*4)
        self.assertEqual(status.render(self.mc)+out,ref[status.START:m.END])
    def test_changed_bytes_or_wrong_revision_rejected(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();out=m.render(self.mc);bad=bytearray(out);bad[-1]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
        wrong=bytearray(ref);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,out)
if __name__=='__main__':unittest.main()
