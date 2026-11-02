# SPDX-License-Identifier: GPL-3.0-only
"""Two independent source-assembled Hexagon controller-address helpers."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
if str(T) not in sys.path:sys.path.insert(0,str(T))
s=importlib.util.spec_from_file_location('adsp_pack_offsets',T/'rwe-next-packed-offsets.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class PackedOffsets(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')

    def test_range_and_linker_fill(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x11108,0x11164,92))
        self.assertEqual((m.FILL_START,m.FILL_END),(0x11164,0x11170))

    def test_two_leaf_functions_no_privileged_opcodes(self):
        if not self.mc:self.skipTest('LLVM6 Hexagon source assembler required')
        b=m.render(self.mc)
        self.assertEqual(len(b),m.SIZE)
        # First helper uses the 0x800000 address window, second is at 0x1113c.
        self.assertEqual(struct.unpack_from('<I',b,0)[0],0x00084000)
        self.assertEqual(struct.unpack_from('<I',b,0x1113c-m.FILE_START)[0],0xf3204102)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)

    def test_corrupted_output_and_fill_fail(self):
        if not self.mc or not self.ref:self.skipTest('Pinned ADSP and LLVM6 required')
        b=m.render(self.mc);orig=Path(self.ref).read_bytes();m.verify(orig,b)
        corrupted=bytearray(b);corrupted[-2]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(orig,corrupted)
        ref=bytearray(orig);ref[m.FILL_START]=1
        with self.assertRaisesRegex(ValueError,'Wrong ADSP'):m.verify(ref,b)
if __name__=='__main__':unittest.main()
