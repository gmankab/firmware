# SPDX-License-Identifier: GPL-3.0-only
"""Keep the tooling root small while retaining documented public commands."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / 'tools'
PUBLIC = {
    '__init__.py', 'check-local.sh', 'firmware.py',
    'low-memory-build.sh', 'verify-xiaomi-nabu.sh', 'verify.sh',
}


class PublicToolEntrypointsTests(unittest.TestCase):
    def test_only_public_entrypoints_live_at_tools_root(self):
        support = {'Containerfile', 'unblob-exclude.txt'}
        actual = {p.name for p in TOOLS.iterdir() if p.is_file()}
        self.assertEqual(actual, PUBLIC | support)

    def test_device_tool_dependencies_are_colocated(self):
        required = {
            'tools/adsp/emit-adsp-entry.py',
            'tools/adsp/emit-adsp-late.py',
            'tools/adsp/adsp_hexagon_v66.py',
            'tools/wlanmdsp/late_packets.py',
            'tools/build/generate-elf-specs.py',
            'tools/build/assemble-a640-exact.py',
            'tools/analysis/mbn-reproducibility.py',
            'tools/analysis/decompile-all.sh',
            'tools/verify/verify-qrisc-against-mesa.sh',
        }
        for name in required:
            with self.subTest(name=name):
                self.assertTrue((ROOT / name).is_file())

    def test_no_assembly_sources_mixed_into_tools(self):
        assembly = [p for p in TOOLS.rglob('*') if p.is_file() and p.suffix in ('.s', '.S')]
        self.assertEqual(assembly, [])

    def test_root_quality_gate_and_cli_are_stable(self):
        self.assertIn('python3 -m tools.firmware status --check',
                      (TOOLS / 'check-local.sh').read_text())
        self.assertTrue((ROOT / 'reference/xiaomi-nabu/coverage.toml').exists())


if __name__ == '__main__':
    unittest.main()
