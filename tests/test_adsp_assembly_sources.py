# SPDX-License-Identifier: GPL-3.0-only
"""ADSP source migration ledger and executable tool-path invariants."""
from __future__ import annotations
from pathlib import Path
import hashlib
import json
import unittest

ROOT = Path(__file__).resolve().parents[1]
MAP = ROOT / 'docs/maintenance/adsp-assembly-paths.json'


class ADSPAssemblySourceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.manifest = json.loads(MAP.read_text())

    def test_every_moved_assembly_source_preserves_bytes(self):
        m = self.manifest
        self.assertEqual(m['count'], 168)
        self.assertEqual(len(m['paths']), 168)
        self.assertEqual(len(set(m['paths'].values())), 168)
        for old, new in m['paths'].items():
            with self.subTest(old=old):
                self.assertFalse((ROOT / old).exists())
                self.assertEqual(hashlib.sha256((ROOT / new).read_bytes()).hexdigest(),
                                 m['sha256'][new])

    def test_assembly_source_is_not_in_executable_tool_root(self):
        self.assertEqual(list((ROOT / 'tools').glob('*.S')), [])
        self.assertEqual(len(list((ROOT / 'sources/xiaomi-nabu/adsp').glob('*.S'))), 168)
        text = (ROOT / 'tools/emit-adsp-late.py').read_text()
        self.assertIn("ADSP_ASM = ROOT / 'sources/xiaomi-nabu/adsp'", text)
        self.assertTrue((ROOT / 'tools/emit-adsp-entry.py').is_file())


if __name__ == '__main__':
    unittest.main()
