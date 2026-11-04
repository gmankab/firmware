# SPDX-License-Identifier: GPL-3.0-only
"""Device-neutral shell tooling must stay in the correct component tree."""
from pathlib import Path
import hashlib
import json
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / 'docs/maintenance/shell-tool-relocation.json'


class ToolOrganizationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.records = json.loads(LEDGER.read_text())['files']

    def test_shell_analysis_and_verifier_paths(self):
        self.assertEqual(len(self.records), 4)
        for item in self.records:
            with self.subTest(script=item['to']):
                new = ROOT / item['to']
                self.assertFalse((ROOT / item['from']).exists())
                self.assertTrue(new.is_file())
                self.assertEqual(hashlib.sha256(new.read_bytes()).hexdigest(),item['after_sha256'])
                self.assertTrue(item['to'].startswith(('tools/analysis/','tools/verify/')))

    def test_scripts_are_valid_bash_and_reject_missing_arguments(self):
        for item in self.records:
            p = ROOT / item['to']
            with self.subTest(script=item['to']):
                self.assertEqual(subprocess.run(['bash','-n',str(p)],capture_output=True).returncode,0)
                self.assertEqual(subprocess.run(['bash',str(p)],capture_output=True,timeout=3).returncode,2)

    def test_verifiers_resolve_workspace_from_another_directory(self):
        scripts = ('venus/verify-venus-controller-complete.sh', 'venus/venus-bridge-verify-full-gap.sh',
                   'halo/verify-halo-pm.sh')
        for name in scripts:
            with self.subTest(script=name):
                script = ROOT / 'tools' / name
                content = script.read_text()
                prelude = content.split('cd "$ROOT"', 1)[0]
                command = prelude.replace('${BASH_SOURCE[0]}', '$0') + '\nprintf "%s" "$ROOT"\n'
                result = subprocess.run(['bash', '-c', command, str(script), str(script)],
                                        cwd='/', capture_output=True, text=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(result.stdout, str(ROOT))

    def test_repo_relative_paths_follow_new_directory_depth(self):
        for item in self.records:
            content = (ROOT / item['to']).read_text()
            if 'ROOT=$(cd' in content:
                with self.subTest(script=item['to']):
                    self.assertIn('/../.." && pwd)',content)
                    self.assertNotIn('dirname "$0")/.." && pwd)',content)

if __name__ == '__main__':
    unittest.main()
