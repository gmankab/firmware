# SPDX-License-Identifier: GPL-3.0-only
"""Independent NDS32 bitfield check for the 22-byte upper ownership seam."""
from pathlib import Path
import hashlib,importlib.util,unittest
P=Path(__file__).resolve().parents[1]/'tools/devices/xiaomi-nabu/verify_novatek-upper_boundary.py'
S=importlib.util.spec_from_file_location('seam',P); M=importlib.util.module_from_spec(S); S.loader.exec_module(M)

class Seam(unittest.TestCase):
    def test_independent_seam_bitfields(self):
        data=M.emit()
        self.assertEqual(len(data),22)
        self.assertEqual(hashlib.sha256(data).hexdigest(),M.EXPECTED_SHA256)
        self.assertEqual(data[:2],b'\x01\x04')
        self.assertEqual(data[-2:],b'\xfc\x80')
        self.assertEqual(data[10:14],b'\x49\xff\xff\xd0')

if __name__=='__main__': unittest.main()
