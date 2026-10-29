# SPDX-License-Identifier: GPL-3.0-only
"""Cycle-helper Hexagon V66 paired PCYCLE source and correctness checks."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
sp=importlib.util.spec_from_file_location('adsp_cycle_helper',T/'adsp-agent20-rwe-next-controller-cycle-helper.py')
assert sp and sp.loader
m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
class CycleHelper(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
 def test_layout_and_source(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x139c8,0x139dc,20))
  src=m.SOURCE.read_text()
  self.assertIn('r3:2=memd(gp+##0xf0010e20)',src)
  self.assertIn('jump controller_accumulate',src)
  self.assertNotIn('.word',src);self.assertNotIn('.incbin',src)
 def test_exact_bytes_and_pcycle(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6 and reference required')
  ref=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(ref,b)
  self.assertEqual(b[:4],struct.pack('<I',0x6f1ec000))
 def test_wrong_revision_and_corrupted_output(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6 and reference required')
  ref=Path(self.ref).read_bytes();b=m.render(self.mc)
  wrong=bytearray(ref);wrong[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,b)
  changed=bytearray(b);changed[8]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,changed)
if __name__=='__main__':unittest.main()
