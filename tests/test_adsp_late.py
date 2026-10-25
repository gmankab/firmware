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
