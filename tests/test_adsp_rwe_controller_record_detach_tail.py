# SPDX-License-Identifier: GPL-3.0-only
"""Complete controller-record detach: source-derived code and failure modes."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def load(name):
 sp=importlib.util.spec_from_file_location(name.replace('-','_'),T/name)
 assert sp and sp.loader
 mod=importlib.util.module_from_spec(sp);sp.loader.exec_module(mod);return mod
prefix=load('rwe-next-controller-record-detach-prefix.py')
tail=load('rwe-next-controller-record-detach-tail.py')
class ControllerDetachComplete(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
 def test_adjacent_and_original_return(self):
  self.assertEqual((tail.START,tail.END,tail.SIZE),(0x13514,0x13610,252))
  self.assertEqual(prefix.END,tail.START)
  source=tail.SOURCE.read_text()
  for word in ('clear_descriptor:', 'linked_loop:', 'deactivate:',
               'callback_cleanup:', 'return_result:', 'dealloc_return',
               'memh(r2+#0)=setbit(#5)', 'memd(r18+#8)=r21:20'):
   self.assertIn(word,source)
  self.assertNotIn('.word',source);self.assertNotIn('.incbin',source)
 def test_full_400_byte_function(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
  ref=Path(self.ref).read_bytes()
  a=prefix.render(self.mc);b=tail.render(self.mc)
  prefix.verify(ref,a);tail.verify(ref,b)
  self.assertEqual(len(a+b),0x13610-0x13480)
  self.assertEqual(a+b,ref[0x13480:0x13610])
  self.assertEqual(b[0x13590-tail.START:0x13594-tail.START],struct.pack('<I',0x6c20c060))
  self.assertEqual((struct.unpack_from('<I',b,0x13538-tail.START)[0]>>14)&3,1)
 def test_corruption_and_wrong_revision_fail_closed(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
  ref=Path(self.ref).read_bytes();b=tail.render(self.mc)
  wrong=bytearray(ref);wrong[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):tail.verify(wrong,b)
  damage=bytearray(b);damage[28]^=1
  with self.assertRaisesRegex(ValueError,'differs'):tail.verify(ref,damage)
if __name__=='__main__':unittest.main()
