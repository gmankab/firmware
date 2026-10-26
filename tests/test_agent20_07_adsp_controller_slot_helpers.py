# SPDX-License-Identifier: GPL-3.0-only
"""Four independently assembled Hexagon controller-slot helper functions."""
from __future__ import annotations
import os,importlib.util,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('.','_').replace('-','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-slot-helpers.py')
release=load('adsp-agent20-rwe-next-controller-slot-release.py')
class SlotHelpers(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_boundaries_and_source(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11d68,0x11e6c,260))
        self.assertEqual(release.END,m.START)
        self.assertEqual(sum(e-b for b,e,_ in m.INTERVALS),m.SIZE)
        self.assertEqual(m.INTERVALS[-1][:2],(0x11e64,0x11e6c))
        s=m.SOURCE.read_text()
        for term in ('memw(r2+#12)=clrbit(#1)','memw(r2+#12)=setbit(#1)',
                     'dealloc_return','r0=##0x80000000','call slot_notify'):
            self.assertIn(term,s)
        self.assertNotIn('.incbin',s)
    def test_exact_reference_and_corruption(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();out=m.render(self.mc);m.verify(ref,out)
        fail=bytearray(out);fail[17]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(ref,fail)
        failref=bytearray(ref);failref[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(failref,out)
    def test_full_controller_registration_release_and_helpers(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        register=load('adsp-agent20-rwe-next-controller-slot-register.py')
        ref=Path(self.ref).read_bytes()
        data=register.render(self.mc)+release.render(self.mc)+m.render(self.mc)
        self.assertEqual(len(data),716)
        self.assertEqual(data,ref[register.START:m.END])
    def test_sentinel_is_llvm_encoded(self):
        if not self.mc:self.skipTest('LLVM6 unavailable')
        r=m.render(self.mc)
        self.assertEqual(r[-8:].hex(),'00400008c03f0048')
if __name__=='__main__':unittest.main()
