# SPDX-License-Identifier: GPL-3.0-only
"""The shared Hexagon encoder and ADSP emitters belong to tools/adsp."""
from pathlib import Path
import hashlib
import json
import unittest

ROOT = Path(__file__).resolve().parents[1]


class AdspToolOwnershipTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.moves = json.loads((ROOT / 'docs/maintenance/adsp-tool-ownership.json').read_text())['moved']

    def test_three_emitters_are_colocated_and_hash_accounted(self):
        self.assertEqual(len(self.moves), 3)
        for move in self.moves:
            with self.subTest(move=move['target']):
                self.assertFalse((ROOT / move['source']).exists())
                path=ROOT / move['target']
                self.assertEqual(path.parent, ROOT / 'tools/adsp')
                self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(),move['installed_sha256'])

    def test_root_of_sources_resolves_from_subdirectory(self):
        entry=(ROOT/'tools/adsp/emit-adsp-entry.py').read_text()
        late=(ROOT/'tools/adsp/emit-adsp-late.py').read_text()
        self.assertIn('parents[2]',entry)
        self.assertIn('parents[2]',late)
        self.assertIn("TOOLS = ROOT / 'tools/adsp'",late)

    def test_indirect_assembly_imports_use_sibling_location(self):
        part=(ROOT/'tools/adsp/rwe-next-packed-offsets.py').read_text()
        scanner=(ROOT/'tools/adsp/rwe-next-controller-slot-scan-complete.py').read_text()
        self.assertIn(".parent/'emit-adsp-entry.py'",part)
        self.assertIn('T/name',scanner)
        self.assertNotIn("T.parent if name=='emit-adsp-entry.py'",scanner)


if __name__=='__main__':unittest.main()
