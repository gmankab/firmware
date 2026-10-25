# SPDX-License-Identifier: GPL-3.0-only
"""Late ADSP executable source reconstruction and bounds/provenance tests."""
from __future__ import annotations

import hashlib
import importlib.util
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'tools' / 'adsp-late-callback-drain.S'
SCRIPT = ROOT / 'tools' / 'emit-adsp-late.py'

spec = importlib.util.spec_from_file_location('adsp_late', SCRIPT)
assert spec is not None and spec.loader is not None
adsp = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = adsp
spec.loader.exec_module(adsp)

LLVM_MC = os.environ.get('HEXAGON_LLVM_MC')
REFERENCE = os.environ.get('NABU_ADSP_REFERENCE')
RIZIN_ASM = os.environ.get('RZ_HEXAGON_ASM')
EXPECTED_DIGEST = 'cc93ab6de47bc8514f946f427a059bc279975b4e99b9cfdc0be450f9d559a233'
ZERO_MEMORY_DIGEST = 'd8c1f7e013d349af5011c3a1a99db6e19689d99bb2fb7c459ed346625f550f1c'
CONTROLLER_DIGEST = '2b6f8caaaded06ece47aadd8e97090ca9269ac2f7f7e16242e21094f128aadbf'
STARTUP_DIGEST = '1c8e4b6d1b7c128032440a941956f3d45ae2968b9b17b36b2acccf5f67d5db03'
JOINED_STARTUP_DIGEST = '8ae46f2b16461df880aff54fcd6bfba13587266045ae619997286b8393524d72'


