# SPDX-License-Identifier: GPL-3.0-only
import importlib.util
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location('tlb',ROOT/'tools'/'verify-cdsp-tlb.py')
M=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

class CdspTlbTests(unittest.TestCase):
    def test_source_describes_two_tlb_writes(self):
        self.assertEqual(M.CODE_SIZE,60)
        self.assertEqual(M.START_FILE_OFFSET,0x3dfc)
        self.assertEqual(len(M.MNEMONICS),15)
        self.assertEqual(M.MNEMONICS.count('tlbw(r3:2,r0)'),2)
        self.assertIn('r3:2 = combine(#0,#0)',M.trailing_llvm_asm())

if __name__=='__main__':unittest.main()
