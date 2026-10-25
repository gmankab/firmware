# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 cache/tag bank instruction encodings, ADSP [0x661c,0x6734)."""
import importlib.util
import os
from pathlib import Path
import struct
import sys
import unittest
from unittest.mock import patch

TOOLS=Path(__file__).resolve().parents[1]/'tools'
sys.path.insert(0,str(TOOLS))
import adsp_hexagon_v66 as v66
SPEC=importlib.util.spec_from_file_location('agent20_07_adsp_rwe',TOOLS/'adsp-agent20-rwe.py')
assert SPEC and SPEC.loader
MODULE=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

class CacheDiagnosticsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.llvm=os.getenv('HEXAGON_LLVM_MC')
        cls.reference=os.getenv('NABU_ADSP_REFERENCE')

    def test_typed_instruction_cache_banks(self):
        self.assertEqual(v66.read_instruction_cache_word(1,10,v66.InstructionCacheRead.TAG),0x55eac001)
        self.assertEqual(v66.read_instruction_cache_word(1,10,v66.InstructionCacheRead.DATA),0x55aac001)
        for dst,src,bank in [(32,10,v66.InstructionCacheRead.TAG),(-1,10,v66.InstructionCacheRead.DATA),
                             (1,32,v66.InstructionCacheRead.TAG),(1,-1,v66.InstructionCacheRead.DATA),
                             (1,10,'tag'),(1,10,0x55e0c000)]:
            with self.subTest(dst=dst,src=src,bank=bank),self.assertRaises(v66.HexagonEncodingError):
                v66.read_instruction_cache_word(dst,src,bank)
        self.assertEqual(v66.ControlRegister.LC0,1)
        self.assertEqual(v66.ControlRegister.LC1,3)

    def test_source_built_cache_diagnostics_matches_reference(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC not set')
        original=MODULE.cache_diagnostics(self.llvm,MODULE.baseline())
        self.assertEqual(len(original),0x118)
        target={0x38:0x6e92c000,0x40:0x6700c012,0x54:0x6a03c00a,0x5c:0x6a01c00a,
                0x68:0x55eac001,0x7c:0x55aac001,0xdc:0x6e92c001,0xe4:0x6701c012}
        for offset,word in target.items():
            self.assertEqual(struct.unpack_from('<I',original,offset)[0],word)
        self.assertEqual(struct.unpack_from('<I',original,0xd8)[0],0x7f00c000)
        # The loop-end NOP must not be mistaken for an unfilled privileged slot.
        self.assertEqual(original[0xd0:0xdc],struct.pack('<III',0x7f008000,0x7f008000,0x7f00c000))
        if self.reference:
            reference=Path(self.reference).read_bytes()
            self.assertEqual(reference[0x661c:0x6734],original)
            MODULE.verify(reference,MODULE.render(self.llvm))

    def test_moved_cache_marker_is_fatal(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC not set')
        old=MODULE.baseline()
        body=bytearray(old.assemble(self.llvm,MODULE.CACHE_DIAGNOSTICS_SOURCE,0x118))
        body[0x68:0x6c]=struct.pack('<I',0x7f00c001)
        fake=type('Fake',(),{'assemble':staticmethod(lambda *_:bytes(body)),
                              'AdspError':ValueError})()
        with self.assertRaisesRegex(ValueError,'marker moved'):
            MODULE.cache_diagnostics(self.llvm,fake)

    def test_nested_loop_terminal_slot_is_strict(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC not set')
        old=MODULE.baseline()
        body=bytearray(old.assemble(self.llvm,MODULE.CACHE_DIAGNOSTICS_SOURCE,0x118))
        body[0xd8:0xdc]=struct.pack('<I',0x7f00c001)
        fake=type('Fake',(),{'assemble':staticmethod(lambda *_:bytes(body)),
                              'AdspError':ValueError})()
        with self.assertRaisesRegex(ValueError,'terminal nop changed'):
            MODULE.cache_diagnostics(self.llvm,fake)

if __name__=='__main__':unittest.main()
