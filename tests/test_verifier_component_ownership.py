# SPDX-License-Identifier: GPL-3.0-only
"""Verify component-owned Python tools retain their original bytes and imports."""
import hashlib
import json
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]


class VerifierComponentOwnershipTests(unittest.TestCase):
    def test_relocated_verifier_byte_integrity(self):
        for component, ledger, key, count in (
            ('novatek', 'novatek-verifier-relocation.json', 'source_sha256', 12),
            ('cdsp', 'cdsp-helper-relocation.json', 'sha256', 15),
        ):
            metadata = json.loads((ROOT/'docs/maintenance'/ledger).read_text())
            self.assertEqual(metadata['moved_files'], count)
            self.assertEqual(len(metadata[key]), count)
            for filename, expected in metadata[key].items():
                with self.subTest(component=component, filename=filename):
                    path = ROOT/'tools'/component/filename
                    self.assertTrue(path.is_file())
                    self.assertFalse((ROOT/'tools'/filename).exists())
                    digest = hashlib.sha256(path.read_bytes()).hexdigest()
                    self.assertEqual(digest, expected)

    def test_cdsp_sibling_helper_imports_resolve(self):
        component = ROOT/'tools/cdsp'
        ref = re.compile(r'with_name\([\'\"](verify-cdsp-[a-z0-9-]+\.py)[\'\"]\)')
        count = 0
        for script in component.glob('*.py'):
            # These scripts are small and don't contain large firmware arrays.
            self.assertLess(script.stat().st_size, 100_000)
            for basename in ref.findall(script.read_text()):
                with self.subTest(source=script.name, dependency=basename):
                    self.assertTrue((component/basename).is_file())
                count += 1
        self.assertGreater(count, 50)


if __name__ == '__main__':
    unittest.main()
