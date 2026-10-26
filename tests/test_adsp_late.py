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
    @unittest.skipUnless(LLVM_MC, 'LLVM6 Hexagon assembler required')
    def test_three_adjacent_controller_helper_functions(self):
        expected={
          'packet-type-check':(0x3d048,20,'a51007446642cff9413dd1ec00a9cc0c114e7053ba7532039fdd24665bd3a099'),
          'stack-marshaller':(0x3d05c,60,'ead3c85cb8d40d603887c843ecd41f24b0b7d8bf015aa1287cd4e34125218912'),
          'controller-shutdown-adapter':(0x3d098,48,'661c544275b730cc2e4e6208660b3680404b019e419c62f20251cad398966a05'),
        }
        outputs=[]
        for name,(off,size,digest) in expected.items():
            with self.subTest(name=name):
                src,begin,length=adsp.FRAGMENTS[name]
                self.assertEqual((begin,length),(off,size))
                text=src.read_text()
                for forbidden in ('.word ','.long ','.byte ','.incbin ','.fill '):
                    self.assertNotIn(forbidden,text)
                b=adsp.compile_source(LLVM_MC,src,length)
                self.assertEqual(hashlib.sha256(b).hexdigest(),digest)
                if REFERENCE: adsp.verify_slice(b,Path(REFERENCE).read_bytes(),begin)
                outputs.append(b)
        self.assertEqual(len(b''.join(outputs)),0x80)
        self.assertEqual(adsp.FRAGMENTS['packet-type-check'][1]+20,0x3d05c)
        self.assertEqual(adsp.FRAGMENTS['stack-marshaller'][1]+60,0x3d098)
        self.assertEqual(adsp.FRAGMENTS['controller-shutdown-adapter'][1]+48,0x3d0c8)
        self.assertIn('if (!p0.new) jump:t next_function',adsp.FRAGMENTS['packet-type-check'][0].read_text())
        self.assertIn('memw(r29+#20) = r6.new',adsp.FRAGMENTS['stack-marshaller'][0].read_text())
        self.assertIn('jump platform_tail',adsp.FRAGMENTS['controller-shutdown-adapter'][0].read_text())

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
        self.assertEqual(len(manifest['fragments']),66)
        self.assertEqual(manifest['total_independently_assembled_executable_bytes'],3956)
        self.assertEqual(sum(map(len,regions.values())),3956)
        self.assertEqual(manifest['gap_bytes_included_in_generated_executables'],0)
        self.assertFalse(manifest['complete_adsp_image'])
        self.assertEqual(manifest['external_original_code_bytes_in_output'],0)
        bounds=[(x['file_offset_start'],x['file_offset_end']) for x in manifest['fragments']]
        self.assertEqual(bounds[0],(0x39000,0x39054))
        self.assertEqual(bounds[-1],(0x3640a8,0x3640f8))

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_indexed_callback_copy_and_atomic_lock_loop(self):
        for name,expected_size,pattern in [('copy-registry-slot',16,'r2.new'),
                                           ('atomic-add',20,'memw_locked')]:
            with self.subTest(fragment=name):
                src,off,size=adsp.FRAGMENTS[name]
                self.assertEqual(size,expected_size)
                self.assertIn(pattern,src.read_text())
                rebuilt=adsp.compile_source(LLVM_MC,src,size)
                self.assertEqual(len(rebuilt),size)
        src=adsp.FRAGMENTS['atomic-add'][0].read_text()
        self.assertIn('retry:',src)
        self.assertIn('if (!p0) jump:nt retry',src)
        self.assertIn('memw_locked(r2,p0) = r4',src)

    @unittest.skipUnless(LLVM_MC,'LLVM6 Hexagon assembler required')
    def test_quro_buffer_handoff_packetized_traps_and_gp_status(self):
        source,off,length=adsp.FRAGMENTS['buffer-handoff']
        self.assertEqual((off,length),(0x394b8,136))
        text=source.read_text()
        for x in ('immext(#0xf01f1000)', 'if (cmp.eq(r2.new,#0))',
                  'memw(gp+##4028567592) = r2.new', 'trap0(#0)',
                  'memd(sp+#24)', 'setup_second:', 'dealloc_return'):
            self.assertIn(x,text)
        self.assertEqual(text.count('trap0(#0)'),3)
        for forbidden in ('.word ','.long ','.byte ','.incbin ','.fill '):
            self.assertNotIn(forbidden,text)
        assembled=adsp.compile_source(LLVM_MC,source,length)
        self.assertEqual(hashlib.sha256(assembled).hexdigest(),
                         'a342e82ea1f2a1b11b39c7358d9b44cf68b4002301dfb29c8a712764c3cafe1c')
        self.assertEqual(len(assembled),136)

    @unittest.skipUnless(LLVM_MC,'LLVM6 Hexagon assembler required')
    def test_buffer_publication_and_conditional_platform_probe(self):
        expected={
          'buffer-publish':(0x39540,40,'9e4563419e1af444327862bf3d9e8e1c00742da846a8aac6669b962e8100bdc2'),
          'conditional-query':(0x39568,32,'7a5cc3040a5578396a857dc6b05209db853d9a6496640e640063f61860dfa319'),
        }
        for name,(off,size,digest) in expected.items():
            with self.subTest(name=name):
                source,start,length=adsp.FRAGMENTS[name]
                self.assertEqual((start,length),(off,size))
                text=source.read_text()
                for forbidden in ('.word ','.long ','.byte ','.incbin ','.fill '):
                    self.assertNotIn(forbidden,text)
                rebuilt=adsp.compile_source(LLVM_MC,source,size)
                self.assertEqual(hashlib.sha256(rebuilt).hexdigest(),digest)
        self.assertIn('mpyi(',adsp.FRAGMENTS['buffer-publish'][0].read_text())
        self.assertIn('if (p0.new) jump:t retry',adsp.FRAGMENTS['buffer-publish'][0].read_text())
        self.assertIn('if (!p0) call platform_check',adsp.FRAGMENTS['conditional-query'][0].read_text())
        # Unlike independent disjoint functions, these last three are adjacent.
        spans=sorted((adsp.FRAGMENTS[name][1],adsp.FRAGMENTS[name][1]+adsp.FRAGMENTS[name][2])
            for name in ('buffer-handoff','buffer-publish','conditional-query'))
        self.assertEqual(spans,[(0x394b8,0x39540),(0x39540,0x39568),(0x39568,0x39588)])

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 required')
    def test_prefix_comparison_and_overlap_copy(self):
        expected={
          'byte-prefix-compare':(0x39590,48,'7665e8ac2959a11688d253c68fa8bb5096213c161d25a6b907b21e866fc1d91b'),
          'bounded-copy':(0x395c0,60,'09e58d0402e5b93a33da381494996e0bff8d9e8bb46fe87b36fa31b9c3792bda'),
        }
        for name,(off,size,digest) in expected.items():
            with self.subTest(name=name):
                src,begin,count=adsp.FRAGMENTS[name]
                self.assertEqual((begin,count),(off,size))
                text=src.read_text()
                for forbidden in ('.word ','.long ','.byte ','.incbin ','.fill '):
                    self.assertNotIn(forbidden,text)
                assembled=adsp.compile_source(LLVM_MC,src,size)
                self.assertEqual(hashlib.sha256(assembled).hexdigest(),digest)
        self.assertIn('memb(r3++#1)',adsp.FRAGMENTS['byte-prefix-compare'][0].read_text())
        cpy=adsp.FRAGMENTS['bounded-copy'][0].read_text()
        self.assertIn('loop0(copy_bytes,r16)',cpy)
        self.assertIn('memb(r0++#1) = r2.new',cpy)
        self.assertIn(':endloop0',cpy)
        self.assertIn('call platform_overlap_copy',cpy)
        # The eight-byte pair of NOPs between adjacent callables is not code.
        self.assertEqual(adsp.FRAGMENTS['conditional-query'][1]+32,0x39588)
        self.assertEqual(adsp.FRAGMENTS['byte-prefix-compare'][1],0x39590)
        self.assertEqual(adsp.FRAGMENTS['bounded-copy'][1],0x395c0)

    @unittest.skipUnless(LLVM_MC, 'LLVM6 Hexagon assembler required')
    def test_late_format_and_callback_helpers_source_build(self):
        # These functions are deliberately distinct from [0x39600, 0x3a86c)
        # whose complex manager has NOT been source reconstructed yet.
        expected = {
            'status-nibbles': (0x3a86c, 96, ('extractu(r1,#4,#24)', 'memb(r0+#8) = #0', 'jumpr r31')),
            'activate-context': (0x3ab00, 40, ('call platform_late_context', 'memw(r16+#4) = #10')),
            'return-mode-two': (0x3ab28, 4, ('r0 = #2', 'jumpr r31')),
            'guarded-counter': (0x3ab2c, 64, ('r4 = ugp', 'memw(r2+#0) += #1')),
            'probe-callbacks': (0x3ab6c, 72, ('callr r3', 'r17:16 = memd(r29+#0)')),
        }
        for name, (start, size, required) in expected.items():
            with self.subTest(name=name):
                source, offset, length=adsp.FRAGMENTS[name]
                self.assertEqual((offset, length), (start, size))
                body=source.read_text()
                for forbidden in ('.long ', '.word ', '.byte ', '.fill ', '.incbin '):
                    self.assertNotIn(forbidden, body)
                for token in required:
                    self.assertIn(token, body)
                generated=adsp.compile_source(LLVM_MC, source, size)
                self.assertEqual(len(generated), size)
                self.assertNotEqual(generated, bytes(size))
        # Four distinct contiguous functions totaling 180 bytes, no padding.
        fours=sorted((adsp.FRAGMENTS[name][1],adsp.FRAGMENTS[name][1]+adsp.FRAGMENTS[name][2])
                    for name in ('activate-context','return-mode-two','guarded-counter','probe-callbacks'))
        self.assertEqual(fours,[(0x3ab00,0x3ab28),(0x3ab28,0x3ab2c),
                                (0x3ab2c,0x3ab6c),(0x3ab6c,0x3abb4)])

    @unittest.skipUnless(LLVM_MC and RIZIN_ASM, 'Independent Hexagon decoder required')
    def test_status_word_and_callback_control_flow_independent_rizin(self):
        for name, expected in [('status-nibbles','extractu'),('probe-callbacks','callr'),
                               ('guarded-counter','memw')]:
            with self.subTest(name=name):
                source,off,size=adsp.FRAGMENTS[name]
                raw=adsp.compile_source(LLVM_MC, source, size)
                result=subprocess.run([RIZIN_ASM,'-a','hexagon','-b','32','-d',raw.hex()],
                                      capture_output=True,text=True,check=False)
                self.assertEqual(result.returncode,0,result.stderr)
                self.assertIn(expected,result.stdout.lower())

    @unittest.skipUnless(LLVM_MC, 'Hexagon LLVM6 assembler required')
    def test_configure_session_uses_correct_nonshuffled_memory_packet(self):
        source,start,size=adsp.FRAGMENTS['configure-session']
        self.assertEqual((start,size),(0x3aa80,128))
        text=source.read_text()
        self.assertIn('{ r2 = #32; r1 = r17; r16 = r0 }',text)
        self.assertIn('call platform_allocate',text)
        self.assertIn('retry_fallback:',text)
        for forbidden in ('.word ', '.long ', '.byte ', '.incbin ', '.fill '):
            self.assertNotIn(forbidden,text)
        original=adsp.compile_source(LLVM_MC,source,size)
        self.assertEqual(len(original),128)
        cluster=[adsp.FRAGMENTS[n] for n in ('configure-session','activate-context',
                  'return-mode-two','guarded-counter','probe-callbacks')]
        self.assertEqual(sum(x[2] for x in cluster),308)
        self.assertEqual(cluster[-1][1]+cluster[-1][2],0x3abb4)

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_release_session_nonshuffled_dual_memory_packet_and_cleanup(self):
        source,start,length=adsp.FRAGMENTS['release-session']
        self.assertEqual((start,length),(0x3a18c,136))
        body=source.read_text()
        self.assertIn('memd(r29+#0) = r19:18; r2 = memw(r16+#24) }:mem_noshuf',body)
        self.assertIn('if (!p0) call optional_release',body)
        self.assertIn('dealloc_return',body)
        for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
            self.assertNotIn(forbidden,body)
        compiled=adsp.compile_source(LLVM_MC,source,length)
        self.assertEqual(len(compiled),136)
        # Signed conditional call is an LLVM6-verified long call with immext.
        self.assertIn(bytes.fromhex('f9410000'),compiled)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         'Hexagon LLVM6 and exact pinned ADSP reference required')
    def test_inventory_every_fragment_uses_correct_original(self):
        inventory,_outputs=adsp.assembled_inventory(LLVM_MC,Path(REFERENCE).read_bytes())
        self.assertTrue(all(x['reference_byte_identical'] for x in inventory['fragments']))
        self.assertEqual(inventory['total_independently_assembled_executable_bytes'],3956)

    @unittest.skipUnless(LLVM_MC and REFERENCE,
                         'Hexagon LLVM6 and exact pinned ADSP reference required')
    def test_all_registered_disjoint_fragments_are_source_identical(self):
        original=Path(REFERENCE).read_bytes()
        total=0
        ranges=[]
        for name,(source,off,size) in adsp.FRAGMENTS.items():
            with self.subTest(fragment=name):
                built=adsp.compile_source(LLVM_MC,source,size)
                adsp.verify_slice(built,original,off)
                total+=len(built)
                ranges.append((off,off+size))
        self.assertEqual(total,3956)
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


