# SPDX-License-Identifier: GPL-3.0-only
"""ADSP codec mode dispatch cases with correct nested function pointer table."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    sp=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-format-selector-cases-a.py')
prior=load('adsp-agent20-rwe-next-controller-format-selector-prefix.py')
class FormatCasesA(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_ranges_and_architecture(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x126dc,0x12764,136))
        self.assertEqual(m.START,prior.END)
        text=m.SOURCE.read_text()
        for substring in ('p0=cmpb.eq(r19,#0x71)','r2=memw(r2<<#2+##0xf000f9f4)',
                          'jumpr r2','memw(r29+#0)=r18'):
            self.assertIn(substring,text)
        self.assertNotIn('.incbin',text)
    def test_exact_and_adjacent(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 or pinned reference unavailable')
        r=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(r,b)
        self.assertEqual(prior.render(self.mc)+b,r[prior.START:m.END])
    def test_corrupted_image_and_output_fail(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        wrong=bytearray(ref);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,b)
        bad=bytearray(b);bad[-6]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(ref,bad)
if __name__=='__main__':unittest.main()
