# SPDX-License-Identifier: GPL-3.0-only
"""Checked byte-equivalent Hexagon callback trampoline assembler."""
import importlib.util,os,struct,unittest
from pathlib import Path
P=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-controller-callback-trampolines.py'
spec=importlib.util.spec_from_file_location('agent11_trampolines',P)
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class Trampoline(unittest.TestCase):
 def test_range_and_mapping(self):
  self.assertEqual((m.START,m.END),(0x3aa8c,0x3aab4))
  self.assertEqual(m.END-m.START,40)
  self.assertEqual(m.START+0x8e008000,0x8e042a8c)
 def test_checked_packet_permutation(self):
  buf=bytearray(40)
  struct.pack_into('<III',buf,8,*m.FIRST)
  struct.pack_into('<III',buf,28,*m.SECOND)
  struct.pack_into('<I',buf,4,0x2ac43040)
  struct.pack_into('<I',buf,24,0x2b843040)
  out=m.normalize(bytes(buf))
  self.assertEqual(struct.unpack_from('<III',out,8),(m.FIRST[1],m.FIRST[2]&~0x8000,m.FIRST[0]|0x8000))
  self.assertEqual(struct.unpack_from('<III',out,28),(m.SECOND[1],m.SECOND[2]&~0x8000,m.SECOND[0]|0x8000))
  for at in (8,12,16,28,32,36,4,24):
   bb=bytearray(buf);bb[at]^=1
   with self.subTest(offset=at),self.assertRaisesRegex(ValueError,'Unexpected'):
    m.normalize(bytes(bb))
 def test_wrong_reference_rejected(self):
  with self.assertRaisesRegex(ValueError,'Wrong modem'):m.verify(bytes(m.END),bytes(40))
 def test_llvm_output_matches_pinned(self):
  if not (mc:=os.environ.get('AGENT11_LLVM_MC')):self.skipTest('LLVM6 unavailable')
  src=P.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_callback_trampolines.s'
  built=m.assemble(Path(mc),src)
  self.assertEqual(len(built),40)
  if ref:=os.environ.get('AGENT11_MODEM_MBN'):m.verify(Path(ref).read_bytes(),built)
if __name__=='__main__':unittest.main()
