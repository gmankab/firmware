# SPDX-License-Identifier: GPL-3.0-only
"""Exact Hexagon instruction packets for first controller-record prefix."""
from __future__ import annotations
import importlib.util,os,sys,unittest,struct
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
p=T/'adsp-agent20-rwe-next-controller-records.py'
spec=importlib.util.spec_from_file_location('adsp_record_prefix',p)
assert spec and spec.loader
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class ControllerRecords(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_layout_classifies_gap_and_partial_function(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x10c40,0x10cb4,116))
        self.assertEqual(0x10c30-0x1084c,996)
        self.assertEqual(0x10c40-0x10c30,16)
    def test_controller_alloc_packet_and_relocations(self):
        if not self.mc:self.skipTest('LLVM6 source assembler required')
        built=m.render(self.mc)
        self.assertEqual(len(built),116)
        # First packet is the original Hexagon ALLOCFRAME duplex.
        self.assertEqual(struct.unpack_from('<I',built,0)[0],0xebf41c40)
        # Final ordinary branch resumes after the setup loop.
        self.assertEqual(struct.unpack_from('<I',built,0x6c)[0],0x5800c004)
        if self.ref:m.verify(Path(self.ref).read_bytes(),built)
    def test_strict_immutable_reference(self):
        if not(self.ref and self.mc):self.skipTest('Original ADSP & LLVM6 needed')
        orig=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(orig,b)
        b=bytearray(b);b[-1]^=1
        with self.assertRaisesRegex(ValueError,'differ'):
            m.verify(orig,b)
if __name__=='__main__':unittest.main()
