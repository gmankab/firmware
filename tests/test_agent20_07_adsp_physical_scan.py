# SPDX-License-Identifier: GPL-3.0-only
"""ADSP four-iteration privileged physical scan [0x67e8,0x6818)."""
import importlib.util
import os
from pathlib import Path
import struct
import sys
import unittest
TOOLS=Path(__file__).resolve().parents[1]/'tools'
sys.path.insert(0,str(TOOLS))
SPEC=importlib.util.spec_from_file_location('snapshot',TOOLS/'adsp-agent20-rwe.py')
assert SPEC and SPEC.loader
MODULE=importlib.util.module_from_spec(SPEC);SPEC.loader.exec_module(MODULE)

class PhysicalScanTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_range(self):
        self.assertEqual(MODULE.PHYSICAL_SCAN_SIZE,48)
        self.assertGreaterEqual(MODULE.FILE_END,0x6818)
        self.assertGreaterEqual(MODULE.SIZE,2024)
    def test_rebuild_source(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC not set')
        fragment=MODULE.physical_scan(self.mc,MODULE.baseline())
        self.assertEqual(len(fragment),48)
        self.assertEqual(struct.unpack_from('<I',fragment,32)[0],MODULE.physical_word_load(3,0,2))
        if self.ref:
            ref=Path(self.ref).read_bytes()
            self.assertEqual(ref[0x67e8:0x6818],fragment)
            MODULE.verify(ref,MODULE.render(self.mc))
    def test_invalid_original_rejected(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC not set')
        with self.assertRaisesRegex(ValueError,'SHA-256 mismatch'):
            MODULE.verify(bytes(500),MODULE.render(self.mc))

if __name__=='__main__':unittest.main()
