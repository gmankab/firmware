# SPDX-License-Identifier: GPL-3.0-only
"""Regression tests for shared ELF/MBN build and analysis tool ownership."""
from pathlib import Path
import hashlib
import json
import unittest

ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / 'docs/maintenance/common-tool-relocation.json'


class CommonToolOwnershipTests(unittest.TestCase):
    def test_all_five_tools_are_in_component_directories(self):
        moves = json.loads(LEDGER.read_text())['moves']
        self.assertEqual(len(moves), 5)
        self.assertEqual(len({entry['after'] for entry in moves}), 5)
        for item in moves:
            with self.subTest(script=item['after']):
                self.assertFalse((ROOT / item['before']).exists())
                installed = ROOT / item['after']
                self.assertTrue(installed.is_file())
                self.assertEqual(hashlib.sha256(installed.read_bytes()).hexdigest(),
                                 item['installed_sha256'])
                self.assertTrue(item['after'].startswith(('tools/analysis/','tools/build/')))

    def test_reproducibility_harness_uses_committed_new_location(self):
        code = (ROOT / 'tools/analysis/mbn-reproducibility.py').read_text()
        self.assertIn('HEAD:tools/analysis/mbn-reproducibility.py', code)
        self.assertIn('root / "tools/analysis/mbn-reproducibility.py"', code)
        self.assertIn('Path(__file__).resolve().parents[2]', code)

    def test_elf_metadata_generator_uses_correct_provenance_header(self):
        source = (ROOT / 'tools/build/generate-elf-specs.py').read_text()
        generated = (ROOT / 'crates/firmware-tools/src/devices/xiaomi_nabu/formats/elf_specs.rs').read_text()
        self.assertIn('tools/build/generate-elf-specs.py', source)
        self.assertIn('tools/build/generate-elf-specs.py', generated)


if __name__ == '__main__':
    unittest.main()
