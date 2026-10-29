# SPDX-License-Identifier: GPL-3.0-only
"""ADSP linked-list record removal: source packets, 12-byte gap and revision."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
sp=importlib.util.spec_from_file_location('linked_record_remove',T/'adsp-agent20-rwe-next-controller-linked-record-remove.py')
assert sp and sp.loader
m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
class LinkedRecordRemove(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
 def test_source_layout(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x13750,0x13828,216))
  self.assertEqual(m.CALL_PACKET,0x13760)
  src=m.SOURCE.read_text()
  for symbol in ('walk_global:', 'walk_owners:', 'remove_head:', 'mark_removed:',
                 'invalid_owner:', 'dealloc_return', 'controller_notify'):
   self.assertIn(symbol,src)
  self.assertNotIn('.word',src);self.assertNotIn('.incbin',src)
 def test_exact_source_assembly(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6 + pinned reference required')
  ref=Path(self.ref).read_bytes();out=m.render(self.mc);m.verify(ref,out)
  self.assertEqual(len(out)//4,54)
  self.assertEqual((struct.unpack_from('<I',out,m.CALL_PACKET-m.START)[0]>>14)&3,1)
 def test_reject_revisions_and_corruption(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6 + pinned reference required')
  ref=Path(self.ref).read_bytes();out=m.render(self.mc)
  wrong=bytearray(ref);wrong[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,out)
  broken=bytearray(out);broken[32]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,broken)
if __name__=='__main__':unittest.main()
