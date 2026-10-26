# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon v66 source packet and privileged-register tests for controller state."""
from __future__ import annotations
import hashlib,importlib.util,os,struct,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
T=ROOT/'tools'
s=importlib.util.spec_from_file_location('sram_state',T/'adsp-agent20-rwe-next-sram-controller-status.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)

class SRAMControllerState(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC')
        cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_region_and_architecture_contract(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x116d8,0x11754,124))
        self.assertEqual(m.PHYSICAL,0x8be0e6d8)
        self.assertEqual(m.SYSTEM_READS,{0x116f0:0x6e92c002,0x11714:0x6e80c00a})
        source=m.SOURCE.read_text()
        self.assertIn('call setup_controller; immext(',source)
        self.assertIn('call initialize_channel; r0=#24',source)
        self.assertIn('if(cmp.gt(r2.new,#1)) jump:t status_fallback',source)
        self.assertNotIn('.incbin',source)
    def test_llvm6_source_and_privileged_substitutions(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC required')
        rebuilt=m.render(self.mc)
        self.assertEqual(len(rebuilt),m.SIZE)
        for off,word in m.SYSTEM_READS.items():
            self.assertEqual(struct.unpack_from('<I',rebuilt,off-m.START)[0],word)
        if self.ref:m.verify(Path(self.ref).read_bytes(),rebuilt)
    def test_strict_reference_and_corruption_rejected(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and reference required')
        reference=Path(self.ref).read_bytes()
        assembled=m.render(self.mc)
        m.verify(reference,assembled)
        bad=bytearray(assembled);bad[-1]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(reference,bad)
        invalid=bytearray(reference);invalid[0]^=1
        with self.assertRaisesRegex(ValueError,'SHA mismatch'):m.verify(invalid,assembled)
    def test_no_unchecked_privileged_positions(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC required')
        assembled=m.render(self.mc)
        self.assertNotIn(m.NOP,[assembled[i:i+4] for i in range(0,len(assembled),4)])

if __name__=='__main__':unittest.main()
