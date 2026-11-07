# SPDX-License-Identifier: GPL-3.0-only
"""Reference-free tests for the canonical firmware coverage and CLI contracts."""
from __future__ import annotations
import hashlib
import json
import os
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

    def _firmware_tools_binary(self):
        supplied = os.environ.get('FIRMWARE_TOOLS_BIN')
        if not supplied:
            message = ('set FIRMWARE_TOOLS_BIN to a freshly built firmware-tools executable '
                       '(tools/check-local.sh builds once and enables '
                       'FIRMWARE_TOOLS_INTEGRATION=1); no cached target binary is used')
            if os.environ.get('FIRMWARE_TOOLS_INTEGRATION') == '1':
                self.fail(message)
            self.skipTest(message)
        executable = Path(supplied).resolve()
        self.assertTrue(executable.is_file() and os.access(executable, os.X_OK),
                        f'FIRMWARE_TOOLS_BIN must be an executable file: {executable}')
        return executable

    def test_real_builder_requires_explicit_binary(self):
        with mock.patch.dict(os.environ, {}, clear=True):
            with self.assertRaisesRegex(unittest.SkipTest, 'freshly built'):
                self._firmware_tools_binary()

    def test_strict_integration_requires_binary(self):
        with mock.patch.dict(os.environ, {'FIRMWARE_TOOLS_INTEGRATION': '1'}, clear=True):
            with self.assertRaisesRegex(AssertionError, 'FIRMWARE_TOOLS_BIN'):
                self._firmware_tools_binary()

    def test_real_builder_rejects_invalid_explicit_binary(self):
        with tempfile.TemporaryDirectory() as directory:
            nonexecutable = Path(directory) / 'not-executable'
            nonexecutable.write_text('not an executable')
            for path in (Path(directory), Path(directory) / 'missing', nonexecutable):
                with self.subTest(path=path), mock.patch.dict(
                        os.environ, {'FIRMWARE_TOOLS_BIN': str(path)}, clear=True):
                    with self.assertRaisesRegex(AssertionError, 'executable file'):
                        self._firmware_tools_binary()

    def test_real_builder_resolves_explicit_binary(self):
        with mock.patch.dict(os.environ, {'FIRMWARE_TOOLS_BIN': sys.executable}, clear=True):
            self.assertEqual(self._firmware_tools_binary(), Path(sys.executable).resolve())

    def test_pinned_revision_records(self):
        self.assertEqual(len(self.data['images']),19)
        self.assertEqual(sum(i['size'] for i in self.data['images']),35409209)
        self.assertEqual(sum(i['credit_bytes'] for i in self.data['images']),979281)

    def test_gmu_reset_prefix_is_not_entire_firmware(self):
        image=next(x for x in self.data['images'] if x['name']=='a640_gmu.bin')
        self.assertEqual(image['credit_bytes'],4628)
        self.assertEqual(image['external_gmu_payload_bytes'],33052)
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

    def test_wmfw_credit_uses_disjoint_memory_plan_union(self):
        image=next(x for x in self.data['images'] if x['name']=='cs35l41-dsp1-spk-prot.wmfw')
        self.assertEqual(image['credit_bytes'],3984+69+3+792+5451)
        self.assertEqual(image['external_dsp_payload_bytes'],20280+3657)
        self.assertEqual(image['credit_bytes']+image['external_dsp_payload_bytes'],image['size'])
        self.assertEqual(image['credit_kind'],'structural_and_reference_observed')
        self.assertFalse(image.get('complete_source',False))

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
        self.assertIn({'start':0x69e44,'end':0x6ac0c,
                       'provenance':'reported_reconstructed',
                       'label':'upper controller dispatch, record copies and service buffer setup (incomplete)'},venus['regions'])
        self.assertEqual(venus['credit_bytes'],667052)

    def test_explicit_venus_gap_partition(self):
        venus=next(x for x in self.data['images'] if x['name']=='venus.mbn')
        gaps=holes(venus['regions'],0,venus['executable_load']['runtime_end'])
        self.assertEqual(len(gaps),2)
        self.assertEqual(sum(g['bytes'] for g in gaps),392264)
        self.assertEqual((gaps[0]['start'],gaps[0]['end']),(0x83608,0xa8000))

    def test_overlap_and_out_of_bounds_fails_closed(self):
        for spans in ([{'start':0,'end':8},{'start':7,'end':12}],
                      [{'start':0,'end':13}], [{'start':10,'end':10}]):
            with self.subTest(spans=spans),self.assertRaises(CoverageError):
                holes(spans,0,12)

    def test_invalid_coverage_window_fails_closed(self):
        for start, end in ((4, 3), (-1, 4), (False, 4), (0, 4.5)):
            with self.subTest(start=start, end=end), self.assertRaises(CoverageError):
                holes([], start, end)

    def test_manifest_rejects_boolean_counts_and_completion_flags(self):
        cases = ('size=true\ncredit_bytes=1',
                 'size=1\ncredit_bytes=true',
                 'size=1\ncredit_bytes=1\ncomplete_source="false"',
                 'size=2\ncredit_bytes=1\nexternal_authentication_bytes=true')
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / 'coverage.toml'
            for fields in cases:
                with self.subTest(fields=fields):
                    manifest.write_text('schema_version=1\n[[images]]\nname="test.bin"\n'
                                        + fields + '\nsha256="' + '0' * 64 + '"\n')
                    with self.assertRaises(CoverageError):
                        load(manifest)

    def test_manifest_rejects_executable_load_outside_image(self):
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / 'coverage.toml'
            for file_base, runtime_end in ((-1, 4), (5, 4), (0, 9), (0, -1)):
                with self.subTest(file_base=file_base, runtime_end=runtime_end):
                    manifest.write_text('schema_version=1\n[[images]]\nname="test.bin"\n'
                                        'size=8\ncredit_bytes=0\nsha256="' + '0' * 64 +
                                        '"\n[images.executable_load]\n'
                                        f'file_base={file_base}\nruntime_end={runtime_end}\n')
                    with self.assertRaises(CoverageError):
                        load(manifest)

    def test_malformed_manifest_cli_reports_error_without_traceback(self):
        cases = ('schema_version=1\nimages=[1]',
                 'schema_version=true\nimages=[]',
                 'schema_version=1\n[[images]]\nname=42',
                 'schema_version=1\n[[images]]\nname=".."',
                 'schema_version=1\n[[images]]\nname="test.bin"\nsize=8\n'
                 'credit_bytes=0\nsha256=42',
                 'schema_version=1\n[[images]]\nname="test.bin"\nsize=8\n'
                 'credit_bytes=0\nsha256="' + '0' * 64 + '"\nexecutable_load={}',
                 'schema_version=1\n[[images]]\nname="test.bin"\nsize=8\n'
                 'credit_bytes=0\nsha256="' + '0' * 64 + '"\n'
                 'regions=[{}]\nexecutable_load={file_base=0,runtime_end=8}',
                 'not valid TOML')
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / 'coverage.toml'
            for content in cases:
                with self.subTest(content=content):
                    manifest.write_text(content)
                    result = subprocess.run([sys.executable, '-m', 'tools.firmware',
                                             '--manifest', str(manifest), 'images'],
                                            cwd=ROOT, capture_output=True, text=True)
                    self.assertEqual(result.returncode, 2, result.stderr)
                    self.assertIn('firmware:', result.stderr)
                    self.assertNotIn('Traceback', result.stderr)

    def test_optional_coverage_metadata_has_consistent_defaults(self):
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / 'coverage.toml'
            manifest.write_text('schema_version=1\n[[images]]\nname="venus.mbn"\n'
                                'size=8\ncredit_bytes=0\nsha256="' + '0' * 64 + '"\n'
                                '[images.executable_load]\nfile_base=0\nruntime_end=8\n')
            self.assertIn('0x0–0x8', status(load(manifest)))
            result = subprocess.run([sys.executable, '-m', 'tools.firmware',
                                     '--manifest', str(manifest), 'coverage', '--json'],
                                    cwd=ROOT, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(json.loads(result.stdout)[0]['credit_kind'], 'unspecified')

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

    def test_all_complete_source_builds_match_pinned_hashes_reproducibly(self):
        executable = self._firmware_tools_binary()
        with tempfile.TemporaryDirectory() as directory:
            for image in self.data['images']:
                if not image.get('complete_source', False):
                    continue
                with self.subTest(image=image['name']):
                    outputs = []
                    for run in range(2):
                        output = Path(directory) / str(run) / image['name']
                        process = subprocess.run(
                            [sys.executable, '-m', 'tools.firmware', 'build', image['name'],
                             '--output', str(output), '--firmware-tools', str(executable)],
                            cwd=ROOT, capture_output=True, text=True)
                        self.assertEqual(process.returncode, 0, process.stderr)
                        content = output.read_bytes()
                        self.assertEqual(len(content), image['size'])
                        self.assertEqual(hashlib.sha256(content).hexdigest(), image['sha256'])
                        outputs.append(content)
                    self.assertEqual(*outputs)

    def test_venus_sparse_cli_rejects_wrong_reference(self):
        executable = self._firmware_tools_binary()
        image = next(x for x in self.data['images'] if x['name'] == 'venus.mbn')
        with tempfile.TemporaryDirectory() as directory:
            reference = Path(directory) / 'venus.mbn'
            for content, error in ((b'', 'wrong full Venus reference length'),
                                   (bytes(image['size']), 'wrong pinned Venus reference SHA-256')):
                with self.subTest(error=error):
                    reference.write_bytes(content)
                    result = subprocess.run([str(executable), 'verify-venus-sparse', str(reference)],
                                            capture_output=True, text=True)
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn(error, result.stderr)
                    self.assertEqual(result.stdout, '')
                    self.assertEqual(reference.read_bytes(), content)
            self.assertEqual(list(Path(directory).iterdir()), [reference])

    def test_venus_sparse_cli_matches_optional_pinned_reference(self):
        executable = self._firmware_tools_binary()
        reference = os.environ.get('NABU_VENUS_REFERENCE') or os.environ.get('VENUS_REFERENCE')
        if reference is None:
            self.skipTest('set NABU_VENUS_REFERENCE to test pinned Venus sparse verification')
        image = next(x for x in self.data['images'] if x['name'] == 'venus.mbn')
        content = Path(reference).read_bytes()
        with tempfile.TemporaryDirectory() as directory:
            copied_reference = Path(directory) / 'venus.mbn'
            copied_reference.write_bytes(content)
            result = subprocess.run([str(executable), 'verify-venus-sparse', str(copied_reference)],
                                    cwd=directory, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            counts = result.stdout.split(': ', 1)[1].split()
            generated, missing, total = (int(counts[index].lstrip('(')) for index in (0, 3, 6))
            self.assertEqual(generated, image['credit_bytes'])
            self.assertGreater(missing, 0)
            self.assertEqual(generated + missing, image['size'])
            self.assertEqual(total, image['size'])
            self.assertIn(f'{generated} generated bytes, {missing} missing bytes', result.stdout)
            self.assertIn('Incomplete firmware; no complete image produced.', result.stdout)
            self.assertEqual(copied_reference.read_bytes(), content)
            self.assertEqual(list(Path(directory).iterdir()), [copied_reference])
            changed = bytearray(content)
            changed[0] ^= 1
            copied_reference.write_bytes(changed)
            rejected = subprocess.run([str(executable), 'verify-venus-sparse', str(copied_reference)],
                                      capture_output=True, text=True)
            self.assertNotEqual(rejected.returncode, 0)
            self.assertIn('wrong pinned Venus reference SHA-256', rejected.stderr)
            self.assertEqual(rejected.stdout, '')

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
