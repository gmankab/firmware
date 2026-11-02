# SPDX-License-Identifier: GPL-3.0-only
"""Check assembly/data migration did not edit source bytes or leave stale paths."""
from __future__ import annotations

from hashlib import sha256
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / 'docs/maintenance/assembly-paths.json'


class FirmwareAssemblySourceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data = json.loads(MANIFEST.read_text())

    def test_all_moved_assembly_and_data_exist_with_original_checksums(self):
        self.assertEqual(self.data['count'], 141)
        self.assertEqual(len(self.data['paths']), 141)
        self.assertEqual(len(set(self.data['paths'].values())), 141)
        for old, new in self.data['paths'].items():
            with self.subTest(old=old):
                self.assertFalse((ROOT / old).exists())
                data = (ROOT / new).read_bytes()
                self.assertEqual(sha256(data).hexdigest(), self.data['sha256'][new])

    def test_assembly_is_outside_rust_module_directory(self):
        root = ROOT / 'crates/firmware-tools/src/devices/xiaomi_nabu'
        self.assertEqual(list(root.glob('*.s')), [])
        self.assertEqual(list(root.glob('*.tsv')), [])
        self.assertEqual(len(list((ROOT / 'sources/xiaomi-nabu/modem').glob('*.s'))), 140)
        self.assertTrue((root / 'novatek/dlm/novatek_dlm_seed_hashes.tsv').is_file())


if __name__ == '__main__':
    unittest.main()
