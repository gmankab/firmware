# SPDX-License-Identifier: GPL-3.0-only
"""V66 controller record snapshot/source assembly, branch and checksum tests."""
from __future__ import annotations
import importlib.util
import os
import struct
import unittest
from pathlib import Path

T = Path(__file__).resolve().parents[1]/'tools/adsp'


def load(name: str):
    spec = importlib.util.spec_from_file_location(name.replace('-', '_'), T / name)
    assert spec and spec.loader
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod

snapshot = load('rwe-next-controller-record-snapshot.py')
prior = load('rwe-next-controller-record-reset.py')


class AdspRecordSnapshot(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc = os.getenv('HEXAGON_LLVM_MC')
        cls.reference = os.getenv('NABU_ADSP_REFERENCE')

    def test_source_boundary_and_named_operations(self):
        self.assertEqual((snapshot.START, snapshot.END, snapshot.SIZE),
                         (0x13390, 0x13478, 232))
        self.assertEqual(prior.END, snapshot.START)
        assembly = snapshot.SOURCE.read_text()
        for element in ('search_loop:', 'found_record:', 'inspect_flags:',
                        'check_clear:', 'return_record:', 'dealloc_return',
                        'if(p0.new) r4=memw(r18+#256)',
                        'r17:16=combine(r2,r3)'):
            self.assertIn(element, assembly)
        self.assertNotIn('.incbin', assembly)
        self.assertNotIn('.word', assembly)
        self.assertEqual(snapshot.PACKET_CONTINUATION_AT, 0x133dc)
        self.assertEqual(snapshot.SGP0_FILE_OFFSET, 0x1339c)

    def test_independent_assembly_is_exact(self):
        if not self.mc or not self.reference:
            self.skipTest('LLVM6 and pinned ADSP reference needed')
        ref = Path(self.reference).read_bytes()
        built = snapshot.render(self.mc)
        snapshot.verify(ref, built)
        self.assertEqual(built[12:16], struct.pack('<I', 0x6e80c002))
        self.assertEqual((struct.unpack_from('<I', built, 0x133dc - snapshot.START)[0] >> 14) & 3, 1)
        self.assertEqual(built[0x133e4-snapshot.START:0x133e8-snapshot.START],
                         ref[0x133e4:0x133e8])
        self.assertEqual(built[-4:], ref[snapshot.END-4:snapshot.END])

    def test_revision_and_corruption_are_rejected(self):
        if not self.mc or not self.reference:
            self.skipTest('LLVM6 and pinned ADSP reference needed')
        ref = Path(self.reference).read_bytes()
        code = snapshot.render(self.mc)
        bad = bytearray(ref)
        bad[-1] ^= 1
        with self.assertRaisesRegex(ValueError, 'revision'):
            snapshot.verify(bad, code)
        changed = bytearray(code)
        changed[44] ^= 1
        with self.assertRaisesRegex(ValueError, 'differs'):
            snapshot.verify(ref, changed)


if __name__ == '__main__':
    unittest.main()
