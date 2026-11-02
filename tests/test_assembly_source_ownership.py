# SPDX-License-Identifier: GPL-3.0-only
"""Ensure relocated ADSP/CDSP ISA sources remain byte-for-byte untouched."""
import hashlib
import json
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]


def file_sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as source:
        while block := source.read(64 * 1024):
            h.update(block)
    return h.hexdigest()


class AssemblySourceOwnershipTests(unittest.TestCase):
    def test_tool_folders_contain_no_hexagon_assembly(self):
        self.assertFalse([p for p in (ROOT / 'tools').rglob('*')
                          if p.is_file() and p.suffix.lower() == '.s'])

    def test_migration_hashes_and_target_paths(self):
        tracked = set(subprocess.check_output(['git', 'ls-files'], cwd=ROOT,
                                              text=True).splitlines())
        manifest = json.loads((ROOT / 'docs/maintenance/path-migrations.json').read_text())
        changed = manifest['implemented_path_renames']
        for component, filename in (('ADSP', 'adsp-assembly-relocation.json'),
                                    ('CDSP', 'cdsp-assembly-relocation.json')):
            record = json.loads((ROOT / 'docs/maintenance' / filename).read_text())
            self.assertEqual(len(record['source_sha256']), record['moved_files'])
            for basename, expected in record['source_sha256'].items():
                with self.subTest(component=component, file=basename):
                    dest = record['destination'] + '/' + basename
                    self.assertIn(dest, tracked)
                    self.assertEqual(file_sha256(ROOT / dest), expected)
                    self.assertNotIn('tools/' + component.lower() + '/' + basename, tracked)
            self.assertTrue(all((ROOT / path).exists() for path in changed.values()))


if __name__ == '__main__':
    unittest.main()
