# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon source build and strict ISA-field tests for the new controller entry."""
import importlib.util, os, struct, unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-controller.py'
spec=importlib.util.spec_from_file_location('modem_rwe_controller',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class ControllerTests(unittest.TestCase):
    def test_bounds_and_elf_addresses(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x39de8,0x39ee8))
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
        self.assertEqual(len(data),256)
        get=lambda i:struct.unpack_from('<I',data,i-mod.FILE_START)[0]
        self.assertEqual(get(0x39de8),0x6e800000|(3<<14)|29)
        self.assertEqual(get(0x39dec),0x5bff7f8e)
        self.assertEqual(get(0x39dfc),0x4980c681)
        self.assertEqual(get(0x39e04),0x1001e060)
        self.assertEqual(get(0x39e08),0x5800d452)
        self.assertEqual(get(0x39e10),0x5bff7d88)
        self.assertEqual(get(0x39e20),0x707c4000)
        self.assertEqual(get(0x39e24),0x59000011)
        self.assertEqual(get(0x39e28),0x5dde6096)
        self.assertEqual(get(0x39e30),0x6e88401c)
        self.assertEqual(get(0x39e40),0x5bfffde8)
        self.assertEqual(get(0x39e44),0x00004045)  # forced extended call
        self.assertEqual(get(0x39e50),0x6e9cc012)  # privileged S28 selector
        self.assertEqual(get(0x39e58),0x49804100)  # continued memory packet
        self.assertEqual(get(0x39e60),0xab81d290)  # source-registered store
        self.assertEqual(get(0x39e84),0x48203f40)  # dealloc_return r0=2
        self.assertEqual(get(0x39e98),0x48003f40)  # return r0=0
        self.assertEqual(get(0x39e9c),0x48103f40)  # return r0=1
        self.assertEqual(get(0x39ea0),0x0c0850e4)
        self.assertEqual(get(0x39ea8),0x2402e018)
        self.assertEqual(get(0x39eac),0x0c08516a)
        self.assertEqual(get(0x39ec4),0xf0a910a8)  # two zero stores
        self.assertEqual(get(0x39ed8),0x00004906)  # forced far branch
        self.assertEqual(get(0x39ee4),0x5800c050)  # far fallback jump
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),data)
if __name__=='__main__':unittest.main()
