# SPDX-License-Identifier: GPL-3.0-only
"""Staged Hexagon controller-slot scanner, reference corruption and packets."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'

def load(name):
    spec=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert spec and spec.loader
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);return mod

m=load('adsp-agent20-rwe-next-controller-slot-scan-operations.py')
prefix=load('adsp-agent20-rwe-next-controller-slot-scan-prefix.py')

class ScanOperations(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_and_boundaries(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x12220,0x12310,240))
        self.assertEqual(prefix.END,m.START)
        self.assertEqual(sum(x[1] for x in m.PARTS),m.SIZE)
        s='\n'.join((T/name).read_text() for name,length in m.PARTS)
        for keyword in ('memw_locked(r17)', 'memh(r17+#212)=r3.new',
                        'loop0', 'memw(GP+##0xf020a0e4)'):
            if keyword!='loop0':self.assertIn(keyword,s)
        self.assertNotIn('.incbin',s)
    def test_roundtrip(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        reference=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(reference,b)
        self.assertEqual(prefix.render(self.mc)+b,reference[prefix.START:m.END])
    def test_rejected_revision_and_mutation(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        original=Path(self.ref).read_bytes();b=m.render(self.mc)
        corrupt=bytearray(b);corrupt[49]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(original,corrupt)
        rev=bytearray(original);rev[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(rev,b)
if __name__=='__main__':unittest.main()
