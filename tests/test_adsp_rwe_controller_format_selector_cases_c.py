# SPDX-License-Identifier: GPL-3.0-only
"""Verify Hexagon V66 mixed conditional branches and shared return packets."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def imp(file):
    sp=importlib.util.spec_from_file_location(file.replace('-','_').replace('.','_'),T/file)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=imp('rwe-next-controller-format-selector-cases-c.py')
a=imp('rwe-next-controller-format-selector-cases-a.py')
b=imp('rwe-next-controller-format-selector-cases-b.py')
start=imp('rwe-next-controller-format-selector-prefix.py')
class SelectorConditionalPaths(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_address_and_source_scope(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x127c8,0x1289c,212))
        self.assertEqual(b.END,m.START)
        s=m.SOURCE.read_text()
        for token in ('if(p0.new) jump:nt case_k','if(p0.new) jump:nt case_l',
                      'if(!p0.new) jump:t case_m','r2=add(r16,mpyi(#0xe0,r2))',
                      'r5=memw(r2+#0x74)','memw(sp+#12)=r5.new','dealloc_return'):
            self.assertIn(token,s)
        self.assertNotIn('.incbin',s)
        self.assertNotIn('.word',s)
    def test_source_assemble_full_adjacent_prefix(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
        ref=Path(self.ref).read_bytes();assembled=m.render(self.mc)
        m.verify(ref,assembled)
        combined=start.render(self.mc)+a.render(self.mc)+b.render(self.mc)+assembled
        self.assertEqual(len(combined),m.END-start.START)
        self.assertEqual(combined,ref[start.START:m.END])
    def test_refuse_revision_mismatch_or_corrupt_packets(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
        ref=Path(self.ref).read_bytes();actual=m.render(self.mc)
        wrong=bytearray(ref);wrong[123]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,actual)
        broken=bytearray(actual);broken[-1]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(ref,broken)
if __name__=='__main__':unittest.main()
