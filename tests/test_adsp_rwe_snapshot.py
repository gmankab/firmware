# SPDX-License-Identifier: GPL-3.0-only
"""Codec controller snapshot regression tests, ADSP file [0x6568,0x65f4)."""
import importlib.util
import os
from pathlib import Path
import struct
import sys
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[1]
TOOLS=ROOT/'tools/adsp'
sys.path.insert(0, str(TOOLS.parent))
sys.path.insert(0,str(TOOLS))
from adsp_hexagon_v66 import (SystemRegister, read_system_register,
                              write_system_register, packet_continuation)
SPEC=importlib.util.spec_from_file_location('agent20_adsp_rwe',TOOLS/'rwe.py')
assert SPEC and SPEC.loader
MODULE=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class SnapshotTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.llvm=os.environ.get('HEXAGON_LLVM_MC')
        cls.reference=os.environ.get('NABU_ADSP_REFERENCE')

    def test_snapshot_bounds(self):
        self.assertEqual(MODULE.REGISTER_SNAPSHOT_SIZE,0x8c)
        self.assertEqual(MODULE.REGISTER_SNAPSHOT_END,0x65f4)
        self.assertGreaterEqual(MODULE.SIZE,0x65f4-0x6030)

    def test_privileged_transfer_register_fields(self):
        words=(read_system_register(1,SystemRegister.SGP0),
               read_system_register(9,SystemRegister.SGP0),
               write_system_register(SystemRegister.ELR,4),
               packet_continuation(read_system_register(3,SystemRegister.HTID)))
        self.assertEqual(words,(0x6e80c001,0x6e80c009,0x6704c003,0x6e884003))
        self.assertEqual((words[-1]>>14)&3,1) # continuation in packet

    def test_snapshot_source_without_reference_opcodes(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC not set')
        b=MODULE.register_snapshot(self.llvm,MODULE.baseline())
        self.assertEqual(len(b),0x8c)
        for offset,val in [(0x18,0x6e80c001),(0x30,0x6e80c009),
                           (0x40,0x6704c003),(0x58,0x6e884003)]:
            self.assertEqual(struct.unpack_from('<I',b,offset)[0],val)
        self.assertNotIn(b'\x00\xc0\x00\x7f',
                         [b[i:i+4] for i in range(0,len(b),4)])
        if self.reference:
            ref=Path(self.reference).read_bytes()
            MODULE.verify(ref,MODULE.render(self.llvm))
            self.assertEqual(ref[0x6568:0x65f4],b)

    def test_moved_register_marker_rejected(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC not set')
        previous=MODULE.baseline()
        original=bytearray(previous.assemble(self.llvm,MODULE.REGISTER_SNAPSHOT_SOURCE,0x8c))
        original[0x40:0x44]=b'\x01\xc0\x00\x7f'
        dummy=type('Dummy',(),{'assemble':staticmethod(lambda *_:bytes(original)),
                               'AdspError':ValueError})()
        with self.assertRaisesRegex(ValueError,'marker moved'):
            MODULE.register_snapshot(self.llvm,dummy)

    def test_wrong_firmware_revision_rejected(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC not set')
        with self.assertRaisesRegex(ValueError,'SHA-256 mismatch'):
            MODULE.verify(b'wrong reference',MODULE.render(self.llvm))


if __name__=='__main__':unittest.main()
