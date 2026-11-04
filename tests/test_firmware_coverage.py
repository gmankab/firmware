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
from unittest import mock

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT))
from tools.build.firmware_coverage import (CoverageError,load,holes,status,
                                         verify_reference,verify_generated_region)


class FirmwareCoverageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.data=load()

    def test_pinned_revision_records(self):
        self.assertEqual(len(self.data['images']),19)
        self.assertEqual(sum(i['size'] for i in self.data['images']),35409209)
        self.assertEqual(sum(i['credit_bytes'] for i in self.data['images']),840046)

    def test_gmu_reset_prefix_is_not_entire_firmware(self):
        image=next(x for x in self.data['images'] if x['name']=='a640_gmu.bin')
        self.assertEqual(image['credit_bytes'],480)
        self.assertEqual(image['external_gmu_payload_bytes'],37200)
        self.assertEqual(image['credit_bytes']+image['external_gmu_payload_bytes'],image['size'])
        self.assertFalse(image.get('complete_source',False))

    def test_bluetooth_headers_do_not_claim_patch_or_nvm_values(self):
        patch=next(x for x in self.data['images'] if x['name']=='crbtfw32.tlv')
        nvm=next(x for x in self.data['images'] if x['name']=='crnv32.bin')
        self.assertEqual(patch['credit_bytes'],28)
        self.assertEqual(patch['external_bt_patch_bytes'],127516)
        self.assertEqual(nvm['credit_bytes'],744)
        self.assertEqual(nvm['external_bt_tag_values_bytes'],4760)
        self.assertFalse(patch.get('complete_source',False))
        self.assertFalse(nvm.get('complete_source',False))

    def test_board2_framing_is_not_calibration(self):
        board=next(x for x in self.data['images'] if x['name']=='board-2.bin')
        self.assertEqual(board['credit_bytes'],1508)
        self.assertEqual(board['credit_kind'],'structured_headers_only')
        self.assertEqual(board['external_calibration_bytes'],459552)
        self.assertEqual(board['credit_bytes']+board['external_calibration_bytes'],board['size'])
        self.assertFalse(board.get('complete_source',False))

    def test_four_tuning_profiles_are_independently_complete_data(self):
        for speaker in ('BL','BR','TL','TR'):
            image=next(i for i in self.data['images'] if i['name']==f'{speaker}-cs35l41-dsp1-spk-prot.bin')
            self.assertTrue(image['complete_source'])
            self.assertEqual(image['credit_bytes'],image['size'])
            self.assertEqual(image['credit_kind'],'typed_wmdr_coefficients_and_metadata')

    def test_cirrus_cli_argument_dispatch(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d)
            stub=root/'builder'
            stub.write_text('#!/bin/sh\n'
                'test "$1" = rebuild-cirrus-tuning || exit 2\n'
                'printf "%s" "$2" > "$3"\n')
            stub.chmod(0o755)
            for speaker in ('BL','BR','TL','TR'):
                target=f'{speaker}-cs35l41-dsp1-spk-prot.bin'
                source=speaker.encode()
                manifest=root/f'{speaker}.toml'
                manifest.write_text('schema_version=1\n[[images]]\n'
                    f'name="{target}"\nsize=2\ncredit_bytes=2\ncomplete_source=true\n'
                    f'sha256="{hashlib.sha256(source).hexdigest()}"\n')
                output=root/f'output-{speaker}'
                process=subprocess.run([sys.executable,'-m','tools.firmware','--manifest',str(manifest),
                    'build',target,'--firmware-tools',str(stub),'--output',str(output)],
                    cwd=ROOT,text=True,capture_output=True)
                self.assertEqual(process.returncode,0,process.stderr)
                self.assertEqual(output.read_bytes(),source)

    def test_full_pinned_firmware_inventory_includes_non_executables(self):
        expected={
            'modemuw.jsn','firmware-5.bin','a640_zap.mbn','venus.mbn',
            'adsp.mbn','cdsp.mbn','modem.mbn','wlanmdsp.mbn',
            'cs35l41-dsp1-spk-prot.wmfw','novatek_nt36523_fw.bin',
            'a630_sqe.fw','a640_gmu.bin','board-2.bin','crbtfw32.tlv','crnv32.bin',
        }|{f'{channel}-cs35l41-dsp1-spk-prot.bin' for channel in ('BL','BR','TL','TR')}
        self.assertEqual({i['name'] for i in self.data['images']},expected)
        self.assertTrue(all(i['credit_bytes']==0 for i in self.data['images']
                            if i['name'] in {'a630_sqe.fw'}))

    def test_dispatch_prefix_is_credited_but_not_a_complete_function(self):
        venus=next(x for x in self.data['images'] if x['name']=='venus.mbn')
        self.assertIn({'start':0x69e44,'end':0x6a8e4,
                       'provenance':'reported_reconstructed',
                       'label':'upper controller dispatch, record copies and service buffer setup (incomplete)'},venus['regions'])
        self.assertEqual(venus['credit_bytes'],531776)

    def test_explicit_venus_gap_partition(self):
        venus=next(x for x in self.data['images'] if x['name']=='venus.mbn')
        gaps=holes(venus['regions'],0,venus['executable_load']['runtime_end'])
        self.assertEqual(len(gaps),6)
        self.assertEqual(sum(g['bytes'] for g in gaps),527540)
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
            with mock.patch.object(Path,'read_bytes',side_effect=AssertionError('unbounded read')):
                self.assertTrue(verify_reference(fixture,'test.bin',original)['matched_pinned_revision'])
                self.assertEqual(verify_generated_region(fixture,'test.bin',original,generated,2)['generated_bytes'],3)
            generated.write_bytes(b'xxx')
            with self.assertRaisesRegex(CoverageError,'difference'):
                verify_generated_region(fixture,'test.bin',original,generated,2)
            original.write_bytes(bytes([255])+bytes(range(1,8)))
            with self.assertRaisesRegex(CoverageError,'SHA-256'):
                verify_reference(fixture,'test.bin',original)

    def test_failing_builder_does_not_replace_existing_firmware(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d)
            stub=root/'bad-builder'
            stub.write_text('#!/bin/sh\nprintf corrupt > "$2"\n')
            stub.chmod(0o755)
            output=root/'firmware-5.bin'
            output.write_bytes(b'keep existing verified bytes')
            cmd=[sys.executable,'-m','tools.firmware','build','firmware-5.bin',
                 '--firmware-tools',str(stub),'--output',str(output)]
            result=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
            self.assertEqual(result.returncode,2,result.stderr)
            self.assertIn('size/SHA-256',result.stderr)
            self.assertEqual(output.read_bytes(),b'keep existing verified bytes')
            self.assertEqual(set(root.iterdir()),{stub,output})

    def test_chunked_compare_crosses_megabyte_boundary(self):
        content=bytes(range(256))*8193
        fixture={'images':[{'name':'image.bin','size':len(content),'credit_bytes':0,
            'sha256':hashlib.sha256(content).hexdigest()}]}
        with tempfile.TemporaryDirectory() as d:
            root=Path(d)
            reference=root/'reference'
            generated=root/'generated'
            reference.write_bytes(content)
            generated.write_bytes(content[37:])
            with mock.patch.object(Path,'read_bytes',side_effect=AssertionError('unbounded read')):
                self.assertEqual(verify_generated_region(fixture,'image.bin',reference,generated,37)['generated_bytes'],len(content)-37)
            corrupted=bytearray(content[37:])
            corrupted[1048576] ^= 1
            generated.write_bytes(corrupted)
            with self.assertRaisesRegex(CoverageError,f'0x{37+1048576:x}'):
                verify_generated_region(fixture,'image.bin',reference,generated,37)

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
