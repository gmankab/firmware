# SPDX-License-Identifier: GPL-3.0-only
"""Checks for the source-generated protected exception dispatch entry."""
import importlib.util
import os
from pathlib import Path
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'tools/assemble-agent11-rwe-exception.py'
spec = importlib.util.spec_from_file_location('modem_rwe_exception', str(SCRIPT))
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)


class ModemRweException(unittest.TestCase):
    def test_exact_address_mapping(self):
        self.assertEqual((module.FILE_START, module.FILE_END), (0x38108, 0x38ce4))
        self.assertEqual(module.VIRTUAL_START-module.FILE_START, 0xc0808000)
        self.assertEqual(module.PHYSICAL_START-module.FILE_START, 0x8e008000)

    def test_packet_normalizer_fails_closed_on_unknown_opcode(self):
        with self.assertRaisesRegex(ValueError,'unrecognized Hexagon LLVM6'):
            module.normalize_llvm6_locked_packets(bytes(module.FILE_END-module.FILE_START))

    def test_wrong_source_revision_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'Wrong modem firmware revision'):
            module.verify_pinned(bytes(module.FILE_END), bytes(module.FILE_END-module.FILE_START))

    def test_llvm6_source_assembly(self):
        mc = os.environ.get('AGENT11_LLVM_MC')
        if not mc:
            self.skipTest('Set AGENT11_LLVM_MC to run LLVM6 Hexagon assembler test')
        source = SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_dispatch.s'
        data = module.assemble(Path(mc), source)
        self.assertEqual(len(data), 3036)
        self.assertEqual(data[0x381b8-module.FILE_START:0x381bc-module.FILE_START],bytes.fromhex('00c0e057'))
        self.assertEqual(data[0:4], bytes.fromhex('00c00058'))
        # First 148 bytes after the restored RTE encode vector resolution,
        # paired controller reads and the protected context snapshot.
        self.assertEqual(data[0x385e8-module.FILE_START:0x385ec-module.FILE_START],
                         (0x850d4300).to_bytes(4, 'little'))
        self.assertEqual(data[0x38638-module.FILE_START:0x3863c-module.FILE_START],
                         (0x68104008).to_bytes(4, 'little'))
        self.assertEqual(data[0x38654-module.FILE_START:0x38658-module.FILE_START],
                         (0x6812c008).to_bytes(4, 'little'))
        self.assertEqual(int.from_bytes(data[0x3868c-module.FILE_START:0x38690-module.FILE_START], 'little'),
                         0x6a114008) # checked Y4 frame-key read
        self.assertEqual(int.from_bytes(data[0x38684-module.FILE_START:0x38688-module.FILE_START], 'little'),
                         0x78004289) # extended packet continues
        self.assertEqual(int.from_bytes(data[0x386bc-module.FILE_START:0x386c0-module.FILE_START], 'little'),0x961ec01e) # framekey dealloc_return
        self.assertEqual(int.from_bytes(data[0x38848-module.FILE_START:0x3884c-module.FILE_START], 'little'),0x57e04000)
        self.assertEqual(int.from_bytes(data[0x38850-module.FILE_START:0x38854-module.FILE_START], 'little'),0x57e0c000)
        # Verify final original RTE and source-defined controller class fields.
        self.assertEqual(data[0x38bec-module.FILE_START:0x38bf0-module.FILE_START],(0x57e00000 | (3<<14)).to_bytes(4,'little'))
        at=lambda file_offset: int.from_bytes(data[file_offset-module.FILE_START:file_offset-module.FILE_START+4],'little')
        self.assertEqual(at(0x38b70), 0x62000000 | (32<<16) | (1<<14) | 4) # P3:0 = R0
        self.assertEqual(at(0x38b8c), 0x63000000 | (38<<16) | (3<<14) | 18) # PKTCOUNT = R7:6
        self.assertEqual(at(0x38bc4), 0x63000000 | (52<<16) | (1<<14) | 12) # CS1:0 = R21:20
        self.assertEqual(at(0x38bd4), 0x67000000 | (24<<16) | (1<<14) | 2) # STID = R24
        self.assertEqual(at(0x38be0), 0x67000000 | (22<<16) | (3<<14) | 7) # CCR = R22
        # MODECTL fallback uses checked SYSCFG privileged-register operands.
        self.assertEqual(at(0x38bf4),0x6e800000|(17<<16)|(3<<14)|1)
        self.assertEqual(at(0x38c40),0x6e800000|(18<<16)|(3<<14)|3)
        self.assertEqual(at(0x38c48),0x67000000|(3<<16)|(3<<14)|18)
        self.assertEqual(at(0x38c50),0x57c0c002) # standalone architecture ISYNC
        # MODECTL/video-ID exception flow: late V60 NMI and CIAD instructions
        # cannot be mnemonic-assembled by LLVM6, so verify architectural
        # register and parse bits for these source-defined operands.
        self.assertEqual(at(0x38c74),0x64000000|((0x60|29)<<16)|(3<<14)|0x40)
        self.assertEqual(at(0x38c78),0x6e800000|(6<<16)|(1<<14)|20)
        self.assertEqual(at(0x38c9c),0x6e800000|(21<<16)|(1<<14)|9)
        self.assertEqual(at(0x38cb4),0x64000000|(31<<16)|(1<<14)|0x60)
        self.assertEqual(at(0x38c94),0x58004080)  # jump continues into L2
        self.assertEqual(at(0x38ca8),0x750940a2)  # producer-first P2 compare
        self.assertEqual(at(0x38cac),0x5cdf6ad8)  # .new control branch
        self.assertEqual(at(0x38ce0),0x7800c022)
        ref = os.environ.get('AGENT11_MODEM_MBN')
        if ref:
            module.verify_pinned(Path(ref).read_bytes(), data)


if __name__ == '__main__':
    unittest.main()
