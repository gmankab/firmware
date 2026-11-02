# SPDX-License-Identifier: GPL-3.0-only
"""Whole 548-byte source-assembled ADSP cache relocation and error handler."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
if str(T) not in sys.path:sys.path.insert(0,str(T))
s=importlib.util.spec_from_file_location('cache_reloc',T/'rwe-next-icache-relocation.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class IcacheRelocation(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_disjoint_source_fragments_and_data_boundary(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x11170,0x11394,548))
        self.assertEqual(m.PARTS[0][:2],(0x111dc,0x1121c))
        self.assertEqual(m.PARTS[-1][:2],(0x112d4,0x11394))
        self.assertEqual((m.DATA_START,m.DATA_END),(0x11394,0x113f0))
    def test_source_only_output_and_hardware_loop(self):
        if not self.mc:self.skipTest('Hexagon LLVM6 required')
        b=m.render(self.mc)
        self.assertEqual(len(b),m.SIZE)
        self.assertEqual(struct.unpack_from('<I',b,0x11174-m.FILE_START)[0],0x6e90c002)
        # Original return and the fatal branch-to-self are both explicit.
        self.assertEqual(struct.unpack_from('<I',b,0x11358-m.FILE_START)[0],0x3e141f40)
        self.assertEqual(struct.unpack_from('<I',b,0x11390-m.FILE_START)[0],0x5800c000)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)
    def test_reference_mismatch_detection(self):
        if not self.mc or not self.ref:self.skipTest('Original ADSP and LLVM6 required')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(ref,b)
        corrupted=bytearray(b);corrupted[312]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(ref,corrupted)
        wrong=bytearray(ref);wrong[m.DATA_START]^=1
        with self.assertRaisesRegex(ValueError,'Wrong pinned'):m.verify(wrong,b)
if __name__=='__main__':unittest.main()
