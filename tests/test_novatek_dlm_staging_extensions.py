# SPDX-License-Identifier: GPL-3.0-only
"""Independent synthetic tests of uncredited two-bank staging dependencies."""
from pathlib import Path
import importlib.util
import unittest

F=Path(__file__).resolve().parents[1]/'tools/verify-novatek-dlm-staging-extensions.py'
S=importlib.util.spec_from_file_location('agent19_staging_extensions',F)
assert S and S.loader
m=importlib.util.module_from_spec(S);S.loader.exec_module(m)

class TestStagingExtensions(unittest.TestCase):
    def test_ranges_nonzero_input_required(self):
        self.assertEqual(sum(n for _,n in m.checked_dependency()),122)
        self.assertEqual(m.SPANS[0],(0x23860,24))
        self.assertEqual(m.SPANS[1],(0x23a13,98))
        for span in m.SPANS:
            primary=bytes((i*37+29)%251 for i in range(span[1]))
            self.assertEqual(m.dependent_copy(span,primary),primary)
            self.assertTrue(any(primary))
            with self.assertRaises(ValueError):m.dependent_copy(span,primary[:-1])
        with self.assertRaises(ValueError):m.dependent_copy((0x100,24),bytes(24))
    def test_reject_invalid_geometry_and_reference_revision(self):
        with self.assertRaises(ValueError):m.checked_dependency([(0x23a13,98),(0x23860,24)])
        with self.assertRaises(ValueError):m.compare_image(bytes(196608),m.POSTMARKETOS)
    def test_byte_corruption_fails_crossbank_check_on_synthetic_fixture(self):
        # Construct a matching *synthetic* reference with a synthetic hash.
        ref=bytearray(196608)
        for start,n in m.SPANS:
            ref[start:start+n]=bytes((i*21+5)%253 for i in range(n))
            ref[start+m.DELTA:start+m.DELTA+n]=ref[start:start+n]
        import hashlib
        sha=hashlib.sha256(ref).hexdigest()
        self.assertEqual(m.compare_image(ref,sha),122)
        ref[m.SPANS[1][0]+m.DELTA+4]^=1
        sha=hashlib.sha256(ref).hexdigest()
        with self.assertRaisesRegex(ValueError,'range #1'):m.compare_image(ref,sha)

if __name__=='__main__': unittest.main()
