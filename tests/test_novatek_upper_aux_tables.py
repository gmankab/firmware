# SPDX-License-Identifier: GPL-3.0-only
"""Independent auxiliary table verification without OEM binary in repository."""
import hashlib
import importlib.util
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
TOOL = ROOT / 'tools/devices/xiaomi-nabu/verify_novatek-upper_aux_tables.py'
SPEC = importlib.util.spec_from_file_location('agent20_18_aux_generator', TOOL)
AUX = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AUX)


class AuxDataIndependent(unittest.TestCase):
    def test_tangent_curve_math_and_observed_samples(self):
        curve = AUX.tangent_curve()
        self.assertEqual(len(curve), 31)
        self.assertEqual(curve[15], 100)
        self.assertEqual(curve[27], 631)
        self.assertEqual(curve[-3:], [953,1913,5730])

    def test_mask_table_closed_under_complement(self):
        values=AUX.mask_sequence()
        self.assertEqual(len(values),72)
        self.assertEqual(values[0] ^ values[8], AUX.MASK20)
        self.assertEqual(values.count(0),8)

    def test_aux_table_hashes_independent_of_emitter_and_reference(self):
        data=AUX.independent_fragments()
        self.assertEqual({start:hashlib.sha256(bytes).hexdigest()
                          for start, bytes in data.items()},AUX.DIGESTS)
        self.assertEqual([len(data[x]) for x in sorted(data)], [484,52])

    def test_revision_change_rejected_even_if_structure_remains_sized(self):
        original=AUX.DIGESTS[AUX.LOOKUPS_START]
        try:
            AUX.DIGESTS[AUX.LOOKUPS_START]='0'*64
            with self.assertRaisesRegex(ValueError,'digest mismatch'):
                AUX.independent_fragments()
        finally:
            AUX.DIGESTS[AUX.LOOKUPS_START]=original


if __name__=='__main__': unittest.main()
