# SPDX-License-Identifier: GPL-3.0-only
"""Keep MBN provenance/signature utilities in their verification component."""
import hashlib
import json
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]


class MbnVerifierOwnershipTests(unittest.TestCase):
    def test_verifiers_and_revision_audit_live_under_verify(self):
        ledger=json.loads((ROOT/'docs/maintenance/mbn-verifier-relocation.json').read_text())
        self.assertEqual(ledger['moved_files'],3)
        for name,digest in ledger['installed_sha256'].items():
            with self.subTest(name=name):
                dest=ROOT/'tools/verify'/name
                self.assertTrue(dest.is_file())
                self.assertFalse((ROOT/'tools'/name).exists())
                self.assertEqual(hashlib.sha256(dest.read_bytes()).hexdigest(),digest)

    def test_audit_scripts_resolve_repo_not_tools_dir(self):
        for name in ('audit-nabu-mbn-sources.py','verify-mbn-hash-readiness.py'):
            script=ROOT/'tools/verify'/name
            with self.subTest(script=name):
                self.assertEqual(script.resolve().parents[2],ROOT)
                self.assertIn('Path(__file__).resolve().parents[2]',script.read_text())


if __name__=='__main__':unittest.main()
