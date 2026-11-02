# SPDX-License-Identifier: GPL-3.0-only
"""Partial ADSP V66 K0 lock, SSR, and controller relay packets."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
if str(T) not in sys.path:sys.path.insert(0,str(T))
s=importlib.util.spec_from_file_location('locked_controller',T/'rwe-next-locked-controller.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class LockedControllerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')

    def test_layout_and_fill_not_counted(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x10fe8,0x11088,160))
        self.assertEqual(m.FILL_END-m.FILL_START,8)

    def test_checked_privilege_slots_and_llvm_packets(self):
        if not self.mc:self.skipTest('LLVM6 unavailable')
        b=m.render(self.mc)
        self.assertEqual(len(b),m.SIZE)
        for addr,expected in m.REPLACEMENTS.items():
            self.assertEqual(struct.unpack_from('<I',b,addr-m.FILE_START)[0],expected)
        self.assertEqual(struct.unpack_from('<I',b,0x11000-m.FILE_START)[0],0xf204c300)
        self.assertEqual(struct.unpack_from('<I',b,0x11084-m.FILE_START)[0],0x901ec01e)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)

    def test_modified_reference_and_generated_code_rejected(self):
        if not self.mc or not self.ref:self.skipTest('Pinned ADSP reference and LLVM6 required')
        r=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(r,b)
        bad=bytearray(b);bad[-1]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(r,bad)
        bad_ref=bytearray(r);bad_ref[m.FILL_START]=1
        with self.assertRaisesRegex(ValueError,'Wrong pinned'):m.verify(bad_ref,b)
if __name__=='__main__':unittest.main()
