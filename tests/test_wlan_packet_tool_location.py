# SPDX-License-Identifier: GPL-3.0-only
"""Keep the Hexagon WLAN packet parser co-located with its builder."""
from pathlib import Path
import hashlib
import json
import unittest

ROOT = Path(__file__).resolve().parents[1]


class WlanPacketLocationTests(unittest.TestCase):
    def test_wlan_packet_parser_is_source_preserved(self):
        ledger = json.loads((ROOT / 'docs/maintenance/wlan-packet-parser-relocation.json').read_text())
        self.assertFalse((ROOT / ledger['source']).exists())
        dest = ROOT / ledger['destination']
        self.assertEqual(dest.parent, ROOT / 'tools/wlanmdsp')
        self.assertEqual(hashlib.sha256(dest.read_bytes()).hexdigest(), ledger['sha256'])

    def test_builder_imports_colocated_parser(self):
        builder = (ROOT / 'tools/wlanmdsp/build-wlanmdsp-late.py').read_text()
        self.assertIn('tools/wlanmdsp/late_packets.py', builder)
        self.assertNotIn('tools/wlan_late_packets.py', builder)


if __name__=='__main__':unittest.main()
