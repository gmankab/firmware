# SPDX-License-Identifier: GPL-3.0-only
"""Shared analysis/build commands must remain usable from their owning paths."""
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT_PATHS = (
    'tools/analysis/elf-byte-budget.py',
    'tools/analysis/elf-segment-view.py',
    'tools/analysis/mbn-reproducibility.py',
    'tools/build/assemble-a640-exact.py',
    'tools/build/generate-elf-specs.py',
    'tools/adsp/emit-adsp-entry.py',
    'tools/adsp/emit-adsp-late.py',
)


class SharedCommandEntrypointsTests(unittest.TestCase):
    def test_all_commands_expose_help_without_reference_firmware(self):
        for name in SCRIPT_PATHS:
            with self.subTest(command=name):
                self.assertTrue((ROOT/name).is_file())
                process = subprocess.run(['python3', str(ROOT/name), '--help'],
                                         cwd=ROOT, capture_output=True,
                                         text=True, timeout=6, check=False)
                self.assertEqual(process.returncode, 0, process.stderr[:220])
                self.assertIn('usage:', process.stdout.lower())

    def test_public_root_has_no_internal_python_helpers(self):
        actual = {p.name for p in (ROOT/'tools').glob('*.py')}
        self.assertEqual(actual, {'firmware.py', '__init__.py'})


if __name__ == '__main__':
    unittest.main()
