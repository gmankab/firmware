# SPDX-License-Identifier: GPL-3.0-only
import importlib.util
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location('walk',ROOT/'tools/cdsp'/'verify-cdsp-tlb-walk.py')
M=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

class WalkTests(unittest.TestCase):
    def test_packet_description_has_no_false_assembler_support(self):
        asm=M.make_asm()
        self.assertEqual(len(M.EXPECTED),13)
        self.assertIn('{ nop }',asm) # *only in LLVM test fixture*
        self.assertEqual(M.TLBW_OFFSET,32)
        self.assertEqual(M.LENGTH,52)
        self.assertEqual(M.PHYSICAL,0x96c00d94)

if __name__=='__main__':unittest.main()
