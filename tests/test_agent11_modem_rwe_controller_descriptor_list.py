# SPDX-License-Identifier: GPL-3.0-only
"""Source-assembled modem V60 descriptor-list primitives and packet checks."""
import importlib.util,os,struct,tempfile,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-controller-descriptor-list.py'
spec=importlib.util.spec_from_file_location('agent11_controller_descriptor_list',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class TestDescriptorList(unittest.TestCase):
 def test_range(self):
  self.assertEqual((mod.START,mod.END),(0x3a650,0x3a72c))
  self.assertEqual(mod.END-mod.START,220)
  self.assertEqual(mod.START+0x8e008000,0x8e042650)
 def test_packet_reordering_with_operand_guard(self):
  buf=bytearray(220)
  struct.pack_into('<III',buf,mod.PACKET_POSITION,mod.BRANCH,mod.COMPARE,mod.PREDICATED_ZERO)
  b=mod.normalize(bytes(buf))
  self.assertEqual(struct.unpack_from('<III',b,mod.PACKET_POSITION),(mod.COMPARE,mod.PREDICATED_ZERO,mod.BRANCH))
  struct.pack_into('<III',buf,mod.PACKET_POSITION,mod.BRANCH^1,mod.COMPARE,mod.PREDICATED_ZERO)
  with self.assertRaisesRegex(ValueError,'Unexpected predicate/branch'):
   mod.normalize(bytes(buf))
 def test_wrong_reference_rejected(self):
  with self.assertRaisesRegex(ValueError,'Wrong modem reference'):
   mod.verify(bytes(mod.END),bytes(220))
 def test_llvm_source_matches_reference(self):
  mc=os.environ.get('AGENT11_LLVM_MC')
  if not mc:self.skipTest('No Hexagon llvm-mc configured')
  src=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_descriptor_list.s'
  b=mod.assemble(Path(mc),src)
  self.assertEqual(len(b),220)
  ref=os.environ.get('AGENT11_MODEM_MBN')
  if ref:mod.verify(Path(ref).read_bytes(),b)
if __name__=='__main__':unittest.main()