class CreateControllerContextTests(unittest.TestCase):
    @unittest.skipUnless(LLVM_MC, 'Hexagon LLVM6 assembler required')
    def test_source_assembler_rebuilds_create_context_with_register_packet(self):
        source,off,size=adsp.FRAGMENTS['create-context']
        self.assertEqual((off,size),(0x3a090,252))
        body=source.read_text()
        for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
            self.assertNotIn(forbidden,body)
        for expected in ('immext(#0x10000); r2 = ##65536; p0 = cmp.eq(r17,#0); r16 = r0',
                         'call platform_register_context', 'report_invalid_context:',
                         'report_second_log:', 'report_diagnostic:', 'dealloc_return'):
            self.assertIn(expected,body)
        rebuilt=adsp.compile_source(LLVM_MC,source,size)
        self.assertEqual(len(rebuilt),252)
        self.assertNotEqual(rebuilt,bytes(252))
        release=adsp.FRAGMENTS['release-session']
        self.assertEqual(off+size,release[1])
        combo=rebuilt+adsp.compile_source(LLVM_MC,release[0],release[2])
        self.assertEqual(len(combo),388)
        self.assertEqual(hashlib.sha256(rebuilt).hexdigest(),
                         'aed2317c061cfa68ab63d95d258f99325c83edb5fa66e97870d27538b6f9a601')

    @unittest.skipUnless(LLVM_MC and REFERENCE,'Pinned ADSP and LLVM6 required')
    def test_combined_create_release_exact_reference(self):
        ref=Path(REFERENCE).read_bytes()
        create=adsp.FRAGMENTS['create-context']
        release=adsp.FRAGMENTS['release-session']
        full=adsp.compile_source(LLVM_MC,create[0],create[2])+adsp.compile_source(LLVM_MC,release[0],release[2])
        adsp.verify_slice(full,ref,0x3a090)


