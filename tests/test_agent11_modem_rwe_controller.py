# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon source build and strict ISA-field tests for the new controller entry."""
import importlib.util, os, struct, unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-controller.py'
spec=importlib.util.spec_from_file_location('modem_rwe_controller',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class ControllerTests(unittest.TestCase):
    def test_bounds_and_elf_addresses(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x39de8,0x39e0c))
        self.assertEqual(mod.VIRTUAL_START-mod.FILE_START,0xc0808000)
        self.assertEqual(mod.PHYSICAL_START-mod.FILE_START,0x8e008000)
    def test_wrong_modem_reference_is_rejected(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem source revision'):
            mod.verify(bytes(mod.FILE_END),bytes(mod.FILE_END-mod.FILE_START))
    def test_normalization_fails_closed(self):
        with self.assertRaisesRegex(ValueError,'Unexpected signed V60'):
            mod.normalize_v60_gp(bytes(mod.FILE_END-mod.FILE_START))
        b=bytearray(mod.FILE_END-mod.FILE_START)
        struct.pack_into('<I',b,0x39df8-mod.FILE_START,0x0c084529)
        with self.assertRaisesRegex(ValueError,'Unexpected generic GP-load'):
            mod.normalize_v60_gp(bytes(b))
    def test_assembler_and_optional_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('Set AGENT11_LLVM_MC to verify original LLVM6 source assembly')
        source=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_controller.s'
        data=mod.assemble(Path(mc),source)
        self.assertEqual(len(data),36)
        get=lambda i:struct.unpack_from('<I',data,i-mod.FILE_START)[0]
        self.assertEqual(get(0x39de8),0x6e800000|(3<<14)|29)
        self.assertEqual(get(0x39dec),0x5bff7f8e)
        self.assertEqual(get(0x39dfc),0x4980c681)
        self.assertEqual(get(0x39e04),0x1001e060)
        self.assertEqual(get(0x39e08),0x5800d452)
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),data)
if __name__=='__main__':unittest.main()
