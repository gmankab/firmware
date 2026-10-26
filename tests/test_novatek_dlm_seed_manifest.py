# SPDX-License-Identifier: GPL-3.0-only
"""Independent Python checks of source/seed hash/offset provenance."""
from __future__ import annotations
import hashlib
import importlib.util
from pathlib import Path
import unittest

R=Path(__file__).resolve().parents[1]
P=R/'tools/verify-novatek-dlm-seed-manifest.py'
S=importlib.util.spec_from_file_location('agent19_seed_manifest',P)
assert S and S.loader
m=importlib.util.module_from_spec(S);S.loader.exec_module(m)
M=R/'crates/firmware-tools/src/devices/xiaomi_nabu/novatek_dlm_seed_hashes.tsv'


class TestDlmExternalSeedManifest(unittest.TestCase):
    def setUp(self):
        self.text=M.read_text();self.entries=m.parse_manifest(self.text)
        self.plan={"schema":"agent20-19-dlm-sparse-v1", "source_dlm_bytes":9353,
          "dependent_unique_bytes":12984,"required_external_seed_bytes":27215,
          "independent_dependent_crosschecks":40,"seed_ranges":[
              {"file_start":a,"file_end":b,"bytes":b-a} for a,b,_ in self.entries]}

    def test_canonical_manifest_boundaries_and_seed_totals(self):
        self.assertEqual(len(self.entries),60)
        self.assertEqual(sum(b-a for a,b,_ in self.entries),27215)
        self.assertEqual(self.entries[0][:2],(0x22000,0x22488))
        self.assertEqual(self.entries[-1][:2],(0x2e18d,0x2e190))
        self.assertIsNone(m.verify_plan(self.plan,self.entries))

    def test_reject_wrong_revision_corrupt_hash_bad_rows(self):
        for changed in (self.text.replace('postmarketos-nabu','map220v'),
                        self.text.replace(m.PINNED,'f'*64),
                        self.text.replace(self.entries[0][2], 'g'*64),
                        self.text.replace('0x022000','0x022001'),
                        self.text+'0x022000\t0x022001\t'+'0'*64+'\n',
                        '\n'.join(self.text.splitlines()[:-1])+'\n'):
            with self.subTest(changed=changed[:35]),self.assertRaises(ValueError):m.parse_manifest(changed)

    def test_reject_reordered_plan_and_wrong_digest(self):
        saved=self.plan['seed_ranges'];self.plan['seed_ranges']=saved[::-1]
        with self.assertRaisesRegex(ValueError,'range #'):m.verify_plan(self.plan,self.entries)
        self.plan['seed_ranges']=saved
        fake=[(a,b'\0'*(b-a)) for a,b,_ in self.entries]
        with self.assertRaisesRegex(ValueError,'SHA-256 mismatch'):m.verify_seeds(self.entries,fake)
        with self.assertRaisesRegex(ValueError,'missing/extra'):m.verify_seeds(self.entries,fake[:-1])
        fake[0]=(self.entries[0][0]+1,fake[0][1])
        with self.assertRaisesRegex(ValueError,'offset'):m.verify_seeds(self.entries,fake)

    def test_crypto_positive_case_using_synthetic_and_no_oem_bytes(self):
        payload=b'synthetic test vector'
        entry=[(0x22000,0x22000+len(payload),hashlib.sha256(payload).hexdigest())]
        self.assertEqual(m.verify_seeds(entry,[(0x22000,payload)]),len(payload))
        bad=payload[:-1]+bytes([payload[-1]^1])
        with self.assertRaisesRegex(ValueError,'SHA-256'):m.verify_seeds(entry,[(0x22000,bad)])

if __name__=='__main__':unittest.main()