class ADSPBufferContextTests(unittest.TestCase):
    @unittest.skipUnless(LLVM_MC, 'Hexagon LLVM6 required')
    def test_context_scanner_and_global_pointer_source_byte_layout(self):
        names={'global-pointer':(0x3a7e0,8,'d0857940dd31103d52be479feb42a84ecbe355de9155b4e1c183677ab17fa53d'),
               'format-buffer-context':(0x3a7f0,124,'ccf7d38f4190bd77aea761db977518dd7170de7f0fbcd0a56a30cffc64ca8684')}
        for name,(off,length,digest) in names.items():
            src,start,size=adsp.FRAGMENTS[name]
            with self.subTest(name=name):
                self.assertEqual((start,size),(off,length))
                data=src.read_text()
                for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
                    self.assertNotIn(forbidden,data)
                built=adsp.compile_source(LLVM_MC,src,size)
                self.assertEqual(hashlib.sha256(built).hexdigest(),digest)
        first=adsp.FRAGMENTS['format-buffer-context'];second=adsp.FRAGMENTS['status-nibbles']
        self.assertEqual(first[1]+first[2],second[1])
        self.assertEqual(first[2]+second[2],220)
        self.assertIn('memw(r29+#4) = r2.new',first[0].read_text())
        self.assertIn('dealloc_return',first[0].read_text())

    @unittest.skipUnless(LLVM_MC and REFERENCE,'Private exact ADSP reference needed')
    def test_buffer_context_and_formatter_contiguous_reference(self):
        original=Path(REFERENCE).read_bytes()
        first=adsp.FRAGMENTS['format-buffer-context'];second=adsp.FRAGMENTS['status-nibbles']
        assembled=adsp.compile_source(LLVM_MC,first[0],first[2])+adsp.compile_source(LLVM_MC,second[0],second[2])
        adsp.verify_slice(assembled,original,first[1])
        self.assertEqual(assembled,original[0x3a7f0:0x3a8cc])


