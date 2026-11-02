# SPDX-License-Identifier: GPL-3.0-only
import importlib.util
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location('ct',ROOT/'tools/cdsp'/'verify-cdsp-conditional-tlb.py')
M=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

class ConditionalTlbTests(unittest.TestCase):
    def test_source_preserves_new_predicate_packet(self):
        self.assertIn('p0 = cmp.eq(r25,#0)',M.asm_source())
        self.assertIn('if (p0.new) jump:nt tlb_end',M.asm_source())
        self.assertEqual(M.LENGTH,52)
        self.assertEqual(len(M.EXPECTED),13)
        self.assertEqual(M.PHYSICAL,0x96c00dc8)

if __name__=='__main__':unittest.main()
