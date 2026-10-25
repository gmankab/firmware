# SPDX-License-Identifier: GPL-3.0-only
"""Packetized Hexagon mode-selector prefix [ADSP 0x6818,0x68bc)."""
import importlib.util
import os
from pathlib import Path
import sys
import unittest
TOOLS=Path(__file__).resolve().parents[1]/'tools'
sys.path.insert(0,str(TOOLS))
SPEC=importlib.util.spec_from_file_location('adsp_rwe',TOOLS/'adsp-agent20-rwe.py')
assert SPEC and SPEC.loader
RWE=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RWE)

class FeatureModeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.llvm=os.getenv('HEXAGON_LLVM_MC')
        cls.reference=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_region_length(self):
        self.assertEqual(RWE.FEATURE_MODES_A_SIZE,0xa4)
        self.assertEqual(RWE.FILE_END,0x6c10)
        self.assertEqual(RWE.SIZE,3040)
    def test_adjacent_bitmask_classifiers_are_independently_compiled(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC required')
        output=RWE.baseline().assemble(self.llvm,RWE.FEATURE_MODES_B_SOURCE,0xc0)
        self.assertEqual(len(output),0xc0)
        if self.reference:
            original=Path(self.reference).read_bytes()
            self.assertEqual(output,original[0x68bc:0x697c])
    def test_dependent_mode_bitfields(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC is required')
        emitted=RWE.baseline()
        for name,length,first in [
            ('FEATURE_MODES_C_SOURCE',0x94,0x697c),
            ('FEATURE_MODES_D_SOURCE',0xa0,0x6a10),
        ]:
            b=emitted.assemble(self.llvm,getattr(RWE,name),length)
            if self.reference:
                ref=Path(self.reference).read_bytes()
                self.assertEqual(b,ref[first:first+length],name)
    def test_final_codec_pattern_branch_hints_and_return(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC is required')
        emitted=RWE.baseline()
        for name,length,first in [
            ('FEATURE_MODES_E_SOURCE',0xa4,0x6ab0),
            ('FEATURE_MODES_F_SOURCE',0xbc,0x6b54),
        ]:
            b=emitted.assemble(self.llvm,getattr(RWE,name),length)
            if self.reference:
                ref=Path(self.reference).read_bytes()
                self.assertEqual(b,ref[first:first+length],name)
        epilogue=emitted.assemble(self.llvm,RWE.FEATURE_MODES_F_SOURCE,0xbc)
        # The classifier closes with an original deallocframe + return, not
        # a synthetic branch added at an arbitrary byte boundary.
        self.assertEqual(len(epilogue[-8:]),8)
    def test_predicate_decode_is_byte_stable(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC is required')
        encoded=RWE.baseline().assemble(self.llvm,RWE.FEATURE_MODES_A_SOURCE,0xa4)
        self.assertEqual(len(encoded),164)
        if self.reference:
            original=Path(self.reference).read_bytes()
            self.assertEqual(original[0x6818:0x68bc],encoded)
            RWE.verify(original,RWE.render(self.llvm))

if __name__=='__main__':unittest.main()
