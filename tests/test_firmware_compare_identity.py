from __future__ import annotations

import hashlib
import io
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT))
from tools.build.firmware_coverage import CoverageError,verify_generated_region
from tools import firmware


class BoundedStream(io.BytesIO):
    def read(self,size=-1):
        if not 0<size<=1024 * 1024:
            raise AssertionError(f'unbounded read: {size}')
        return super().read(size)


class FirmwareCompareIdentityTests(unittest.TestCase):
    def setUp(self):
        self.directory=tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.reference=Path(self.directory.name)/'reference'
        self.generated=Path(self.directory.name)/'generated'
        self.content=bytes(range(256))*8193
        self.offset=37
        self.reference.write_bytes(self.content)
        self.generated.write_bytes(self.content[self.offset:-19])
        self.data={'images':[{'name':'image.bin','size':len(self.content),
                             'sha256':hashlib.sha256(self.content).hexdigest()}]}

    def verify(self):
        return verify_generated_region(self.data,'image.bin',self.reference,
                                       self.generated,self.offset)

    def test_success_hashes_full_reference_with_bounded_reads(self):
        path_open=Path.open
        streams=[]

        def open_stream(path,*args,**kwargs):
            if path==self.reference:
                stream=BoundedStream(self.content)
                streams.append(stream)
                return stream
            return path_open(path,*args,**kwargs)

        with mock.patch.object(Path,'open',open_stream):
            result=self.verify()
        self.assertEqual(len(streams),1)
        self.assertTrue(result['matched'])
        self.assertEqual(result['generated_bytes'],len(self.content)-self.offset-19)
        self.assertEqual(result['generated_sha256'],
                         hashlib.sha256(self.content[self.offset:-19]).hexdigest())

    def test_reference_opens_swap_cannot_validate_unpinned_comparison(self):
        swapped=b'X'*len(self.content)
        self.generated.write_bytes(swapped[self.offset:-19])
        path_open=Path.open
        opens=0

        def open_stream(path,*args,**kwargs):
            nonlocal opens
            if path==self.reference:
                opens+=1
                return BoundedStream(self.content if opens==1 else swapped)
            return path_open(path,*args,**kwargs)

        with mock.patch.object(Path,'open',open_stream):
            with self.assertRaisesRegex(CoverageError,'difference at file 0x25'):
                self.verify()
        self.assertEqual(opens,1)

    def test_reference_stream_mutation_outside_region_is_rejected(self):
        content=self.content
        offset=self.offset
        region_length=len(content)-offset-19

        class MutatingStream(BoundedStream):
            def read(self,size=-1):
                if self.tell()==offset and size==min(region_length,1024 * 1024):
                    position=self.tell()
                    self.seek(-1,io.SEEK_END)
                    self.write(b'X')
                    self.seek(position)
                return super().read(size)

        path_open=Path.open

        def open_stream(path,*args,**kwargs):
            if path==self.reference:
                return MutatingStream(content)
            return path_open(path,*args,**kwargs)

        with mock.patch.object(Path,'open',open_stream):
            with self.assertRaisesRegex(CoverageError,'reference SHA-256 mismatch'):
                self.verify()

    def test_reference_prefix_and_suffix_differences_are_rejected(self):
        for index in (0,len(self.content)-1):
            with self.subTest(index=index):
                changed=bytearray(self.content)
                changed[index]^=1
                self.reference.write_bytes(changed)
                with self.assertRaisesRegex(CoverageError,'reference SHA-256 mismatch'):
                    self.verify()

    def test_reference_length_changes_are_rejected(self):
        for content in (self.content[:-1],self.content+b'X'):
            with self.subTest(length=len(content)):
                self.reference.write_bytes(content)
                with self.assertRaisesRegex(CoverageError,'reference size .* differs from pinned'):
                    self.verify()

    def test_difference_offsets_across_chunk_boundary(self):
        for index in (0,1024 * 1024,1024 * 1024+7):
            with self.subTest(index=index):
                changed=bytearray(self.content[self.offset:-19])
                changed[index]^=1
                self.generated.write_bytes(changed)
                with self.assertRaisesRegex(CoverageError,
                        f'difference at file 0x{self.offset+index:x}$'):
                    self.verify()

    def test_generated_length_change_is_rejected(self):
        path_open=Path.open

        def open_stream(path,*args,**kwargs):
            if path==self.generated:
                return BoundedStream(self.content[self.offset:-20])
            return path_open(path,*args,**kwargs)

        with mock.patch.object(Path,'open',open_stream):
            with self.assertRaisesRegex(CoverageError,'generated file changed during verification'):
                self.verify()


