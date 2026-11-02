# SPDX-License-Identifier: GPL-3.0-only
"""Bounded ADSP Hexagon V66 thread registration instruction source tests."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
if str(T) not in sys.path:sys.path.insert(0,str(T))
p=T/'rwe-next-thread-record.py'
s=importlib.util.spec_from_file_location('adsp_thread_register',p)
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class ThreadRegister(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
    cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
 def test_disjoint_function_boundaries(self):
    self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x10dc4,0x10eb4,240))
    self.assertEqual((m.FILL_START,m.FILL_END),(0x10eb4,0x10ec0))
 def test_source_packets_and_checked_privileged_operands(self):
    if not self.mc:self.skipTest('Hexagon LLVM6 required')
    from adsp_hexagon_v66 import (SystemRegister,read_system_register,
         read_system_register_pair,KernelZeroLock)
    b=m.render(self.mc)
    self.assertEqual(struct.unpack_from('<I',b,0)[0],0x529fc000)
    self.assertEqual(struct.unpack_from('<I',b,0x10)[0],
                     read_system_register(18,SystemRegister.HTID))
    self.assertEqual(struct.unpack_from('<I',b,0x14)[0],read_system_register_pair(20,56))
    self.assertEqual(struct.unpack_from('<I',b,0x90)[0],int(KernelZeroLock.LOCK))
    self.assertEqual(struct.unpack_from('<I',b,0xa8)[0],int(KernelZeroLock.UNLOCK))
    self.assertEqual(struct.unpack_from('<I',b,m.SIZE-4)[0],0x901ec01e)
    if self.ref:m.verify(Path(self.ref).read_bytes(),b)
 def test_reference_errors(self):
    if not(self.ref and self.mc):self.skipTest('Reference and LLVM6 required')
    ref=Path(self.ref).read_bytes();original=m.render(self.mc)
    m.verify(ref,original)
    corrupt=bytearray(original);corrupt[0]^=1
    with self.assertRaisesRegex(ValueError,'differs'):
        m.verify(ref,corrupt)
if __name__=='__main__':unittest.main()
