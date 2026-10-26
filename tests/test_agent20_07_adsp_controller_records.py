# SPDX-License-Identifier: GPL-3.0-only
"""Exact Hexagon instruction packets for first controller-record prefix."""
from __future__ import annotations
import importlib.util,os,sys,unittest,struct
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
p=T/'adsp-agent20-rwe-next-controller-records.py'
spec=importlib.util.spec_from_file_location('adsp_record_prefix',p)
assert spec and spec.loader
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class ControllerRecords(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_layout_classifies_gap_and_complete_function(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x10c40,0x10dc4,388))
        self.assertEqual(0x10c30-0x1084c,996)
        self.assertEqual(0x10c40-0x10c30,16)
    def test_controller_alloc_packet_and_relocations(self):
        if not self.mc:self.skipTest('LLVM6 source assembler required')
        built=m.render(self.mc)
        self.assertEqual(len(built),388)
        # First packet is the original Hexagon ALLOCFRAME duplex.
        self.assertEqual(struct.unpack_from('<I',built,0)[0],0xebf41c40)
        # Final ordinary branch resumes after the setup loop.
        self.assertEqual(struct.unpack_from('<I',built,0x6c)[0],0x5800c004)
        if self.ref:m.verify(Path(self.ref).read_bytes(),built)
    def test_pcyclelo_checked_field_and_p0_duplex_packet(self):
        if not self.mc:self.skipTest('LLVM6 required')
        from adsp_hexagon_v66 import (SystemRegister,read_system_register,
                                         packet_continuation)
        source=m.base().assemble(self.mc,m.SOURCE,m.SIZE)
        built=m.render(self.mc)
        p0=0x10d54-m.FILE_START
        counter=0x10d5c-m.FILE_START
        self.assertEqual(struct.unpack_from('<I',source,p0)[0],0x7504c000)
        self.assertEqual(struct.unpack_from('<I',built,p0)[0],
                         packet_continuation(0x7504c000))
        self.assertEqual(struct.unpack_from('<I',source,counter)[0],0x7f00c000)
        self.assertEqual(struct.unpack_from('<I',built,counter)[0],
                         read_system_register(4,SystemRegister.PCYCLELO))
        self.assertEqual(struct.unpack_from('<I',built,m.SIZE-8)[0],0x961ec01e)
        self.assertEqual(struct.unpack_from('<I',built,m.SIZE-4)[0],0x48003fc0)

    def test_strict_immutable_reference(self):
        if not(self.ref and self.mc):self.skipTest('Original ADSP & LLVM6 needed')
        orig=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(orig,b)
        b=bytearray(b);b[-1]^=1
        with self.assertRaisesRegex(ValueError,'differ'):
            m.verify(orig,b)
if __name__=='__main__':unittest.main()
