# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 record selection, linked pointers and normal return path."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def imp(name):
    s=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=imp('rwe-next-controller-record-select.py')
prior=imp('rwe-next-controller-record-lookup.py')
class AdspRecordSelect(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_valid_source_and_neighbor(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x12f50,0x13084,308))
        self.assertEqual(prior.END,m.START)
        s=m.SOURCE.read_text()
        for x in ('r2=asl(r20,#5)', 'memw(r2<<#0+##0xf02d1258)',
                  'r20=add(##0xf02d1250,asl(r20,#5))',
                  'p0=bitsclr(r2,r3)', 'memh(r3+#0)=setbit(#5)', 'dealloc_return'):
            self.assertIn(x,s)
        self.assertNotIn('.incbin',s)
        self.assertNotIn('.word',s)
    def test_independent_reassembly_and_adjacent_records(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 or reference missing')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(ref,b)
        self.assertEqual(b[m.SGP0-m.START:m.SGP0-m.START+4],struct.pack('<I',0x6e80c002))
        self.assertEqual(prior.render(self.mc)+b,ref[prior.START:m.END])
    def test_corruption_and_wrong_revision_rejected(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 or reference missing')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        corrupted=bytearray(ref);corrupted[19]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(corrupted,b)
        broken=bytearray(b);broken[-11]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,broken)
if __name__=='__main__':unittest.main()
