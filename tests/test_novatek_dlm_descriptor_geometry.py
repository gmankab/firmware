"""Independent Python checks for externally observed DLM descriptor geometry."""
import importlib.util
import json
import os
from pathlib import Path
import struct
import unittest

SRC=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('dlm_descriptor_geometry', SRC/'tools/novatek/verify-novatek-dlm-descriptor-geometry.py')
mod=importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)


def fake_image():
    data=bytearray(mod.SIGNATURE_COPY+mod.SIGNATURE_SIZE)
    for start,n in mod.PAIRS:
        for bank in (0,1):
            value=mod.geometry_index(start,n,bank)
            for i in range(n):
                at=start+bank*mod.DELTA+i*mod.RECORD_SIZE+mod.POINTER_OFFSET
                struct.pack_into('<H',data,at,value|(0x8000 if i%3==2 else 0))
    signature=bytes([3,9,7,5,2,3,1,1,5,8,13,21])
    for at in (mod.SIGNATURE_PRIMARY,mod.SIGNATURE_COPY):
        data[at:at+mod.SIGNATURE_SIZE]=signature
    return data


def fake_ledger():
    return {'source_dlm_data_bytes':9353,'unknown_dlm_bytes':40199,'regions':[{'file_start':mod.BASE,'file_end':mod.SIGNATURE_COPY+mod.SIGNATURE_SIZE,'kind':'unknown_dlm'}]}


class TestNovatekDlmDescriptorGeometry(unittest.TestCase):
    def test_exact_pointer_index_from_array_geometry(self):
        self.assertEqual(mod.geometry_index(0x22a64,17,0),0x4ec)
        self.assertEqual(mod.geometry_index(0x22a64,17,1),0x14ec)
        self.assertEqual(mod.geometry_index(0x25230,4,0),0xd18)
        self.assertEqual(mod.geometry_index(0x25230,4,1),0x1d18)
        for case in ((0,4,0),(0x25230,4,2),(0x22a64,17,-1)):
            with self.assertRaisesRegex(ValueError,'unrecognized'):mod.geometry_index(*case)

    def test_42_records_are_masked_without_crediting_flag_bits(self):
        image=fake_image()
        self.assertEqual(mod.verify_geometry(image),42)
        self.assertEqual(mod.verify_signature(image),12)
        self.assertTrue(mod.verify_no_source_credit(fake_ledger()))
        report=mod.verify(image,fake_ledger(),check_hash=False)
        self.assertEqual(report['additional_independently_source_authored_bytes'],0)
        self.assertTrue(report['selector_flag_source_unknown'])

    def test_corrupted_pointer_and_truncation_fail_with_address(self):
        image=fake_image()
        image[0x22a64+mod.POINTER_OFFSET]^=1
        with self.assertRaisesRegex(ValueError,'0x22aac'):mod.verify_geometry(image)
        image=fake_image()
        with self.assertRaisesRegex(ValueError,'truncated'):mod.verify_geometry(image[:0x22a64])

    def test_corrupted_repeated_profile_does_not_pass(self):
        image=fake_image()
        image[mod.SIGNATURE_COPY+6]^=1
        with self.assertRaisesRegex(ValueError,'0x2df92'):mod.verify_signature(image)
        with self.assertRaisesRegex(ValueError,'truncated'):mod.verify_signature(image[:-3])

    def test_provenance_claim_and_misleading_reference_hash_fail(self):
        image=fake_image()
        ledger=fake_ledger()
        ledger['regions']=[
            {'file_start':mod.BASE,'file_end':0x22aac,'kind':'unknown_dlm'},
            {'file_start':0x22aac,'file_end':0x22aad,'kind':'source_dlm_data'},
            {'file_start':0x22aad,'file_end':mod.SIGNATURE_COPY+12,'kind':'unknown_dlm'},
        ]
        with self.assertRaisesRegex(ValueError,'overlaps credited source'):mod.verify_no_source_credit(ledger)
        with self.assertRaisesRegex(ValueError,'SHA-256'):mod.verify(image,fake_ledger(),check_hash=True)
        ledger=fake_ledger();ledger['unknown_dlm_bytes']-=2
        with self.assertRaisesRegex(ValueError,'unexpected DLM independent'):mod.verify(image,ledger,check_hash=False)

    @unittest.skipUnless(os.environ.get('NOVATEK_REFERENCE') and os.environ.get('NOVATEK_DLM_LEDGER'),
                         'pinned private reference and ledger are not set')
    def test_pinned_original_and_provenance(self):
        image=Path(os.environ['NOVATEK_REFERENCE']).read_bytes()
        ledger=json.loads(Path(os.environ['NOVATEK_DLM_LEDGER']).read_text())
        obj=mod.verify(image,ledger,check_hash=True)
        self.assertEqual(obj['geometric_pointer_records_checked'],42)
        self.assertEqual(obj['dependent_profile_signature_bytes'],12)
        self.assertEqual(obj['additional_independently_source_authored_bytes'],0)

if __name__=='__main__':unittest.main()
