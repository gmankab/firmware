# SPDX-License-Identifier: GPL-3.0-only
"""Regression tests for the 100-byte Hexagon V66 nested-lookup/return path."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def load(name):
    p=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert p and p.loader
    m=importlib.util.module_from_spec(p);p.loader.exec_module(m);return m
m=load('rwe-next-controller-format-selector-cases-b.py')
a=load('rwe-next-controller-format-selector-cases-a.py')
b=load('rwe-next-controller-format-selector-prefix.py')
class ControllerFormatModeCasesB(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_has_real_scaled_lookup_and_epilogue(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x12764,0x127c8,100))
        self.assertEqual(a.END,m.START)
        asm=m.SOURCE.read_text()
        for x in ('r2=and(r3,#7)','r4=+mpyi(r2,#0xe0)',
                  'r2=add(r16,mpyi(#0xe0,r2))','if(cmp.eq(r4.new,r3)) jump:t match',
                  'r17:16=memd(r29+#24)','r19:18=memd(r29+#16)',
                  'dealloc_return'):
            self.assertIn(x,asm)
        self.assertNotIn('.incbin',asm)
        self.assertNotIn('.word',asm)
    def test_exact_reference_and_contiguous_cases(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned firmware required')
        reference=Path(self.ref).read_bytes();data=m.render(self.mc)
        m.verify(reference,data)
        self.assertEqual(b.render(self.mc)+a.render(self.mc)+data,reference[b.START:m.END])
    def test_reject_wrong_revision_and_corrupt_packet(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned firmware required')
        reference=Path(self.ref).read_bytes();data=m.render(self.mc)
        wrong=bytearray(reference);wrong[-1]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,data)
        corrupt=bytearray(data);corrupt[12]^=0x40
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(reference,corrupt)
if __name__=='__main__':unittest.main()
