# SPDX-License-Identifier: GPL-3.0-only
"""Byte-for-byte source integrity for function-bounded Venus tail sections.

No private firmware is needed: the manifest records current Rust source identity
and historical source provenance separately. Runtime Rust unit tests check the
emitted ARM words.
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
        cls.header = cls.main.split(
            b'\n// Firmware routines and literal pools remain in their original item namespace.', 1
        )[0]
        cls.source = cls.header + b''.join(
            (SRC / entry['path']).read_bytes() for entry in cls.index['sections']
        )

    def test_all_section_checksums_and_current_source_are_exact(self):
        idx = self.index
        anchor = b'\n// Firmware routines and literal pools remain in their original item namespace.'
        self.assertIn(anchor, self.main)
        self.assertEqual(hashlib.sha256(self.header).hexdigest(), idx['main_header_sha256'])
        for entry in idx['sections']:
            with self.subTest(path=entry['path']):
                contents = (SRC / entry['path']).read_bytes()
                self.assertEqual(len(contents), entry['bytes'])
                self.assertEqual(hashlib.sha256(contents).hexdigest(), entry['sha256'])
        self.assertEqual(len(self.source), idx['source_bytes'])
        self.assertEqual(hashlib.sha256(self.source).hexdigest(), idx['source_sha256'])
        self.assertEqual(len(self.source.splitlines()), idx['source_lines'])

    def test_historical_source_provenance_before_reference_env_change(self):
        original = self.index['original_source']
        self.assertEqual(original['predates_commit'], 'e77edbd5ca79de8105d97ba207674d052d3cbe65')
        current_lookup = (
            b'std::env::var_os("NABU_VENUS_REFERENCE")'
            b'.or_else(|| std::env::var_os("AGENT20_06_REFERENCE_VENUS"))'
        )
        original_lookup = b'std::env::var_os("AGENT20_06_REFERENCE_VENUS")'
        self.assertEqual(self.source.count(current_lookup), 1)
        historical = self.source.replace(current_lookup, original_lookup, 1)
        self.assertEqual(len(historical), original['bytes'])
        self.assertEqual(hashlib.sha256(historical).hexdigest(), original['sha256'])
        self.assertEqual(len(historical.splitlines()), original['lines'])

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