class ControllerHandoffTailTests(unittest.TestCase):
    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler needed')
    def test_semantic_source_rebuilds_shared_handoff_tail(self):
        source,off,size=adsp.FRAGMENTS['controller-handoff-tail']
        self.assertEqual((off,size),(0x3a7b8,40))
        text=source.read_text()
        self.assertIn('{ call platform_trigger; immext(#0xf0299000); r0 = ##-268208144 }',text)
        self.assertIn('{ jump shared_restore; deallocframe }',text)
        for directive in ('.word ','.long ','.byte ','.incbin ','.fill '):
            self.assertNotIn(directive,text)
        rebuilt=adsp.compile_source(LLVM_MC,source,size)
        self.assertEqual(hashlib.sha256(rebuilt).hexdigest(),'f72a2df2163b895a5a7fd77f9b5ac009cb7a265e1f05ff420073e88ebe0d29f9')
        self.assertEqual(off+size,adsp.FRAGMENTS['global-pointer'][1])

    @unittest.skipUnless(LLVM_MC and REFERENCE,'Exact ADSP reference and LLVM6 required')
    def test_handoff_fragment_matches_pinned_original(self):
        source,off,size=adsp.FRAGMENTS['controller-handoff-tail']
        adsp.verify_slice(adsp.compile_source(LLVM_MC,source,size),Path(REFERENCE).read_bytes(),off)


class ControllerShutdownTests(unittest.TestCase):
    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler needed')
    def test_conditional_before_after_callback_source(self):
        source,off,size=adsp.FRAGMENTS['controller-shutdown']
        self.assertEqual((off,size),(0x3afa0,68))
        text=source.read_text()
        for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
            self.assertNotIn(forbidden,text)
        self.assertIn('no_pre_callback:',text)
        self.assertIn('return_empty:',text)
        self.assertIn('call platform_cleanup',text)
        self.assertIn('if (cmp.eq(r2.new,#0)) jump:nt no_pre_callback',text)
        built=adsp.compile_source(LLVM_MC,source,size)
        self.assertEqual(hashlib.sha256(built).hexdigest(),'bf3b1b5096280746287d320a30ba84f658edcc4fdea416d42046e2aceae1199a')
        self.assertEqual(len(built),68)

    @unittest.skipUnless(LLVM_MC and REFERENCE,'Private exact ADSP needed')
    def test_controller_shutdown_exact_reference(self):
        source,off,size=adsp.FRAGMENTS['controller-shutdown']
        adsp.verify_slice(adsp.compile_source(LLVM_MC,source,size),Path(REFERENCE).read_bytes(),off)


