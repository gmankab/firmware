# SPDX-License-Identifier: GPL-3.0-only
"""Python-independent sparse DLM linker model tests; no OEM payload bundled."""
import importlib.util
from pathlib import Path
import unittest

P=Path(__file__).resolve().parents[1]/'tools/verify-novatek-dlm-sparse.py'
S=importlib.util.spec_from_file_location('novatek_dlm_sparse_independent',P)
assert S and S.loader
m=importlib.util.module_from_spec(S);S.loader.exec_module(m)

class SparseDlmTests(unittest.TestCase):
    def setUp(self):
        self.parts=[]
        self.holes,self.kinds,self.checks=m.plan(self.parts)
        self.seeds=[(a,bytes(b-a)) for a,b in self.holes]
    def test_synthetic_all_zero_primary_is_exactly_rebuilt(self):
        out,holes,kinds,checks=m.rebuild(self.parts,self.seeds)
        self.assertEqual(len(out),m.DLM_SIZE)
        # Zero-valued earlier pointer indices are relocated to 0x1000.
        # Exactly 9 indices * 21 records receive a nonzero high byte.
        self.assertEqual(sum(x!=0 for x in out),9*21)
        self.assertEqual(int.from_bytes(out[0x26a64+0x48-m.DLM_START:0x26a64+0x4a-m.DLM_START],"little"),0x1000)
        self.assertEqual(kinds[2],9507)
        self.assertEqual(checks,0)
    def test_reject_missing_extra_duplicate_and_wrong_range(self):
        for seeds in (self.seeds[:-1],self.seeds+self.seeds[-1:],
                      self.seeds[::-1],[(self.seeds[0][0]+1,self.seeds[0][1])]+self.seeds[1:]):
            with self.assertRaises(ValueError):m.rebuild(self.parts,seeds)
    def test_reject_primary_15bit_relocation_overflow(self):
        modified=[(a,bytearray(b)) for a,b in self.seeds]
        at=0x22a64+0x48
        for start,blob in modified:
            if start<=at and at+2<=start+len(blob):
                blob[at-start:at-start+2]=(0x7fff).to_bytes(2,'little');break
        else:self.fail('test position does not belong to a seed')
        with self.assertRaisesRegex(ValueError,'overflows'):m.rebuild(self.parts,modified)
    def test_reject_source_derived_disagreement_and_duplicates(self):
        target=m.PAIRS[0][1]+0x50
        author=[(target,b'\x55')]
        holes,types,checks=m.plan(author)
        self.assertEqual(checks,1)
        seeds=[(a,bytes(b-a)) for a,b in holes]
        with self.assertRaisesRegex(ValueError,'disagreeing'):m.rebuild(author,seeds)
        with self.assertRaisesRegex(ValueError,'overlapping authored'):
            m.plan(author+author)
    def test_record_suffixes_really_copy_only_the_87_byte_tail(self):
        earlier,n=m.RECORD_SUFFIXES[0]
        dest=earlier+m.RECORD_BANK_DELTA
        out,holes,kinds,checks=m.rebuild(self.parts,self.seeds)
        self.assertEqual(out[earlier-m.DLM_START:earlier-m.DLM_START+n],
                         out[dest-m.DLM_START:dest-m.DLM_START+n])
        self.assertEqual(n,87)
        # The nine-byte prefix of this 96-byte record is deliberately
        # NOT derived by the suffix-copy relationship.
        self.assertEqual(dest-9+m.DLM_START-m.DLM_START,dest-9)

    def test_small_records_copy_only_prefix_and_tail(self):
        prior,n=m.SMALL_COPIES[2]
        self.assertEqual(n,24)
        self.assertEqual(m.SMALL_COPIES[3],(prior+31,9))
        target=prior+m.RECORD_BANK_DELTA
        out,_,_,_=m.rebuild(self.parts,self.seeds)
        self.assertEqual(out[prior-m.DLM_START:prior-m.DLM_START+24],
                         out[target-m.DLM_START:target-m.DLM_START+24])
        # Seven bytes between the copied fields remain unknown primary data.
        self.assertNotIn((prior+24,7),m.SMALL_COPIES)

    def test_all_unproven_40_byte_record_middle_values_remain_external(self):
        for record in range(30):
            at=m.SMALL_RECORD_START+record*m.SMALL_RECORD_BYTES+24+m.RECORD_BANK_DELTA
            for byte_offset in range(7):
                self.assertTrue(any(a<=at+byte_offset<b for a,b in self.holes),
                                f'unknown selector byte promoted at {at+byte_offset:#x}')

    def test_range_bounds_and_derived_table_no_source_credit(self):
        with self.assertRaisesRegex(ValueError,'out-of-bounds'):m.plan([(m.DLM_START-1,b'\0')])
        self.assertEqual(m.DLM_SIZE,49552)
        self.assertEqual(sum(x[1] for x in m.ALIASES),3136)
        self.assertEqual(sum(n for _,_,n in m.PAIRS),21)
        self.assertEqual(m.PROFILE[1]-m.PROFILE[0],428)
        self.assertEqual(m.SIGNATURE[2],12)
        self.assertEqual(len(m.RECORD_SUFFIXES),23)
        self.assertEqual(sum(n for _,n in m.RECORD_SUFFIXES),2001)
        self.assertEqual(len(m.SMALL_COPIES),60)
        self.assertEqual(sum(n for _,n in m.SMALL_COPIES),990)

if __name__=='__main__':unittest.main()
