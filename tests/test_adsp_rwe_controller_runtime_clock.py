# SPDX-License-Identifier: GPL-3.0-only
"""ADSP controller runtime-clock source, privileged ops and reference checks."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
sp=importlib.util.spec_from_file_location('controller_clock',T/'rwe-next-controller-runtime-clock.py')
assert sp and sp.loader
m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
class ControllerClock(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
 def test_exact_bounds_and_source(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x13850,0x138ec,156))
  self.assertEqual(len(m.SPECIAL),5)
  self.assertEqual(m.PACKET_CONTINUATIONS,(0x1386c,0x13878))
  source=m.SOURCE.read_text()
  for key in ('dccleaninva(r0)','controller_prepare','controller_clock',
              'controller_wakeup','controller_complete','deallocframe'):
   self.assertIn(key,source)
  self.assertNotIn('.word',source);self.assertNotIn('.incbin',source)
 def test_source_matches_pinned_firmware(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6/pinned ADSP required')
  ref=Path(self.ref).read_bytes();code=m.render(self.mc);m.verify(ref,code)
  self.assertEqual(code[0x13864-m.START:0x13868-m.START],struct.pack('<I',0x6f1ec000))
  self.assertEqual(code[0x138c0-m.START:0x138c4-m.START],struct.pack('<I',0x6f1ec002))
  self.assertEqual(code[0x138ac-m.START:0x138b0-m.START],struct.pack('<I',0x6460c020))
 def test_corruption_and_revision_fail_closed(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6/pinned ADSP required')
  ref=Path(self.ref).read_bytes();b=m.render(self.mc)
  badref=bytearray(ref);badref[30]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(badref,b)
  bad=bytearray(b);bad[0]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
if __name__=='__main__':unittest.main()
