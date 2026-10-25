# SPDX-License-Identifier: GPL-3.0-only
"""Disjoint ADSP V66 sparse-layout source assembly and gap accounting."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools'
spec=importlib.util.spec_from_file_location('rwe_sparse',TOOLS/'adsp-agent20-rwe-sparse.py')
assert spec and spec.loader
sparse=importlib.util.module_from_spec(spec);spec.loader.exec_module(sparse)
class SparseRwe(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC')
        cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_file_layout_is_explicit_and_disjoint(self):
        self.assertEqual((sparse.FILE_START,sparse.FILE_END,sparse.SIZE),(0x10330,0x1034c,28))
        self.assertEqual(sparse.FILL_END-sparse.FILL_START,0xff20-0x6c6c)
        self.assertEqual(sparse.UNKNOWN_RECORD_END-sparse.UNKNOWN_RECORD_START,0x90)
        self.assertEqual(sparse.FILL2_END-sparse.FILL2_START,0x380)
    def test_source_packets_and_strict_reference(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC required')
        result=sparse.render(self.mc)
        self.assertEqual(len(result),28)
        # Function return is not a packet boundary: packet continues with
        # extended GP load from the next function, so no artificial split.
        self.assertEqual((struct.unpack_from('<I',result,0x10)[0]>>14)&3,1)
        self.assertEqual((struct.unpack_from('<I',result,0x18)[0]>>14)&3,3)
        if self.ref:
            original=Path(self.ref).read_bytes()
            sparse.verify(original,result)
    def test_reject_reference_mutation(self):
        if not self.mc or not self.ref:self.skipTest('reference and LLVM6 required')
        b=sparse.render(self.mc)
        ref=bytearray(Path(self.ref).read_bytes())
        ref[sparse.FILE_START]^=1
        with self.assertRaises(ValueError):sparse.verify(bytes(ref),b)
