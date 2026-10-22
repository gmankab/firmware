# SPDX-License-Identifier: GPL-3.0-only
"""Synthetic unit tests. No vendor executable or signature bytes included."""
from __future__ import annotations
import hashlib
import importlib.util
from pathlib import Path
import struct
import tempfile
import unittest

SCRIPTS = Path(__file__).resolve().parents[1] / 'tools'

def import_script(name: str):
    spec = importlib.util.spec_from_file_location(name.replace('-', '_').replace('/', '_'), SCRIPTS / name)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

elf = import_script('elf-segment-view.py')
provenance = import_script('devices/xiaomi-nabu/verify-apk-sources.py')
metadata_generator = import_script('generate-elf-specs.py')
byte_budget = import_script('elf-byte-budget.py')

class ElfViewTests(unittest.TestCase):
    def make_elf(self) -> bytes:
        b = bytearray(256)
        ident = b'\x7fELF\x01\x01\x01' + b'\0' * 9
        struct.pack_into(elf.ELF32_HDR, b, 0, ident, 2, 164, 1, 0x5000,
                         52, 0, 3, 52, 32, 1, 0, 0, 0)
        struct.pack_into(elf.PHDR, b, 52, 1, 128, 0x5000, 0x5000, 16,
                         16, 7, 0x1000)
        b[128:144] = bytes(range(16))  # deliberately synthetic dummy data
        return bytes(b)

    def test_disassemblable_view_preserves_binary_data(self):
        orig = self.make_elf()
        result, infos = elf.sectionize(orig)
        self.assertEqual(len(infos), 1)
        self.assertEqual(infos[0]['address'], 0x5000)
        self.assertEqual(infos[0]['file_offset'], 128)
        self.assertEqual(result[52:len(orig)], orig[52:])
        self.assertEqual(result[128:144], bytes(range(16)))
        fields = struct.unpack_from(elf.ELF32_HDR, result)
        self.assertEqual(fields[12], 3)  # null, text, shstrtab
        sht = fields[6]
        self.assertEqual(struct.unpack_from('<I', result, sht + 40 + 8)[0], 6)

    def test_rejects_bad_truncated_elf(self):
        with self.assertRaises(ValueError):
            elf.sectionize(self.make_elf()[:65])

class FirmwareProvenanceTests(unittest.TestCase):
    names = ['a640_zap.mbn', 'adsp.mbn', 'cdsp.mbn', 'modem.mbn',
             'venus.mbn', 'wlanmdsp.mbn', 'modemuw.jsn',
             'cs35l41-dsp1-spk-prot.wmfw', 'novatek_nt36523_fw.bin']

    def create_inputs(self, root: Path):
        hashes = []
        for name in self.names:
            repo = provenance.SPECIAL_SOURCES.get(name, provenance.DEFAULT_SOURCE)
            folder = root / repo
            folder.mkdir(exist_ok=True)
            payload = b'dummy unit test data: ' + name.encode()
            (folder / name).write_bytes(payload)
            hashes.append(f'{hashlib.sha256(payload).hexdigest()}  ./mock/{name}')
        hashfile = root / 'hashes.txt'
        hashfile.write_text('\n'.join(hashes) + '\n')
        # The other repository has a different revision of the same file.
        (root / provenance.DEFAULT_SOURCE / 'novatek_nt36523_fw.bin').write_bytes(b'another synthetic version')
        return hashfile

    def test_select_correct_sources(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            records = provenance.check_sources(self.create_inputs(root), root)
            self.assertEqual(len(records), 9)
            self.assertTrue(all(r['matches_package'] for r in records))
            nv = next(x for x in records if x['file'] == 'novatek_nt36523_fw.bin')
            self.assertEqual(nv['source'], 'postmarketos-nabu-firmware')

    def test_detect_corrupt_source(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            h = self.create_inputs(root)
            (root / provenance.DEFAULT_SOURCE / 'a640_zap.mbn').write_bytes(b'corrupted')
            result = provenance.check_sources(h, root)
            self.assertFalse(next(x for x in result if x['file'] == 'a640_zap.mbn')['matches_package'])

class GenericMetadataTests(unittest.TestCase):
    @staticmethod
    def sample_named_image() -> bytes:
        # Synthetic ELF32LE of a fictitious future device. No vendor bytes.
        b = bytearray(4096 + 64)
        ident = b'\x7fELF\x01\x01\x01' + bytes(9)
        struct.pack_into('<16sHHIIIIIHHHHHH', b, 0,
                         ident, 2, 40, 1, 0x8000, 52, 0, 0,
                         52, 32, 1, 40, 0, 0)
        struct.pack_into('<IIIIIIII', b, 52,
                         1, 4096, 0x8000, 0x8000, 64, 64, 5, 4096)
        for i in range(64):
            b[4096+i] = (i * 3) & 0xff
        return bytes(b)

    def test_specs_work_for_an_unlisted_future_device(self):
        with tempfile.TemporaryDirectory() as td:
            source = Path(td) / 'completely_new_board.elf'
            source.write_bytes(self.sample_named_image())
            spec_lines = metadata_generator.spec(source)
            generated = '\n'.join(spec_lines)
            self.assertIn('completely_new_board.elf', generated)
            self.assertIn('machine: 40', generated)
            self.assertIn('offset: 0x1000', generated)

    def test_byte_budget_uses_actual_ranges_for_generic_elf(self):
        with tempfile.TemporaryDirectory() as td:
            source = Path(td) / 'different_board.mbn'
            source.write_bytes(self.sample_named_image())
            m = byte_budget.inventory(source)
            self.assertEqual(m['file'], source.name)
            self.assertEqual(m['first_page_accounted_bytes'], 4096)
            self.assertEqual(m['classes']['executable_load'], 64)
            self.assertEqual(m['classes']['header'], 4096)
            self.assertEqual(m['length'], 4160)
            self.assertEqual(m['unmapped_nonzero_count'], 0)

if __name__ == '__main__':
    unittest.main()
