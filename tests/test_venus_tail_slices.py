# SPDX-License-Identifier: GPL-3.0-only
"""Byte-for-byte source integrity for function-bounded Venus tail sections.

No private firmware is needed: the manifest proves we preserved the original
Rust source exactly, and runtime Rust unit tests check the emitted ARM words.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / 'crates/venus-tail/src'
PARTS = SRC / 'parts'


class VenusTailSourceLayoutTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.index = json.loads((PARTS / 'manifest.json').read_text())
        cls.main = (SRC / 'venus_tail_program.rs').read_bytes()

    def test_all_section_checksums_and_original_file_are_exact(self):
        idx = self.index
        anchor = b'\n// Firmware routines and literal pools remain in their original item namespace.'
        self.assertIn(anchor, self.main)
        header = self.main.split(anchor, 1)[0]
        self.assertEqual(hashlib.sha256(header).hexdigest(), idx['main_header_sha256'])
        reconstructed = bytearray(header)
        for entry in idx['sections']:
            with self.subTest(path=entry['path']):
                contents = (SRC / entry['path']).read_bytes()
                self.assertEqual(len(contents), entry['bytes'])
                self.assertEqual(hashlib.sha256(contents).hexdigest(), entry['sha256'])
                reconstructed.extend(contents)
        self.assertEqual(len(reconstructed), idx['original_source_bytes'])
        self.assertEqual(hashlib.sha256(reconstructed).hexdigest(), idx['source_sha256'])

    def test_expansion_order_and_section_count_are_fixed(self):
        names = re.findall(rb'include!\("(parts/section_\d{3}\.rs)"\);', self.main)
        expected = [entry['path'].encode() for entry in self.index['sections']]
        self.assertEqual(names, expected)
        self.assertEqual(len(names), self.index['section_count'])

    def test_no_slice_exceeds_original_function_boundary_budget(self):
        lengths = [len((SRC / x['path']).read_text().splitlines()) for x in self.index['sections']]
        self.assertEqual(max(lengths), self.index['maximum_section_lines'])
        self.assertLessEqual(max(lengths), 1700)
        self.assertEqual(self.index['source_lines'], 63602)


if __name__ == '__main__':
    unittest.main()
