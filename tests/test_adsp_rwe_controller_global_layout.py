# SPDX-License-Identifier: GPL-3.0-only
"""ADSP global MMIO layout source assembly and exact reference regression."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
s=importlib.util.spec_from_file_location('adsp_global_layout',T/'rwe-next-controller-global-layout.py')
assert s and s.loader
mod=importlib.util.module_from_spec(s);s.loader.exec_module(mod)
startspec=importlib.util.spec_from_file_location('adsp_global_start',T/'rwe-next-controller-global-startup.py')
assert startspec and startspec.loader
start=importlib.util.module_from_spec(startspec);startspec.loader.exec_module(start)

class GlobalLayoutTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_bounds_are_adjacent_to_last_checkpoint(self):
        self.assertEqual(mod.FILE_START,0x11548)
        self.assertEqual((mod.FILE_START,mod.FILE_END,mod.SIZE),(0x11548,0x11624,220))
        self.assertEqual(sum(b-a for _,a,b,_ in mod.PARTS),220)
        self.assertEqual(mod.PHYSICAL_START,0x8be0e548)
    def test_llvm_assembled_no_replacements(self):
        if not self.mc:self.skipTest('LLVM6 required')
        data=mod.render(self.mc)
        self.assertEqual(len(data),220)
        # The pointer-layout section starts after the 76-byte call/GP chain.
        self.assertEqual(struct.unpack_from('<I',data,0x4c)[0],0x00004400)
        if self.ref:mod.verify(Path(self.ref).read_bytes(),data)
    def test_combined_startup_three_sections_byte_identical(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned original required')
        sys.path.insert(0,str(T))
        m=importlib.util.spec_from_file_location('adsp_mem',T/'rwe-next-controller-global-memory.py')
        assert m and m.loader
        mem=importlib.util.module_from_spec(m);m.loader.exec_module(mem)
        whole=start.render(self.mc)+mem.render(self.mc)+mod.render(self.mc)
        self.assertEqual(len(whole),0x11624-0x11450)
        self.assertEqual(whole,Path(self.ref).read_bytes()[0x11450:0x11624])
    def test_rejects_mismatch_and_bad_reference(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
        ref=Path(self.ref).read_bytes();original=mod.render(self.mc)
        mod.verify(ref,original)
        corrupt=bytearray(original);corrupt[8]^=0x20
        with self.assertRaisesRegex(ValueError,'differ'):mod.verify(ref,corrupt)
        wrong_ref=bytearray(ref);wrong_ref[mod.FILE_START]^=1
        with self.assertRaisesRegex(ValueError,'Wrong pinned'):mod.verify(wrong_ref,original)
if __name__=='__main__':unittest.main()
