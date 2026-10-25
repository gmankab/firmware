# SPDX-License-Identifier: GPL-3.0-only
"""Source generation and explicit reference-decoded address ownership."""
import hashlib
import importlib.util
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]
TOOL=ROOT/'tools/devices/xiaomi-nabu/verify_agent20_18_allocations.py'
SPEC=importlib.util.spec_from_file_location('controller_allocations',TOOL)
M=importlib.util.module_from_spec(SPEC);SPEC.loader.exec_module(M)

class ControllerAddressTable(unittest.TestCase):
    def test_strides_and_named_origins(self):
        words=M.addresses()
        self.assertEqual(len(words),24)
        self.assertEqual(words[0],0x39284)
        self.assertEqual(words[23],0x38068)
        self.assertTrue(all(pair[1]-pair[0]==0x190 for pair in [words[i:i+2] for i in range(0,8,2)]))
        for start in (8,12):
            self.assertEqual([y-x for x,y in zip(words[start:start+3],words[start+1:start+4])],[0xc80]*3)
        self.assertEqual([y-x for x,y in zip(words[16:19],words[17:20])],[0x640]*3)

    def test_encoded_addresses_are_little_endian(self):
        data=M.serialize()
        self.assertEqual(len(data),96)
        self.assertEqual(int.from_bytes(data[:4],'little'),0x39284)
        self.assertEqual(hashlib.sha256(data).hexdigest(),M.EXPECTED_SHA256)

if __name__=='__main__':unittest.main()
