# SPDX-License-Identifier: GPL-3.0-only
"""Protect firmware-oriented file ownership and migrated source locations."""
from __future__ import annotations
import json
from pathlib import Path
import subprocess
import tomllib
import unittest

ROOT=Path(__file__).resolve().parents[1]

class RepositoryLayoutTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.paths=set(subprocess.check_output(['git','ls-files'],cwd=ROOT,text=True).splitlines())
        cls.plan=json.loads((ROOT/'docs/maintenance/path-migrations.json').read_text())

    def test_no_active_contributor_specific_paths(self):
        paths=[p for p in self.paths if 'agent' in p.lower()]
        self.assertEqual(paths,[])

    def test_available_migrations_one_to_one(self):
        changed=self.plan['implemented_path_renames']
        self.assertEqual(len(changed),1029)
        self.assertEqual(len(changed),len(set(changed.values())))
        retired=self.plan.get('retired_historical_files',{})
        self.assertTrue(set(retired).issubset(set(changed.values())))
        for path, sha256 in retired.items():
            with self.subTest(archived=path):
                self.assertTrue(path.startswith('docs/history/phase-'))
                self.assertTrue(path.endswith('/progress.md'))
                self.assertEqual(len(sha256),64)
                self.assertTrue(all(ch in '0123456789abcdef' for ch in sha256))
                self.assertNotIn(path,self.paths)
                self.assertFalse((ROOT/path).exists())
        for old,new in changed.items():
            with self.subTest(old=old):
                if new not in retired:
                    self.assertIn(new,self.paths)
                    self.assertTrue((ROOT/new).is_file())
                self.assertNotIn(old,self.paths)

    def test_missing_sources_are_not_fabricated(self):
        self.assertEqual(len(self.plan['unavailable_paths']),16)
        for old in self.plan['unavailable_paths']:
            self.assertNotIn(old,self.paths)

    def test_crate_ownership(self):
        workspace=tomllib.loads((ROOT/'Cargo.toml').read_text())
        self.assertIn('crates/venus-controller',workspace['workspace']['members'])
        self.assertIn('crates/venus-tail',workspace['workspace']['members'])
        mod=(ROOT/'crates/venus-controller/src/lib.rs').read_text()
        self.assertEqual(mod.count('pub mod venus_controller_'),55)
        for line in mod.splitlines():
            if line.startswith('#[path = '):
                rel=line.split('"')[1]
                self.assertTrue((ROOT/'crates/venus-controller/src'/rel).exists())
        self.assertTrue((ROOT/'crates/venus-tail/src/venus_tail_program.rs').exists())
        self.assertIn('pub mod venus_tail_program;', (ROOT/'crates/venus-tail/src/lib.rs').read_text())

if __name__=='__main__':unittest.main()
