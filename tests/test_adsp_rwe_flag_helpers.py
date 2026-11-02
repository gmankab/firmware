# SPDX-License-Identifier: GPL-3.0-only
"""Controller flag and conditional MMIO update source-only disjoint layout."""
from __future__ import annotations
import importlib.util,os,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
if str(T) not in sys.path:sys.path.insert(0,str(T))
s=importlib.util.spec_from_file_location('adsp_flag_helpers',T/'rwe-next-flag-helpers.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class FlagHelpers(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_exclusive_regions_and_noninstruction_gaps(self):
        self.assertEqual(sum(b-a for a,b,_ in m.PARTS.values()),80)
        self.assertEqual(m.REQUIRED_ZERO_FILL,((0x113f8,0x11400),(0x11428,0x11430)))
        self.assertEqual((m.DATA_START,m.DATA_END),(0x11394,0x113f0))
    def test_three_independently_source_assembled_functions(self):
        if not self.mc:self.skipTest('Hexagon LLVM6 required')
        result=m.render(self.mc)
        self.assertEqual({k:len(v) for k,v in result.items()},
          {'dispatch-stub':8,'flag-window':40,'flag-sync':32})
        if self.ref:m.verify(Path(self.ref).read_bytes(),result)
    def test_missing_part_fails_closed(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
        original=Path(self.ref).read_bytes();out=m.render(self.mc);m.verify(original,out)
        with self.assertRaisesRegex(ValueError,'Missing'):m.verify(original,{'flag-window':out['flag-window']})
        out['flag-sync']=out['flag-sync'][:-4]+b'\x00'*4
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(original,out)
if __name__=='__main__':unittest.main()