class LaterADSPParameterHelpersTests(unittest.TestCase):
    @unittest.skipUnless(LLVM_MC,'Exact LLVM6 Hexagon assembler needed')
    def test_small_helpers_build_without_oem_opcode_directives(self):
        expected={
            'guarded-multiply':(0x3bbe4,20,'4f39a724829cc889a92f114f6bbff204337596cb94669ae557dcea4cb1baa379'),
            'platform-calculate':(0x3bbf8,16,'57e584847d38c36f21e3ee18c272e75a166fe976d34ebcb27319bf89051e6b54'),
            'parameter-forward':(0x3bc08,20,'a066f025c4f231fec925ed367bfde296a81ba7c1cf9bda298f11414dbf42727b'),
            'parameter-combine':(0x3be70,16,'12aa1bda4529576709d32bd2a877444f1852c3a2cd5f57d914664270e4c79b6a'),
            'halfword-order':(0x3be80,28,'0a0883a467f72bcaf184f486a79372f3ec6f11a366cc069057de3bcd2d688c8a'),
        }
        for name,(start,length,digest) in expected.items():
            with self.subTest(name=name):
                source,off,size=adsp.FRAGMENTS[name]
                self.assertEqual((off,size),(start,length))
                text=source.read_text()
                for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
                    self.assertNotIn(forbidden,text)
                self.assertEqual(hashlib.sha256(adsp.compile_source(LLVM_MC,source,size)).hexdigest(),digest)
        self.assertIn('r2 = cl0(r0)',adsp.FRAGMENTS['halfword-order'][0].read_text())
        self.assertIn('mpyi(r3,r3)',adsp.FRAGMENTS['guarded-multiply'][0].read_text())
        self.assertIn('{ r2 = memw(r0+#0) }',adsp.FRAGMENTS['parameter-forward'][0].read_text())
        a=[adsp.FRAGMENTS[x] for x in ('guarded-multiply','platform-calculate','parameter-forward')]
        b=[adsp.FRAGMENTS[x] for x in ('parameter-combine','halfword-order')]
        self.assertEqual([x[1] for x in a],[0x3bbe4,0x3bbf8,0x3bc08])
        self.assertEqual(sum(x[2] for x in a),56)
        self.assertEqual(sum(x[2] for x in b),44)

    @unittest.skipUnless(LLVM_MC and REFERENCE,'Pinned ADSP and LLVM6 needed')
    def test_both_contiguous_small_helper_groups_match_original(self):
        original=Path(REFERENCE).read_bytes()
        for names,off in [(['guarded-multiply','platform-calculate','parameter-forward'],0x3bbe4),
                          (['parameter-combine','halfword-order'],0x3be70)]:
            data=b''.join(adsp.compile_source(LLVM_MC,adsp.FRAGMENTS[name][0],adsp.FRAGMENTS[name][2]) for name in names)
            adsp.verify_slice(data,original,off)


class ADSPBufferBitfieldTests(unittest.TestCase):
    @unittest.skipUnless(LLVM_MC, 'Hexagon LLVM6 assembler needed')
    def test_bitfield_source_and_packetized_lookup(self):
        source,off,size=adsp.FRAGMENTS['buffer-bitfield']
        self.assertEqual((off,size),(0x3ba68,88))
        text=source.read_text()
        for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
            self.assertNotIn(forbidden,text)
        for operation in ('r2 = ct0(r2)', 'r2 = extractu(r0,#22,#0)',
                          'r4 &= and(r0,r5); immext(#0xf01f2000)',
                          'return_zero:', 'dealloc_return'):
            self.assertIn(operation,text)
        assembled=adsp.compile_source(LLVM_MC,source,size)
        self.assertEqual(hashlib.sha256(assembled).hexdigest(),'ee842302127e5e22ce797bd744e6f65115531946096d4a7ae987a89d44572a34')

    @unittest.skipUnless(LLVM_MC and REFERENCE, 'Private pinned reference required')
    def test_bitfield_roundtrip_against_private_source(self):
        source,off,size=adsp.FRAGMENTS['buffer-bitfield']
        adsp.verify_slice(adsp.compile_source(LLVM_MC,source,size),Path(REFERENCE).read_bytes(),off)


class ADSPDescriptorAtomicOpsTests(unittest.TestCase):
    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 needed')
    def test_descriptor_comparison_and_atomic_event_bit_source(self):
        expected={'descriptor-tag-compare':(0x3bfb8,20,'e4ebe0bdc31185ef82f51f7af811e4024388f6266a9ee56ec102c1ecbd1cae7c'),
                  'atomic-event-bit':(0x3bfcc,68,'074fa2c9e6615afe849b3e387e835ac40d81d0a1a69a244a29bff124112f7755')}
        for name,(off,size,digest) in expected.items():
            with self.subTest(name=name):
                src,start,length=adsp.FRAGMENTS[name]
                self.assertEqual((start,length),(off,size))
                text=src.read_text()
                for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
                    self.assertNotIn(forbidden,text)
                self.assertEqual(hashlib.sha256(adsp.compile_source(LLVM_MC,src,size)).hexdigest(),digest)
        word_source=adsp.FRAGMENTS['atomic-event-bit'][0].read_text()
        self.assertIn('memd_locked(r2,p0)',word_source)
        self.assertIn('if (!p0) jump:nt retry_locked',word_source)
        self.assertIn('asl(r1:0,r16)',word_source)
        self.assertEqual(adsp.FRAGMENTS['descriptor-tag-compare'][1]+20,
                         adsp.FRAGMENTS['atomic-event-bit'][1])

    @unittest.skipUnless(LLVM_MC and REFERENCE,'Pinned ADSP reference needed')
    def test_descriptor_and_atomic_combined_region_matches_original(self):
        original=Path(REFERENCE).read_bytes()
        names=('descriptor-tag-compare','atomic-event-bit')
        assembled=b''.join(adsp.compile_source(LLVM_MC,adsp.FRAGMENTS[name][0],adsp.FRAGMENTS[name][2]) for name in names)
        self.assertEqual(len(assembled),88)
        adsp.verify_slice(assembled,original,0x3bfb8)


