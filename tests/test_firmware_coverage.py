# SPDX-License-Identifier: GPL-3.0-only
"""Reference-free tests for the canonical firmware coverage and CLI contracts."""
from __future__ import annotations
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT))
from tools.build.firmware_coverage import (CoverageError,load,holes,status,
                                         verify_reference,verify_generated_region)


class FirmwareCoverageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.data=load()

    def test_nine_pinned_revision_records(self):
        self.assertEqual(len(self.data['images']),9)
        self.assertEqual(sum(i['size'] for i in self.data['images']),34723697)
        self.assertEqual(sum(i['credit_bytes'] for i in self.data['images']),813146)

    def test_explicit_venus_gap_partition(self):
        venus=next(x for x in self.data['images'] if x['name']=='venus.mbn')
        gaps=holes(venus['regions'],0,venus['executable_load']['runtime_end'])
        self.assertEqual(len(gaps),6)
        self.assertEqual(sum(g['bytes'] for g in gaps),530260)
        self.assertEqual((gaps[0]['start'],gaps[0]['end']),(0xaa0c,0x12e5c))

    def test_overlap_and_out_of_bounds_fails_closed(self):
        for spans in ([{'start':0,'end':8},{'start':7,'end':12}],
                      [{'start':0,'end':13}], [{'start':10,'end':10}]):
            with self.subTest(spans=spans),self.assertRaises(CoverageError):
                holes(spans,0,12)

    def test_status_deterministic_and_generated_file_current(self):
        expected=status(self.data)
        self.assertEqual(expected,(ROOT/'docs/STATUS.md').read_text())
        self.assertEqual(expected,status(load()))

    def test_reject_modified_reference_and_generated_slice(self):
        fixture={'images':[{'name':'test.bin','size':8,'credit_bytes':0,
            'credit_kind':'unreconstructed','sha256':hashlib.sha256(bytes(range(8))).hexdigest()}]}
        with tempfile.TemporaryDirectory() as d:
            original=Path(d)/'reference.bin';original.write_bytes(bytes(range(8)))
            generated=Path(d)/'span.bin';generated.write_bytes(bytes([2,3,4]))
            self.assertTrue(verify_reference(fixture,'test.bin',original)['matched_pinned_revision'])
            self.assertEqual(verify_generated_region(fixture,'test.bin',original,generated,2)['generated_bytes'],3)
            generated.write_bytes(b'xxx')
            with self.assertRaisesRegex(CoverageError,'difference'):
                verify_generated_region(fixture,'test.bin',original,generated,2)
            original.write_bytes(bytes([255])+bytes(range(1,8)))
            with self.assertRaisesRegex(CoverageError,'SHA-256'):
                verify_reference(fixture,'test.bin',original)

    def test_cli_disallows_missing_full_image_source(self):
        cmd=[sys.executable,'-m','tools.firmware','build','venus.mbn','--output','/tmp/forbidden-venus.mbn']
        result=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
        self.assertEqual(result.returncode,2)
        self.assertIn('refusing to substitute original bytes',result.stderr)

    def test_cli_direct_execution_from_other_directory(self):
        result=subprocess.run([sys.executable,str(ROOT/'tools/firmware.py'),'status','--check'],
                              cwd='/',capture_output=True,text=True)
        self.assertEqual(result.returncode,0,result.stderr)

    def test_refuse_duplicate_and_missing_image(self):
        with tempfile.TemporaryDirectory() as d:
            manifest=Path(d)/'coverage.toml'
            manifest.write_text('schema_version=1\n[[images]]\nname="a.bin"\nsize=3\ncredit_bytes=4\nsha256="'+'0'*64+'"\n')
            with self.assertRaisesRegex(CoverageError,'invalid byte counts'):load(manifest)


if __name__=='__main__':unittest.main()
