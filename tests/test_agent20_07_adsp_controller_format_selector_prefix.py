# SPDX-License-Identifier: GPL-3.0-only
"""Partial indexed format dispatch and strict emitter/reference boundary."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-format-selector-prefix.py')
prev=load('adsp-agent20-rwe-next-controller-global-query-complete.py')
class FormatSelector(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_coverage_isa_and_noninline_table(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x126a8,0x126dc,52))
        self.assertEqual(prev.END+4,m.START)
        s=m.SOURCE.read_text()
        for term in ('mem_noshuf','##0xf000f9bc','jumpr r4','cmp.gtu(r4.new,#13)'):
            self.assertIn(term,s)
        self.assertNotIn('.incbin',s)
    def test_exact_reference(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        reference=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(reference,b)
        self.assertEqual(prev.render(self.mc)+bytes(4)+b,reference[prev.START:m.END])
    def test_wrong_image_and_mutation_rejected(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        r=Path(self.ref).read_bytes();b=m.render(self.mc)
        bad=bytearray(b);bad[0]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(r,bad)
        wrong=bytearray(r);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,b)
if __name__=='__main__':unittest.main()
