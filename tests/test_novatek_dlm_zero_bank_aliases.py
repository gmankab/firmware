# SPDX-License-Identifier: GPL-3.0-only
"""Independent Python model for reference-dependent zero-bank DLM windows."""
import importlib.util
from pathlib import Path
import unittest

S=importlib.util.spec_from_file_location('novatek_zero_alias',
    Path(__file__).resolve().parents[1]/'tools/novatek/verify-novatek-dlm-zero-bank-aliases.py')
assert S and S.loader
m=importlib.util.module_from_spec(S);S.loader.exec_module(m)

class ZeroAliasTests(unittest.TestCase):
    def test_exact_geometry_and_source_remains_external(self):
        self.assertEqual(m.TOTAL_BYTES,2543)
        self.assertEqual(m.ALIASES[0],(0x2271c,336))
        self.assertEqual(m.ALIASES[-1],(0x262ec,1812))
        for start,ln in m.ALIASES:
            self.assertTrue(0x22000<=start<start+ln<=0x2e190)
            self.assertTrue(start+ln<=start+m.BANK_DELTA)
    def test_chain_is_an_unknown_copy_not_literal_zero_fill(self):
        image=bytearray(196608)
        source=0x2271c+23
        image[source]=0x3c
        image[source+m.BANK_DELTA]=0x3c
        image[source+2*m.BANK_DELTA]=0x3c
        self.assertEqual(m.check_spans(image)['copied_bytes'],2543)
        # Change the lower byte: the output is no longer correct, so a real
        # source input must be supplied to the later conditional linker.
        image[source]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.check_spans(image)
    def test_rejects_missing_truncated_or_unpinned_reference(self):
        with self.assertRaisesRegex(ValueError,'truncated'):m.check_spans(bytes(500))
        with self.assertRaisesRegex(ValueError,'pinned'):m.verify_pinned(bytes(196608))
        with self.assertRaisesRegex(ValueError,'pinned'):m.verify_pinned(bytes(100))
    def test_require_zero_is_reference_observation_only(self):
        image=bytearray(196608)
        image[0x2271c]=0xf1;image[0x2671c]=0xf1;image[0x2a71c]=0xf1
        self.assertEqual(m.check_spans(image,require_zero=False)['copied_bytes'],2543)
        with self.assertRaisesRegex(ValueError,'nonzero'):m.check_spans(image,require_zero=True)

if __name__=='__main__':unittest.main()