class FormatConstraintTests(unittest.TestCase):
    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler needed')
    def test_format_constraints_packet_source(self):
        source,offset,size=adsp.FRAGMENTS['format-constraint']
        self.assertEqual((offset,size),(0x3bdd4,156))
        text=source.read_text()
        for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
            self.assertNotIn(forbidden,text)
        for required in ('vaslw(r19:18,#12)', 'r2 = #7; r5 = r16; r4 = #3',
                         'apply_alternative:', 'apply_fallback:', 'tstbit(r2,r5)'):
            self.assertIn(required,text)
        rebuilt=adsp.compile_source(LLVM_MC,source,size)
        self.assertEqual(hashlib.sha256(rebuilt).hexdigest(),'80a4e3800a5f851e51271d456ef637b44317fcc331901fba008759cfa541ed01')
        self.assertEqual(offset+size,adsp.FRAGMENTS['parameter-combine'][1])
        self.assertEqual(adsp.FRAGMENTS['parameter-combine'][1]+adsp.FRAGMENTS['parameter-combine'][2],
                         adsp.FRAGMENTS['halfword-order'][1])

    @unittest.skipUnless(LLVM_MC and REFERENCE,'Pinned reference required')
    def test_200_byte_format_constraints_and_helpers_are_adjacent(self):
        names=['format-constraint','parameter-combine','halfword-order']
        assembled=b''.join(adsp.compile_source(LLVM_MC,adsp.FRAGMENTS[name][0],adsp.FRAGMENTS[name][2]) for name in names)
        self.assertEqual(len(assembled),200)
        adsp.verify_slice(assembled,Path(REFERENCE).read_bytes(),0x3bdd4)

class ADSPControllerParameterAccessorTests(unittest.TestCase):
    """Five new 28-byte source-built consecutive controller field getters."""

    ORDER = (28, 52, 56, 48, 112)
    FIRST = 0x3EE58
    JOINED_SHA256 = '24a23bc0d2103ca689d9420c8402b5d6d0ce7a608302895a64cbc04a1faae3ab'

    def test_source_registry_ranges_are_disjoint_and_explicit(self):
        for index, selector in enumerate(self.ORDER):
            name = f'controller-param-{selector}'
            source, offset, length = adsp.FRAGMENTS[name]
            self.assertEqual((offset, length), (self.FIRST + index * 28, 28))
            asm = source.read_text()
            self.assertIn(f'r2 = #{selector}', asm)
            self.assertIn('call platform_fetch_parameter', asm)
            self.assertIn('dealloc_return', asm)
            for forbidden in ('.word ', '.long ', '.byte ', '.incbin ', '.fill '):
                self.assertNotIn(forbidden, asm)
        self.assertEqual(self.FIRST + len(self.ORDER) * 28, 0x3EEE4)

    @unittest.skipUnless(LLVM_MC, 'LLVM6 Hexagon assembler required')
    def test_each_callback_selector_and_entire_140_byte_span(self):
        fragments = [adsp.compile_source(LLVM_MC, adsp.FRAGMENTS[f'controller-param-{selector}'][0], 28)
                     for selector in self.ORDER]
        combined = b''.join(fragments)
        self.assertEqual(len(combined), 140)
        self.assertEqual(hashlib.sha256(combined).hexdigest(), self.JOINED_SHA256)
        if REFERENCE:
            original = Path(REFERENCE).read_bytes()
            adsp.verify_slice(combined, original, self.FIRST)
            for index, fragment in enumerate(fragments):
                adsp.verify_slice(fragment, original, self.FIRST + 28 * index)


