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
        self.assertEqual(module.FILE_END,0x6568)
        self.assertEqual(module.PADDR,0x8BE03030)
        self.assertEqual(module.SIZE,1336)
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

    def test_privileged_state_and_duplex_layout(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        b=module.render(self.mc)
        offset=module.STARTUP_SIZE+module.DISPATCH_SIZE
        self.assertEqual(struct.unpack_from('<I',b,offset+0x20)[0],
                         read_system_register(0,SystemRegister.SYSCFG))
        self.assertEqual(struct.unpack_from('<I',b,offset+0x2c)[0],
                         write_system_register(SystemRegister.SYSCFG,0))
        self.assertEqual(struct.unpack_from('<I',b,offset+0x30)[0],
                         read_system_register(0,SystemRegister.SSR))
        self.assertEqual(struct.unpack_from('<I',b,offset+0x38)[0],
                         write_system_register(SystemRegister.SSR,0))
        self.assertEqual(struct.unpack_from('<I',b,offset+0x68)[0],0x29803A01)
        self.assertEqual(struct.unpack_from('<I',b,offset+0x6c)[0],0x5400C000)
        self.assertEqual(struct.unpack_from('<I',b,offset+0x70)[0],0x5800C000)

    def test_poll_packet_extenders_and_duplex_writes(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        b=module.render(self.mc)
        off=module.STARTUP_SIZE+module.DISPATCH_SIZE+module.FINALIZE_SIZE
        self.assertEqual(struct.unpack_from('<I',b,off)[0],0x0F004956)
        self.assertEqual(struct.unpack_from('<I',b,off+0x0c)[0],0x0F0049BC)
        self.assertEqual(struct.unpack_from('<I',b,off+0x18)[0],0x0F004955)
        # Two simultaneous zero writes occupy one Hexagon duplex word.
        self.assertEqual(struct.unpack_from('<I',b,off+0x24)[0],0xF0091008)
        self.assertEqual(struct.unpack_from('<I',b,off+0x28)[0],0xF0041005)
        self.assertEqual(struct.unpack_from('<I',b,off+0x48)[0],0x5800C000)

    def test_rwe_memory_init_privilege_and_packet_continuation(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        code=module.render(self.mc)
        off=module.STARTUP_SIZE+module.DISPATCH_SIZE+module.FINALIZE_SIZE+module.POLL_SIZE
        self.assertEqual(struct.unpack_from('<I',code,off+0x20)[0],
                         read_system_register(1,SystemRegister.HTID))
        self.assertEqual(struct.unpack_from('<I',code,off+0x30)[0],
                         module.packet_continuation(module.read_control_register(
                             28,module.ControlRegister.P3_0)))
        self.assertEqual(struct.unpack_from('<I',code,off+0x80)[0],0xF0191018)
        self.assertEqual(struct.unpack_from('<I',code,off+0x88)[0],0x3F41C0D5)

    def test_thread_guard_v66_pair_and_predicate_write(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        b=module.render(self.mc)
        off=(module.STARTUP_SIZE+module.DISPATCH_SIZE+module.FINALIZE_SIZE+
             module.POLL_SIZE+module.MEMORY_INIT_SIZE)
        self.assertEqual(struct.unpack_from('<I',b,off+0x08)[0],
                         read_system_register(1,SystemRegister.HTID))
        self.assertEqual(struct.unpack_from('<I',b,off+0x2c)[0],
                         module.packet_continuation(module.write_predicate_group_from_gpr(28)))
        self.assertEqual(struct.unpack_from('<I',b,off+0x20)[0],0x9213C000)
        self.assertEqual(struct.unpack_from('<I',b,off+0x28)[0],0x5C00E836)

    def test_frame_initialization_exact_packet_layout(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        b=module.render(self.mc)
        off=(module.STARTUP_SIZE+module.DISPATCH_SIZE+module.FINALIZE_SIZE+
             module.POLL_SIZE+module.MEMORY_INIT_SIZE+module.THREAD_GUARD_SIZE)
        self.assertGreaterEqual(len(b)-off,module.FRAME_INIT_SIZE)
        self.assertEqual(module.FRAME_INIT_SIZE,0x6c)
        # The non-extended mpyi operation must be in its original packet
        # position. An explicit extender would change this word/size.
        self.assertEqual(struct.unpack_from('<I',b,off+0x0c)[0],0xe11c5c01)
        if self.ref:module.verify(Path(self.ref).read_bytes(),b)

    def test_runtime_system_bank_and_new_value_packet(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        code=module.render(self.mc)
        off=(module.STARTUP_SIZE+module.DISPATCH_SIZE+module.FINALIZE_SIZE+
             module.POLL_SIZE+module.MEMORY_INIT_SIZE+
             module.THREAD_GUARD_SIZE+module.FRAME_INIT_SIZE+
             module.RUNTIME_CALLS_SIZE)
        self.assertGreaterEqual(len(code)-off,module.RUNTIME_REGISTERS_SIZE)
        values=[(0x00,module.packet_continuation(module.read_system_register(
                    1,SystemRegister.HTID))),
                (0x10,module.packet_continuation(module.read_system_register(
                    1,SystemRegister.MODECTL))),
                (0x24,module.read_system_register(0,SystemRegister.SYSCFG)),
                (0x2c,module.write_system_register(SystemRegister.SYSCFG,0)),
                (0x30,module.packet_continuation(module.read_system_register(
                    0,SystemRegister.SSR))),
                (0x3c,module.write_system_register(SystemRegister.SSR,0))]
        for at,v in values:self.assertEqual(struct.unpack_from('<I',code,off+at)[0],v)
        self.assertEqual(struct.unpack_from('<I',code,off+0x14)[0],0xa1a0d200)
        if self.ref:module.verify(Path(self.ref).read_bytes(),code)

    def test_runtime_diagnostics_privileged_and_control_flow(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        fragment=module.render(self.mc)
        off=(module.STARTUP_SIZE+module.DISPATCH_SIZE+module.FINALIZE_SIZE+
             module.POLL_SIZE+module.MEMORY_INIT_SIZE+
             module.THREAD_GUARD_SIZE+module.FRAME_INIT_SIZE+
             module.RUNTIME_CALLS_SIZE+module.RUNTIME_REGISTERS_SIZE)
        self.assertEqual(struct.unpack_from('<I',fragment,off+8)[0],
                         module.read_system_register(1,SystemRegister.SSR))
        self.assertEqual(struct.unpack_from('<I',fragment,off+0xc)[0],
                         module.packet_continuation(module.read_system_register(
                             0,SystemRegister.ELR)))
        self.assertEqual(struct.unpack_from('<I',fragment,off+0x18)[0],
                         module.packet_continuation(module.read_system_register(
                             1,SystemRegister.DIAG)))
        self.assertEqual(struct.unpack_from('<I',fragment,off+0x24)[0],
                         module.packet_continuation(module.read_system_register(
                             0,SystemRegister.SYSCFG)))
        poll_off=off+module.RUNTIME_DIAGNOSTICS_SIZE+module.RUNTIME_RECOVERY_SIZE
        poll=fragment[poll_off:poll_off+module.RUNTIME_POLL_SIZE]
        # End is an explicit one-word no-progress controller loop.
        self.assertEqual(struct.unpack_from('<I',poll,len(poll)-4)[0],0x5800c000)
        if self.ref:module.verify(Path(self.ref).read_bytes(),fragment)

    def test_launch_controller_special_registers_and_handoff(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        b=module.render(self.mc)
        launch_off=(len(b)-module.RESET_CONTROLLER_SIZE-
                    module.THREAD_LAUNCH_SIZE-module.STATE_READY_SIZE)
        self.assertEqual(struct.unpack_from('<I',b,launch_off+0x14)[0],
            module.packet_continuation(module.read_system_register(
                1,SystemRegister.HTID)))
        self.assertEqual(struct.unpack_from('<I',b,launch_off+0x24)[0],
            module.packet_continuation(module.read_system_register(
                1,SystemRegister.MODECTL)))
        self.assertEqual(struct.unpack_from('<I',b,launch_off+0x3c)[0],0x5800c000)
        # Thin handoff trampoline immediately follows ready-state return.
        self.assertEqual(struct.unpack_from('<I',b,launch_off+module.THREAD_LAUNCH_SIZE+module.STATE_READY_SIZE-4)[0],0x5803f218)
        if self.ref:module.verify(Path(self.ref).read_bytes(),b)

    def test_controller_reset_exact_syscfg_and_ssr_slots(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC missing')
        b=module.render(self.mc)
        off=len(b)-module.RESET_CONTROLLER_SIZE
        checks={0x04:module.read_system_register(0,SystemRegister.SSR),
                0x0c:module.write_system_register(SystemRegister.SSR,0),
                0x54:module.read_system_register(0,SystemRegister.SYSCFG),
                0x5c:module.write_system_register(SystemRegister.SYSCFG,0),
                0x60:module.packet_continuation(module.read_system_register(
                    0,SystemRegister.SSR)),
                0x6c:module.write_system_register(SystemRegister.SSR,0)}
        for offset,word in checks.items():
            self.assertEqual(struct.unpack_from('<I',b,off+offset)[0],word)
        self.assertEqual(struct.unpack_from('<I',b,len(b)-4)[0],0x5800c000)
        if self.ref:module.verify(Path(self.ref).read_bytes(),b)

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