class FirmwareReproducibilityTests(unittest.TestCase):
    def setUp(self):
        self.ledger=firmware.load()
        self.images=[image for image in self.ledger['images'] if image.get('complete_source')]
        self.payloads={image['name']:image['name'].encode() for image in self.images}
        for image in self.images:
            payload=self.payloads[image['name']]
            image.update(size=len(payload),sha256=hashlib.sha256(payload).hexdigest())
        self.targets=[]
        self.emissions=[]

    def run_stub(self,command,**kwargs):
        self.assertTrue(kwargs['check'])
        if command[0]=='cargo':
            self.assertEqual(command[:2],['cargo','build'])
            self.assertIn('--offline',command)
            self.assertIn('--locked',command)
            target=Path(command[command.index('--target-dir')+1])
            self.assertFalse(target.exists())
            self.assertEqual(kwargs['cwd'],ROOT)
            self.assertEqual(kwargs['env']['CARGO_TARGET_DIR'],str(target))
            self.assertEqual(kwargs['env']['CARGO_INCREMENTAL'],'0')
            self.assertNotIn('RUSTC_WRAPPER',kwargs['env'])
            self.targets.append(target)
            binary=target/'debug'/'firmware-tools'
            binary.parent.mkdir(parents=True)
            binary.touch()
        else:
            self.assertEqual(Path(command[0]),self.targets[-1]/'debug'/'firmware-tools')
            if command[1]=='rebuild-modemuw':
                name='modemuw.jsn'
            elif command[1]=='rebuild-ath10k-metadata':
                name='firmware-5.bin'
            else:
                self.assertEqual(command[1],'rebuild-cirrus-tuning')
                name=f'{command[2]}-cs35l41-dsp1-spk-prot.bin'
            self.emissions.append((self.targets[-1],name))
            Path(command[-1]).write_bytes(self.payloads[name])

    def test_two_fresh_builds_emit_and_compare_all_six_images(self):
        with mock.patch.object(firmware.subprocess,'run',side_effect=self.run_stub), \
             mock.patch.object(firmware.filecmp,'cmp',wraps=firmware.filecmp.cmp) as compare, \
             mock.patch.object(firmware,'load',return_value=self.ledger), \
             mock.patch('sys.stdout',new_callable=io.StringIO):
            self.assertEqual(firmware.main(['verify-reproducible']),0)
        self.assertEqual(len(self.targets),2)
        self.assertNotEqual(*self.targets)
        self.assertEqual(len(self.emissions),12)
        self.assertEqual(compare.call_count,6)
        self.assertTrue(all(call.kwargs=={'shallow':False} for call in compare.call_args_list))
        self.assertTrue(all(not target.parent.exists() for target in self.targets))

    def test_bad_builder_output_fails_pinned_verification_and_cleans_up(self):
        self.payloads[self.images[0]['name']]=b'bad'
        with mock.patch.object(firmware.subprocess,'run',side_effect=self.run_stub):
            with self.assertRaisesRegex(CoverageError,'generated size/SHA-256'):
                firmware.verify_reproducible(self.ledger)
        self.assertEqual(len(self.targets),1)
        self.assertFalse(self.targets[0].parent.exists())

    def test_second_cargo_failure_cleans_both_builds(self):
        def fail_second(command,**kwargs):
            if command[0]=='cargo' and self.targets:
                raise firmware.subprocess.CalledProcessError(1,command)
            self.run_stub(command,**kwargs)
        with mock.patch.object(firmware.subprocess,'run',side_effect=fail_second):
            with self.assertRaises(firmware.subprocess.CalledProcessError):
                firmware.verify_reproducible(self.ledger)
        self.assertFalse(self.targets[0].parent.parent.exists())

    def test_comparison_failure_is_rejected_and_cleaned(self):
        with mock.patch.object(firmware.subprocess,'run',side_effect=self.run_stub), \
             mock.patch.object(firmware.filecmp,'cmp',return_value=False):
            with self.assertRaisesRegex(CoverageError,'independent build outputs differ'):
                firmware.verify_reproducible(self.ledger)
        self.assertTrue(all(not target.parent.exists() for target in self.targets))


if __name__=='__main__':
    unittest.main()
