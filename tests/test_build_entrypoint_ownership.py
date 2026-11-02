# SPDX-License-Identifier: GPL-3.0-only
"""Prevent relocated modem/WLAN build scripts from resolving the wrong root."""
import json
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]


class BuildEntrypointOwnershipTests(unittest.TestCase):
    def test_builders_are_device_owned(self):
        ledger=json.loads((ROOT/'docs/maintenance/build-entrypoint-relocation.json').read_text())
        self.assertEqual(len(ledger['moves']),7)
        for oldname,relative in ledger['moves'].items():
            with self.subTest(tool=oldname):
                self.assertFalse((ROOT/'tools'/oldname).exists())
                script=ROOT/relative
                self.assertTrue(script.is_file())
                self.assertLess(script.stat().st_size,250_000)

    def test_relocated_scripts_resolve_repository_root(self):
        for name in ('build-wlanmdsp-late.py','build-wlanmdsp-late-tables.py',
                     'build-wlanmdsp-startup.py'):
            script=ROOT/'tools/wlanmdsp'/name
            with self.subTest(script=name):
                self.assertIn('ROOT = Path(__file__).resolve().parents[2]',script.read_text())
                self.assertEqual(script.resolve().parents[2],ROOT)
        for name in ('assemble-modem-dispatch.py','assemble-modem-tlb.py',
                     'assemble-modem-system.py','assemble-modem-late.py'):
            script=ROOT/'tools/modem'/name
            with self.subTest(script=name):
                self.assertIn('Path(__file__).resolve().parents[2]',script.read_text())
                self.assertEqual(script.resolve().parents[2],ROOT)


if __name__=='__main__':
    unittest.main()
