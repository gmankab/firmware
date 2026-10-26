# SPDX-License-Identifier: GPL-3.0-only
"""Independent Python tests for strict DLM bank alias provenance."""
import importlib.util
from pathlib import Path
import json
import unittest
import sys
P=Path(__file__).resolve().parents[1]/'tools'/'verify-novatek-dlm-bank-aliases.py'
spec=importlib.util.spec_from_file_location('agent19_bank_alias',P)
mod=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=mod
spec.loader.exec_module(mod)


def image():
    data=bytearray(mod.DLM_END)
    for label,src,n in mod.ALIASES:
        sample=bytes(((src+53*i) % 256) for i in range(n))
        data[src:src+n]=sample
        data[src+mod.BANK_DELTA:src+mod.BANK_DELTA+n]=sample
    return data


def ledger():
    return {'source_dlm_data_bytes':9353,'unknown_dlm_bytes':40199,
            'regions':[{'file_start':0,'file_end':mod.DLM_START,'kind':'source_layout'},
                       {'file_start':mod.DLM_START,'file_end':mod.DLM_END,'kind':'unknown_dlm'}]}


class BankAliasTests(unittest.TestCase):
    def test_four_exact_dependent_spans_do_not_add_authorship(self):
        r=mod.verify_aliases(image(),ledger(),check_hash=False)
        self.assertEqual(r['reference_dependent_matched_bytes'],1432)
        self.assertEqual(r['new_source_dlm_bytes'],0)
        self.assertEqual(r['source_dlm_data_bytes'],9353)
        self.assertEqual([x['bytes'] for x in r['aliases']],[406,228,182,616])
        self.assertTrue(all(x['dependent_file_start']-x['primary_file_start']==0x4000 for x in r['aliases']))

    def test_mutated_dependent_copy_and_truncated_input_fail(self):
        data=image();data[0x28186+79]^=1
        with self.assertRaisesRegex(ValueError,'file 0x281d5'):
            mod.verify_aliases(data,check_hash=False)
        data=image()
        with self.assertRaisesRegex(ValueError,'incomplete'):
            mod.verify_aliases(data[:mod.ALIASES[-1][1]+mod.BANK_DELTA+mod.ALIASES[-1][2]-1],check_hash=False)
        with self.assertRaisesRegex(ValueError,'not pinned'):
            mod.verify_aliases(data,check_hash=True)

    def test_both_primary_and_later_bank_must_remain_unknown(self):
        data=image()
        claimed=ledger(); claimed['regions']=[
            {'file_start':0,'file_end':mod.DLM_START,'kind':'source_layout'},
            {'file_start':mod.DLM_START,'file_end':0x2387c,'kind':'unknown_dlm'},
            {'file_start':0x2387c,'file_end':0x2387d,'kind':'source_dlm_data'},
            {'file_start':0x2387d,'file_end':mod.DLM_END,'kind':'unknown_dlm'},
        ]
        with self.assertRaisesRegex(ValueError,'overlaps source-owned'):
            mod.verify_aliases(data,claimed,check_hash=False)
        claimed['regions']=[
            {'file_start':0,'file_end':mod.DLM_START,'kind':'source_layout'},
            {'file_start':mod.DLM_START,'file_end':0x2787c,'kind':'unknown_dlm'},
            {'file_start':0x2787c,'file_end':0x2787d,'kind':'source_dlm_data'},
            {'file_start':0x2787d,'file_end':mod.DLM_END,'kind':'unknown_dlm'},
        ]
        with self.assertRaisesRegex(ValueError,'overlaps source-owned'):
            mod.verify_aliases(data,claimed,check_hash=False)

    def test_bad_accounting_is_rejected(self):
        data=image()
        l=ledger();l['source_dlm_data_bytes']=1432+9353
        with self.assertRaisesRegex(ValueError,'unexpected DLM'):
            mod.verify_aliases(data,l,check_hash=False)

if __name__=='__main__':unittest.main()
