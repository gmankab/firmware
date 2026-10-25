# SPDX-License-Identifier: GPL-3.0-only
import importlib.util
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('cdsp_v66_decoder',ROOT/'tools'/'verify-cdsp-v66-init.py')
cdsp=importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(cdsp)

class CdspV66DecoderTests(unittest.TestCase):
    def test_decodes_scalar_packet_metadata(self):
        lines='0x96c00024 4 06c00067 [ SSR = R0\n0x96c00028 4 00c0926e [ R0 = SYSCFG\n'
        self.assertEqual(cdsp.decode_output(lines), ['ssr=r0','r0=syscfg'])
    def test_rejects_unknown_packet_endings(self):
        with self.assertRaises(ValueError):
            cdsp.decode_output('0x96c00024 4 06c00067 / SSR = R0\n')
    def test_expects_seventeen_instructions(self):
        self.assertEqual(len(cdsp.EXPECTED_MNEMONICS), 17)
        self.assertEqual(len(cdsp.SYS_CFG_RESET_MNEMONICS), 11)
        self.assertEqual(cdsp.PROFILES["syscfg-reset"][1], 0x3d40)
        self.assertEqual(len(cdsp.SYS_CFG_DISABLE_MNEMONICS), 5)
        self.assertEqual(cdsp.PROFILES["syscfg-disable"][1], 0x3d80)

if __name__=='__main__':unittest.main()
