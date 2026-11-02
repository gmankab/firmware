# SPDX-License-Identifier: GPL-3.0-only
"""ADSP controller slot search, bitmap release and original return."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
ASSEMBLY_DIR=Path(__file__).resolve().parents[1]/'sources/xiaomi-nabu/adsp/rwe'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('rwe-next-controller-slot-remove-complete.py')
prev=load('rwe-next-controller-slot-scan-complete.py')
class SlotRemove(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_structured_source(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x12430,0x1259c,364))
        self.assertEqual(m.START,prev.END+4)
        self.assertEqual(sum(n for _,n in m.PARTS),m.SIZE)
        contents='\n'.join((ASSEMBLY_DIR/name).read_text() for name,_ in m.PARTS)
        for term in ('memw_locked(r18)','memw_locked(r19,p0)=r4',
                     'if(!cmp.eq(r2.new,r16))','dealloc_return'):
            self.assertIn(term,contents)
        self.assertNotIn('.incbin',contents)
    def test_full_roundtrip(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and reference unavailable')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(ref,b)
        self.assertEqual(prev.render(self.mc)+bytes(4)+b,ref[prev.START:m.END])
    def test_reject_wrong_reference_and_mutated_byte(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and reference unavailable')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        wrong=bytearray(ref);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,b)
        corrupt=bytearray(b);corrupt[21]^=0x10
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,corrupt)
if __name__=='__main__':unittest.main()
