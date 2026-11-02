# SPDX-License-Identifier: GPL-3.0-only
"""Verify all 18 source-built controller format diagnostic reports."""
from __future__ import annotations
import importlib.util,os,re,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def imp(name):
    sp=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=imp('rwe-next-controller-format-selector-reports.py')
c=imp('rwe-next-controller-format-selector-cases-c.py')
b=imp('rwe-next-controller-format-selector-cases-b.py')
a=imp('rwe-next-controller-format-selector-cases-a.py')
prefix=imp('rwe-next-controller-format-selector-prefix.py')
class FormatReports(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_all_eighteen_handlers_are_instruction_source(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x1289c,0x129bc,288))
        self.assertEqual(c.END,m.START)
        self.assertEqual(len(m.FORMAT_MESSAGES),18)
        asm=m.SOURCE.read_text()
        for address in m.FORMAT_MESSAGES:
            self.assertIn(f'r0=##{address:#x}',asm)
        self.assertEqual(len(re.findall(r'^diagnostic_case_\d+:',asm,re.M)),18)
        self.assertEqual(asm.count('{ jump common_handler }'),18)
        self.assertNotIn('.incbin',asm)
        self.assertNotIn('.word',asm)
    def test_reference_and_full_selector_continuity(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and reference required')
        reference=Path(self.ref).read_bytes();generated=m.render(self.mc)
        m.verify(reference,generated)
        whole=(prefix.render(self.mc)+a.render(self.mc)+b.render(self.mc)+
               c.render(self.mc)+generated)
        self.assertEqual(len(whole),m.END-prefix.START)
        self.assertEqual(whole,reference[prefix.START:m.END])
    def test_wrong_revision_and_corrupt_word_rejected(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
        reference=Path(self.ref).read_bytes();data=m.render(self.mc)
        tampered=bytearray(reference);tampered[0]^=0x80
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(tampered,data)
        broken=bytearray(data);broken[0]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(reference,broken)
if __name__=='__main__':unittest.main()
