# SPDX-License-Identifier: GPL-3.0-only
"""Exact packet reconstruction and byte-provenance for slot-state lookup."""
from __future__ import annotations
import importlib.util,os,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def load(name):
    s=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=load('rwe-next-controller-slot-status.py')
previous=load('rwe-next-controller-slot-helpers.py')
class StatusLookup(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_and_bounds(self):
        self.assertEqual((m.START,m.END,m.SIZE,m.PHYSICAL),(0x11e6c,0x11ed0,100,0x8be0ee6c))
        self.assertEqual(previous.END,m.START)
        s=m.SOURCE.read_text()
        for expr in ('bitsplit(r16,#5)','r2=memw(##0xf000eac4)',
                     'r2=memw(##0xf000eab8)','p0=tstbit(r2,r0)',
                     'r17:16=memd(r29+#0); dealloc_return'):
            self.assertIn(expr,s)
        self.assertNotIn('.incbin',s)
    def test_exact_compilation_and_both_bitmaps(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference needed')
        ref=Path(self.ref).read_bytes();data=m.render(self.mc);m.verify(ref,data)
        self.assertEqual(len(data),25*4)
        self.assertEqual(previous.render(self.mc)+data,ref[previous.START:m.END])
    def test_rejects_changed_code_and_revision(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference needed')
        ref=Path(self.ref).read_bytes();data=m.render(self.mc);incorrect=bytearray(data);incorrect[-1]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,incorrect)
        wrong=bytearray(ref);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,data)
    def test_no_external_oem_opcode_extraction(self):
        if not self.mc:self.skipTest('LLVM6 unavailable')
        self.assertEqual(len(m.render(self.mc)),m.SIZE)
        self.assertNotIn('read_bytes()',m.SOURCE.read_text())
if __name__=='__main__':unittest.main()
