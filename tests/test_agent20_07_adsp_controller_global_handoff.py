# SPDX-License-Identifier: GPL-3.0-only
"""ADSP controller physical handoff source/packet and reference verification."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
s=importlib.util.spec_from_file_location('global_handoff',T/'adsp-agent20-rwe-next-controller-global-handoff.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)

class HandoffTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_layout_is_strict_and_instruction_only(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x11624,0x1167c,88))
        self.assertEqual(m.PHYSICAL_START,0x8be0e624)
    def test_source_assembled(self):
        if not self.mc:self.skipTest('LLVM6 missing')
        b=m.render(self.mc)
        self.assertEqual(len(b),88)
        self.assertEqual(struct.unpack_from('<I',b,0)[0],0x5a00c25e)
        self.assertEqual(struct.unpack_from('<I',b,0x54)[0],0x901ec01e)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)
    def test_reject_modified_output(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned reference required')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(ref,b)
        bad=bytearray(b);bad[4]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
    def test_reject_other_reference_revision(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned reference required')
        ref=bytearray(Path(self.ref).read_bytes());ref[m.FILE_START]^=1
        with self.assertRaisesRegex(ValueError,'Wrong pinned'):m.verify(ref,m.render(self.mc))
if __name__=='__main__':unittest.main()
