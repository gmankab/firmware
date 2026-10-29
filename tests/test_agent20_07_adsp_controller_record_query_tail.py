# SPDX-License-Identifier: GPL-3.0-only
"""Check Hexagon record-query insertion/recovery instructions and K0 lock."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def imp(name):
    sp=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=imp('adsp-agent20-rwe-next-controller-record-query-tail.py')
prior=imp('adsp-agent20-rwe-next-controller-record-query.py')
class RecordQueryRecovery(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_and_k0_boundary(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x131e0,0x13250,112))
        self.assertEqual(prior.END,m.START)
        s=m.SOURCE.read_text()
        for x in ('find_insert:', 'k0_publish:',
                  'if(!cmp.gtu(r5.new,r4)) jump:t find_insert',
                  'memw(r18+#0)=r2; memw(r3+#0)=r18',
                  'jump k0_publish; memw(r18+#0)=#0',
                  'jump k0_publish; memw(r18+#0)=r2'):
            self.assertIn(x,s)
        self.assertNotIn('.incbin',s)
        self.assertNotIn('.word',s)
    def test_exact_original_and_combined_query(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and reference required')
        original=Path(self.ref).read_bytes();data=m.render(self.mc)
        m.verify(original,data)
        self.assertEqual(data[m.K0_LOCK-m.START:m.K0_LOCK-m.START+4],struct.pack('<I',0x6c20c060))
        self.assertEqual(prior.render(self.mc)+data,original[prior.START:m.END])
    def test_mutations_fail(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and reference required')
        original=Path(self.ref).read_bytes();data=m.render(self.mc)
        wrong=bytearray(original);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,data)
        changed=bytearray(data);changed[-1]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(original,changed)
if __name__=='__main__':unittest.main()
