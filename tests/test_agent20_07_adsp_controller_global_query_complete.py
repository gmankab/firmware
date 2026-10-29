# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild and verify the controller-global query helper with checked SGP0."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-global-query-complete.py')
prior=load('adsp-agent20-rwe-next-controller-slot-remove-complete.py')
class ControllerQuery(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_routines_and_source(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x125a0,0x126a4,260))
        self.assertEqual(prior.END+4,m.START)
        self.assertEqual(sum(length for _,length in m.PARTS),m.SIZE)
        s='\n'.join((T/name).read_text() for name,_ in m.PARTS)
        for op in ('loop0(scan_entries,r6)','memw(GP+##0xf0026d68)',
                   'r1=or(r4,r1)','dealloc_return'):
            self.assertIn(op,s)
        self.assertNotIn('.incbin',s)
    def test_roundtrip_and_system_register(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        original=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(original,b)
        self.assertEqual(struct.unpack_from('<I',b,m.SGP0_AT-m.START)[0],0x6e80c002)
        self.assertEqual(prior.render(self.mc)+bytes(4)+b,original[prior.START:m.END])
    def test_detect_wrong_revision_and_corruption(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        orig=Path(self.ref).read_bytes();b=m.render(self.mc)
        corrupt=bytearray(b);corrupt[108]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(orig,corrupt)
        wrong=bytearray(orig);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,b)
if __name__=='__main__':unittest.main()
