# SPDX-License-Identifier: GPL-3.0-only
"""ADSP 0x6c10 TLB restore: independently source-assembled V66 packet checks."""
from __future__ import annotations
import importlib.util, os, struct, sys, unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools'
sys.path.insert(0,str(TOOLS))
spec=importlib.util.spec_from_file_location('rwe',TOOLS/'adsp-agent20-rwe.py')
assert spec and spec.loader
rwe=importlib.util.module_from_spec(spec)
spec.loader.exec_module(rwe)
from adsp_hexagon_v66 import tlb_write_pair_r3_r2_index_r0

class TlbRestoreTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.llvm=os.getenv('HEXAGON_LLVM_MC')
        cls.original=os.getenv('NABU_ADSP_REFERENCE')

    def test_guarded_tlb_encoder(self):
        self.assertEqual(tlb_write_pair_r3_r2_index_r0(),0x6c02c000)
        self.assertEqual(rwe.TLB_RESTORE_SIZE,0x20)
        self.assertEqual(rwe.FILE_END,0x6c30)

    def test_restoration_loop_packetization(self):
        if not self.llvm:self.skipTest('HEXAGON_LLVM_MC required')
        emitter=rwe.baseline()
        original=emitter.assemble(self.llvm,rwe.TLB_RESTORE_SOURCE,0x20)
        self.assertEqual(struct.unpack_from('<I',original,0x10)[0],0x7f00c000)
        self.assertEqual(struct.unpack_from('<I',original,0x14)[0],0x147064fc)
        output=rwe.tlb_restore(self.llvm,emitter)
        self.assertEqual(len(output),0x20)
        self.assertEqual(struct.unpack_from('<I',output,0x10)[0],tlb_write_pair_r3_r2_index_r0())
        if self.original:
            ref=Path(self.original).read_bytes()
            self.assertEqual(output,ref[0x6c10:0x6c30])
            self.assertEqual(rwe.render(self.llvm),ref[0x6030:0x6c30])
