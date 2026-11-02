# SPDX-License-Identifier: GPL-3.0-only
"""Contract tests for reference-assisted (not source-only) ILM construction.

Test fixtures use invented bytes. No original OEM calibration is embedded in
this repository, and the production command keeps all digests pinned.
"""
import copy
import hashlib
import importlib.util
from pathlib import Path
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[1]
TOOL=ROOT/'tools/devices/xiaomi-nabu/assemble_novatek_upper.py'
SPEC=importlib.util.spec_from_file_location('novatek_upper_checked_assembly',TOOL)
ASM=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ASM)


def fixture():
    intervals = [
        (0x10000,0x10016,22,0,0), (0x10016,0x17dc0,32102,67,1),
        (0x17dc0,0x185c0,2048,0,0), (0x185c0,0x18620,0,96,0),
        (0x195b0,0x19614,0,100,0), (0x19614,0x19678,0,100,0),
        (0x19678,0x196b8,0,64,0), (0x196b8,0x196f8,0,64,0),
        (0x196f8,0x198dc,0,484,0), (0x198dc,0x19994,0,182,2),
        (0x19994,0x19a4c,0,182,2), (0x19a4c,0x19b04,0,182,2),
        (0x19b04,0x19bbc,0,182,2), (0x19bbc,0x19bf0,0,52,0),
        (0x19bf0,0x1ac80,0,4240,0),
    ]
    fragments=[];files={}
    for i,(start,end,code,data,fill) in enumerate(intervals):
        filename=f'dummy-{i:02}.bin'
        fragments.append(dict(file=filename,file_start=start,file_end=end,
                              executable_bytes=code,data_bytes=data,layout_fill_bytes=fill))
        files[filename]=bytes([i+1])*(end-start)
    calibration=bytes([(index*17+5) & 255 for index in range(ASM.CAL_END-ASM.CAL_START)])
    source_blob=b''.join(files[f['file']] for f in fragments)
    combined=(b''.join(files[f['file']] for f in fragments[:4])+calibration+
              b''.join(files[f['file']] for f in fragments[4:]))
    manifest=dict(reserved_start=ASM.START,reserved_end=ASM.END,
                  covered_bytes=40176,complete_image=False,checkpoint50_integration=False,
                  fragments=fragments,unresolved=[dict(file_start=ASM.CAL_START,
                  file_end=ASM.CAL_END,bytes=len(calibration))])
    components=[(start,end,ASM.digest(calibration[start-ASM.CAL_START:end-ASM.CAL_START]))
                for start,end,_ in ASM.CAL_GROUPS]
    return manifest,files,calibration,combined,dict(source_hash=ASM.digest(source_blob),
        complete_hash=ASM.digest(combined),calibration_hash=ASM.digest(calibration),
        calibration_parts=components)


class CheckedExternalCalibration(unittest.TestCase):
    def test_reference_assisted_output_has_precise_provenance(self):
        manifest,files,calibration,expected,hashes=fixture()
        actual,report=ASM.assemble_source_and_calibration(manifest,files,calibration,**hashes)
        self.assertEqual(actual,expected)
        self.assertEqual(len(actual),44160)
        self.assertEqual((report['source_generated_bytes'],report['external_original_calibration_bytes']),
                         (40176,3984))
        self.assertFalse(report['source_only_complete'])
        self.assertFalse(report['full_firmware_complete'])
        self.assertTrue(report['uses_reference_calibration'])

    def test_hash_and_length_protection_fail_closed(self):
        manifest,files,calibration,expected,hashes=fixture()
        with self.assertRaisesRegex(ValueError,'has 3983 bytes'):
            ASM.assemble_source_and_calibration(manifest,files,calibration[:-1],**hashes)
        altered=bytearray(calibration);altered[3]^=0x80
        with self.assertRaisesRegex(ValueError,'SHA-256'):
            ASM.assemble_source_and_calibration(manifest,files,bytes(altered),**hashes)
        tampered=files.copy(); name=manifest['fragments'][0]['file']
        tampered[name]=bytes([7])*len(files[name])
        with self.assertRaisesRegex(ValueError,'source-fragment hash'):
            ASM.assemble_source_and_calibration(manifest,tampered,calibration,**hashes)

    def test_reject_missing_duplicate_out_of_order_and_fabricated_padding(self):
        manifest,files,calibration,_,hashes=fixture()
        damaged=copy.deepcopy(manifest)
        damaged['unresolved']=[]
        with self.assertRaisesRegex(ValueError,'only the pinned'):
            ASM.assemble_source_and_calibration(damaged,files,calibration,**hashes)
        damaged=copy.deepcopy(manifest)
        damaged['fragments'][4]['file_start']-=4
        with self.assertRaisesRegex(ValueError,'nonadjacent'):
            ASM.assemble_source_and_calibration(damaged,files,calibration,**hashes)
        damaged=copy.deepcopy(manifest)
        damaged['fragments'][0]['file']='../outside.bin'
        with self.assertRaisesRegex(ValueError,'unsafe'):
            ASM.assemble_source_and_calibration(damaged,files,calibration,**hashes)
        damaged=copy.deepcopy(manifest)
        damaged['fragments'][1]['layout_fill_bytes']+=1
        with self.assertRaisesRegex(ValueError,'bad source byte classes'):
            ASM.assemble_source_and_calibration(damaged,files,calibration,**hashes)

    def test_external_calibration_is_not_allowed_as_committed_output(self):
        inside=ROOT/'do-not-emit-calibration.bin'
        with self.assertRaisesRegex(ValueError,'inside Git'):
            ASM.refuse_inside_git_tree(inside)
        with tempfile.TemporaryDirectory() as td:
            f=Path(td)/'assembled.bin'
            ASM.refuse_inside_git_tree(f)
            ASM.write_atomic(f,bytes([1,2,3]))
            self.assertEqual(f.read_bytes(),bytes([1,2,3]))

    def test_known_hashes_and_segments_are_pinned_to_one_firmware_revision(self):
        self.assertEqual(ASM.PINNED_CAL_SHA256,
             'b61f09fc42c18ff4f1bc768ed1f59657a57f5e3d16d473aaeb3b3c94005b5007')
        self.assertEqual(sum(z-a for a,z,_ in ASM.CAL_GROUPS),3984)
        self.assertEqual(ASM.CAL_GROUPS[0][0],ASM.CAL_START)
        self.assertEqual(ASM.CAL_GROUPS[-1][1],ASM.CAL_END)

if __name__=='__main__': unittest.main()
