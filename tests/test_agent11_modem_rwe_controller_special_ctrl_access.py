# SPDX-License-Identifier: GPL-3.0-only
"""Source-built Hexagon C-register system control tests."""
import importlib.util,os,struct,unittest
from pathlib import Path
P=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-controller-special-ctrl-access.py'
spec=importlib.util.spec_from_file_location('agent11_ctrl_special',P)
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class ControllerAccess(unittest.TestCase):
 def test_range(self):
  self.assertEqual((m.START,m.END),(0x3aab4,0x3ab1c))
  self.assertEqual(m.END-m.START,104)
  self.assertEqual(m.START+0x8e008000,0x8e042ab4)
 def test_privileged_register_field_checks(self):
  self.assertEqual(m.creg_read(2,6),0x6e864002)
  self.assertEqual(m.creg_write(2,2),0x6c82c002)
  self.assertEqual(m.creg_write(1,2),0x6c82c001)
  for args in ((32,6),(-1,1),(2,64)):
   with self.subTest(args=args),self.assertRaises(ValueError):m.creg_read(*args)
  for args in ((64,2),(2,32),(-1,2)):
   with self.subTest(args=args),self.assertRaises(ValueError):m.creg_write(*args)
 def test_reference_pin_and_opcode_mutation(self):
  with self.assertRaisesRegex(ValueError,'Wrong pinned modem'):m.verify(bytes(m.END),bytes(104))
  b=bytearray(104)
  for i,value in ((0,m.creg_read(2,6)),(6,m.creg_write(2,2)),(13,m.creg_read(2,6)),(19,m.creg_write(1,2)),(12,0x529fc000),(23,0x529fc000),(25,0x529fc000),(7,0x75627ff0),(8,0x535f4800),(9,0x4681c000)):
   struct.pack_into('<I',b,4*i,value)
  self.assertEqual(len(m.validate(bytes(b))),104)
  for i in (0,6,12,13,19,23,25):
   damaged=bytearray(b);damaged[i*4]^=1
   with self.subTest(i=i),self.assertRaisesRegex(ValueError,'Unexpected'):m.validate(damaged)
 def test_llvm_assembled_matches_pinned_modem(self):
  if not (mc:=os.environ.get('AGENT11_LLVM_MC')):self.skipTest('Hexagon LLVM6 unavailable')
  src=P.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_special_ctrl_access.s'
  b=m.assemble(Path(mc),src)
  self.assertEqual(len(b),104)
  if ref:=os.environ.get('AGENT11_MODEM_MBN'):m.verify(Path(ref).read_bytes(),b)
if __name__=='__main__':unittest.main()