class ADSPCallbackTableAndStateTests(unittest.TestCase):
    """Three distinct but adjacent platform callback/state routines."""
    NAMES = ('callback-table-walk','controller-state-flush','controller-state-tail')
    EXPECTED_SHA256 = 'ced5bf90b5263e9f778bebf2e0a878b34fcc0bc3732f4898ba196b77b03ab621'

    def test_exact_source_offsets_and_safety(self):
        expected=((0x3EC1C,40),(0x3EC44,32),(0x3EC64,16))
        for name,(off,n) in zip(self.NAMES,expected):
            p,o,l=adsp.FRAGMENTS[name]
            self.assertEqual((o,l),(off,n))
            text=p.read_text()
            for forbidden in ('.word ','.long ','.byte ','.incbin ','.fill '):
                self.assertNotIn(forbidden,text)
        self.assertEqual(expected[-1][0]+expected[-1][1],0x3EC74)
        self.assertIn(':mem_noshuf',adsp.FRAGMENTS['controller-state-flush'][0].read_text())
        self.assertIn('if (!cmp.eq(r2.new,#0)) jump:t loop_callbacks',
                      adsp.FRAGMENTS['callback-table-walk'][0].read_text())

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 required')
    def test_callback_walk_and_state_joined_bytes(self):
        pieces=[adsp.compile_source(LLVM_MC,adsp.FRAGMENTS[x][0],adsp.FRAGMENTS[x][2])
                for x in self.NAMES]
        region=b''.join(pieces)
        self.assertEqual(len(region),88)
        self.assertEqual(hashlib.sha256(region).hexdigest(),self.EXPECTED_SHA256)
        if REFERENCE:
            adsp.verify_slice(region,Path(REFERENCE).read_bytes(),0x3EC1C)


class ADSPDescriptorDispatchAndInitTests(unittest.TestCase):
    """Two source-built controller routines with a four-byte excluded NOP."""
    def test_distinct_src_offsets_and_no_oem_opcode_directives(self):
        expect={
            'call-descriptor-dispatch':(0x3E0A4,40),
            'controller-runtime-init':(0x3E0D0,128),
        }
        for name,(offset,size) in expect.items():
            with self.subTest(name=name):
                source,got_offset,got_size=adsp.FRAGMENTS[name]
                self.assertEqual((got_offset,got_size),(offset,size))
                text=source.read_text()
                for token in ('.word ','.long ','.byte ','.incbin ','.fill '):
                    self.assertNotIn(token,text)
        self.assertEqual(adsp.FRAGMENTS['call-descriptor-dispatch'][1]+40,0x3E0CC)
        self.assertEqual(adsp.FRAGMENTS['controller-runtime-init'][1],0x3E0D0)
        self.assertIn('if (!p0.new) r2 = memw(r0+#24)',
                      adsp.FRAGMENTS['call-descriptor-dispatch'][0].read_text())

    @unittest.skipUnless(LLVM_MC,'LLVM6 Hexagon assembler required')
    def test_checked_hexagon_source_digests_and_reference(self):
        expected={
            'call-descriptor-dispatch':'d589c602047ca1c36e67a15c6a42b3a2b7409bd1bccd5b4ea810537bb91ceb6f',
            'controller-runtime-init':'b16e7800f8f4e9b1ab8f7dec2a489536857a43265e4c4677525d2218d5a712ab',
        }
        for name,digest in expected.items():
            source,off,size=adsp.FRAGMENTS[name]
            code=adsp.compile_source(LLVM_MC,source,size)
            self.assertEqual(hashlib.sha256(code).hexdigest(),digest)
            if REFERENCE:
                adsp.verify_slice(code,Path(REFERENCE).read_bytes(),off)


class ADSPControllerDescriptorRefreshTests(unittest.TestCase):
    """Controller tag guard and tail-call to earlier validated shutdown."""
    @unittest.skipUnless(LLVM_MC,'LLVM6 Hexagon assembler required')
    def test_descriptor_guard_and_shutdown_tail_is_reference_identical(self):
        p,off,n=adsp.FRAGMENTS['controller-descriptor-refresh']
        self.assertEqual((off,n),(0x3E480,80))
        text=p.read_text()
        for forbidden in ('.word ','.long ','.byte ','.fill ','.incbin '):
            self.assertNotIn(forbidden,text)
        for term in ('p0 = cmp.eq(r2,##-1381126739)',
                     'jump shared_shutdown_controller',
                     '.set shared_shutdown_controller, adsp_late_controller_descriptor_refresh-0x34e0'):
            self.assertIn(term,text)
        code=adsp.compile_source(LLVM_MC,p,n)
        self.assertEqual(hashlib.sha256(code).hexdigest(),
            '9bd8cf35a95e3e0979b840e7b31a65c2ab38b8e0b79f284ffe3d54189081ac7d')
        if REFERENCE:adsp.verify_slice(code,Path(REFERENCE).read_bytes(),off)


class ADSPCheckedArgumentForwardTests(unittest.TestCase):
    """Bounded controller-mode check plus complete 6-slot stack argument relay."""

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_complete_forwarder_is_independently_assembled(self):
        src,off,n=adsp.FRAGMENTS['checked-argument-forward']
        self.assertEqual((off,n),(0x3efa8,76))
        text=src.read_text()
        for forbidden in ('.word ','.long ','.byte ','.incbin ','.fill '):
            self.assertNotIn(forbidden,text)
        for insn in ('if (!p0.new) jump:nt forward',
                     'r6 = memw(r29+#52)', 'memw(r29+#20) = r6.new',
                     'r13 = memw(r29+#32)',
                     '.set platform_forward, adsp_late_checked_argument_forward+0xeb920',
                     'dealloc_return'):
            self.assertIn(insn,text)
        built=adsp.compile_source(LLVM_MC,src,n)
        self.assertEqual(hashlib.sha256(built).hexdigest(),
            'dad239580745f2b380bdf94ba096884c5ff4ef5f95bb7c0c4b662fef1759b932')
        if REFERENCE: adsp.verify_slice(built,Path(REFERENCE).read_bytes(),off)
        # Preceding three NOPs after the prior function are not source bytes.
        self.assertEqual(off-0x3ef94,20)


