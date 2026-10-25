# SPDX-License-Identifier: GPL-3.0-only
"""ADSP second RWE segment: strict mapped address and packed V66 checks."""
import importlib.util
import os
from pathlib import Path
import struct
import sys
import unittest
from unittest.mock import patch
TOOLS=Path(__file__).resolve().parents[1]/'tools'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
from adsp_hexagon_v66 import SystemRegister,read_system_register,write_system_register
s=importlib.util.spec_from_file_location('agent20_adsp_rwe',TOOLS/'adsp-agent20-rwe.py')
assert s and s.loader
module=importlib.util.module_from_spec(s);s.loader.exec_module(module)

class RweEntryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.environ.get('HEXAGON_LLVM_MC')
        cls.ref=os.environ.get('NABU_ADSP_REFERENCE')
    def test_ptload_offset_mapping(self):
        self.assertEqual(module.FILE_START,0x6030)
        self.assertEqual(module.FILE_END,0x60d4)
        self.assertEqual(module.PADDR,0x8BE03030)
        self.assertEqual(module.SIZE,164)
    def test_source_assembler_with_checked_v66_registers(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        b=module.render(self.mc)
        for at,expected in [(0x14,read_system_register(1,SystemRegister.SYSCFG)),
                            (0x1c,write_system_register(SystemRegister.SYSCFG,1)),
                            (0x20,read_system_register(0,SystemRegister.SGP0))]:
            self.assertEqual(struct.unpack_from('<I',b,at)[0],expected)
        self.assertEqual(len(b),module.SIZE)
        # Distinct input-firmware variants must not silently verify.
        if self.ref:module.verify(Path(self.ref).read_bytes(),b)
    def test_calls_and_mode_dispatch_are_source_built(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        code=module.render(self.mc)
        # Packetized five external calls and two mode tests follow startup.
        # No original OEM code bytes are imported by render().
        self.assertEqual(struct.unpack_from('<I',code,0x5c)[0],0x5A03F9C0)
        self.assertEqual(struct.unpack_from('<I',code,0x60)[0],0xF501C016)
        self.assertEqual(struct.unpack_from('<I',code,0x70)[0],0x1010C27A)
        self.assertEqual(struct.unpack_from('<I',code,0x74)[0],0x1000E152)
        self.assertEqual(struct.unpack_from('<I',code,0x80)[0],0x5A00C5C0)

    def test_wrong_firmware_is_rejected(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        with self.assertRaisesRegex(ValueError,'SHA'):
            module.verify(b'garbage',module.render(self.mc))
    def test_unmoved_privileged_slots_required(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        actual=module.baseline()
        b=bytearray(actual.assemble(self.mc,module.SOURCE,module.STARTUP_SIZE))
        b[0x20:0x24]=struct.pack('<I',0x7f00c001)
        with patch.object(module,'baseline',return_value=type('Fake',(),{
            'assemble':staticmethod(lambda *x:bytes(b)),
            'AdspError':ValueError,
        })()):
            with self.assertRaisesRegex(ValueError,'sentinel moved'):
                module.render(self.mc)
