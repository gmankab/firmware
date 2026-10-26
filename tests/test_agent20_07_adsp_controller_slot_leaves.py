# SPDX-License-Identifier: GPL-3.0-only
"""Four controller slot leaves: exact function boundaries and checked SGP0."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('.','_').replace('-','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-slot-leaves.py')
prior=load('adsp-agent20-rwe-next-controller-slot-deactivate.py')
class SlotLeaves(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_file_and_function_boundaries(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x12118,0x121bc,164))
        self.assertEqual(m.FUNCTIONS,((0x12118,0x12140),(0x12140,0x12168),
                                     (0x12168,0x121a0),(0x121a0,0x121bc)))
        self.assertEqual(m.START,prior.END+4)
        s=m.SOURCE.read_text()
        for fragment in ('r0=mux(p0,#1,#0)','r6=setbit(r2,#17)',
                         'memw(r3+r1<<#2)=r4.new','r4=setbit(r2,#16)',':mem_noshuf'):
            self.assertIn(fragment,s)
        self.assertNotIn('.incbin',s)
    def test_render_and_exact_comparison(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();data=m.render(self.mc);m.verify(ref,data)
        self.assertEqual(struct.unpack_from('<I',data,m.SGP0_AT-m.START)[0],0x6e80c003)
        self.assertEqual(prior.render(self.mc)+bytes(4)+data,ref[prior.START:m.END])
    def test_reject_invalid_revision_and_mutation(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();data=m.render(self.mc)
        bad=bytearray(data);bad[26]^=4
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
        rev=bytearray(ref);rev[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(rev,data)
if __name__=='__main__':unittest.main()
