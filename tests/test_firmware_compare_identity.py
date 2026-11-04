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


if __name__=='__main__':
    unittest.main()
