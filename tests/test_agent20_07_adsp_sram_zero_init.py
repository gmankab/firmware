# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon SRAM-zero loop, shuffled-memory packet and alignment tests."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
s=importlib.util.spec_from_file_location('adsp_sram',T/'adsp-agent20-rwe-next-sram-zero-init.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)

class SRAMInit(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_distinct_intervals_and_mem_shuffle_contract(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x116a0,0x116d8,56))
        self.assertEqual(m.FILL,(0x11698,0x116a0))
        self.assertIn(':mem_noshuf',m.SOURCE.read_text())
        self.assertIn(':endloop0',m.SOURCE.read_text())
    def test_hexagon_llvm6_compiles_packet_in_original_order(self):
        if not self.mc:self.skipTest('LLVM6 required')
        b=m.render(self.mc)
        self.assertEqual(len(b),56)
        self.assertEqual(struct.unpack_from('<I',b,4)[0],0xa1dd5200)
        self.assertEqual(struct.unpack_from('<I',b,0x28)[0],0xabc2c008)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)
    def test_wrong_reference_and_modified_fragment_rejected(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned original required')
        reference=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(reference,b)
        bad=bytearray(b);bad[-1]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(reference,bad)
        corrupt=bytearray(reference);corrupt[m.FILL[0]]=1
        with self.assertRaisesRegex(ValueError,'Wrong pinned'):m.verify(corrupt,b)
if __name__=='__main__':unittest.main()
