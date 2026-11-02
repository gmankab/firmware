# SPDX-License-Identifier: GPL-3.0-only
"""ADSP V66 instruction-cache snapshot: checked ISA and source packet tests."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools/adsp'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
P=TOOLS/'rwe-next-icache.py'
spec=importlib.util.spec_from_file_location('icache_snapshot',P)
assert spec is not None and spec.loader is not None
icache=importlib.util.module_from_spec(spec)
spec.loader.exec_module(icache)

class InstructionCacheSnapshot(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC')
        cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_disjoint_layout_and_zero_fills_are_explicit(self):
        self.assertEqual((icache.FILE_START,icache.FILE_END,icache.SIZE),
                         (0x106e8,0x1084c,356))
        self.assertEqual((icache.FILL_START,icache.FILL_END),
                         (0x10674,0x106e8))
        self.assertEqual(icache.FOLLOW_FILL_END,0x10c30)
    def test_source_assembler_and_privileged_operations(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC needed')
        from adsp_hexagon_v66 import (SystemRegister,ControlRegister,
          InstructionCacheRead,read_system_register,write_system_register,
          read_control_register,read_instruction_cache_word,packet_continuation)
        base=icache.base()
        ordinary=base.assemble(self.mc,icache.SOURCE,icache.SIZE)
        actual=icache.emit(self.mc)
        expected={
          0x00:read_system_register(0,SystemRegister.REV),
          0x40:read_system_register(0,SystemRegister.SYSCFG),
          0x48:write_system_register(SystemRegister.SYSCFG,0),
          0x5c:packet_continuation(read_control_register(10,ControlRegister.LC1)),
          0x6c:packet_continuation(read_control_register(10,ControlRegister.LC0)),
          0xa8:read_instruction_cache_word(1,10,InstructionCacheRead.TAG),
          0xf8:read_instruction_cache_word(1,10,InstructionCacheRead.DATA),
          0x124:read_system_register(1,SystemRegister.SYSCFG),
          0x12c:write_system_register(SystemRegister.SYSCFG,1),
        }
        for off,value in expected.items():
            self.assertEqual(struct.unpack_from('<I',actual,off)[0],value)
            self.assertNotEqual(actual[off:off+4],ordinary[off:off+4])
        self.assertEqual(actual[0x118:0x124],ordinary[0x118:0x124])
        self.assertEqual(struct.unpack_from('<I',actual,0x114)[0],0x10f4eff4)
        self.assertEqual(actual[-8:],ordinary[-8:])
        if self.ref:
            icache.verify(Path(self.ref).read_bytes(),actual)
    def test_strict_reference_and_gap_mutations(self):
        if not (self.ref and self.mc):self.skipTest('reference and LLVM6 needed')
        original=Path(self.ref).read_bytes()
        built=icache.emit(self.mc)
        icache.verify(original,built)
        for off in (0x106e8,0x10790,0x10800):
            mutated=bytearray(built)
            mutated[off-icache.FILE_START]^=0x01
            with self.assertRaisesRegex(ValueError,'not reference-identical'):
                icache.verify(original,mutated)
        for off in (0x10674,0x1084c,0x10c2f):
            altered=bytearray(original);altered[off]=1
            # Fails pinned hash before checking holes, as intended.
            with self.assertRaisesRegex(ValueError,'Wrong ADSP reference'):
                icache.verify(altered,built)

if __name__=='__main__':unittest.main()
