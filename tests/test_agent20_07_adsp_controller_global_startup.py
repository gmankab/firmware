# SPDX-License-Identifier: GPL-3.0-only
"""ADSP Hexagon controller-global startup typed/source-assembled prefix."""
from __future__ import annotations
import importlib.util, os, struct, sys, unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
spec=importlib.util.spec_from_file_location('adsp_global',TOOLS/'adsp-agent20-rwe-next-controller-global-startup.py')
assert spec and spec.loader
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
from adsp_hexagon_v66 import (ControllerAddressCommand,controller_address_command,
    SystemRegister,write_system_register,HexagonEncodingError)

class GlobalStartupTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_prefix_layout_and_checked_privileged_instructions(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x11450,0x114a8,88))
        self.assertEqual(m.PHYSICAL_START,0x8be0e450)
        self.assertEqual(m.REPLACEMENTS,{
            0x11468:0x6400c020,
            0x1146c:0x6400c060,
            0x11474:0x6701c00a})
    def test_privileged_encoders_are_strict(self):
        self.assertEqual(controller_address_command(
            ControllerAddressCommand.CACHE_SOFTWARE_INTERRUPT,0),0x6400c020)
        self.assertEqual(controller_address_command(
            ControllerAddressCommand.CACHE_INVALIDATE_ADDRESS,0),0x6400c060)
        self.assertEqual(write_system_register(SystemRegister.IMASK,3),0x6703c00a)
        for r in (-1,1,4,31,32,'r0'):
            with self.subTest(register=r):
                with self.assertRaises(HexagonEncodingError):
                    controller_address_command(ControllerAddressCommand.CACHE_INVALIDATE_ADDRESS,r)
        with self.assertRaises(HexagonEncodingError):
            controller_address_command('ciad',0)
    def test_source_build_and_reference(self):
        if not self.mc:self.skipTest('LLVM6 Hexagon assembler unavailable')
        b=m.render(self.mc)
        self.assertEqual(len(b),m.SIZE)
        for addr,expected in m.REPLACEMENTS.items():
            self.assertEqual(struct.unpack_from('<I',b,addr-m.FILE_START)[0],expected)
        self.assertEqual(struct.unpack_from('<I',b,0)[0],0xebf41c20)
        self.assertEqual(struct.unpack_from('<I',b,0x48)[0],0xf2634210)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)
    def test_rejects_corrupted_firmware_and_rebuilt_output(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
        ref=Path(self.ref).read_bytes();code=m.render(self.mc)
        m.verify(ref,code)
        bad=bytearray(code);bad[-1]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(ref,bad)
        badref=bytearray(ref);badref[m.FILE_START]^=1
        with self.assertRaisesRegex(ValueError,'Wrong pinned'):m.verify(badref,code)
if __name__=='__main__':unittest.main()
