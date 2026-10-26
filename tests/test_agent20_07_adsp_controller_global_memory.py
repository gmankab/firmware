# SPDX-License-Identifier: GPL-3.0-only
"""Source-built 160-byte controller MMIO publication and register loops."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
s=importlib.util.spec_from_file_location('controller_mem',T/'adsp-agent20-rwe-next-controller-global-memory.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
sp=importlib.util.spec_from_file_location('controller_start',T/'adsp-agent20-rwe-next-controller-global-startup.py')
assert sp and sp.loader
start=importlib.util.module_from_spec(sp);sp.loader.exec_module(start)

class GlobalMemoryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_contiguous_with_prior_startup(self):
        self.assertEqual((start.FILE_END,m.FILE_START,m.FILE_END),(0x114a8,0x114a8,0x11548))
        self.assertEqual((m.SIZE,m.PHYSICAL_START),(160,0x8be0e4a8))
    def test_llvm6_assembles_unpatched_hexagon_packets(self):
        if not self.mc:self.skipTest('LLVM6 unavailable')
        rebuilt=m.render(self.mc)
        self.assertEqual(len(rebuilt),160)
        # Packet parse-bit patterns of the controller's first full packet
        self.assertEqual(struct.unpack_from('<I',rebuilt,0x0)[0],0x78004011)
        if self.ref:m.verify(Path(self.ref).read_bytes(),rebuilt)
    def test_original_combined_startup_is_adjacent_and_exact(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned reference required')
        ref=Path(self.ref).read_bytes();a=start.render(self.mc);b=m.render(self.mc)
        self.assertEqual(len(a+b),248)
        self.assertEqual(a+b,ref[start.FILE_START:m.FILE_END])
    def test_reject_modified_reference_or_assembled_bytes(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned reference required')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(ref,b)
        corrupt=bytearray(b);corrupt[-1]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(ref,corrupt)
        corrupt_ref=bytearray(ref);corrupt_ref[m.FILE_START]^=1
        with self.assertRaisesRegex(ValueError,'Wrong pinned'):m.verify(corrupt_ref,b)
if __name__=='__main__':unittest.main()
