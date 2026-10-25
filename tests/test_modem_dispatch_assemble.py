# SPDX-License-Identifier: GPL-3.0-only
"""Tests for the typed Hexagon modem revision and dispatch build steps."""
from pathlib import Path
import importlib.util
import unittest

ROOT=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location('modem_dispatch', ROOT/'tools/assemble-modem-dispatch.py')
modem=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(modem)

class DispatchChecks(unittest.TestCase):
    def test_contiguous_size(self):
        self.assertEqual(modem.FILE_START,0x3644)
        self.assertEqual(modem.FILE_END,0x36e0)
        self.assertEqual(modem.LENGTH,156)
        self.assertEqual(modem.PREFIX_LENGTH,1604)
        self.assertEqual(modem.LENGTH+modem.PREFIX_LENGTH,1760)

    def test_rejects_wrong_revision(self):
        with self.assertRaisesRegex(ValueError,'wrong modem reference'):
            modem.verify(b'wrong',b'abc',0x3644)

    def test_source_has_symbolic_branches_and_packet_groupings(self):
        text=(ROOT/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_boot_dispatch.s').read_text()
        self.assertIn('HEX_SYSTEM_READ 2,29,1',text)
        self.assertIn('r4 = memw(r25+##-1064821884)',text)
        self.assertIn('jump:nt modem_ready_handler',text)
        self.assertIn('if (p0.new) memw(r23+#0) = r0',text)
        self.assertNotIn('.incbin',text)

if __name__=='__main__':unittest.main()
