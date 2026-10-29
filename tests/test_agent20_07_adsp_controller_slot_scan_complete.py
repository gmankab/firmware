# SPDX-License-Identifier: GPL-3.0-only
"""Complete ADSP slot scanner control-flow, byte-identity and revision tests."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'

def load(name):
    s=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-slot-scan-complete.py')
prior=load('adsp-agent20-rwe-next-controller-slot-leaves.py')

class CompleteScan(unittest.TestCase):
    @classmethod
    def setUpClass(cls): cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_explicit_coverage_and_isa(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x121c0,0x1242c,620))
        self.assertEqual(sum(length for _,length in m.PARTS),m.SIZE)
        self.assertEqual(prior.END+4,m.START)
        text='\n'.join((T/name).read_text() for name in (
            'adsp-agent20-rwe-next-controller-slot-scan-prefix.S',
            'adsp-agent20-rwe-next-controller-slot-scan-setup.S',
            'adsp-agent20-rwe-next-controller-slot-scan-atomic.S',
            'adsp-agent20-rwe-next-controller-slot-scan-tag.S',
            'adsp-agent20-rwe-next-controller-slot-scan-bitmap.S',
            'adsp-agent20-rwe-next-controller-slot-scan-classes.S',
            'adsp-agent20-rwe-next-controller-slot-scan-epilogue.S'))
        for mnemonic in ('memw_locked(r17)', 'loop0(scan_slots,r2)',
                         'memw_locked(r3,p0)=r5','dealloc_return'):
            self.assertIn(mnemonic,text)
        self.assertNotIn('.incbin',text)
    def test_complete_roundtrip(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        reference=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(reference,b)
        self.assertEqual(prior.render(self.mc)+bytes(4)+b,reference[prior.START:m.END])
    def test_bad_image_bad_bytes_and_missing_alignment(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        broken=bytearray(b);broken[-8]^=8
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,broken)
        wrong=bytearray(ref);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,b)
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,b[:-4])
if __name__=='__main__':unittest.main()
