# SPDX-License-Identifier: GPL-3.0-only
"""ISA source and checked packet forms for Hexagon V60 bounded vector stores."""
import importlib.util,os,struct,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-controller-vector-store.py'
spec=importlib.util.spec_from_file_location('agent11_vector_store',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class TestVectorStore(unittest.TestCase):
 def test_interval_and_virtual_mapping(self):
  self.assertEqual((mod.START,mod.END),(0x3a8ec,0x3aa1c))
  self.assertEqual(mod.END-mod.START,304)
  self.assertEqual(mod.START+0xc0808000,0xc08428ec)
 def test_entry_order_and_duplex_guard(self):
  b=bytearray(304)
  struct.pack_into('<III',b,4,*mod.ENTRY)
  struct.pack_into('<II',b,304-8,mod.RETURN_MARKER,mod.DUAL_ADD)
  generated=mod.normalize(bytes(b))
  self.assertEqual(struct.unpack_from('<III',generated,4),(mod.ENTRY[1],mod.ENTRY[2]&~0x8000,mod.ENTRY[0]|0x8000))
  self.assertEqual(struct.unpack_from('<I',generated,304-8)[0],0x529f4000)
  for offset in (4,304-4):
   altered=bytearray(b);altered[offset]^=1
   with self.assertRaisesRegex(ValueError,'Unexpected'):
    mod.normalize(bytes(altered))
 def test_reject_wrong_original_revision(self):
  with self.assertRaisesRegex(ValueError,'Wrong pinned modem revision'):
   mod.verify(bytes(mod.END),bytes(304))
 def test_llvm_source_match(self):
  mc=os.environ.get('AGENT11_LLVM_MC')
  if not mc:self.skipTest('Hexagon LLVM6 unavailable')
  src=SCRIPT.parents[1]/'sources/xiaomi-nabu/modem/modem_rwe_controller_vector_store.s'
  b=mod.assemble(Path(mc),src)
  self.assertEqual(len(b),304)
  ref=os.environ.get('AGENT11_MODEM_MBN')
  if ref:mod.verify(Path(ref).read_bytes(),b)
if __name__=='__main__':unittest.main()
