# SPDX-License-Identifier: GPL-3.0-only
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools';s=importlib.util.spec_from_file_location('sram_leaf',T/'adsp-agent20-rwe-next-sram-state-publish-leaf.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class TestSRAMPublishLeaf(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_and_gap(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11898,0x118b8,32))
        self.assertEqual(m.FOLLOWING_FILL,(0x118b8,0x118c0))
        self.assertIn('call read_active_controller; allocframe(#0)',m.SOURCE.read_text())
        self.assertNotIn('.incbin',m.SOURCE.read_text())
    def test_assembly_and_v66_encoding(self):
        if not self.mc:self.skipTest('LLVM6 not set')
        data=m.render(self.mc);self.assertEqual(len(data),32)
        self.assertEqual(struct.unpack_from('<I',data,m.SGP0_AT-m.START)[0],0x6e80c00a)
        if self.ref:m.verify(Path(self.ref).read_bytes(),data)
    def test_strict_original_and_modified_reject(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and reference required')
        ref=Path(self.ref).read_bytes();data=m.render(self.mc);m.verify(ref,data)
        bad=bytearray(data);bad[0]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
        corrupt=bytearray(ref);corrupt[0]^=1
        with self.assertRaisesRegex(ValueError,'reference SHA'):m.verify(corrupt,data)
if __name__=='__main__':unittest.main()
