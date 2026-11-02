# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon SRAM finalizer privileged and exact source-assembly tests."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools/adsp'
s=importlib.util.spec_from_file_location('sram_final',TOOLS/'rwe-next-sram-state-finalize.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class SRAMStateFinalize(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_original_boundaries_and_privileged_fields(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x1180c,0x11894,136))
        self.assertEqual(m.NEXT_FILL,(0x11894,0x11898))
        self.assertEqual(m.PRIVILEGED,{
            0x1184c:0x6e92c000,0x11854:0x6700c012,
            0x11860:0x6e80c002,0x11878:0x6e80c002,
        })
        src=m.SOURCE.read_text()
        self.assertIn('call finalize_controller; r0=memw(r2+#256)',src)
        self.assertIn('call close_controller; r0=memw(r2+#256)',src)
        self.assertIn('jump cleanup_handoff; deallocframe',src)
        self.assertNotIn('.incbin',src)
    def test_exact_binary_and_context(self):
        if not self.mc:self.skipTest('LLVM6 needed')
        b=m.render(self.mc)
        self.assertEqual(len(b),136)
        for pos,value in m.PRIVILEGED.items():
            self.assertEqual(struct.unpack_from('<I',b,pos-m.START)[0],value)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)
    def test_corruption_and_wrong_reference(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 plus original ADSP')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(ref,b)
        bad=bytearray(b);bad[16]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
        invalid=bytearray(ref);invalid[2]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(invalid,b)
    def test_no_unresolved_placeholder(self):
        if not self.mc:self.skipTest('LLVM6 needed')
        b=m.render(self.mc)
        self.assertNotIn(m.NOP,[b[i:i+4] for i in range(0,len(b),4)])
if __name__=='__main__':unittest.main()
