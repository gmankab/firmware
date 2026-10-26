# SPDX-License-Identifier: GPL-3.0-only
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
s=importlib.util.spec_from_file_location('controller_publish',T/'adsp-agent20-rwe-next-controller-publish.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class SRAMControllerPublish(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.environ.get('HEXAGON_LLVM_MC');cls.ref=os.environ.get('NABU_ADSP_REFERENCE')
    def test_source_and_boundaries(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11798,0x1180c,116))
        text=m.SOURCE.read_text()
        self.assertIn('call setup_hardware; immext(',text)
        self.assertIn('call issue_status; immext(',text)
        self.assertIn('memw(gp+##0xf01f10f0)=r2.new',text)
        self.assertNotIn('.incbin',text)
    def test_all_llvm_source_words_match(self):
        if not self.mc:self.skipTest('LLVM6 missing')
        b=m.render(self.mc)
        self.assertEqual(len(b),116)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)
    def test_reject_wrong_revision_or_changed_output(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and ADSP ref required')
        ref=Path(self.ref).read_bytes();original=m.render(self.mc);m.verify(ref,original)
        bad=bytearray(original);bad[-3]^=1
        with self.assertRaisesRegex(ValueError,'mismatch'):m.verify(ref,bad)
        wrong=bytearray(ref);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,original)
if __name__=='__main__':unittest.main()
