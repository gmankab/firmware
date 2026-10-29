# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon source build and strict ISA-field tests for the new controller entry."""
import importlib.util, os, struct, unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-controller.py'
spec=importlib.util.spec_from_file_location('modem_rwe_controller',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class ControllerTests(unittest.TestCase):
    def test_bounds_and_elf_addresses(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x39de8,0x3a1dc))
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
        self.assertEqual(len(data),1012)
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
        self.assertEqual(get(0x39ee8),0x6509c000)  # typed SGP0 bank swap
        self.assertEqual(get(0x39f10),0x6a04401f)  # P3:0 to r31
        self.assertEqual(get(0x39f34),0x751e4460)  # P0 producer first
        self.assertEqual(get(0x39f38),0x5c004810)  # corresponding branch
        self.assertEqual(get(0x39f44),0x751e43a0)  # producer first
        self.assertEqual(get(0x39f48),0x5c00c808)  # branch ends packet
        self.assertEqual(get(0x39f50),0xd2167c80)  # checked boundscheck
        self.assertEqual(get(0x39f6c),0x6509c000)  # re-swap SGP0
        self.assertEqual(get(0x39f88),0x6509c000)  # fallback swap
        self.assertEqual(get(0x39f8c),0x6300c002) # checked extended-control write
        self.assertEqual(get(0x39ff0),0xbfe37fe3) # slot counter decrement
        self.assertEqual(get(0x39ff4),0x2572c1e8) # loop exit predicate
        self.assertEqual(get(0x39ff8),0x5800c060) # recovery branch
        self.assertEqual(get(0x3a00c),0x6c82c003) # privileged state commit
        self.assertEqual(get(0x3a018),0x9780ffe2) # negative-index memory load
        self.assertEqual(get(0x3a028),0x67004003) # protected system-reg write
        self.assertEqual(get(0x3a02c),0xf200c003) # original predicate packet
        self.assertEqual(get(0x3a030),0x0c085004)  # extended locked-word address
        self.assertEqual(get(0x3a038),0x9202c003)  # protected locked read
        self.assertEqual(get(0x3a044),0xa0a2c300)  # conditional locked store
        self.assertEqual(get(0x3a04c),0x6e864002)  # privileged S6 read
        self.assertEqual(get(0x3a058),0xc7824300)  # hardware mask bitsclr
        self.assertEqual(get(0x3a06c),0x62434000)  # trace packet continues
        self.assertEqual(get(0x3a070),0xc683c243)  # trace packet closes
        self.assertEqual(get(0x3a074),0x75034010)  # producer-first predicate
        self.assertEqual(get(0x3a078),0x5c005812)  # conditional jump
        self.assertEqual(get(0x3a07c),0x0c085004)  # extended memory store
        self.assertEqual(get(0x3a080),0x4880c300)  # end packet
        self.assertEqual(get(0x3a084),0x6ebfc002)  # S63 interrupt mask read
        self.assertEqual(get(0x3a08c),0x8cc2d022)  # clear interrupt flag bit
        self.assertEqual(get(0x3a090),0x6702c03f)  # S63 <= r2, checked fields
        self.assertEqual(get(0x3a098),0x78004003)  # reset lock flag
        self.assertEqual(get(0x3a0a0),0x48a0d204)  # GP-linked store
        self.assertEqual(get(0x3a0a4),0x68224000)  # C35:34 -> r1:0
        self.assertEqual(get(0x3a0b0),0x6c20c040)  # protected frame-mode select
        self.assertEqual(get(0x3a0b4),0x57e0c000)  # architectural RTE
        self.assertEqual(get(0x3a0b8),0x0c08508d)  # alternate frame pointer
        self.assertEqual(get(0x3a0c4),0x6822c000)  # selected C35:34 read
        self.assertEqual(get(0x3a0c8),0x6509c000)  # SGP0 exchange
        self.assertEqual(get(0x3a0e4),0xa1c94007)  # saved exception pair
        self.assertEqual(get(0x3a0ec),0x6a04c001)  # P3:0 read
        self.assertEqual(get(0x3a100),0x62214004)  # predicate bank write
        self.assertEqual(get(0x3a104),0x5c00d20c)  # absolute target 0x3a118
        self.assertEqual(get(0x3a10c),0x8cdcc13c)  # protected status bit
        self.assertEqual(get(0x3a128),0xa1c9de0a)  # save C31:30
        self.assertEqual(get(0x3a154),0x681a2a05)  # paired alloc/record
        self.assertEqual(get(0x3a160),0x6509c000)  # SGP0 exchange
        self.assertEqual(get(0x3a16c),0x10006014)  # grouped pointer test/load
        self.assertEqual(get(0x3a178),0x1040400e)  # next pointer test/load
        self.assertEqual(get(0x3a188),0xf0091008)  # duplex descriptor reset
        self.assertEqual(get(0x3a194),0x8f33c801)  # first inserted frame ID
        self.assertEqual(get(0x3a1a8),0x6be24103)  # class predicate OR
        self.assertEqual(get(0x3a1c8),0x59ff7e12)  # original controller return
        self.assertEqual(get(0x3a1d4),0x172a4068)  # alternate return branch
        self.assertEqual(get(0x3a1d8),0x901ec01e)  # saved frame deallocation
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),data)
if __name__=='__main__':unittest.main()
