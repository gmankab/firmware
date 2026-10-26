# SPDX-License-Identifier: GPL-3.0-only
"""Independent source reconstruction and bounds for the disjoint RWE lock."""
import importlib.util
import os
from pathlib import Path
import unittest

SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-followup.py'
spec=importlib.util.spec_from_file_location('modem_rwe_followup',str(SCRIPT))
mod=importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(mod)

class ExceptionFollowup(unittest.TestCase):
    def test_disjoint_bounds_and_no_implicit_vendor_fill(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x39920,0x39de8))
        self.assertEqual(mod.FILE_END-mod.FILE_START,1224)
        self.assertGreater(mod.FILE_START,0x390dc)
        self.assertEqual(mod.FILE_START-0x390dc,0x844)

    def test_pinned_revision_fails_closed(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem source revision'):
            mod.verify(bytes(mod.FILE_END),bytes(mod.FILE_END-mod.FILE_START))

    def test_gp_load_normalizer_fails_closed(self):
        import struct
        fake=bytearray(mod.FILE_END-mod.FILE_START)
        struct.pack_into('<I',fake,0x39aec-mod.FILE_START,0x919cc101)
        with self.assertRaisesRegex(ValueError,'GP-relative'):
            mod.normalize_hexagon_gp_relative(bytes(fake))

    def test_source_assembler_and_optional_byte_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('Set AGENT11_LLVM_MC for Hexagon source assembly test')
        source=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_followup.s'
        data=mod.assemble(Path(mc),source)
        self.assertEqual(len(data),1224)
        word=lambda off:int.from_bytes(data[off-mod.FILE_START:off-mod.FILE_START+4],'little')
        self.assertEqual(word(0x39920),0x9200c012) # memw_locked(r0)
        self.assertEqual(word(0x39930),0xa0a0d200) # checked memw_locked(r0,p0)=r18
        self.assertEqual(word(0x39944),0x529f4000) # packet-local jumpr lr
        self.assertEqual(word(0x3994c),0x529f4000) # failure-path return
        # Four hardware IMASK selectors, S57:56, and exact dealloc_return.
        self.assertEqual(word(0x3997c),0x6f000000|(56<<16)|(3<<14)|18)
        self.assertEqual(word(0x399b8),0x66000000|(1<<16)|(3<<14)|28)
        self.assertEqual(word(0x399c4),0x66000000|(1<<16)|(3<<14)|28)
        self.assertEqual(word(0x399d0),0x66000000|(1<<16)|(3<<14)|28)
        self.assertEqual(word(0x399dc),0x66000000|(1<<16)|(3<<14)|28)
        self.assertEqual(word(0x39a00),0x62000000|((19+32)<<16)|(1<<14)|4)
        self.assertEqual(word(0x39a08),0x961ec01e)
        # Source-authored exception-frame republication: paired memd copy,
        # control registers and system register classes with parse markers.
        self.assertEqual(word(0x39a0c),0x91c040e2)  # memd(r0+#56)
        self.assertEqual(word(0x39a2c),0x6a000000 | (4<<16) | (1<<14) | 2)  # P3:0
        self.assertEqual(word(0x39a34),0x6a000000 | (8<<16) | (1<<14) | 3)  # USR
        self.assertEqual(word(0x39a94),0x6e800000 | (7<<16) | (1<<14) | 2)  # CCR
        self.assertEqual(word(0x39aac),0x68000000 | (16<<16) | (3<<14) | 2) # C17:16
        for a,sys,r in [(0x39ab4,6,3),(0x39ab8,3,2),
                        (0x39ac0,9,2),(0x39ac4,2,3)]:
            self.assertEqual(word(a),0x6e800000 | (sys<<16) | (3<<14) | r)
        self.assertEqual(word(0x39ad0),0x529fc000)  # independent jumpr lr packet
        # SGP0 swap, NMI dispatch, and V60 GP-relative load class.
        self.assertEqual(word(0x39ad4),0x65000000|(9<<16)|(3<<14))
        self.assertEqual(word(0x39ae0),0x65000000|(9<<16)|(3<<14))
        self.assertEqual(word(0x39aec)>>16,0x4980)
        self.assertEqual(word(0x39b08),0x64000000|((0x60|1)<<16)|(3<<14)|0x40)
        self.assertEqual(word(0x39b0c),0x529fc000)
        # Two nested hardware loops: loop1 enters the loop0 setup packet;
        # loop0 scans four mask lanes, closed by the three-NOP endloop01.
        self.assertEqual(word(0x39b20)>>16,0x4980)
        self.assertEqual(word(0x39b24),0x6024c008)
        self.assertEqual(word(0x39b28),0x6900c028)
        self.assertEqual(word(0x39b68),0x7f00c000)
        self.assertEqual(word(0x39b6c),0x529fc000)
        # MODECTL bounded poll: decrement first, conditionally retry, then
        # mask the controller GP-relative status after extracting two fields.
        self.assertEqual(word(0x39b80),0xbfe47fe4)
        self.assertEqual(word(0x39b84),0x1044c008)
        self.assertEqual(word(0x39ba0)>>16,0x4980)
        self.assertEqual(word(0x39bbc),0x529fc000)
        self.assertEqual(word(0x39bc0),0x6c200080|(3<<14)) # K0 unlock
        for address in [0x39bc8,0x39bd4,0x39be0,0x39bec,0x39c1c]:
            self.assertEqual(word(address)>>16,0x4980) # independent V60 GP class
        self.assertEqual(word(0x39c14),0x3f40c0d5) # frame status clrbit(21)
        # Protected frame register restoration and lock-mask updates.
        self.assertEqual(word(0x39c24),0x6e800000 | (0<<16) | (3<<14) | 29) # SGP0
        self.assertEqual(word(0x39c64),0x67000000 | (9<<16) | (3<<14) | 6) # SSR
        self.assertEqual(word(0x39c78),0x75094000) # predicate before branch
        self.assertEqual(word(0x39c7c),0x5cff68fe)
        self.assertEqual(word(0x39ca8)>>16,0x4980)
        self.assertEqual(word(0x39cb4),0x75094010)
        self.assertEqual(word(0x39cb8),0x5c005812)
        self.assertEqual(word(0x39cc0)>>16,0x4880) # generic GP store class
        self.assertEqual(word(0x39ce0)>>16,0x48a0) # .new GP store class
        self.assertEqual(word(0x39cf8),0x63000000|((8+32)<<16)|(3<<14)|16)
        self.assertEqual(word(0x39d00),0x57e0c000) # RTE
        self.assertEqual(word(0x39d04),0x529fc000) # alternate jumpr lr
        # Full controller state republished to a client frame; all original
        # packet-local data dependencies remain source-assembled.
        self.assertEqual(word(0x39d08),0x9180c001)
        self.assertEqual(word(0x39d6c),0x68000000 | (6<<16) | (1<<14) | 2)
        self.assertEqual(word(0x39da4),0x68000000 | (16<<16) | (3<<14) | 2)
        self.assertEqual(word(0x39de4),0x529fc000)
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),data)

if __name__=='__main__':unittest.main()
