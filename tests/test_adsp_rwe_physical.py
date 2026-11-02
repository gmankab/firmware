# SPDX-License-Identifier: GPL-3.0-only
"""ADSP V66 cache copy and physical controller, file [0x6734,0x67e8)."""
import importlib.util
import os
from pathlib import Path
import struct
import sys
import unittest
from unittest.mock import patch
TOOLS=Path(__file__).resolve().parents[1]/'tools/adsp'
sys.path.insert(0, str(TOOLS.parent))
sys.path.insert(0,str(TOOLS))
import adsp_hexagon_v66 as v66
SPEC=importlib.util.spec_from_file_location('source_rwe',TOOLS/'rwe.py')
assert SPEC and SPEC.loader
SRC=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SRC)

class PhysicalControllerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.llvm=os.getenv('HEXAGON_LLVM_MC')
        cls.ref=os.getenv('NABU_ADSP_REFERENCE')

    def test_layout_and_length(self):
        self.assertEqual((SRC.CACHE_COPY_SIZE,SRC.PHYSICAL_CONTROLLER_SIZE,SRC.PHYSICAL_SYNC_SIZE),(56,60,64))
        self.assertGreaterEqual(SRC.FILE_END,0x67e8)
        self.assertGreaterEqual(SRC.SIZE,1976)

    def test_bounded_physical_fields(self):
        self.assertEqual(v66.physical_word_load(0,2,0),0x9202e000)
        for args in [(32,2,0),(0,-1,0),(0,2,32),(0,2,'0')]:
            with self.subTest(args=args),self.assertRaises(v66.HexagonEncodingError):
                v66.physical_word_load(*args)
        self.assertEqual(v66.packet_continuation(v66.read_system_register(0,v66.SystemRegister.CFGBASE)),0x6e9b4000)

    def test_exact_three_source_fragments(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC not set')
        orig=SRC.baseline()
        code=orig.assemble(self.llvm,SRC.CACHE_COPY_SOURCE,SRC.CACHE_COPY_SIZE)
        phys=SRC.physical_controller(self.llvm,orig)
        wrap=orig.assemble(self.llvm,SRC.PHYSICAL_SYNC_SOURCE,SRC.PHYSICAL_SYNC_SIZE)
        self.assertEqual(len(code+phys+wrap),180)
        self.assertEqual(struct.unpack_from('<I',code,0x30)[0],0xa001c000) # dccleana(r1)
        for at,val in [(0x00,0x6e9b4000),(0x14,0x9202e000),(0x28,0x9202e000)]:
            self.assertEqual(struct.unpack_from('<I',phys,at)[0],val)
        if self.ref:
            ref=Path(self.ref).read_bytes()
            self.assertEqual(code+phys+wrap,ref[0x6734:0x67e8])
            SRC.verify(ref,SRC.render(self.llvm))

    def test_physical_slot_checks_reject_relocation(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC not set')
        previous=SRC.baseline()
        b=bytearray(previous.assemble(self.llvm,SRC.PHYSICAL_CONTROLLER_SOURCE,60))
        b[0x14:0x18]=struct.pack('<I',0x7f00c001)
        fake=type('Fake',(),{'assemble':staticmethod(lambda *_:bytes(b)),
                             'AdspError':ValueError})()
        with self.assertRaisesRegex(ValueError,'sentinel moved'):
            SRC.physical_controller(self.llvm,fake)

if __name__=='__main__':unittest.main()
