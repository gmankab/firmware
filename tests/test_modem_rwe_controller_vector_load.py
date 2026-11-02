# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon controller vector-loader predicate packet tests."""
import importlib.util,os,struct,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-controller-vector-load.py'
spec=importlib.util.spec_from_file_location('agent11_modem_vector_load',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class TestControllerVectorLoad(unittest.TestCase):
 def test_file_interval_and_virtual_address(self):
  self.assertEqual((mod.START,mod.END),(0x3a778,0x3a8ec))
  self.assertEqual(mod.END-mod.START,372)
  self.assertEqual(mod.START+0xc0808000,0xc0842778)
 def test_packet_normalizer_is_fail_closed(self):
  d=bytearray(372)
  struct.pack_into('<III',d,4,mod.ENTRY_BRANCH,mod.ENTRY_PAIR,mod.ENTRY_MOV)
  for a,w in mod.SIGNED_BRANCHES.items():struct.pack_into('<II',d,a-mod.START,w,mod.BYTE_LOOP_ADD)
  b=mod.normalize(bytes(d))
  self.assertEqual(struct.unpack_from('<I',b,0x0c)[0],mod.ENTRY_BRANCH|0x8000)
  for a,w in mod.SIGNED_BRANCHES.items():
   self.assertEqual(struct.unpack_from('<II',b,a-mod.START),(mod.BYTE_LOOP_ADD&~0x8000,w|0x8000))
  struct.pack_into('<I',d,4,mod.ENTRY_BRANCH^1)
  with self.assertRaisesRegex(ValueError,'Unexpected source-assembled vector entry'):
   mod.normalize(bytes(d))
  with self.assertRaisesRegex(ValueError,'Vector load length'):
   mod.normalize(bytes(368))
 def test_wrong_reference_revision_rejected(self):
  with self.assertRaisesRegex(ValueError,'Wrong pinned modem revision'):
   mod.verify(bytes(mod.END),bytes(mod.END-mod.START))
 def test_source_bytes_match_pinned_reference(self):
  mc=os.environ.get('AGENT11_LLVM_MC')
  if not mc:self.skipTest('Hexagon LLVM6 unavailable')
  src=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_vector_load.s'
  b=mod.assemble(Path(mc),src)
  self.assertEqual(len(b),372)
  ref=os.environ.get('AGENT11_MODEM_MBN')
  if ref:mod.verify(Path(ref).read_bytes(),b)
if __name__=='__main__':unittest.main()
