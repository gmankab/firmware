# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 SRAM state switch source/branch and reference tests."""
import os,importlib.util,unittest
from pathlib import Path
P=Path(__file__).resolve().parents[1]/'tools'
s=importlib.util.spec_from_file_location('adsp_state',P/'adsp-agent20-rwe-next-sram-state-cases.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class TestSRAMStateCases(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_coverage_and_packet_source(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11754,0x11798,68))
        source=m.SOURCE.read_text()
        self.assertIn('if(p0.new) memw(r18+#0)=r3.new',source)
        self.assertIn('if(!p0.new) jump:nt state_published',source)
        self.assertIn('r2=#0; memw(r18+#0)=r17',source)
        self.assertNotIn('.incbin',source)
    def test_llvm_compiles_all_17_words(self):
        if not self.mc:self.skipTest('LLVM6 is required')
        code=m.render(self.mc)
        self.assertEqual(len(code),68)
        if self.ref:m.verify(Path(self.ref).read_bytes(),code)
    def test_corruption_and_revision_check(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and original ADSP required')
        base=Path(self.ref).read_bytes()
        code=m.render(self.mc);m.verify(base,code)
        wrong=bytearray(code);wrong[12]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(base,wrong)
        source=bytearray(base);source[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(source,code)
if __name__=='__main__':unittest.main()
