# SPDX-License-Identifier: GPL-3.0-only
"""Checks for the source-generated protected exception dispatch entry."""
import importlib.util
import os
from pathlib import Path
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'tools/modem/assemble-rwe-exception.py'
spec = importlib.util.spec_from_file_location('modem_rwe_exception', str(SCRIPT))
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)


class ModemRweException(unittest.TestCase):
    def test_exact_address_mapping(self):
        self.assertEqual((module.FILE_START, module.FILE_END), (0x38108, 0x390dc))
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
        source = SCRIPT.parents[1]/'sources/xiaomi-nabu/modem/modem_rwe_exception_dispatch.s'
        data = module.assemble(Path(mc), source)
        self.assertEqual(len(data), 4052)
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
        # Additional frame-snapshot save path and lock retry: validate the
        # original producer-first P1 .new packet and the literal-free NMI
        # return from four-word CRSWAP/compact-branch packet.
        self.assertEqual(at(0x38ce8), 0x70714000)
        self.assertEqual(at(0x38cec), 0x750b4001)
        self.assertEqual(at(0x38cf0), 0x5c20e926)
        self.assertEqual(at(0x38d04), 0x65000000 | (31<<16) | (3<<14))
        self.assertEqual(at(0x38d2c), 0x78004000)
        self.assertEqual(at(0x38d30), 0xa0a00000 | (3<<14) | (1<<8))
        self.assertEqual(at(0x38d74), 0x62000000 | ((21+32)<<16) | (1<<14) | 4)
        self.assertEqual(at(0x38d8c), 0x91800000 | (31<<16) | (1<<14))
        self.assertEqual(at(0x38d90),0x2606e000 | ((((-0x10c)>>9)&3)<<20) | (((-0x10c)>>1)&255))
        # CIAD frame publication and cacheline initialization packet:
        # a p0.new branch, GP/UGP field snapshot and packet-local callback.
        self.assertEqual(at(0x38dc8),0x64000000|(31<<16)|(3<<14)|0x60)
        self.assertEqual(at(0x38dcc),0xf2005e00)
        self.assertEqual(at(0x38dd4),0x5c004868)
        self.assertEqual(at(0x38de8),0xb0005015)
        self.assertEqual(at(0x38df4),0x6a000000|(8<<16)|(1<<14)|29)
        self.assertEqual(at(0x38e04),0x6a000000|(10<<16)|(1<<14)|23)
        self.assertEqual(at(0x38e10),0x6a000000|(11<<16)|(1<<14)|22)
        self.assertEqual(at(0x38e20),0x5a004000)
        self.assertEqual(at(0x38e28),0x28123009) # source-assembled duplex
        # Saved codec controller register-bank state; the dedicated V60
        # GP-relative classes differ from LLVM6's generic r28 encoding.
        self.assertEqual(at(0x38e34),0x4980c68b)
        self.assertEqual(at(0x38e64),0x6e83c00c) # S3 (ELR) read
        self.assertEqual(at(0x38e68),0x6a01c015) # LC0 config read
        self.assertEqual(at(0x38ea4),0x49804614)
        self.assertEqual(at(0x38ec0),0x49804594)
        self.assertEqual(at(0x38ec8),0x67000000 | (20<<16) | (1<<14) | 7)
        # Check the newly added frame-publication path: two paired-control
        # reads, the HTID video-lane snapshot, and byte-descriptor write.
        self.assertEqual(at(0x38ed4), 0x68000000 | (16<<16) | (3<<14) | 2)
        self.assertEqual(at(0x38ed8), 0x68000000 | (18<<16) | (3<<14) | 4)
        self.assertEqual(at(0x38ee8), 0x6e800000 | (8<<16) | (1<<14) | 1)
        self.assertEqual(at(0x38ef8), 0x6e800000 | (2<<16) | (1<<14) | 5)
        self.assertEqual(at(0x38f08) & (3<<14), (3<<14))
        # Checked extended PC-relative codec frame dispatch. The compound
        # P0 compare-equal and branch uses architectural register/immediate
        # fields, NOT a copied vendor instruction word.
        self.assertEqual(at(0x38f0c)&(3<<14),1<<14)
        self.assertEqual(at(0x38f14),0x10000000|(2<<16)|(1<<14)|0x60)
        self.assertEqual(at(0x38f1c),0x0fff7585) # distinct first call target
        self.assertEqual(at(0x38f24),0x0fff757e) # distinct second call target
        self.assertEqual(at(0x38f40),0x3a0e490e) # indexed byte load r14
        self.assertEqual(at(0x38f4c),0x6e800000|(8<<16)|(1<<14)|1)
        self.assertEqual(at(0x38f5c),0x6e800000|(8<<16)|(1<<14)|2)
        # Frame/register-bank capture must preserve paired control fields,
        # original 32-byte aligned cache zero and the return + duplex packet.
        self.assertEqual(at(0x38f74),0x6e800000|(6<<16)|(3<<14)|23) # SSR
        self.assertEqual(at(0x38f88),0x67000000|(23<<16)|(1<<14)|6) # SSR write
        self.assertEqual(at(0x38f90),0x6a000000|(10<<16)|(1<<14)|17) # UGP
        self.assertEqual(at(0x38fc0),0x68000000|(0<<16)|(1<<14)|20) # C1:0
        self.assertEqual(at(0x38fdc),0x6a000000|(8<<16)|(1<<14)|21) # USR
        self.assertEqual(at(0x38fe4),0x6a000000|(4<<16)|(1<<14)|20) # P3:0
        self.assertEqual(at(0x38ffc),0x68000000|(16<<16)|(1<<14)|2) # C17:16
        self.assertEqual(at(0x39004),0x529f4000) # jumpr LR, continue packet
        self.assertEqual(at(0x39008),0xeaf90a2f) # duplex frame stores
        # Exception-ring access: HTID snapshot, paired S57:56 read, and
        # loop0 with a paired descriptor-word store and explicit endloop.
        self.assertEqual(at(0x3900c),0x6e800000|(8<<16)|(1<<14)|12)
        self.assertEqual(at(0x39018),0x850f4e00)
        self.assertEqual(at(0x3903c),0x6f000000|(56<<16)|(1<<14)|14)
        self.assertEqual(at(0x39054),0x6002c008)
        self.assertEqual(at(0x39058),0x7f008000) # loop-body NOP
        self.assertEqual(at(0x3905c),0x60400001) # loop0 word-copy duplex
        self.assertEqual(at(0x39060),0x529fc000) # protected return
        # The PCYCLE reset helper and two long diagnostic branch trampolines
        # must preserve the short-to-long Hexagon packet form.
        self.assertEqual(at(0x39064),0x6d000000|(3<<14)|30)
        self.assertEqual(at(0x39068),0x529fc000)
        self.assertEqual(at(0x3906c),at(0x39078)) # same long jump/call target
        self.assertEqual(at(0x39090),0x85075100)
        self.assertEqual(at(0x39094),0x74096007)
        self.assertEqual(at(0x39098),0x5cddc812)
        self.assertEqual(at(0x390a0),0x16000000|(2<<16)|(1<<14)|0x40)
        self.assertEqual(at(0x390c0),0x529f4000)
        self.assertEqual(at(0x390d0),0x5800c068)
        self.assertEqual(at(0x390d8),0x5800c018)
        ref = os.environ.get('AGENT11_MODEM_MBN')
        if ref:
            module.verify_pinned(Path(ref).read_bytes(), data)


if __name__ == '__main__':
    unittest.main()
