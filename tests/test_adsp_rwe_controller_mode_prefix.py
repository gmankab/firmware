# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 controller mode dispatch prefix source and exact byte checks."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
sp=importlib.util.spec_from_file_location('adsp_mode_prefix',T/'rwe-next-controller-mode-prefix.py')
assert sp and sp.loader
m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)

class ControllerModePrefix(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.mc=os.environ.get('HEXAGON_LLVM_MC');cls.ref=os.environ.get('NABU_ADSP_REFERENCE')
 def test_layout_and_source(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x139e0,0x13a2c,76))
  src=m.SOURCE.read_text()
  self.assertIn('if(p0.new) jump:nt normal_case',src)
  self.assertIn('r2=memw(gp+##0xf02d1488)',src)
  self.assertIn('if(cmp.eq(r2.new,#0)) jump:nt absent_controller',src)
  self.assertNotIn('.word',src);self.assertNotIn('.incbin',src)
 def test_exact_bytes_and_parse_field(self):
  if not self.mc or not self.ref:self.skipTest('Hexagon LLVM6 and pinned reference required')
  b=m.render(self.mc);ref=Path(self.ref).read_bytes();m.verify(ref,b)
  self.assertEqual(b[0x139ec-m.START:0x139f0-m.START],struct.pack('<I',0x6e80c002))
  for i in (0x139e0,0x139f8,0x13a04):
   self.assertEqual(struct.unpack_from('<I',b,i-m.START)[0]&0xc000,0x4000)
 def test_wrong_reference_and_corruption_fail_closed(self):
  if not self.mc or not self.ref:self.skipTest('Hexagon LLVM6 and pinned reference required')
  b=m.render(self.mc);ref=Path(self.ref).read_bytes()
  bad=bytearray(ref);bad[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(bad,b)
  changed=bytearray(b);changed[12]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,changed)
if __name__=='__main__':unittest.main()
