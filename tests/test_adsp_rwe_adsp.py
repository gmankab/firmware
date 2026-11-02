# SPDX-License-Identifier: GPL-3.0-only
"""Source-level tests for Agent20-07 ADSP TLB-controller tail."""
import importlib.util
import os
from pathlib import Path
import struct
import sys
import unittest
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parents[1] / 'tools/adsp'
sys.path.insert(0, str(TOOLS.parent))
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

import adsp_hexagon_v66 as isa

spec = importlib.util.spec_from_file_location('agent20_adsp07', TOOLS / 'entry.py')
assert spec is not None and spec.loader is not None
emitter = importlib.util.module_from_spec(spec)
spec.loader.exec_module(emitter)


class AdspTailTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.llvm = os.environ.get('HEXAGON_LLVM_MC')
        cls.reference = os.environ.get('NABU_ADSP_REFERENCE')

    def test_checked_tlb_pair_encoding(self):
        self.assertEqual(isa.tlb_read_pair(2, 0), 0x6C40C002)
        # A different occurrence already used by Agent 04.
        self.assertEqual(isa.tlb_read_pair(4, 3), isa.tlb_read_pair_r5_r4_index_r3())
        for rd, src in ((1,0),(31,3),(-2,0),(32,0),(2,-1),(2,32)):
            with self.subTest(rd=rd,src=src), self.assertRaises(isa.HexagonEncodingError):
                isa.tlb_read_pair(rd,src)
        for rd,src in ((2.,0),(2,'r0')):
            with self.assertRaises(isa.HexagonEncodingError):
                isa.tlb_read_pair(rd,src)

    def test_metadata_constants_bound_to_first_text_segment(self):
        self.assertEqual(emitter.FILE_START, 0x51dc)
        self.assertEqual(emitter.FILE_END, 0x521c)
        self.assertEqual(emitter.SIZE, 64)
        self.assertEqual(emitter.PHYSICAL_START, 0x8be021dc)

    def test_compiles_fixed_v66_tlb_snapshot(self):
        if not self.llvm:
            self.skipTest('HEXAGON_LLVM_MC required')
        out=emitter.render(self.llvm)
        self.assertEqual(len(out),64)
        self.assertEqual(struct.unpack_from('<I',out,0x00)[0],
                         isa.read_system_register(23,isa.SystemRegister.S54))
        self.assertEqual(struct.unpack_from('<I',out,0x1c)[0],isa.tlb_read_pair(2,0))
        self.assertNotIn(struct.pack('<I',0x7F00C000),
                         [out[i:i+4] for i in range(0,len(out),4)])
        if self.reference:
            emitter.verify(Path(self.reference).read_bytes(),out)

    def test_invalid_reference_is_rejected(self):
        if not self.llvm:
            self.skipTest('HEXAGON_LLVM_MC required')
        with self.assertRaises(ValueError):
            emitter.verify(b'junk',emitter.render(self.llvm))

    def test_rejects_changed_sentinel(self):
        if not self.llvm:
            self.skipTest('HEXAGON_LLVM_MC required')
        real=emitter.load_baseline_emitter()
        original=real.assemble(self.llvm,emitter.SOURCE,emitter.SIZE)
        changed=bytearray(original)
        changed[0x1c:0x20]=struct.pack('<I',0x7F00C001)
        with patch.object(emitter,'load_baseline_emitter',return_value=type('Proxy',(),{
                'assemble':staticmethod(lambda *args: bytes(changed)),
                'AdspError':ValueError,
                })()):
            with self.assertRaisesRegex(ValueError,'sentinel moved'):
                emitter.render(self.llvm)


if __name__=='__main__':
    unittest.main()
