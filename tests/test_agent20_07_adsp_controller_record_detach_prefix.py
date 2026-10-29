# SPDX-License-Identifier: GPL-3.0-only
"""ADSP controller detach source, partial bounds, and revision safety."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def import_file(name):
 sp=importlib.util.spec_from_file_location(name.replace('-','_'),T/name)
 assert sp and sp.loader
 mod=importlib.util.module_from_spec(sp);sp.loader.exec_module(mod);return mod
m=import_file('adsp-agent20-rwe-next-controller-record-detach-prefix.py')
prior=import_file('adsp-agent20-rwe-next-controller-record-snapshot.py')
class ControllerDetachPrefix(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.reference=os.getenv('NABU_ADSP_REFERENCE')
 def test_original_boundary_and_function_structure(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x13480,0x13514,148))
  self.assertEqual(prior.END+8,m.START)
  s=m.SOURCE.read_text()
  for part in ('scan_list:', 'matched_link:', 'not_found:', 'have_match:',
               'if(!cmp.gtu(r3.new,r17))','controller_lookup','controller_notify'):
   self.assertIn(part,s)
  self.assertNotIn('.word',s);self.assertNotIn('.incbin',s)
 def test_assembled_byte_equality(self):
  if not self.mc or not self.reference:self.skipTest('LLVM6 and pinned firmware required')
  ref=Path(self.reference).read_bytes();built=m.render(self.mc);m.verify(ref,built)
  self.assertEqual(built[12:16],struct.pack('<I',0x6e80c002))
  self.assertEqual(len(built)//4,37)
 def test_fail_closed_on_bad_ref_and_corrupted_code(self):
  if not self.mc or not self.reference:self.skipTest('LLVM6 and pinned firmware required')
  ref=Path(self.reference).read_bytes();b=m.render(self.mc)
  rev=bytearray(ref);rev[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(rev,b)
  bad=bytearray(b);bad[24]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
if __name__=='__main__':unittest.main()
