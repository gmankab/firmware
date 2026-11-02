# SPDX-License-Identifier: GPL-3.0-only
"""Controller global record purge: source encodings and linked-list paths."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def load(name):
 sp=importlib.util.spec_from_file_location(name.replace('-','_'),T/name)
 assert sp and sp.loader
 m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=load('rwe-next-controller-record-purge.py')
a=load('rwe-next-controller-record-detach-prefix.py')
b=load('rwe-next-controller-record-detach-tail.py')
class ControllerRecordPurge(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.reference=os.getenv('NABU_ADSP_REFERENCE')
 def test_exact_boundaries_and_instruction_ids(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x13610,0x13744,308))
  self.assertEqual(b.END,m.START)
  self.assertEqual(len(m.V66_OPS),4)
  s=m.SOURCE.read_text()
  for part in ('release_lock:', 'linked_loop:', 'deactivate:', 'advance_link:',
               'loop_finished:', 'callback_cleanup' if 'callback_cleanup' in s else 'notify_done:',
               'dealloc_return', 'check_next:'):
   self.assertIn(part,s)
  self.assertNotIn('.word',s);self.assertNotIn('.incbin',s)
 def test_reference_and_seam_reconstruction(self):
  if not self.mc or not self.reference:self.skipTest('LLVM6/pinned ADSP required')
  ref=Path(self.reference).read_bytes();out=m.render(self.mc);m.verify(ref,out)
  self.assertEqual(out[0x13624-m.START:0x13628-m.START],struct.pack('<I',0x6e80c002))
  self.assertEqual(out[0x13650-m.START:0x13654-m.START],struct.pack('<I',0x6c20c080))
  x=a.render(self.mc)+b.render(self.mc)+out
  self.assertEqual(x,ref[0x13480:m.END]);self.assertEqual(len(x),708)
  self.assertEqual(ref[0x13744:0x13750],bytes(12))
 def test_wrong_revision_and_mutation(self):
  if not self.mc or not self.reference:self.skipTest('LLVM6/pinned ADSP required')
  ref=Path(self.reference).read_bytes();out=m.render(self.mc)
  corrupted=bytearray(ref);corrupted[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(corrupted,out)
  code=bytearray(out);code[42]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,code)
if __name__=='__main__':unittest.main()
