"""Independent Python verification for the uncredited DLM duplicate profile."""
import importlib.util
import struct
import unittest
from pathlib import Path

MODULE=Path(__file__).resolve().parents[1]/'tools/novatek/verify-novatek-calibration-alias.py'
spec=importlib.util.spec_from_file_location('novatek_calibration_alias',MODULE)
dlm=importlib.util.module_from_spec(spec)
spec.loader.exec_module(dlm)

class TestDlmDependentCalibration(unittest.TestCase):
    def setUp(self):
        self.image=bytearray(dlm.MIRROR_END)
        self.first=bytes((index*7+3)%256 for index in range(dlm.LENGTH))
        self.image[dlm.START:dlm.END]=self.first
        self.image[dlm.MIRROR_START:dlm.MIRROR_END]=self.first

    def test_dependent_duplicate_is_exact_but_never_source_credit(self):
        obj=dlm.verify_pair(self.image)
        self.assertEqual(obj['matched_reference_dependent_bytes'],428)
        self.assertEqual(obj['additional_independently_authored_source_bytes'],0)
        self.assertEqual(obj['mirror_file_start'],0x2d648)
        self.assertFalse(obj['hash_verified'])

    def test_pinned_profile_headers_are_inspected_but_not_transcribed(self):
        part=bytearray(dlm.LENGTH)
        struct.pack_into('<4H',part,4,1600,48,32,1)
        struct.pack_into('<4H',part,0xd6,2600,48,70,1)
        h=dlm.inspect_observed_profiles(part)
        self.assertEqual([entry['mapped_span'] for entry in h],[1600,2600])
        self.assertEqual([entry['controller_parameter'] for entry in h],[32,70])
        self.assertEqual([entry['rows'] for entry in h],[48,48])

    def test_mismatched_source_mirror_and_bounds_rejected(self):
        self.image[dlm.MIRROR_START+210]^=1
        with self.assertRaisesRegex(ValueError,'0x2d71a'):
            dlm.verify_pair(self.image)
        with self.assertRaisesRegex(ValueError,'incomplete'):
            dlm.verify_pair(self.image[:-1])
        with self.assertRaisesRegex(ValueError,'exactly 428'):
            dlm.derive_reference_dependent_alias(self.first[:-1])

    def test_original_image_hash_is_required_before_pinned_claim(self):
        with self.assertRaisesRegex(ValueError,'SHA256'):
            dlm.verify_pair(self.image,check_hash=True)

if __name__=='__main__':unittest.main()
