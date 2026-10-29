# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 K0 lock and aligned controller error continuation."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def imp(name):
    sp=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=imp('adsp-agent20-rwe-next-controller-record-recovery.py')
selection=imp('adsp-agent20-rwe-next-controller-record-select.py')
lookup=imp('adsp-agent20-rwe-next-controller-record-lookup.py')
class RecordRecovery(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_boundary_and_symbolic_k0(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x13084,0x130c4,64))
        self.assertEqual(selection.END,m.START)
        s=m.SOURCE.read_text()
        for fragment in ('memw(r20+#12)=r2','memd(r19+#0x88)=r17:16',
                         'memw(r19+#0xec)=##0xf002111c','call controller_cleanup',
                         'nop } // checked V66 k0lock'):
            self.assertIn(fragment,s)
        self.assertNotIn('.word',s)
        self.assertNotIn('.incbin',s)
    def test_byte_identical_and_three_adjacent_fragments(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and ADSP reference required')
        ref=Path(self.ref).read_bytes();output=m.render(self.mc)
        m.verify(ref,output)
        self.assertEqual(output[m.K0_LOCK-m.START:m.K0_LOCK-m.START+4],struct.pack('<I',0x6c20c060))
        self.assertEqual(lookup.render(self.mc)+selection.render(self.mc)+output,ref[lookup.START:m.END])
        self.assertEqual(ref[m.END:m.END+12],bytes(12))
    def test_wrong_revision_and_corrupt_code(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and ADSP reference required')
        ref=Path(self.ref).read_bytes();actual=m.render(self.mc)
        bad=bytearray(ref);bad[12]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(bad,actual)
        altered=bytearray(actual);altered[20]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,altered)
if __name__=='__main__':unittest.main()
