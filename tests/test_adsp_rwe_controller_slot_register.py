# SPDX-License-Identifier: GPL-3.0-only
"""Controller slot registration atomic-loop and V66 instruction checks."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools/adsp'
def load(name):
    p=TOOLS/name;s=importlib.util.spec_from_file_location(name.replace('.','_').replace('-','_'),p)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('rwe-next-controller-slot-register.py')
prior=load('rwe-next-controller-table-init.py')
class ControllerSlot(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_entry_and_original_return(self):
        self.assertEqual((m.START,m.END,m.SIZE,m.READ_SGP0_AT),(0x11ba0,0x11c88,232,0x11bb0))
        s=m.SOURCE.read_text()
        for cmd in ('r3=cl0(r18)','r2=ct0(r18)','memw_locked(r17,p0)=r19',
                    'memw_locked(r2,p0)=r4','memw_locked(r3,p0)=r4',
                    'call publish_registration','dealloc_return'):
            self.assertIn(cmd,s)
        self.assertNotIn('.incbin',s)
    def test_checked_v66_register(self):
        if not self.mc:self.skipTest('LLVM6 not configured')
        x=m.render(self.mc)
        self.assertEqual(struct.unpack_from('<I',x,m.READ_SGP0_AT-m.START)[0],0x6e80c002)
        self.assertNotIn(m.NOP,[x[i:i+4] for i in range(0,m.SIZE,4)])
    def test_exact_and_corruption_checks(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference needed')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(ref,b)
        bad=bytearray(b);bad[10]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
        wrong=bytearray(ref);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,b)
    def test_contiguous_two_function_code(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference needed')
        ref=Path(self.ref).read_bytes();prev=prior.render(self.mc);new=m.render(self.mc)
        self.assertEqual(prior.END,m.START)
        self.assertEqual(len(prev+new),424)
        self.assertEqual(prev+new,ref[prior.START:m.END])
if __name__=='__main__':unittest.main()
