# SPDX-License-Identifier: GPL-3.0-only
"""Complete ADSP 584-byte global-controller function from five source sections."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools/adsp'
p=TOOLS/'rwe-next-controller-global-full.py'
spec=importlib.util.spec_from_file_location('adsp_global_complete',p)
assert spec and spec.loader
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)

class GlobalComplete(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_exact_start_end_and_oem_zero_fill_separation(self):
        self.assertEqual((module.START,module.END,module.SIZE),(0x11450,0x11698,584))
        self.assertEqual((module.FILL_START,module.FILL_END),(0x11698,0x116a0))
        self.assertEqual(module.PHYSICAL_START,0x8be0e450)
    def test_five_contiguous_section_boundaries(self):
        cursor=module.START
        for _,a,b,filename in module.PARTS:
            self.assertEqual(a,cursor)
            self.assertTrue((TOOLS/filename).is_file())
            cursor=b
        self.assertEqual(cursor,0x1167c)
    def test_full_source_build_and_real_tail_call(self):
        if not self.mc:self.skipTest('LLVM6 required')
        code=module.render(self.mc)
        self.assertEqual(len(code),584)
        # Original routine's physical branch-to-caller is the final instruction.
        self.assertEqual(struct.unpack_from('<I',code,0x11694-module.START)[0],0x5802c13e)
        if self.ref:module.verify(Path(self.ref).read_bytes(),code)
    def test_fail_closed_on_altered_original_and_generated_code(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and original required')
        ref=Path(self.ref).read_bytes();built=module.render(self.mc)
        module.verify(ref,built)
        bad=bytearray(built);bad[-1]^=1
        with self.assertRaisesRegex(ValueError,'differs'):module.verify(ref,bad)
        wrong=bytearray(ref);wrong[module.FILL_START]=1
        with self.assertRaisesRegex(ValueError,'Wrong pinned'):module.verify(wrong,built)
if __name__=='__main__':unittest.main()
