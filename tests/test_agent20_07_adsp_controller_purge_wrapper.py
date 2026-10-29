# SPDX-License-Identifier: GPL-3.0-only
"""Source-assembled purge wrapper, packet grouping and revision checking."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def imp(name):
 sp=importlib.util.spec_from_file_location(name.replace('-','_'),T/name)
 assert sp and sp.loader
 x=importlib.util.module_from_spec(sp);sp.loader.exec_module(x);return x
m=imp('adsp-agent20-rwe-next-controller-purge-wrapper.py')
prior=imp('adsp-agent20-rwe-next-controller-linked-record-remove.py')
class PurgeWrapper(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
 def test_source_and_seam(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x13828,0x13844,28))
  self.assertEqual(prior.END,m.START)
  src=m.SOURCE.read_text()
  self.assertIn('extractu(r2,#7,#8); call controller_record_purge',src)
  self.assertIn('dealloc_return',src)
  self.assertNotIn('.word',src);self.assertNotIn('.incbin',src)
 def test_compiles_and_matches(self):
  if not self.mc or not self.ref:self.skipTest('Need LLVM6 and ADSP reference')
  ref=Path(self.ref).read_bytes();out=m.render(self.mc);m.verify(ref,out)
  self.assertEqual(out[4:8],struct.pack('<I',0x6e80c002))
  self.assertEqual(prior.render(self.mc)+out,ref[prior.START:m.END])
 def test_bad_revision_and_output_rejected(self):
  if not self.mc or not self.ref:self.skipTest('Need LLVM6 and ADSP reference')
  ref=Path(self.ref).read_bytes();out=m.render(self.mc)
  wrong=bytearray(ref);wrong[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,out)
  broken=bytearray(out);broken[-1]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,broken)
if __name__=='__main__':unittest.main()
