# SPDX-License-Identifier: GPL-3.0-only
"""Independent check of uncredited Novatek narrow-controller bank transforms."""
import importlib.util
from pathlib import Path
import unittest

SRC=Path(__file__).resolve().parents[1]/'tools/novatek/verify-novatek-dlm-narrow-bank.py'
S=importlib.util.spec_from_file_location('novatek_narrow_bank_independent',SRC)
assert S and S.loader
m=importlib.util.module_from_spec(S); S.loader.exec_module(m)


class NarrowBankRelocations(unittest.TestCase):
    def test_four_directory_indices_and_unproved_flag_preserved(self):
        lower=(0x00000c8c,0x00000caf,0x80000cd2,0x04000cf5)
        b=b''.join(v.to_bytes(4,'little') for v in lower)
        upper=m.reconstruct_lower_span(m.DIRECTORY[0],b)
        actual=tuple(int.from_bytes(upper[4*i:4*i+4],'little') for i in range(4))
        self.assertEqual(actual,tuple(v+0x1000 for v in lower))
    def test_record_pair_indices_and_exempt_identity(self):
        b=bytearray([0x5a]*m.NARROW_WINDOW[1])
        for i in range(m.COUNT):
            at=m.RECORD_START-m.NARROW_WINDOW[0]+i*m.STRIDE
            b[at:at+4]=bytes((7,0,15,0))
        out=m.reconstruct_lower_span(m.NARROW_WINDOW[0],b)
        for i in range(m.COUNT):
            at=m.RECORD_START-m.NARROW_WINDOW[0]+i*m.STRIDE
            self.assertEqual(out[at:at+4], bytes((7,0,15,0)) if i in m.EXEMPT else bytes((23,0,31,0)))
        self.assertEqual(out[:0x40],b[:0x40])
    def test_refuses_overflow_truncation_and_wrong_window(self):
        with self.assertRaisesRegex(ValueError,'overflow'):
            m.reconstruct_lower_span(m.DIRECTORY[0],(0x7fff).to_bytes(4,'little')*4)
        x=bytearray(m.NARROW_WINDOW[1])
        x[0x40:0x42]=(0xfff5).to_bytes(2,'little')
        with self.assertRaisesRegex(ValueError,'overflow'):
            m.reconstruct_lower_span(m.NARROW_WINDOW[0],x)
        with self.assertRaises(ValueError):m.reconstruct_lower_span(m.NARROW_WINDOW[0],x[:-1])
        with self.assertRaises(ValueError):m.reconstruct_lower_span(0,x)
    def test_rejects_unpinned_reference_bytes(self):
        with self.assertRaisesRegex(ValueError,'pinned'):
            m.verify(bytes(196608))
        with self.assertRaisesRegex(ValueError,'pinned'):
            m.verify(bytes(100))

if __name__=='__main__':unittest.main()
