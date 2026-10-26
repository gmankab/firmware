# SPDX-License-Identifier: GPL-3.0-only
"""Checks for the source-generated protected exception dispatch entry."""
import importlib.util
import os
from pathlib import Path
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'tools/assemble-agent11-rwe-exception.py'
spec = importlib.util.spec_from_file_location('modem_rwe_exception', str(SCRIPT))
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)


class ModemRweException(unittest.TestCase):
    def test_exact_address_mapping(self):
        self.assertEqual((module.FILE_START, module.FILE_END), (0x38108, 0x383cc))
        self.assertEqual(module.VIRTUAL_START-module.FILE_START, 0xc0808000)
        self.assertEqual(module.PHYSICAL_START-module.FILE_START, 0x8e008000)

    def test_wrong_source_revision_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'Wrong modem firmware revision'):
            module.verify_pinned(bytes(module.FILE_END), bytes(module.FILE_END-module.FILE_START))

    def test_llvm6_source_assembly(self):
        mc = os.environ.get('AGENT11_LLVM_MC')
        if not mc:
            self.skipTest('Set AGENT11_LLVM_MC to run LLVM6 Hexagon assembler test')
        source = SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_dispatch.s'
        data = module.assemble(Path(mc), source)
        self.assertEqual(len(data), 708)
        self.assertEqual(data[0x381b8-module.FILE_START:0x381bc-module.FILE_START],bytes.fromhex('00c0e057'))
        self.assertEqual(data[0:4], bytes.fromhex('00c00058'))
        self.assertEqual(data[-4:], bytes.fromhex('04d2a048'))
        ref = os.environ.get('AGENT11_MODEM_MBN')
        if ref:
            module.verify_pinned(Path(ref).read_bytes(), data)


if __name__ == '__main__':
    unittest.main()
