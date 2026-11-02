# SPDX-License-Identifier: GPL-3.0-only
"""Bounded source-built Hexagon V66 controller startup/test guards."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools/adsp'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
spec=importlib.util.spec_from_file_location('early_controller',TOOLS/'rwe-next-early-controller.py')
assert spec and spec.loader
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)

class EarlyController(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')

    def test_bounded_layout_not_firmware_repack(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x10ec0,0x10f64,164))
        self.assertEqual(m.FILL_END-m.FILL_START,12)

    def test_typed_system_registers_and_hexagon_packets(self):
        if not self.mc:self.skipTest('Hexagon LLVM6 source assembler required')
        from adsp_hexagon_v66 import (SystemRegister,read_system_register,
           write_system_register,KernelZeroLock,TlbControl)
        rebuilt=m.render(self.mc)
        self.assertEqual(len(rebuilt),164)
        tested={
            0x10ed0:read_system_register(2,SystemRegister.SGP0),
            0x10ef8:read_system_register(16,SystemRegister.HTID),
            0x10f1c:read_system_register(0,SystemRegister.SYSCFG),
            0x10f24:write_system_register(SystemRegister.SYSCFG,0),
            0x10f38:int(KernelZeroLock.UNLOCK),
            0x10f3c:int(TlbControl.UNLOCK),
            0x10f50:read_system_register(2,SystemRegister.SGP0),
        }
        for addr,expected in tested.items():
            self.assertEqual(struct.unpack_from('<I',rebuilt,addr-m.FILE_START)[0],expected)
        # Two-word LLVM6 packet: continuation on call, last on r16=r0.
        self.assertEqual(struct.unpack_from('<I',rebuilt,0x10eec-m.FILE_START)[0],0x5a0344aa)
        self.assertEqual(struct.unpack_from('<I',rebuilt,0x10f60-m.FILE_START)[0],0x529fc000)
        if self.ref:m.verify(Path(self.ref).read_bytes(),rebuilt)

    def test_reference_corruption_is_detected(self):
        if not self.mc or not self.ref:self.skipTest('Pinned ADSP reference and LLVM6 required')
        ref=Path(self.ref).read_bytes()
        rebuilt=m.render(self.mc)
        m.verify(ref,rebuilt)
        damaged=bytearray(rebuilt);damaged[-1]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,damaged)
        incorrect=bytearray(ref);incorrect[m.FILL_START]=1
        # Hash verification fails before the fill check, as intended.
        with self.assertRaisesRegex(ValueError,'Wrong ADSP reference'):
            m.verify(incorrect,rebuilt)

if __name__=='__main__':unittest.main()