class ADSPExternalsAndCodeTests(unittest.TestCase):
    def test_source_is_original_semantic_assembly(self):
        src = SOURCE.read_text()
        for code in ('allocframe(#8)', 'memw(r29+#0) = r27',
                     'drain_callbacks:', 'next_slot:', 'jump next_slot',
                     'p0 = cmp.eq(r0,#0)', 'deallocframe', 'jumpr lr'):
            self.assertIn(code, src)
        for forbidden in ('.word ', '.long ', '.byte ', '.incbin ', '.fill '):
            self.assertNotIn(forbidden, src)

    def test_exact_executable_segments_and_code_boundaries(self):
        self.assertEqual(adsp.SEGMENTS[0][:4],
                         (0x39000, 0xf0036000, 0x8be36000, 0x1b97e8))
        self.assertEqual(adsp.SEGMENTS[1][:4],
                         (0x364000, 0xb0000000, 0x8c500000, 0x94b790))
        _src, begin, size = adsp.FRAGMENTS['queue-drain']
        self.assertEqual((begin, begin+size), (0x39000,0x39054))
        self.assertTrue(begin+size <= adsp.SEGMENTS[0][0]+adsp.SEGMENTS[0][3])
        self.assertFalse(adsp.SEGMENTS[0][0] <= 0x364000 < adsp.SEGMENTS[0][0]+adsp.SEGMENTS[0][3])

    def test_checked_bounds_reject_malformed_ranges(self):
        for offset, length in [(-1,0),(0,-1),(17,0),(1,17),(20,5)]:
            with self.subTest(offset=offset,length=length):
                with self.assertRaises(adsp.LateAdspError):
                    adsp.checked_range(16,offset,length)
        adsp.checked_range(16,0,16)

    def test_non_elf_and_wrong_revision_rejected_before_compare(self):
        with self.assertRaisesRegex(adsp.LateAdspError,'ELF32'):
            adsp.validate_reference(b'not an ELF')

    @unittest.skipUnless(LLVM_MC, 'Hexagon LLVM6 assembler required')
    def test_source_compiles_and_uses_correct_packet_order(self):
        built = adsp.compile_source(LLVM_MC, SOURCE, 0x54)
        self.assertEqual(len(built),0x54)
        self.assertEqual(hashlib.sha256(built).hexdigest(),EXPECTED_DIGEST)
        # Verify the special triple-immediate packet; order matters for duplex.
        self.assertNotEqual(built[0x18:0x20],b'\x00'*8)
        self.assertEqual(built[0x14:0x18], adsp.compile_source(LLVM_MC,SOURCE,0x54)[0x14:0x18])
        with self.assertRaisesRegex(adsp.LateAdspError, 'reference'):
            adsp.verify_slice(built,b'WRONG MBN',0x39000)
    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_zero_memory_source_is_signed_length_bounded(self):
        source = adsp.FRAGMENTS['zero-memory'][0]
        text=source.read_text()
        for expected in ('r3 = add(r0,#7)','r3 = insert(r4,#3,#0)',
                         'memd(r0++#8) = r5:4','memb(r0++#1) = r4',
                         'finished:', 'jumpr lr'):
            self.assertIn(expected,text)
        raw=adsp.compile_source(LLVM_MC,source,0x50)
        self.assertEqual(hashlib.sha256(raw).hexdigest(),ZERO_MEMORY_DIGEST)
        self.assertEqual(len(raw),80)

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_controller_dispatch_source_and_assembler(self):
        source,start,length=adsp.FRAGMENTS['controller-dispatch']
        text=source.read_text()
        for token in ('memw(##0xf02cb68c) = r3','memd(r29+#-8) = r1:0',
                      'call begin_controller','trap:', 'jumpr lr',
                      '.set seventh_controller'):
            self.assertIn(token,text)
        self.assertNotIn('.word ',text)
        self.assertNotIn('.incbin ',text)
        result=adsp.compile_source(LLVM_MC,source,length)
        self.assertEqual(hashlib.sha256(result).hexdigest(),CONTROLLER_DIGEST)
        self.assertEqual(length,120)

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_second_rx_startup_calls_zero_filler_at_exact_boundary(self):
        source,off,length=adsp.FRAGMENTS['rx-startup']
        self.assertEqual((off,length),(0x364000,168))
        body=source.read_text()
        for token in ('framelimit = r0', 'gp = r0', 'stop_forever:',
                      'call init_platform', '.set init_platform',
                      'adsp_late_rx_startup+0xa8'):
            self.assertIn(token,body)
        original_startup=adsp.compile_source(LLVM_MC,source,length)
        self.assertEqual(hashlib.sha256(original_startup).hexdigest(),STARTUP_DIGEST)
        filler_src, next_off, filllen=adsp.FRAGMENTS['zero-memory']
        self.assertEqual(off+length,next_off)
        self.assertEqual(len(original_startup),0xa8)
        composite=original_startup+adsp.compile_source(LLVM_MC,filler_src,filllen)
        self.assertEqual(len(composite),248)
        self.assertEqual(hashlib.sha256(composite).hexdigest(),JOINED_STARTUP_DIGEST)

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_additional_late_functions_compile_with_correct_digests(self):
        expected={
          'save-control':(0x390d8,36,'f5289532abf9a3fcc90a50e1c77615cd07a13dae6e70049739d8a1af46c8387c'),
          'clear-ready':(0x39100,20,'36e0a7e92392cef26762304bbc9755cb32aa8a9c1086ec5d84992030a041e992'),
          'callback-registry':(0x39120,128,'47c5dc248bef3234e0de96fd62ffbd04102e5a6f2ec346df1a6180b265e91775'),
        }
        for name,(off,size,digest) in expected.items():
            with self.subTest(fragment=name):
                source,source_off,source_len=adsp.FRAGMENTS[name]
                self.assertEqual((off,size),(source_off,source_len))
                body=source.read_text()
                for forbidden in ('.word ','.long ','.byte ','.incbin '):
                    self.assertNotIn(forbidden,body)
                built=adsp.compile_source(LLVM_MC,source,size)
                self.assertEqual(hashlib.sha256(built).hexdigest(),digest)

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_registry_lifecycle_functions_byte_layout(self):
        expected={
            'unregister-registry':(64,'df9c133d787c1b33d1247520b1aa2dcb6d59296610f0ba1aab9b48bea9d7aa98'),
            'initialize-registry':(80,'e57c396c397d4e9b9fa49302b8d12226bf8032d7ad73f253d7ed36e82246224e'),
        }
        for name,(length,digest) in expected.items():
            with self.subTest(name=name):
                source,offset,reported_len=adsp.FRAGMENTS[name]
                self.assertEqual(reported_len,length)
                output=adsp.compile_source(LLVM_MC,source,length)
                self.assertEqual(hashlib.sha256(output).hexdigest(),digest)
                self.assertNotIn('.word ',source.read_text())
        self.assertIn('loop0(copy_loop,r4)',
                      adsp.FRAGMENTS['callback-registry'][0].read_text())
        self.assertIn('init_registry',
                      adsp.FRAGMENTS['initialize-registry'][0].read_text())

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_registry_allocator_and_probe_digests(self):
        expected={
            'registry-allocator':(0x39230,152,'0e8f500eb05bb0f2e7e47bdd399938a00e6146adbab5c3881543ea3ee2bfe14c'),
            'registry-probe':(0x392c8,80,'3c2b3c17db9e33f7f22bf0b8b1d3e8728809d2734ecb5b4b6454a57dbe6a869f'),
        }
        for name,(off,size,digest) in expected.items():
            with self.subTest(name=name):
                src,ref_off,length=adsp.FRAGMENTS[name]
                self.assertEqual((off,size),(ref_off,length))
                rebuilt=adsp.compile_source(LLVM_MC,src,size)
                self.assertEqual(hashlib.sha256(rebuilt).hexdigest(),digest)
                self.assertIn('call',src.read_text())
                for forbidden in ('.word ','.long ','.byte ','.incbin '):
                    self.assertNotIn(forbidden,src.read_text())

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_complete_fragment_inventory_rejects_overlaps(self):
        manifest,regions=adsp.assembled_inventory(LLVM_MC,None)
        self.assertEqual(len(manifest['fragments']),11)
        self.assertEqual(manifest['total_independently_assembled_executable_bytes'],1012)
        self.assertEqual(sum(map(len,regions.values())),1012)
        self.assertEqual(manifest['gap_bytes_included_in_generated_executables'],0)
        self.assertFalse(manifest['complete_adsp_image'])
        self.assertEqual(manifest['external_original_code_bytes_in_output'],0)
        bounds=[(x['file_offset_start'],x['file_offset_end']) for x in manifest['fragments']]
        self.assertEqual(bounds[0],(0x39000,0x39054))
        self.assertEqual(bounds[-1],(0x3640a8,0x3640f8))

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         'Hexagon LLVM6 and exact pinned ADSP reference required')
    def test_inventory_every_fragment_uses_correct_original(self):
        inventory,_outputs=adsp.assembled_inventory(LLVM_MC,Path(REFERENCE).read_bytes())
        self.assertTrue(all(x['reference_byte_identical'] for x in inventory['fragments']))
        self.assertEqual(inventory['total_independently_assembled_executable_bytes'],1012)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         'Hexagon LLVM6 and exact pinned ADSP reference required')
    def test_all_eleven_disjoint_fragments_are_source_identical(self):
        original=Path(REFERENCE).read_bytes()
        total=0
        ranges=[]
        for name,(source,off,size) in adsp.FRAGMENTS.items():
            with self.subTest(fragment=name):
                built=adsp.compile_source(LLVM_MC,source,size)
                adsp.verify_slice(built,original,off)
                total+=len(built)
                ranges.append((off,off+size))
        self.assertEqual(total,1012)
        ranges.sort()
        self.assertEqual(ranges[0],(0x39000,0x39054))
        self.assertEqual(ranges[-1],(0x3640a8,0x3640f8))
        self.assertTrue(all(left[1]<=right[0] for left,right in zip(ranges,ranges[1:])))
        self.assertEqual(original[0x39054:0x39060],b'\0'*12)

    @unittest.skipUnless(LLVM_MC and RIZIN_ASM,'Independent Rizin Hexagon disassembler required')
    def test_callback_registry_rizin_recognizes_hardware_loop(self):
        source,_off,size=adsp.FRAGMENTS['callback-registry']
        built=adsp.compile_source(LLVM_MC,source,size)
        proc=subprocess.run([RIZIN_ASM,'-a','hexagon','-b','32','-d',built.hex()],
                           capture_output=True,text=True,check=False)
        self.assertEqual(proc.returncode,0,proc.stderr)
        text=proc.stdout.lower()
        self.assertIn('loop0',text)
        self.assertIn('memw',text)
        self.assertIn('dealloc',text)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         'Hexagon LLVM6 and exact pinned ADSP reference required')
    def test_second_rx_contiguous_bootstrap_and_memory_fill_reference(self):
        source,off,length=adsp.FRAGMENTS['rx-startup']
        memory_src, _, size=adsp.FRAGMENTS['zero-memory']
        composite=adsp.compile_source(LLVM_MC,source,length)+adsp.compile_source(LLVM_MC,memory_src,size)
        reference=Path(REFERENCE).read_bytes()
        adsp.verify_slice(composite,reference,off)
        mutated=bytearray(composite);mutated[0x74]^=1
        with self.assertRaisesRegex(adsp.LateAdspError, 'Mismatch'):
            adsp.verify_slice(bytes(mutated),reference,off)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         'Hexagon LLVM6 and exact pinned ADSP reference required')
    def test_controller_dispatch_exact_reference_bytes(self):
        source,start,length=adsp.FRAGMENTS['controller-dispatch']
        rebuilt=adsp.compile_source(LLVM_MC,source,length)
        reference=Path(REFERENCE).read_bytes()
        adsp.verify_slice(rebuilt,reference,start)
        mutated=bytearray(rebuilt);mutated[0x68]^=0x01
        with self.assertRaisesRegex(adsp.LateAdspError,'Mismatch'):
            adsp.verify_slice(bytes(mutated),reference,start)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         'Hexagon LLVM6 and exact pinned ADSP reference required')
    def test_zero_memory_second_segment_exact(self):
        source,start,length = adsp.FRAGMENTS['zero-memory']
        built=adsp.compile_source(LLVM_MC,source,length)
        adsp.verify_slice(built,Path(REFERENCE).read_bytes(),start)
        edited=bytearray(built)
        edited[32]^=2
        with self.assertRaisesRegex(adsp.LateAdspError,'Mismatch'):
            adsp.verify_slice(edited,Path(REFERENCE).read_bytes(),start)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         'Hexagon LLVM6 and exact pinned ADSP reference required')
    def test_pinned_reference_exact_identity_and_mutation(self):
        built=adsp.compile_source(LLVM_MC,SOURCE,0x54)
        ref=Path(REFERENCE).read_bytes()
        adsp.verify_slice(built,ref,0x39000)
        mutated=bytearray(built);mutated[0x18]^=1
        with self.assertRaisesRegex(adsp.LateAdspError, 'Mismatch'):
            adsp.verify_slice(bytes(mutated),ref,0x39000)
        wrong=bytearray(ref);wrong[0x39000]^=1
        with self.assertRaisesRegex(adsp.LateAdspError, 'Wrong original ADSP revision'):
            adsp.verify_slice(built,wrong,0x39000)
        with self.assertRaisesRegex(adsp.LateAdspError, 'outside ADSP executable'):
            adsp.verify_slice(built,ref,0x1f3000)

    @unittest.skipUnless(LLVM_MC and RIZIN_ASM,'Independent Rizin Hexagon disassembler required')
    def test_rizin_agrees_on_packet_and_callback_semantics(self):
        built=adsp.compile_source(LLVM_MC,SOURCE,0x54)
        proc=subprocess.run([RIZIN_ASM,'-a','hexagon','-b','32','-d',built.hex()],
                           capture_output=True,text=True,check=False)
        self.assertEqual(proc.returncode,0,proc.stderr)
        text=proc.stdout.lower()
        self.assertIn('allocframe',text)
        self.assertIn('memw',text)
        self.assertIn('callr',text)
        self.assertIn('deallocframe',text)
        self.assertIn('jumpr',text)


if __name__=='__main__':
    unittest.main()
