# SPDX-License-Identifier: GPL-3.0-only
"""Controller-mode PCYCLE snapshot and startup-assert trap, source-built V66."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
sp=importlib.util.spec_from_file_location('adsp_mode_clock',T/'rwe-next-controller-mode-clock.py')
assert sp and sp.loader
m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
class ModeClock(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
 def test_source_and_address_bounds(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x13a2c,0x13a7c,80))
  text=m.SOURCE.read_text()
  for fragment in ('r2=memw(gp+##0xf0024328)','memd(gp+##0xf000d6a0)=r1:0','jump trap_mode'):
   self.assertIn(fragment,text)
  self.assertNotIn('.incbin',text);self.assertNotIn('.word',text)
 def test_exact_reference_and_cycle_register(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6 and original ADSP needed')
  out=m.render(self.mc);m.verify(Path(self.ref).read_bytes(),out)
  self.assertEqual(out[:4],struct.pack('<I',0x6f1ec000))
 def test_wrong_revision_and_modified_output_rejected(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6 and original ADSP needed')
  ref=Path(self.ref).read_bytes();b=m.render(self.mc)
  bad=bytearray(ref);bad[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(bad,b)
  bad=bytearray(b);bad[8]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
if __name__=='__main__':unittest.main()
