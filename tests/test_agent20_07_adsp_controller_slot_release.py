# SPDX-License-Identifier: GPL-3.0-only
"""ADSP source-generated slot release with atomic-loop and bitmap checks."""
from __future__ import annotations
import os,importlib.util,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def openmod(filename):
    s=importlib.util.spec_from_file_location(filename.replace('-','_').replace('.','_'),T/filename)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=openmod('adsp-agent20-rwe-next-controller-slot-release.py')
prev=openmod('adsp-agent20-rwe-next-controller-slot-register.py')
class SlotRelease(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_scope_and_contiguous_functions(self):
        self.assertEqual((m.START,m.END,m.SIZE,m.READ_SGP0_AT),(0x11c88,0x11d68,224,0x11c94))
        self.assertEqual(prev.END,m.START)
        s=m.SOURCE.read_text()
        for cmd in ('memw_locked(r20,p0)=r2','memw_locked(r3,p0)=r21',
                    'memw_locked(r2,p0)=r3','r19=memw(r2<<#0+##0xf04148a8)',
                    'call resource_sync','dealloc_return'):
            self.assertIn(cmd,s)
        self.assertNotIn('.incbin',s)
    def test_checked_sgp0_slot(self):
        if not self.mc:self.skipTest('LLVM6 not configured')
        out=m.render(self.mc)
        self.assertEqual(struct.unpack_from('<I',out,m.READ_SGP0_AT-m.START)[0],0x6e80c002)
        self.assertNotIn(m.NOP,[out[i:i+4] for i in range(0,len(out),4)])
    def test_direct_reference_and_rejects_bad_output(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference required')
        ref=Path(self.ref).read_bytes();o=m.render(self.mc);m.verify(ref,o)
        fail=bytearray(o);fail[0]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,fail)
        failref=bytearray(ref);failref[3]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(failref,o)
    def test_previous_slot_registration_contiguous_456_bytes(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference required')
        b=prev.render(self.mc)+m.render(self.mc)
        self.assertEqual(len(b),456)
        self.assertEqual(b,Path(self.ref).read_bytes()[prev.START:m.END])
if __name__=='__main__':unittest.main()
