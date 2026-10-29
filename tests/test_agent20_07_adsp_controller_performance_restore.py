# SPDX-License-Identifier: GPL-3.0-only
"""PMU state restoration: architecture fields, reference and corruption."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
sp=importlib.util.spec_from_file_location('pmu_restore',T/'adsp-agent20-rwe-next-controller-performance-restore.py')
assert sp and sp.loader
m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
class ControllerPerformanceRestore(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
 def test_source_and_explicit_registers(self):
  self.assertEqual((m.START,m.END,m.SIZE),(0x138f0,0x139c8,216))
  self.assertEqual(len(m.SYSTEM_TRANSFERS),16)
  self.assertEqual(m.SYSTEM_TRANSFERS[0x1393c],0x6704c030)
  self.assertEqual(m.SYSTEM_TRANSFERS[0x13994],0x6702c036)
  src=m.SOURCE.read_text()
  for item in ('memw_locked', 'PMUCNT0=R4', 'PMUEVTCFG=R4',
               'acquire_second:', 'SYSCFG=R0', 'isync', 'jumpr lr'):
   self.assertIn(item,src)
  self.assertNotIn('.word',src);self.assertNotIn('.incbin',src)
 def test_source_byte_identity(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6/pinned ADSP required')
  ref=Path(self.ref).read_bytes();out=m.render(self.mc);m.verify(ref,out)
  self.assertEqual(out[0x1393c-m.START:0x13940-m.START],struct.pack('<I',0x6704c030))
  self.assertEqual(out[0x139ac-m.START:0x139b0-m.START],struct.pack('<I',0x6e92c000))
 def test_reject_bad_revision_and_corrupted_binary(self):
  if not self.mc or not self.ref:self.skipTest('LLVM6/pinned ADSP required')
  ref=Path(self.ref).read_bytes();out=m.render(self.mc)
  refbad=bytearray(ref);refbad[-1]^=1
  with self.assertRaisesRegex(ValueError,'revision'):m.verify(refbad,out)
  outputbad=bytearray(out);outputbad[50]^=1
  with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,outputbad)
if __name__=='__main__':unittest.main()