class ADSPControllerFieldShimTests(unittest.TestCase):
    """Five controller-address shims, two packet forms and excluded linker NOPs."""

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_field_accessor_and_forwarder_pairs_round_trip(self):
        expected=[
            ('00-get',0x3ec80,12),('00-forward',0x3ec8c,16),
            ('01-get',0x3eca0,12),('01-forward',0x3ecac,16),
            ('02-get',0x3ecc0,12),('02-forward',0x3eccc,16),
            ('03-get',0x3ece0,12),('04-get',0x3ecf0,12),
            ('03-forward',0x3ecfc,16),('04-forward',0x3ed0c,16),
        ]
        self.assertEqual(sum(x[2] for x in expected),140)
        original=Path(REFERENCE).read_bytes() if REFERENCE else None
        for name,off,n in expected:
            with self.subTest(name=name):
                src,got_off,got_n=adsp.FRAGMENTS['controller-field-'+name]
                self.assertEqual((got_off,got_n),(off,n))
                text=src.read_text()
                for forbidden in ('.word ','.byte ','.long ','.fill ','.incbin '):
                    self.assertNotIn(forbidden,text)
                self.assertIn('jump shared_field_',text)
                if name.endswith('forward'):
                    self.assertIn('r1:0 = combine(r2,##',text)
                else:
                    self.assertIn('r0 = ##',text)
                payload=adsp.compile_source(LLVM_MC,src,n)
                self.assertEqual(len(payload),n)
                if original:adsp.verify_slice(payload,original,off)
        self.assertEqual(adsp.FRAGMENTS['controller-field-00-get'][1],0x3ec80)
        self.assertEqual(adsp.FRAGMENTS['controller-field-04-forward'][1]+16,0x3ed1c)
        # Original 4-byte NOPs separate these functions; they remain ungenerated.
        forbidden_gaps=((0x3ec9c,0x3eca0),(0x3ecbc,0x3ecc0),
                        (0x3ecdc,0x3ece0),(0x3ecec,0x3ecf0))
        covered=set()
        for _name,off,n in expected: covered.update(range(off,off+n))
        self.assertTrue(all(not (covered & set(range(a,b))) for a,b in forbidden_gaps))


class ADSPControllerDescriptorSetupTests(unittest.TestCase):
    """Descriptor registry initialization and bounded allocation loop."""

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_complete_descriptor_setup_is_byte_identical(self):
        src,off,n=adsp.FRAGMENTS['controller-descriptor-setup']
        self.assertEqual((off,n),(0x3edd0,136))
        self.assertEqual(off+n,adsp.FRAGMENTS['controller-param-28'][1])
        body=src.read_text()
        for forbidden in ('.word ','.long ','.byte ','.incbin ','.fill '):
            self.assertNotIn(forbidden,body)
        for insn in ('r2 = #0; r16 = r0; memd(r29+#0) = r19:18',
                     'r19 = add(r19,#-1)',
                     'if (!cmp.eq(r19.new,#0)) jump:t next_queue',
                     'r17:16 = memd(r29+#8)',
                     'dealloc_return'):
            self.assertIn(insn,body)
        a=adsp.compile_source(LLVM_MC,src,n)
        self.assertEqual(hashlib.sha256(a).hexdigest(),
            '237791bfa2488cb43ceb0739be371e84906679119a3ec2beebac6b010d8c8db4')
        if REFERENCE:adsp.verify_slice(a,Path(REFERENCE).read_bytes(),off)


class ADSPControllerFieldInitializationTests(unittest.TestCase):
    """Original multi-field initialization and hardware callback handoff."""

    @unittest.skipUnless(LLVM_MC,'Hexagon LLVM6 assembler required')
    def test_source_rebuilds_controller_fields_with_packet_exactness(self):
        src,off,n=adsp.FRAGMENTS['controller-field-initialize']
        self.assertEqual((off,n),(0x3ed1c,172))
        self.assertEqual(off+n,0x3edc8)
        self.assertEqual(adsp.FRAGMENTS['controller-descriptor-setup'][1],0x3edd0)
        body=src.read_text()
        for forbidden in ('.word ','.long ','.byte ','.incbin ','.fill '):
            self.assertNotIn(forbidden,body)
        for insn in ('memh(r16=##4028568856)', 'r2 = #28',
                     'memh(r16+#12)', 'memh(r16+#10)',
                     'r4 = #1; immext(#0); jump initialize_field; deallocframe',
                     '.set initialize_field, adsp_late_controller_field_initialize+0xb4'):
            self.assertIn(insn,body)
        generated=adsp.compile_source(LLVM_MC,src,n)
        self.assertEqual(hashlib.sha256(generated).hexdigest(),
            'a59c40d44e11ef97f978883d3c01f7dc8eec74bb70ab329cdf61c570f66384f8')
        if REFERENCE:adsp.verify_slice(generated,Path(REFERENCE).read_bytes(),off)
