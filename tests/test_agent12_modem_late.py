# SPDX-License-Identifier: GPL-3.0-only
"""Source-only regression tests for isolated later modem Hexagon routines."""
import importlib.util
import os
from pathlib import Path
import struct
import unittest

ROOT = Path(__file__).resolve().parents[1]
TOOL = ROOT / "tools" / "assemble-modem-late.py"
spec = importlib.util.spec_from_file_location("assemble_modem_late", TOOL)
assert spec and spec.loader
agent12 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(agent12)


class ModemLateTests(unittest.TestCase):
    def test_aux_controller_phase13_gap_four_word_packet(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE13_GAP_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE13_GAP_END),
                         (0x447d28,0x447d38))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE13_GAP_SOURCE.read_text()
        self.assertIn('p0 = bitsclr(r2,r3)',src)
        self.assertIn('if (!p0.new) jump:t process_remaining_resources',src)
        self.assertIn('if (p0) r23 = #0',src)
        self.assertIn('if (p0.new) r23 = #0',src) # documented fused ISA
        self.assertNotIn('.word',src)
        words=(0x5c205812,0xc782c300,0x5800c02a,0x7e00c017)
        b=agent12.restore_aux_large_controller_phase13_gap_packet(
            struct.pack('<4I',*words))
        self.assertEqual(struct.unpack('<4I',b),
                         (0x5c205812,0xc7824300,0x5800402e,0x7e00e017))
        for idx,bad,why in [
            (0,0x7c205812,'opcode semantics'),
            (1,0xc782c301,'opcode semantics'),
            (2,0x5800c02b,'opcode semantics'),
            (3,0x7e00c016,'opcode semantics'),
            (1,0xc7824300,'packet markers'),
        ]:
            other=list(words);other[idx]=bad
            with self.assertRaisesRegex(ValueError,why):
                agent12.restore_aux_large_controller_phase13_gap_packet(
                    struct.pack('<4I',*other))
        with self.assertRaisesRegex(ValueError,'16 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase13_gap_packet(bytes(12))

    def test_phase37_signed_decrement_packet_is_source_derived(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE37_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE37_END),
                         (0x4485f8,0x448628))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE37_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,src)
        self.assertIn('r2 += add(r6,#-1)',src)
        self.assertIn('jump:t alternate_result',src)
        w=[0x7800C000]*12
        w[8:12]=[0x919dc2a3,0xe206dfe2,0x85434000,0x5c20d814]
        out=struct.unpack('<12I',agent12.restore_aux_large_controller_phase37_accumulate_packet(
            struct.pack('<12I',*w)))
        self.assertEqual(out[9],0xe2065fe2)
        for idx,bad,why in [
            (9,0xf206dfe2,'opcode class'),
            (9,0xe206dfe3,'register/immediate operands'),
            (9,0xe2065fe2,'packet markers'),
            (10,0x85434001,'predicate or branch target'),
            (11,0x5c20d815,'predicate or branch target'),
            (8,0x919d42a3,'packet markers'),
        ]:
            other=w.copy();other[idx]=bad
            with self.assertRaisesRegex(ValueError,why):
                agent12.restore_aux_large_controller_phase37_accumulate_packet(
                    struct.pack('<12I',*other))
        with self.assertRaisesRegex(ValueError,'48 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase37_accumulate_packet(bytes(44))
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            raw=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                 agent12.AUX_LARGE_CONTROLLER_PHASE37_SOURCE)
            fixed=agent12.restore_aux_large_controller_phase37_accumulate_packet(raw)
            if os.getenv('NABU_MODEM_REFERENCE'):
                ref=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(fixed,ref[0x4485f8:0x448628])

    def test_phase40_predicated_trace_packet_placement(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE40_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE40_END),
                         (0x4486cc,0x44873c))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE40_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,src)
        self.assertIn('if (!p0.new) r3 = ##507577979',src)
        self.assertIn('if (!p0.new) jump:t fallback_report',src)
        w=[0x7800c000]*28
        w[15:18]=[0x5c205812,0x01e44419,0x7e80e763]
        out=struct.unpack('<28I',agent12.restore_aux_large_controller_phase40_predicated_trace(
            struct.pack('<28I',*w)))
        self.assertEqual(out[15:18],(0x01e44419,0x7e806763,0x5c20d812))
        for index,bad,reason in [
            (15,0x4c205812,'opcode classes'),
            (16,0x11e44419,'opcode classes'),
            (17,0x6e80e763,'opcode classes'),
            (15,0x5c205813,'operands/targets'),
            (16,0x01e4441a,'operands/targets'),
            (17,0x7e80e762,'operands/targets'),
            (15,0x5c20d812,'packet markers'),
            (17,0x7e806763,'packet markers'),
        ]:
            corrupt=w.copy();corrupt[index]=bad
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase40_predicated_trace(
                    struct.pack('<28I',*corrupt))
        with self.assertRaisesRegex(ValueError,'112 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase40_predicated_trace(bytes(108))
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            assembled=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                       agent12.AUX_LARGE_CONTROLLER_PHASE40_SOURCE)
            reconstructed=agent12.restore_aux_large_controller_phase40_predicated_trace(assembled)
            if os.getenv('NABU_MODEM_REFERENCE'):
                reference=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(reconstructed,reference[0x4486cc:0x44873c])

    def test_phase39_callback_resolution_and_registry_tail(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE39_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE39_END),
                         (0x448668,0x4486cc))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE39_SOURCE.read_text()
        for forbidden in ('.word', '.byte', '.incbin'):
            self.assertNotIn(forbidden,src)
        for expected in ('callr r4','if (p0.new) jump:t no_result_callback',
                         'call platform_notifier','jump resource_scanner',
                         'call registry_resolver'):
            self.assertIn(expected,src)
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            raw=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                 agent12.AUX_LARGE_CONTROLLER_PHASE39_SOURCE)
            self.assertEqual(len(raw),100)
            if os.getenv('NABU_MODEM_REFERENCE'):
                reference=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(raw,reference[0x448668:0x4486cc])

    def test_phase38_report_packet_markers_are_checked(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE38_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE38_END),
                         (0x448628,0x448668))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE38_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,src)
        self.assertIn('call alternate_callback',src)
        w=[0x7800C000]*16
        w[9:13]=[0x7065c001,0x01e44418,0x68632806,0x5a00db90]
        out=struct.unpack('<16I',agent12.restore_aux_large_controller_phase38_report_packets(
            struct.pack('<16I',*w)))
        self.assertEqual(out[9],0x70654001)
        self.assertEqual(out[12],0x5a005b90)
        for idx,bad,why in [
            (9,0x6065c001,'opcode classes'),
            (11,0x78632806,'opcode classes'),
            (12,0x5a00db91,'instruction operands/callback'),
            (10,0x01e44419,'instruction operands/callback'),
            (9,0x70654001,'packet markers'),
            (12,0x5a005b90,'packet markers'),
        ]:
            other=w.copy();other[idx]=bad
            with self.assertRaisesRegex(ValueError,why):
                agent12.restore_aux_large_controller_phase38_report_packets(
                    struct.pack('<16I',*other))
        with self.assertRaisesRegex(ValueError,'64 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase38_report_packets(bytes(60))
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            raw=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                 agent12.AUX_LARGE_CONTROLLER_PHASE38_SOURCE)
            fixed=agent12.restore_aux_large_controller_phase38_report_packets(raw)
            if os.getenv('NABU_MODEM_REFERENCE'):
                ref=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(fixed,ref[0x448628:0x448668])

    def test_phase36_packet_comparison_precedes_branch(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE36_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE36_END),
                         (0x448580,0x4485f8))
        asm=agent12.AUX_LARGE_CONTROLLER_PHASE36_SOURCE.read_text()
        for raw in ('.word','.byte','.incbin'):
            self.assertNotIn(raw,asm)
        for fragment in ('!cmp.gtu(r6,r24)', 'call trace_callback',
                         'call alternate_callback', 'call record_finalizer'):
            self.assertIn(fragment,asm)
        w=[0x7800C000]*30
        w[:2]=[0x5c00583c,0xf266d810]
        out=struct.unpack('<30I',agent12.restore_aux_large_controller_phase36_limit_branch(
            struct.pack('<30I',*w)))
        self.assertEqual(out[:2],(0xf2665810,0x5c00d83c))
        for i,value,why in [
            (0,0x7c00583c,'opcode classes'),
            (1,0xe266d810,'opcode classes'),
            (0,0x5c00583d,'instruction operands'),
            (1,0xf266d811,'instruction operands'),
            (0,0x5c00d83c,'packet markers'),
            (1,0xf2665810,'packet markers'),
        ]:
            changed=w.copy();changed[i]=value
            with self.assertRaisesRegex(ValueError,why):
                agent12.restore_aux_large_controller_phase36_limit_branch(
                    struct.pack('<30I',*changed))
        with self.assertRaisesRegex(ValueError,'120 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase36_limit_branch(bytes(116))
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            source=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                    agent12.AUX_LARGE_CONTROLLER_PHASE36_SOURCE)
            result=agent12.restore_aux_large_controller_phase36_limit_branch(source)
            self.assertEqual(len(result),120)
            if os.getenv('NABU_MODEM_REFERENCE'):
                ref=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(result,ref[0x448580:0x4485f8])

    def test_phase35_signed_buffer_comparison_and_callback_branch(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE35_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE35_END),
                         (0x448520,0x448580))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE35_SOURCE.read_text()
        for raw in ('.word','.byte','.incbin'):
            self.assertNotIn(raw,src)
        for fragment in ('cmp.eq(r25:24,r27:26)','if (p0.new) jump:t same_size',
                         'call platform_notifier','jump resource_scanner'):
            self.assertIn(fragment,src)
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            b=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                               agent12.AUX_LARGE_CONTROLLER_PHASE35_SOURCE)
            self.assertEqual(len(b),96)
            if os.getenv('NABU_MODEM_REFERENCE'):
                ref=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(b,ref[0x448520:0x448580])

    def test_phase34_controller_indirect_callbacks_are_source_assembled(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE34_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE34_END),
                         (0x4484b4,0x448520))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE34_SOURCE.read_text()
        for token in ('.word','.byte','.incbin'):
            self.assertNotIn(token,src)
        for expected in ('callr r3','callr r4','call platform_notifier',
                         'jump previous_resource_scanner',
                         'if (p0.new) jump:t valid_callback_result'):
            self.assertIn(expected,src)
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            b=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                               agent12.AUX_LARGE_CONTROLLER_PHASE34_SOURCE)
            self.assertEqual(len(b),108)
            if os.getenv('NABU_MODEM_REFERENCE'):
                ref=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(b,ref[0x4484b4:0x448520])

    def test_phase33_direct_controller_report_callback_branches(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE33_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE33_END),
                         (0x448464,0x4484B4))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE33_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,src)
        self.assertIn('if (!p0.new) jump:t alternate_report',src)
        self.assertIn('if (p0.new) memd(r29+#16) = r25:24',src)
        self.assertIn('call platform_report',src)
        self.assertIn('call report_callback',src)
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            raw=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                 agent12.AUX_LARGE_CONTROLLER_PHASE33_SOURCE)
            self.assertEqual(len(raw),80)
            if os.getenv('NABU_MODEM_REFERENCE'):
                original=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(raw,original[0x448464:0x4484B4])

    def test_phase32_controller_pointer_load_and_scaled_offset_order(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE32_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE32_END),
                         (0x4483D0,0x448464))
        source=agent12.AUX_LARGE_CONTROLLER_PHASE32_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,source)
        self.assertIn('memd(r29+#32) = r13:12',source)
        self.assertIn('r4 = insert(r2,#1,#2)',source)
        w=[0x7800C000]*37
        i=0x34//4
        w[i:i+2]=[0xc40740a2,0x9191c120]
        out=struct.unpack('<37I',agent12.restore_aux_large_controller_phase32_descriptor_order(
            struct.pack('<37I',*w)))
        self.assertEqual(out[i:i+2],(w[i+1],w[i]))
        for index,bad,reason in [
            (i,0xd40740a2,'opcode classes'),
            (i+1,0x8191c120,'opcode classes'),
            (i,0xc40740a3,'instruction operands'),
            (i+1,0x9191c121,'instruction operands'),
            (i,0xc407c0a2,'packet markers'),
            (i+1,0x91914120,'packet markers'),
        ]:
            other=w.copy();other[index]=bad
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase32_descriptor_order(
                    struct.pack('<37I',*other))
        with self.assertRaisesRegex(ValueError,'148 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase32_descriptor_order(bytes(144))
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            raw=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                 agent12.AUX_LARGE_CONTROLLER_PHASE32_SOURCE)
            rebuilt=agent12.restore_aux_large_controller_phase32_descriptor_order(raw)
            self.assertEqual(len(rebuilt),148)
            if os.getenv('NABU_MODEM_REFERENCE'):
                original=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(rebuilt,original[0x4483d0:0x448464])

    def test_phase31_checked_controller_report_packets(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE31_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE31_END),
                         (0x448394,0x4483D0))
        asm=agent12.AUX_LARGE_CONTROLLER_PHASE31_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,asm)
        self.assertIn('memw(r24=##507614244) = r0',asm)
        self.assertIn('if (!p0.new) jump:t use_default_mapping',asm)
        w=[0x7800C000]*15
        w[:3]=[0x919d42a2,0x01e44650,0xab98c0a4]
        w[4:7]=[0x5c20789a,0x01e44419,0x7e80e283]
        emitted=agent12.restore_aux_large_controller_phase31_report_packets(
            struct.pack('<15I',*w))
        out=struct.unpack('<15I',emitted)
        self.assertEqual(out[:3],(0x01e44650,0xab9840a4,0x919dc2a2))
        self.assertEqual(out[4:7],(0x01e44419,0x7e806283,0x5c20f89a))
        for index,bad,reason in [
            (0,0x819d42a2,'opcode classes'),
            (2,0xbb98c0a4,'opcode classes'),
            (1,0x01e44651,'instruction operands'),
            (0,0x919d42a3,'instruction operands'),
            (2,0xab9840a4,'packet markers'),
            (4,0x7c20789a,'opcode classes'),
            (6,0x7f80e283,'opcode classes'),
            (5,0x01e44418,'instruction operands'),
            (4,0x5c20f89a,'packet markers'),
        ]:
            other=w.copy();other[index]=bad
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase31_report_packets(
                    struct.pack('<15I',*other))
        with self.assertRaisesRegex(ValueError,'60 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase31_report_packets(bytes(56))
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            source=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                    agent12.AUX_LARGE_CONTROLLER_PHASE31_SOURCE)
            rebuilt=agent12.restore_aux_large_controller_phase31_report_packets(source)
            self.assertEqual(len(rebuilt),60)
            if os.getenv('NABU_MODEM_REFERENCE'):
                ref=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(rebuilt,ref[0x448394:0x4483D0])

    def test_phase30_checked_predicate_and_jump_packet_reordering(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE30_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE30_END),
                         (0x448320,0x448394))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE30_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,src)
        self.assertIn('call lookup_handle',src)
        self.assertIn('jump update_config',src)
        self.assertIn('if (!p0.new) jump:nt format_unknown',src)
        words=[0x7800C000]*29
        words[7:10]=[0x00004014,0x10424200,0x7411E000]
        words[20:22]=[0x101243AA,0x7411E000]
        packed=struct.pack('<29I',*words)
        out=struct.unpack('<29I',agent12.restore_aux_large_controller_phase30_conditional_packets(packed))
        self.assertEqual(out[7:10],(0x74116000,0x00004014,0x1042C200))
        self.assertEqual(out[20:22],(0x74116000,0x1012C3AA))
        for i,bad,reason in [
            (7,0x01004014,'opcode classes'),
            (8,0x11424200,'opcode classes'),
            (9,0x7511E000,'opcode classes'),
            (8,0x10424204,'operands or branch target'),
            (9,0x7411E001,'operands or branch target'),
            (7,0x0000C014,'packet markers'),
            (20,0x111243AA,'opcode classes'),
            (21,0x7411E001,'operands or branch target'),
            (20,0x1012C3AA,'packet markers'),
        ]:
            modified=words.copy();modified[i]=bad
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase30_conditional_packets(
                    struct.pack('<29I',*modified))
        with self.assertRaisesRegex(ValueError,'116 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase30_conditional_packets(bytes(112))
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            obj=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                 agent12.AUX_LARGE_CONTROLLER_PHASE30_SOURCE)
            got=agent12.restore_aux_large_controller_phase30_conditional_packets(obj)
            self.assertEqual(len(got),116)
            if os.getenv('NABU_MODEM_REFERENCE'):
                r=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(got,r[0x448320:0x448394])

    def test_phase29_frame_record_advance_uses_symbolic_labels(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE29_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE29_END),
                         (0x4482CC,0x448320))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE29_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,text)
        self.assertIn('jump:t prior_scan',text)
        self.assertIn('jump:nt after_empty_record',text)
        self.assertIn('memd(r29+#64) = r1:0',text)
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            source=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                    agent12.AUX_LARGE_CONTROLLER_PHASE29_SOURCE)
            self.assertEqual(len(source),84)
            if os.getenv('NABU_MODEM_REFERENCE'):
                original=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(source,original[0x4482cc:0x448320])

    def test_phase28_branch_reordering_checks_instruction_semantics(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE28_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE28_END),
                         (0x448278, 0x4482CC))
        source=agent12.AUX_LARGE_CONTROLLER_PHASE28_SOURCE.read_text()
        for fragment in ('.word', '.byte', '.incbin'):
            self.assertNotIn(fragment,source)
        self.assertIn('if (!p0) jump:nt alternate_report',source)
        self.assertIn('call alternate_callback',source)
        self.assertIn('r18 = add(r18,#1)',source)
        w=[0x7800C000]*21
        w[4]=0x5C204010 # Valid LLVM6 branch with CONTINUE packet marker.
        w[5]=0xF302C403 # Valid LLVM6 ADD with END packet marker.
        result=agent12.restore_aux_large_controller_phase28_size_branch(struct.pack('<21I',*w))
        actual=list(struct.unpack('<21I',result))
        self.assertEqual(actual[4:6],[(w[5]&~agent12.MARKER_MASK)|agent12.CONTINUE_MARKER,
                                       (w[4]&~agent12.MARKER_MASK)|agent12.END_MARKER])
        for index, value, reason in [
            (4,0x7C204010,'opcode classes'),
            (5,0xA302C403,'opcode classes'),
            (4,0x5C204014,'instruction operands'),
            (5,0xF302C407,'instruction operands'),
            (4,0x5C20C010,'packet markers'),
            (5,0xF3024403,'packet markers'),
        ]:
            changed=w.copy();changed[index]=value
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase28_size_branch(struct.pack('<21I',*changed))
        with self.assertRaisesRegex(ValueError,'84 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase28_size_branch(bytes(80))
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            compiled=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                      agent12.AUX_LARGE_CONTROLLER_PHASE28_SOURCE)
            rebuilt=agent12.restore_aux_large_controller_phase28_size_branch(compiled)
            self.assertEqual(len(rebuilt),84)
            if os.getenv('NABU_MODEM_REFERENCE'):
                original=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(rebuilt,original[0x448278:0x4482cc])

    def test_phase27_callback_dispatch_preserves_symbolic_targets(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE27_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE27_END),
                         (0x448228,0x448278))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE27_SOURCE.read_text()
        for prohibited in ('.word','.byte','.incbin'):
            self.assertNotIn(prohibited,text)
        self.assertIn('if (p0.new) jump:t no_callback',text)
        self.assertIn('if (cmp.eq(r4.new,#0)) jump:nt callback_empty',text)
        self.assertIn('call controller_notify',text)
        self.assertIn('jump prior_resource_scan',text)
        self.assertIn('phase27 - 0x1a88',text)
        self.assertIn('phase27 - 0x788',text)
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            assembled=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                       agent12.AUX_LARGE_CONTROLLER_PHASE27_SOURCE)
            self.assertEqual(len(assembled),80)
            if os.getenv('NABU_MODEM_REFERENCE'):
                reference=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(assembled,reference[0x448228:0x448278])

    def test_phase26_indexed_buffer_stores_join_only_expected_packets(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE26_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE26_END),
                         (0x4481c4,0x448228))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE26_SOURCE.read_text()
        for forbidden in ('.word','.byte','.incbin'):
            self.assertNotIn(forbidden,text)
        self.assertIn('r2 = +mpyi(r18,#28)',text)
        self.assertIn('memw(r4+r2<<#0) = r5.new',text)
        self.assertIn('callr r3',text)
        cases=[(0x10,5,6,4,5),(0x20,5,4,5,4),
               (0x30,4,5,3,4),(0x48,4,20,6,4)]
        words=[0x7800c000]*25
        for off,base,value,index,load_dest in cases:
            i=off//4
            words[i]=(0xa1<<24)|(base<<16)|(value<<8)|index|agent12.END_MARKER
            words[i+1]=(0x91<<24)|(17<<16)|load_dest|agent12.END_MARKER
        source=struct.pack('<25I',*words)
        out=struct.unpack('<25I',agent12.restore_aux_large_controller_phase26_indexed_stores(source))
        for off,base,value,index,load_dest in cases:
            i=off//4
            self.assertEqual(out[i],(words[i]&~agent12.MARKER_MASK)|agent12.CONTINUE_MARKER)
            self.assertEqual(out[i+1],words[i+1])
        for position,broken,reason in [
            (0x10//4,0x7800c000,'store opcode/operands'),
            (0x20//4,words[0x20//4]^0x00000100,'store opcode/operands'),
            (0x30//4+1,0x7800c000,'load opcode/operands'),
            (0x48//4+1,words[0x48//4+1]^0x00000001,'load opcode/operands'),
            (0x10//4,words[0x10//4]^agent12.MARKER_MASK,'packet markers'),
            (0x20//4+1,words[0x20//4+1]^agent12.MARKER_MASK,'packet markers'),
        ]:
            mutated=words.copy();mutated[position]=broken
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase26_indexed_stores(
                    struct.pack('<25I',*mutated))
        with self.assertRaisesRegex(ValueError,'100 bytes'):
            agent12.restore_aux_large_controller_phase26_indexed_stores(source[:-4])
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            assembled=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                       agent12.AUX_LARGE_CONTROLLER_PHASE26_SOURCE)
            self.assertEqual(len(assembled),100)
            emitted=agent12.restore_aux_large_controller_phase26_indexed_stores(assembled)
            if os.getenv('NABU_MODEM_REFERENCE'):
                reference=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(emitted,reference[0x4481c4:0x448228])

    def test_phase25_controller_callback_compiles_without_correction(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE25_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE25_END),
                         (0x44817c,0x4481c4))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE25_SOURCE.read_text()
        for raw in ('.word','.byte','.incbin'):
            self.assertNotIn(raw,text)
        self.assertIn('if (!cmp.eq(r3.new,#0)) jump:t valid_session',text)
        self.assertIn('if (cmp.eq(r4.new,#0)) jump:nt absent_callback',text)
        self.assertIn('call platform_reporting',text)
        self.assertIn('jump resource_list_loop',text)
        if os.getenv('NABU_HEXAGON_LLVM_MC'):
            result=agent12.assemble(Path(os.environ['NABU_HEXAGON_LLVM_MC']),
                                    agent12.AUX_LARGE_CONTROLLER_PHASE25_SOURCE)
            self.assertEqual(len(result),72)
            if os.getenv('NABU_MODEM_REFERENCE'):
                src=Path(os.environ['NABU_MODEM_REFERENCE']).read_bytes()
                self.assertEqual(result,src[0x44817c:0x4481c4])

    def test_phase24_size_report_branch_order(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE24_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE24_END),
                         (0x448138,0x44817c))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE24_SOURCE.read_text()
        for raw in ('.word','.byte','.incbin'):
            self.assertNotIn(raw,text)
        self.assertIn('r3 = add(r2,r4)',text)
        self.assertIn('if (!p0) jump:nt alternate_error',text)
        self.assertIn('memw(r29+#0) = r2',text)
        w=[0x7800c000]*17
        w[1]=(0x5c<<24)|agent12.CONTINUE_MARKER|0x10
        w[2]=(0xf3<<24)|agent12.END_MARKER|0x403
        source=struct.pack('<17I',*w)
        result=struct.unpack('<17I',agent12.restore_aux_large_controller_phase24_size_report(source))
        self.assertEqual(result[1:3],
                         ((w[2]&~agent12.MARKER_MASK)|agent12.CONTINUE_MARKER,
                          (w[1]&~agent12.MARKER_MASK)|agent12.END_MARKER))
        self.assertEqual(result[:1],tuple(w[:1]))
        self.assertEqual(result[3:],tuple(w[3:]))
        for index,bad,reason in [
            (1,0x78004010,'opcode classes'),
            (2,0x7800c403,'opcode classes'),
            (1,0x5c204011,'instruction operands'),
            (2,0xf302c404,'instruction operands'),
            (1,0x5c20c010,'packet markers'),
            (2,0xf3024403,'packet markers'),
        ]:
            broken=w.copy();broken[index]=bad
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase24_size_report(
                    struct.pack('<17I',*broken))
        with self.assertRaisesRegex(ValueError,'68 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase24_size_report(source[:-4])

    def test_phase22_extender_and_predicated_branch_packet_reconstruction(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE22_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE22_END),
                         (0x4480b8,0x448104))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE22_SOURCE.read_text()
        self.assertIn('p0 = r2',text)
        self.assertIn('if (!p0.new) jump:t error_message',text)
        self.assertIn('call trace_sink',text)
        for fragment in ('.word','.byte','.incbin'):
            self.assertNotIn(fragment,text)
        w=[0x7800c000]*19
        i=0x30//4
        w[i:i+3]=[(0x5c<<24)|agent12.CONTINUE_MARKER|0x1812,
                  (0x01<<24)|agent12.CONTINUE_MARKER|0x0414,
                  (0x7e<<24)|agent12.END_MARKER|0x21c3]
        out=struct.unpack('<19I',agent12.restore_aux_large_controller_phase22_diagnostic_packets(
            struct.pack('<19I',*w)))
        self.assertEqual(out[i:i+3],
            ((w[i+1]&~agent12.MARKER_MASK)|agent12.CONTINUE_MARKER,
             (w[i+2]&~agent12.MARKER_MASK)|agent12.CONTINUE_MARKER,
             (w[i]&~agent12.MARKER_MASK)|agent12.END_MARKER))
        for offset,altered,error in [
            (i,0x78005812,'opcode classes'),
            (i,0x5c205813,'operands'),
            (i+1,0x01e44415,'operands'),
            (i+2,0x7e80e1c2,'operands'),
            (i,0x5c20d812,'packet markers'),
            (i+2,0x7e8061c3,'packet markers'),
        ]:
            corrupt=w.copy();corrupt[offset]=altered
            with self.assertRaisesRegex(ValueError,error):
                agent12.restore_aux_large_controller_phase22_diagnostic_packets(
                    struct.pack('<19I',*corrupt))
        with self.assertRaisesRegex(ValueError,'76 bytes'):
            agent12.restore_aux_large_controller_phase22_diagnostic_packets(bytes(72))

    def test_phase23_memory_store_then_load_packet(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE23_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE23_END),
                         (0x448104,0x448138))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE23_SOURCE.read_text()
        self.assertIn('memw(r29+#80) = r26',text)
        self.assertIn('r2 = memw(r17+#28)',text)
        for fragment in ('.word','.byte','.incbin'):
            self.assertNotIn(fragment,text)
        w=[0x7800c000]*13
        i=0x1c//4
        w[i]=(0xa1<<24)|(29<<16)|(26<<8)|20|agent12.END_MARKER
        w[i+1]=(0x91<<24)|(17<<16)|2|agent12.END_MARKER
        original=struct.pack('<13I',*w)
        converted=list(struct.unpack('<13I',
            agent12.restore_aux_large_controller_phase23_descriptor_packet(original)))
        self.assertEqual(converted[i],(w[i]&~agent12.MARKER_MASK)|agent12.CONTINUE_MARKER)
        self.assertEqual(converted[i+1],w[i+1])
        for off,changed,error in [
            (i,0x7800c000,'store opcode/operands'),
            (i,0xa19dda15,'store opcode/operands'),
            (i+1,0x7800c000,'load opcode/operands'),
            (i,0xa19d5a14,'packet markers'),
        ]:
            corrupt=w.copy();corrupt[off]=changed
            with self.assertRaisesRegex(ValueError,error):
                agent12.restore_aux_large_controller_phase23_descriptor_packet(
                    struct.pack('<13I',*corrupt))
        with self.assertRaisesRegex(ValueError,'52 bytes'):
            agent12.restore_aux_large_controller_phase23_descriptor_packet(bytes(48))

    def test_phase21_joins_source_zero_and_register_pair_packet(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE21_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE21_END),
                         (0x448068,0x4480b8))
        source=agent12.AUX_LARGE_CONTROLLER_PHASE21_SOURCE.read_text()
        for raw in ('.word','.byte','.incbin'):
            self.assertNotIn(raw,source)
        self.assertIn('r1:0 = combine(#0,r20)',source)
        self.assertIn('if (p0.new) r2 = memw(r29+#84)',source)
        self.assertIn('call platform_report',source)
        w=[0x7800c000]*20
        i=0x28//4
        w[i]=0x78000000|agent12.END_MARKER|2
        w[i+1]=0x5d000000|0x983
        w[i+2]=0x50000000|agent12.END_MARKER
        packed=struct.pack('<20I',*w)
        actual=struct.unpack('<20I',agent12.restore_aux_large_controller_phase21_frame_packet(packed))
        self.assertEqual(actual[i],0x78000000|agent12.CONTINUE_MARKER|2)
        self.assertEqual(actual[i+1:],tuple(w[i+1:]))
        for position,corrupt,error in [
            (i,0x7400c002,'opcode/operand'),
            (i,0x7800c003,'opcode/operand'),
            (i+1,0x78000983,'opcode/operands'),
            (i+1,0x5d000982,'opcode/operands'),
            (i+2,0x7800c000,'opcode/operands'),
            (i,0x78004002,'packet markers'),
            (i+2,0x50004000,'packet markers'),
        ]:
            damaged=w.copy(); damaged[position]=corrupt
            with self.assertRaisesRegex(ValueError,error):
                agent12.restore_aux_large_controller_phase21_frame_packet(
                    struct.pack('<20I',*damaged))
        with self.assertRaisesRegex(ValueError,'80 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase21_frame_packet(packed[:-4])

    def test_phase20_compound_branch_uses_symbolic_displacement(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE20_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE20_END),
                         (0x44800c,0x448068))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE20_SOURCE.read_text()
        for src in ('.word','.byte','.incbin'):
            self.assertNotIn(src,text)
        self.assertIn('if (cmp.eq(r4.new,#0)) jump:nt callback_unavailable',text)
        self.assertIn('if (!p0.new) r3 = ##507577536',text)
        self.assertIn('memw(r29+#0) = r2.new',text)
        w=[0x7800c000]*23
        w[1]=0x24000000|(2<<16)|agent12.END_MARKER|((0x2c+0x18)//2)
        converted=list(struct.unpack('<23I',agent12.restore_aux_large_controller_phase20_branch(
            struct.pack('<23I',*w))))
        self.assertEqual(converted[1]&0x3fff,0x2c//2)
        self.assertEqual(converted[1]&agent12.MARKER_MASK,agent12.END_MARKER)
        self.assertEqual(converted[:1],w[:1])
        self.assertEqual(converted[2:],w[2:])
        for replacement,regex in [
            (0x7800c022,'opcode/operands'),
            (0x2403c022,'opcode/operands'),
            (0x24024022,'packet marker'),
            (0x2402c024,'target displacement'),
        ]:
            damaged=w.copy();damaged[1]=replacement
            with self.assertRaisesRegex(ValueError,regex):
                agent12.restore_aux_large_controller_phase20_branch(
                    struct.pack('<23I',*damaged))
        with self.assertRaisesRegex(ValueError,'92 bytes'):
            agent12.restore_aux_large_controller_phase20_branch(bytes(88))

    def test_aux_controller_phase19_direct_symbolic_calls(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE19A_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE19A_END),
                         (0x447fa8,0x447fe4))
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE19B_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE19B_END),
                         (0x447fe4,0x44800c))
        a=agent12.AUX_LARGE_CONTROLLER_PHASE19A_SOURCE.read_text()
        b=agent12.AUX_LARGE_CONTROLLER_PHASE19B_SOURCE.read_text()
        for src in (a,b):
            self.assertNotIn('.word',src)
            self.assertNotIn('.byte',src)
            self.assertNotIn('.incbin',src)
        self.assertIn('call trace_sink',a)
        self.assertIn('jump continued_controller_processing',a)
        self.assertIn('p0 = r3',a)
        self.assertIn('p0 = cmp.eq(r0,#-1)',b)
        self.assertIn('if (!p0) jump:nt',b)
        self.assertIn('callr r6',b)
        self.assertIn('memw(r2+#4) = r0',b)
        self.assertIn('memw(r29+#0) = r20',b)

    @unittest.skipUnless(os.getenv('NABU_HEXAGON_LLVM_MC'), 'set Hexagon LLVM6 MC path')
    def test_aux_controller_phase19_assembles_without_patches(self):
        mc=Path(os.environ['NABU_HEXAGON_LLVM_MC'])
        first=agent12.assemble(mc,agent12.AUX_LARGE_CONTROLLER_PHASE19A_SOURCE)
        second=agent12.assemble(mc,agent12.AUX_LARGE_CONTROLLER_PHASE19B_SOURCE)
        self.assertEqual(len(first),60)
        self.assertEqual(len(second),40)
        # Both source files are directly executable Hexagon V60 assembly and
        # no assembler-produced opcode requires any post-processing.
        self.assertEqual(len(first+second),agent12.AUX_LARGE_CONTROLLER_PHASE19B_END-
                         agent12.AUX_LARGE_CONTROLLER_PHASE19A_START)

    def test_aux_controller_phase18_stores_join_descriptor_loads(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE18_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE18_END),
                         (0x447f38,0x447fa8))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE18_SOURCE.read_text()
        self.assertIn('r3:2 = add(r3:2,r5:4)', text)
        self.assertIn('p0 = bitsclr(r2,r7)', text)
        self.assertIn('r3 = insert(r9,#18,#0)', text)
        self.assertNotIn('.word', text)
        # Build synthetic A32 V60 STORE/LOAD words from public register/offset
        # encoding fields, not from proprietary firmware bytes.
        w=[0x7800c000]*28
        for offset,base,source,index in [
            (0x40,6,3,3), (0x48,2,0,0),
            (0x5c,2,3,5), (0x64,2,20,6),
        ]:
            i=offset//4
            w[i]=0xa1000000|(base<<16)|(source<<8)|index|agent12.END_MARKER
            w[i+1]=0x91000000|(17<<16)|(0x80<<8)|2|agent12.END_MARKER
        encoded=struct.pack('<28I',*w)
        generated=struct.unpack('<28I',agent12.restore_aux_large_controller_phase18_packets(encoded))
        for offset,_,_,_ in [(0x40,6,3,3),(0x48,2,0,0),(0x5c,2,3,5),(0x64,2,20,6)]:
            i=offset//4
            self.assertEqual(generated[i],(w[i]&~agent12.MARKER_MASK)|agent12.CONTINUE_MARKER)
            self.assertEqual(generated[i+1],w[i+1])
        for offset,replacement,kind in [
            (0x40,0x7800c000,'store opcode/operands'),
            (0x40,0xa186c302,'store opcode/operands'),
            (0x40,0xa1864303,'packet marker'),
            (0x44,0x7800c000,'load opcode/operands'),
            (0x44,0x919140e2,'packet marker'),
        ]:
            malformed=w.copy();malformed[offset//4]=replacement
            with self.assertRaisesRegex(ValueError,kind):
                agent12.restore_aux_large_controller_phase18_packets(struct.pack('<28I',*malformed))
        with self.assertRaisesRegex(ValueError,'112 bytes'):
            agent12.restore_aux_large_controller_phase18_packets(encoded[:-4])

    def test_aux_controller_phase17_hwloop_packet_encoding(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE17_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE17_END),
                         (0x447ee0,0x447f38))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE17_SOURCE.read_text()
        self.assertIn('loop0(inner_scan,r4)',src)
        self.assertIn('}:endloop0',src)
        self.assertIn('maxu(r3:2,r13:12)',src)
        self.assertNotIn('.word',src)
        w=[0x7800c000]*22
        w[:6]=[0x1044600c,0x7e806006,0xfb15f685,
                0x58004026,0x7c7f7fe0,0x7c00c002]
        w[0x30//4:0x38//4]=[0x58004010,0xf509c800]
        converted=struct.unpack('<22I',agent12.restore_aux_large_controller_phase17_packets(
            struct.pack('<22I',*w)))
        self.assertEqual(converted[:6],
            (0x7e806006,0xfb157685,0x1044e00c,0x7c7f7fe0,0x7c004002,0x5800c026))
        self.assertEqual(converted[0x30//4:0x38//4],(0xf5094800,0x5800c010))
        for at,value,error in [
            (0,0x7800c000,'opcode classes'),
            (0,0x1044600d,'operands'),
            (3,0x58004027,'opcode fields'),
            (4,0x7c7f7fe1,'opcode fields'),
            (0x30//4,0x78004010,'classes/fields'),
            (5,0x7c004002,'packet markers'),
            (0x34//4,0xf5094800,'packet markers'),
        ]:
            bad=w.copy();bad[at]=value
            with self.assertRaisesRegex(ValueError,error):
                agent12.restore_aux_large_controller_phase17_packets(
                    struct.pack('<22I',*bad))
        with self.assertRaisesRegex(ValueError,'88 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase17_packets(bytes(84))

    def test_aux_controller_phase16_branch_predicate_issue_order(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE16_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE16_END),
                         (0x447e80,0x447ee0))
        text=agent12.AUX_LARGE_CONTROLLER_PHASE16_SOURCE.read_text()
        self.assertIn('##controller_state_fail',text)
        self.assertIn('if (!p0.new) jump:t unsupported_buffer_mode',text)
        self.assertNotIn('.word',text)
        words=[0x7800c000]*24
        words[0x10//4:0x1c//4]=[0x11d26376,0x74196002,0x741ae004]
        result=struct.unpack('<24I',agent12.restore_aux_large_controller_phase16_packet(
            struct.pack('<24I',*words)))
        self.assertEqual(result[0x10//4:0x1c//4],
                         (0x74196002,0x741a6004,0x11d2e376))
        for offset,bad,reason in [
            (0x10//4,0x7800c000,'opcode classes'),
            (0x10//4,0x11d26377,'operands'),
            (0x14//4,0x74196003,'operands'),
            (0x18//4,0x741a6004,'packet markers'),
        ]:
            other=words.copy();other[offset]=bad
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase16_packet(
                    struct.pack('<24I',*other))
        with self.assertRaisesRegex(ValueError,'96 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase16_packet(bytes(92))

    def test_aux_controller_phase15_fail_closed_state_packet(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE15_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE15_END),
                         (0x447e1c,0x447e80))
        source=agent12.AUX_LARGE_CONTROLLER_PHASE15_SOURCE.read_text()
        self.assertIn('loop0(init_frame_loop,r18)',source)
        self.assertIn('}:endloop0',source)
        self.assertNotIn('.word',source)
        self.assertNotIn('.incbin',source)
        words=[0x7800c000]*25
        words[0x20//4:0x28//4]=[0xa191d208,0x9195c0f6]
        words[0x34//4:0x44//4]=[0x0000402b,0x10044070,0x7e806012,0x7490e997]
        output=struct.unpack('<25I',agent12.restore_aux_large_controller_phase15_packets(
            struct.pack('<25I',*words)))
        self.assertEqual(output[0x20//4],0xa1915208)
        self.assertEqual(output[0x34//4:0x44//4],
                         (0x7e806012,0x74906997,0x0000402b,0x1004c070))
        for index,bad,why in [
            (0x20//4,0x7800c000,'state-store'),
            (0x24//4,0x9195c0f7,'state-load'),
            (0x34//4,0x0100402b,'ISA classes'),
            (0x38//4,0x10044071,'predicate, extender or branch'),
            (0x3c//4,0x7e806013,'predicate, extender or branch'),
            (0x40//4,0x74906997,'packet markers'),
        ]:
            other=words.copy();other[index]=bad
            with self.assertRaisesRegex(ValueError,why):
                agent12.restore_aux_large_controller_phase15_packets(
                    struct.pack('<25I',*other))
        with self.assertRaisesRegex(ValueError,'100 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase15_packets(bytes(96))

    def test_aux_controller_phase14_callback_source(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE14_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE14_END),
                         (0x447d90,0x447e1c))
        source=agent12.AUX_LARGE_CONTROLLER_PHASE14_SOURCE.read_text()
        self.assertEqual(source.count('call resource_callback_logger'),2)
        self.assertIn('r18 = mux(p0,#1,r23)',source)
        self.assertIn('r22 = +mpyi(r18,#28)',source)
        self.assertIn('callr r2',source)
        self.assertNotIn('.word',source)
        self.assertNotIn('.incbin',source)
        self.assertEqual(agent12.AUX_LARGE_CONTROLLER_PHASE13_END,
                         agent12.AUX_LARGE_CONTROLLER_PHASE14_START)

    def test_aux_controller_phase13_disjoint_source_and_packet(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE13_GATE_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE13_GATE_END),
                         (0x447d20,0x447d28))
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE13_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE13_END),
                         (0x447d38,0x447d90))
        self.assertEqual(agent12.AUX_LARGE_CONTROLLER_PHASE13_START-
                         agent12.AUX_LARGE_CONTROLLER_PHASE13_GATE_END,16)
        self.assertEqual(agent12.AUX_LARGE_CONTROLLER_PHASE13_GAP_END,
                         agent12.AUX_LARGE_CONTROLLER_PHASE13_START)
        a=agent12.AUX_LARGE_CONTROLLER_PHASE13_SOURCE.read_text()
        b=agent12.AUX_LARGE_CONTROLLER_PHASE13_GATE_SOURCE.read_text()
        for src in [a,b]:
            self.assertNotIn('.word',src)
            self.assertNotIn('.incbin',src)
        self.assertIn('loop0(scan_body,r6)',a)
        self.assertIn('}:endloop0',a)
        self.assertIn('r3 = ##65535',b)
        words=[0x7800c000]*22
        words[0x14//4:0x20//4]=[0xbfe27fe6,0x7582c021,0x480307d4]
        result=struct.unpack('<22I',agent12.restore_aux_large_controller_phase13_packets(
            struct.pack('<22I',*words)))
        self.assertEqual(result[0x18//4],0x75824021)
        for idx,bad,reason in [
            (0x14//4,0x7f007fe6,'ISA fields'),
            (0x18//4,0x7582c022,'ISA fields'),
            (0x1c//4,0x480307d3,'ISA fields'),
            (0x18//4,0x75824021,'packet markers'),
            (0x1c//4,0x480347d4,'packet markers'),
        ]:
            other=words.copy();other[idx]=bad
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_large_controller_phase13_packets(
                    struct.pack('<22I',*other))
        with self.assertRaisesRegex(ValueError,'88 source-assembled bytes'):
            agent12.restore_aux_large_controller_phase13_packets(bytes(84))

    def test_insert_controller_source_and_guarded_packet_reordering(self):
        self.assertEqual((agent12.AUX_INSERT_CONTROLLER_START,
                          agent12.AUX_INSERT_CONTROLLER_END),(0x447234,0x4472d0))
        source=agent12.AUX_INSERT_CONTROLLER_SOURCE.read_text()
        self.assertIn('memw(r3+#0)',source)
        self.assertIn('dealloc_return',source)
        self.assertNotIn('.word',source)
        words=[0x7800c000]*39
        first=(0x44723c-agent12.AUX_INSERT_CONTROLLER_START)//4
        branch=(0x4472a4-agent12.AUX_INSERT_CONTROLLER_START)//4
        lo=(0x4472b0-agent12.AUX_INSERT_CONTROLLER_START)//4
        last=(0x4472c4-agent12.AUX_INSERT_CONTROLLER_START)//4
        words[first]=0xa1ddd200
        words[branch:branch+2]=[0x5cdf78fa,0xf204c310]
        words[lo:lo+2]=[0x91854084,0xa190c528]
        words[last]=0xa183c200
        assembled=struct.pack('<39I',*words)
        reordered=struct.unpack('<39I',agent12.restore_aux_insert_controller_packets(assembled))
        self.assertEqual(reordered[first],0xa1dd5200)
        self.assertEqual(reordered[branch:branch+2],(0xf2044310,0x5cdff8fa))
        self.assertEqual(reordered[lo:lo+2],(0xa1904528,0x9185c084))
        self.assertEqual(reordered[last],0xa1834200)
        for idx,wrong,pattern in [
            (first,0x7800c000,'instruction semantics'),
            (first,0xa1dd5200,'packet marker'),
            (branch,0x7800c000,'opcode classes'),
            (lo+1,0x7800c000,'opcode classes'),
            (last,0xa1834200,'packet marker')
        ]:
            bad=words.copy();bad[idx]=wrong
            with self.assertRaisesRegex(ValueError,pattern):
                agent12.restore_aux_insert_controller_packets(struct.pack('<39I',*bad))
        with self.assertRaisesRegex(ValueError,'156 compiled source bytes'):
            agent12.restore_aux_insert_controller_packets(assembled[:-4])

    def test_remove_controller_reorders_only_checked_hexagon_ops(self):
        self.assertEqual((agent12.AUX_REMOVE_CONTROLLER_START,
                          agent12.AUX_REMOVE_CONTROLLER_END),(0x4472d0,0x447324))
        source=agent12.AUX_REMOVE_CONTROLLER_SOURCE.read_text()
        self.assertIn('dealloc_return',source)
        self.assertNotIn('.word',source)
        words=[0x7800c000]*21
        first=(0x447300-agent12.AUX_REMOVE_CONTROLLER_START)//4
        second=(0x447310-agent12.AUX_REMOVE_CONTROLLER_START)//4
        words[first:first+2]=[0x3c42c280,0x91ddc010]
        words[second:second+2]=[0x479044e2,0xa183c204]
        packed=struct.pack('<21I',*words)
        out=struct.unpack('<21I',agent12.restore_aux_remove_controller_packets(packed))
        self.assertEqual(out[first],0x3c424280)
        self.assertEqual(out[second:second+2],(0xa1834204,0x4790c4e2))
        for index,bad_word,pattern in [
            (first,0x7800c000,'opcode classes'),
            (first,0x3c424280,'packet markers'),
            (second,0x48004000,'opcode classes'),
            (second+1,0xa1834204,'packet markers')
        ]:
            invalid=words.copy();invalid[index]=bad_word
            with self.assertRaisesRegex(ValueError,pattern):
                agent12.restore_aux_remove_controller_packets(struct.pack('<21I',*invalid))
        with self.assertRaisesRegex(ValueError,'84 source bytes'):
            agent12.restore_aux_remove_controller_packets(packed[:-4])

    def test_late_callback_and_controller_scan_bounds(self):
        self.assertEqual((agent12.AUX_CALLBACK_START,agent12.AUX_CALLBACK_END),
                         (0x447324,0x447388))
        self.assertEqual((agent12.AUX_FIND_CONTROLLER_START,
                          agent12.AUX_FIND_CONTROLLER_END),(0x447388,0x447410))
        self.assertEqual((agent12.AUX_RELEASE_REGISTRY_START,
                          agent12.AUX_RELEASE_REGISTRY_END),(0x447410,0x447448))
        sources=[agent12.AUX_CALLBACK_SOURCE,agent12.AUX_FIND_CONTROLLER_SOURCE,
                 agent12.AUX_RELEASE_REGISTRY_SOURCE]
        for path in sources:
            text=path.read_text()
            self.assertNotIn('.word',text)
            self.assertIn('dealloc_return',text)
        self.assertIn('loop0',agent12.AUX_FIND_CONTROLLER_SOURCE.read_text())

    def test_callback_extender_duplex_rewrite_rejects_unrelated_source(self):
        words=[0x7800c000]*25
        i=(0x447340-agent12.AUX_CALLBACK_START)//4
        words[i-1:i+2]=[0x01e44630,0x7800c086,0x48050124]
        packed=struct.pack('<25I',*words)
        built=struct.unpack('<25I',agent12.restore_aux_callback_extender_packet(packed))
        self.assertEqual(built[i],0x78004086)
        self.assertEqual(built[:i],tuple(words[:i]))
        self.assertEqual(built[i+1:],tuple(words[i+1:]))
        for change,expected in [
            ((i-1,0x7e004000),'opcode classes'),
            ((i,0x7800c087),'operand'),
            ((i+1,0x4805c124),'packet markers'),
        ]:
            bad=words.copy();bad[change[0]]=change[1]
            with self.assertRaisesRegex(ValueError,expected):
                agent12.restore_aux_callback_extender_packet(struct.pack('<25I',*bad))
        with self.assertRaisesRegex(ValueError,'100 source-assembled bytes'):
            agent12.restore_aux_callback_extender_packet(packed[:-4])

    def test_aux_table_scanners_are_independent_source_functions(self):
        find=agent12.AUX_FIND_FREE_SOURCE.read_text()
        resolve=agent12.AUX_RESOLVE_SOURCE.read_text()
        self.assertIn('loop0',find)
        self.assertIn('jumpr r31',find)
        self.assertIn('loop0',resolve)
        self.assertIn('dealloc_return',resolve)
        self.assertNotIn('.word',find)
        self.assertNotIn('.word',resolve)
        self.assertEqual(agent12.AUX_RESOLVE_END-agent12.AUX_FIND_FREE_START,216)

    def test_aux_session_prefix_explicitly_stops_before_framekey(self):
        self.assertEqual(agent12.AUX_SESSION_PREFIX_END,0x446EE8)
        source=agent12.AUX_SESSION_PREFIX_SOURCE.read_text()
        self.assertNotIn('.word',source)
        self.assertNotIn('framekey',source.split('.text',1)[1])
        self.assertIn('bitsplit',source)
        self.assertEqual(agent12.AUX_SESSION_PREFIX_END-agent12.AUX_SESSION_PREFIX_START,88)
        self.assertEqual(agent12.AUX_DIAGNOSTIC_HELPERS_END-agent12.AUX_DIAGNOSTIC_HELPERS_START,116)

    def test_aux_registration_packet_change_rejects_unrelated_ops(self):
        words=[0x7800c000]*129
        i=(0x446D04-agent12.AUX_REGISTRATION_START)//4
        words[i:i+2]=[0x11b040f6,0xfd11f180]
        rebuilt=struct.unpack('<129I',agent12.restore_aux_registration_new_value_packet(
            struct.pack('<129I',*words)))
        self.assertEqual([x>>24 for x in rebuilt[i:i+2]],[0xfd,0x11])
        self.assertEqual([x&agent12.MARKER_MASK for x in rebuilt[i:i+2]],
                         [agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[i]=0x780040f6
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_aux_registration_new_value_packet(struct.pack('<129I',*bad))
        bad=words.copy();bad[i+1]=0xfd117180
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_registration_new_value_packet(struct.pack('<129I',*bad))
        with self.assertRaisesRegex(ValueError,'516 source-assembled bytes'):
            agent12.restore_aux_registration_new_value_packet(bytes(512))

    def test_aux_duplex_compaction_rejects_nonmatching_semantics(self):
        words=[0x7f00c000]*16
        a=0x24//4
        words[a:a+3]=[0x0fff7ff9,0x58004050,0x7800c003]
        b=0x38//4  # second pair in 64-byte assembler version
        words[b:b+2]=[0x59ff7f2c,0x7c00c002]
        output=agent12.compact_aux_controller_veneer_duplex(struct.pack('<16I',*words))
        outwords=struct.unpack('<15I',output)
        self.assertEqual(outwords[a+1],0x1603c050)
        self.assertEqual(outwords[0x34//4],0x7c004002)
        self.assertEqual(outwords[0x38//4],0x59ffff2c)
        bad=words.copy();bad[a+2]=0x7800c004
        with self.assertRaisesRegex(ValueError,'not r3=#0'):
            agent12.compact_aux_controller_veneer_duplex(struct.pack('<16I',*bad))
        bad=words.copy();bad[a+1]=0x78004050
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.compact_aux_controller_veneer_duplex(struct.pack('<16I',*bad))
        bad=words.copy();bad[b]=0x59ffff2c
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.compact_aux_controller_veneer_duplex(struct.pack('<16I',*bad))
        with self.assertRaisesRegex(ValueError,'64 source-assembled bytes'):
            agent12.compact_aux_controller_veneer_duplex(bytes(60))

    def test_aux_runtime_predicate_and_store_rewrites_fail_closed(self):
        words=[0x7800c000]*93
        first=(0x446a84-agent12.AUX_RUNTIME_INIT_START)//4
        second=(0x446ab8-agent12.AUX_RUNTIME_INIT_START)//4
        tail=(0x446b4c-agent12.AUX_RUNTIME_INIT_START)//4
        words[first:first+2]=[0x5c0058a4,0x7e00e032]
        words[second:second+2]=[0x10406188,0x7800c012]
        words[tail:tail+3]=[0x91934000,0x01e4467c,0x4800d228]
        packed=struct.pack('<93I',*words)
        rewritten=struct.unpack('<93I',agent12.restore_aux_runtime_init_packets(packed))
        self.assertEqual([x>>24 for x in rewritten[first:first+2]],[0x7e,0x5c])
        self.assertEqual([x>>24 for x in rewritten[second:second+2]],[0x78,0x10])
        self.assertEqual([x>>24 for x in rewritten[tail:tail+3]],[0x01,0x48,0x91])
        self.assertEqual([x&agent12.MARKER_MASK for x in rewritten[tail:tail+3]],
                         [agent12.CONTINUE_MARKER,agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[first]=0x78004000
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_aux_runtime_init_packets(struct.pack('<93I',*bad))
        bad=words.copy();bad[tail+1]=0x01e4c67c
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_runtime_init_packets(struct.pack('<93I',*bad))
        with self.assertRaisesRegex(ValueError,'372 source bytes'):
            agent12.restore_aux_runtime_init_packets(bytes(368))

    def test_aux_validator_predicate_packet_guard(self):
        words=[0x7800c000]*153
        i=(0x4468f0-agent12.AUX_VALIDATOR_START)//4
        j=(0x446994-agent12.AUX_VALIDATOR_START)//4
        words[i:i+2]=[0x5c205852,0x75424720]
        words[j:j+2]=[0x5c20585a,0x7542c4e0]
        result=struct.unpack('<153I',agent12.restore_aux_validator_predicate_packets(
            struct.pack('<153I',*words)))
        self.assertEqual([x>>24 for x in result[i:i+2]],[0x75,0x5c])
        self.assertEqual([x>>24 for x in result[j:j+2]],[0x75,0x5c])
        self.assertEqual([x & agent12.MARKER_MASK for x in result[j:j+2]],
                         [agent12.CONTINUE_MARKER,agent12.END_MARKER])
        malformed=words.copy();malformed[i]=0x78004000
        with self.assertRaisesRegex(ValueError,'instruction classes'):
            agent12.restore_aux_validator_predicate_packets(struct.pack('<153I',*malformed))
        malformed=words.copy();malformed[j+1]=0x754240e0
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_validator_predicate_packets(struct.pack('<153I',*malformed))
        with self.assertRaisesRegex(ValueError,'612 source bytes'):
            agent12.restore_aux_validator_predicate_packets(bytes(608))

    def test_aux_entry_is_source_encoded_not_oem_bytes(self):
        self.assertTrue(agent12.AUX_ENTRY_SOURCE.is_file())
        source=agent12.AUX_ENTRY_SOURCE.read_text()
        self.assertIn("call controller_call",source)
        self.assertIn("deallocframe",source)
        self.assertNotIn(".word",source)
        self.assertEqual(agent12.AUX_ENTRY_END-agent12.AUX_ENTRY_START,20)

    def test_aux_registry_core_rearrangements_reject_other_packets(self):
        source=agent12.AUX_REGISTRY_CORE_SOURCE.read_text()
        self.assertIn('if (!p0.new) r0 = add(r16,#0)',source)
        self.assertIn('memw(##507617076) = r16',source)
        self.assertIn('dealloc_return',source)
        self.assertNotIn('.word',source)
        self.assertEqual(agent12.AUX_LOOKUP_END,agent12.AUX_REGISTRY_CORE_START)
        self.assertEqual(agent12.AUX_REGISTRY_CORE_END,agent12.AUX_DISPATCH_START)
        words=[0x7800c000]*114
        words[0x10//4:0x18//4]=[0x10486022,0x74906000]
        words[0x108//4:0x114//4]=[0x5c205812,0x01e44407,0x7e80e763]
        words[0x17c//4:0x184//4]=[0x479044e2,0xa183c204]
        for off,raw in [(0xd4,0x2432c0d2),(0xdc,0x59ffffcc),
                        (0x13c,0x5a00def4),(0x18c,0x5a00c74e)]:
            words[off//4]=raw
        binary=struct.pack('<114I',*words)
        rebuilt=struct.unpack('<114I',agent12.restore_aux_registry_core_packets(binary))
        self.assertEqual([x>>24 for x in rebuilt[0x10//4:0x18//4]],[0x74,0x10])
        self.assertEqual([x>>24 for x in rebuilt[0x108//4:0x114//4]],[0x01,0x7e,0x5c])
        self.assertEqual([x>>24 for x in rebuilt[0x17c//4:0x184//4]],[0xa1,0x47])
        self.assertEqual(rebuilt[0xd4//4]&0x3f,0x16)
        self.assertEqual(rebuilt[0xdc//4]&0x3f,0x10)
        self.assertEqual(rebuilt[0x13c//4]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(rebuilt[0x18c//4]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        for off,modified,pattern in [
            (0x10,0x78004022,'classes'),
            (0x14,0x7490c000,'parse markers'),
            (0x108,0x78004012,'classes'),
            (0x10c,0x01e4c407,'packet markers'),
            (0x17c,0x780044e2,'classes'),
            (0x180,0xa1834204,'parse markers'),
            (0x13c,0x5a005ef4,'packet marker'),
            (0xd4,0x2432c0d6,'branch class/immediate'),
        ]:
            bad=words.copy();bad[off//4]=modified
            with self.assertRaisesRegex(ValueError,pattern):
                agent12.restore_aux_registry_core_packets(struct.pack('<114I',*bad))
        with self.assertRaisesRegex(ValueError,'456 source-assembled bytes'):
            agent12.restore_aux_registry_core_packets(binary[:-4])

    def test_public_framekey_encoding_and_parser_layout(self):
        # This tests an ISA class with multiple independently decoded GPRs,
        # rather than embedding the firmware's four-byte instruction word.
        self.assertEqual(agent12.encode_v60_framekey(3),0x6a114003)
        self.assertEqual(agent12.encode_v60_framekey(16),0x6a114010)
        self.assertEqual(agent12.encode_v60_framekey(23,end_packet=True),0x6a11c017)
        for invalid in (0,8,13,15,24,31,-1):
            with self.assertRaisesRegex(ValueError,'framekey Rd'):
                agent12.encode_v60_framekey(invalid)
        code=agent12.AUX_SESSION_VALIDATION_SOURCE.read_text()
        self.assertIn('.space 4',code)
        self.assertIn('framekey',code)
        self.assertNotIn('.word',code)
        self.assertIn('p0 = bitsset',code)
        self.assertIn('dealloc_return',code)
        self.assertEqual(agent12.AUX_SESSION_PREFIX_END,agent12.AUX_SESSION_VALIDATION_START)
        self.assertEqual(agent12.AUX_SESSION_VALIDATION_END,agent12.AUX_FIND_FREE_START)

    def test_framekey_packet_adjustment_rejects_unrelated_source(self):
        words=[0x7800c000]*80
        words[0]=0  # deliberate placeholder of independent assembly
        for offset, group in [
            (0x58,[0x104e4032,0xfd11f000]),
            (0x78,[0x5c00481a,0x75165fc0,0xfd11f000]),
            (0x84,[0x5c20481c,0x7516dfe0]),
            (0xa4,[0x469dd218,0x5800c026]),
            (0x11c,[0x5bff7bce,0x01e44404,0x6bf32808]),
        ]:
            words[offset//4:offset//4+len(group)]=group
        for offset,w in [(0x48,0x1600c074), (0xac,0xb01dc183),
                         (0xcc,0xb01dc183), (0xfc,0x5800c01a),
                         (0x108,0x2402c00e), (0x118,0x50a4c000)]:
            words[offset//4]=w
        source=struct.pack('<80I',*words)
        output=struct.unpack('<80I',agent12.restore_aux_session_validation_packets(source))
        self.assertEqual(output[0],agent12.encode_v60_framekey(3))
        self.assertEqual(output[0x58//4]>>24,0xfd)
        self.assertEqual(output[0x5c//4]>>24,0x10)
        self.assertEqual([x>>24 for x in output[0x78//4:0x84//4]],
                         [0x75,0xfd,0x5c])
        self.assertEqual(output[0xa4//4]>>24,0x58)
        self.assertEqual(output[0x124//4]>>24,0x5b)
        for offset,mutated,pattern in [
            (0,0x7800c003,'framekey layout slot'),
            (0x58,0x78004032,'opcode classes'),
            (0x78,0x5c00c81a,'parser bits'),
            (0xa4,0x7800c000,'opcode classes'),
            (0xac,0xb01d4183,'stack-pointer add'),
            (0x108,0x2402c010,'PC-relative target'),
            (0x118,0x50a44000,'callr opcode'),
            (0x11c,0x78004000,'opcode classes'),
        ]:
            wrong=words.copy();wrong[offset//4]=mutated
            with self.assertRaisesRegex(ValueError,pattern):
                agent12.restore_aux_session_validation_packets(
                    struct.pack('<80I',*wrong))
        with self.assertRaisesRegex(ValueError,'320 independently assembled bytes'):
            agent12.restore_aux_session_validation_packets(source[:-4])

    def test_session_registry_duplex_encodes_architectural_fields(self):
        # Hexagon A/A duplex: slot1 add-immediate, slot0 increment.
        expected = agent12.encode_v60_duplex_addi_inc(
            add_dest=21, add_src=13, add_imm=16, inc_dest=20, inc_src=20)
        self.assertEqual(expected, 0x210d31cc)
        self.assertEqual((expected >> 29) << 1 | ((expected >> 13) & 1),3)
        self.assertEqual((expected >> 16) & 0x1fff,0x10d)
        self.assertEqual(expected & 0x1fff,0x11cc)
        self.assertEqual((expected>>14)&3,0)  # Duplex ends packet.
        for args in (
            dict(add_dest=21,add_src=20,add_imm=16,inc_dest=20,inc_src=20),
            dict(add_dest=21,add_src=13,add_imm=32,inc_dest=20,inc_src=20),
            dict(add_dest=21,add_src=13,add_imm=16,inc_dest=13,inc_src=20),
            dict(add_dest=21,add_src=13,add_imm=16,inc_dest=20,inc_src=13),
        ):
            with self.assertRaises(ValueError):
                agent12.encode_v60_duplex_addi_inc(**args)

    def test_registry_insert_compaction_fails_closed(self):
        # Build a synthetic LLVM6 word stream with the exactly expected
        # instruction *classes* and parser state at affected offsets.
        words=[0x7800c000]*78
        words[0x18//4:0x20//4]=[0x1049601e,0x7490e000]
        words[0x118//4:0x120//4]=[0xb00d4215,0xb014c034]
        for off,raw in [(0x50,0x5800c06c),(0x90,0x5800c04c),
                        (0xb4,0x2402c03c),(0x114,0x2502cf0c),
                        (0x124,0x2132f4d2)]:
            words[off//4]=raw
        binary=struct.pack('<78I',*words)
        packed=agent12.restore_aux_session_registry_packets(binary)
        self.assertEqual(len(packed),agent12.AUX_SESSION_REGISTRY_END-agent12.AUX_SESSION_REGISTRY_START)
        result=struct.unpack('<77I',packed)
        self.assertEqual(result[0x118//4],0x210d31cc)
        self.assertEqual(result[0x18//4]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(result[0x1c//4]&agent12.MARKER_MASK,agent12.END_MARKER)
        self.assertEqual(result[0x50//4]&0x3f,0x2a)
        self.assertEqual(result[0x90//4]&0x3f,0x0a)
        self.assertEqual(result[0xb4//4]&0x3f,0x3a)
        self.assertEqual(result[0x114//4]&0x3f,0x0a)
        self.assertEqual(result[0x120//4]&0x3f,0x14)
        for idx,word,why in [
            (0x18//4,0x78004000,'opcode classes'),
            (0x1c//4,0x74906000,'packet markers'),
            (0x118//4,0xa00d4215,'scalar A2 add'),
            (0x11c//4,0xb0144034,'parse markers'),
            (0x50//4,0x7a00c06c,'branch ISA class'),
        ]:
            damaged=words.copy();damaged[idx]=word
            with self.assertRaisesRegex(ValueError,why):
                agent12.restore_aux_session_registry_packets(struct.pack('<78I',*damaged))
        with self.assertRaisesRegex(ValueError,'312 independently assembled bytes'):
            agent12.restore_aux_session_registry_packets(binary[:-4])

    def test_factory_format_and_class_scanners_reject_malformed_packets(self):
        for file,lines in [
            (agent12.AUX_FACTORY_SOURCE,('allocframe(#8)','dealloc_return')),
            (agent12.AUX_FIND_FORMAT_SOURCE,('loop0(',':endloop0')),
            (agent12.AUX_FIND_CLASS_SOURCE,('loop0(',':endloop0')),
            (agent12.AUX_CONTROLLER_SLOT_SOURCE,('memw(r0+#48)','jumpr r31')),
        ]:
            text=file.read_text()
            self.assertNotIn('.word',text)
            for term in lines:
                self.assertIn(term,text)
        self.assertEqual((agent12.AUX_FACTORY_START,agent12.AUX_FACTORY_END),
                         (0x4475c0,0x447608))
        self.assertEqual((agent12.AUX_FIND_FORMAT_START,agent12.AUX_FIND_FORMAT_END),
                         (0x447608,0x447664))
        self.assertEqual((agent12.AUX_FIND_CLASS_START,agent12.AUX_FIND_CLASS_END),
                         (0x447664,0x4476b8))
        self.assertEqual((agent12.AUX_CONTROLLER_SLOT_START,agent12.AUX_CONTROLLER_SLOT_END),
                         (0x4476b8,0x4476bc))
        fmt=[0x7800c000]*23
        fmt[0x24//4:0x34//4]=[0x5c00480e,0x00204000,0x75044000,0x7e00e000]
        original=struct.pack('<23I',*fmt)
        out=struct.unpack('<23I',agent12.restore_aux_find_format_predicate_packet(original))
        self.assertEqual(out[0x24//4:0x34//4],
                         (0x00204000,0x75044000,0x7e006000,0x5c00c80e))
        for off,bad,reason in [
            (0x24,0x7800400e,'opcode classes'),
            (0x28,0x0020c000,'packet markers'),
            (0x30,0x7e006000,'packet markers')]:
            mutated=fmt.copy();mutated[off//4]=bad
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_find_format_predicate_packet(struct.pack('<23I',*mutated))
        with self.assertRaisesRegex(ValueError,'92 compiled bytes'):
            agent12.restore_aux_find_format_predicate_packet(original[:-4])
        cls=[0x7800c000]*21
        cls[0x08//4:0x14//4]=[0x5c005814,0xf2474110,0xfb21e787]
        cls[0x14//4]=0x4585c0e6
        packed=struct.pack('<21I',*cls)
        result=struct.unpack('<21I',agent12.restore_aux_find_class_new_predicate(packed))
        self.assertEqual(result[0x08//4:0x14//4],
                         (0xf2474110,0xfb216787,0x5c005814))
        self.assertEqual(result[0x14//4],0x4785c0e6)
        for off,bad,why in [
            (0x08,0x78004014,'ISA classes'),
            (0x10,0xfb216787,'packet markers'),
            (0x14,0x4585c0e7,'operand/ISA class'),
            (0x14,0x458540e6,'packet marker')]:
            mutated=cls.copy();mutated[off//4]=bad
            with self.assertRaisesRegex(ValueError,why):
                agent12.restore_aux_find_class_new_predicate(struct.pack('<21I',*mutated))
        with self.assertRaisesRegex(ValueError,'84 compiled bytes'):
            agent12.restore_aux_find_class_new_predicate(packed[:-4])

    def test_auxiliary_cleanup_packet_producers_are_checked(self):
        source=agent12.AUX_CLEANUP_SOURCE.read_text()
        self.assertIn('memw(r16+#32) = r2.new',source)
        self.assertIn('r3 = add(r3,mpyi(#28,r2))',source)
        self.assertIn('dealloc_return',source)
        self.assertNotIn('.word',source)
        self.assertEqual((agent12.AUX_CLEANUP_START,agent12.AUX_CLEANUP_END),
                         (0x4474b0,0x4475c0))
        words=[0x7800c000]*68
        for off,branch in [(0x88,0x10c2401c),(0xb4,0x10b260f0)]:
            words[off//4:off//4+3]=[branch,0xbfe27fe2,0xa1b0d208]
        packed=struct.pack('<68I',*words)
        got=struct.unpack('<68I',agent12.restore_aux_cleanup_compare_packets(packed))
        self.assertEqual(got[0x88//4:0x94//4],
                         (0xbfe27fe2,0x10c2401c,0xa1b0d408))
        self.assertEqual(got[0xb4//4:0xc0//4],
                         (0xbfe27fe2,0x10b260f0,0xa1b0d408))
        for offset,value,reason in [
            (0x88,0x7800401c,'ISA classes'),
            (0x8c,0xbfe2ffe2,'packet markers'),
            (0x90,0xa1b0d408,'new-value producer slot'),
            (0xb4,0x780040f0,'ISA classes'),
            (0xb8,0xbfe2ffe2,'packet markers'),
            (0xbc,0xa1b0d608,'new-value producer slot'),
        ]:
            damaged=words.copy();damaged[offset//4]=value
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_cleanup_compare_packets(struct.pack('<68I',*damaged))
        with self.assertRaisesRegex(ValueError,'272 independently source-assembled bytes'):
            agent12.restore_aux_cleanup_compare_packets(packed[:-4])

    def test_auxiliary_notification_packet_reordering_is_fail_closed(self):
        path=agent12.AUX_NOTIFY_SCAN_SOURCE
        self.assertTrue(path.is_file())
        source=path.read_text()
        self.assertIn('memd(r29+#0) = r19:18',source)
        self.assertIn('if (!p0.new) r18 = add(r17,##168)',source)
        self.assertIn('dealloc_return',source)
        self.assertNotIn('.word',source)
        self.assertEqual(agent12.AUX_RELEASE_REGISTRY_END,agent12.AUX_NOTIFY_SCAN_START)
        self.assertEqual(agent12.AUX_NOTIFY_SCAN_END-agent12.AUX_NOTIFY_SCAN_START,104)
        words=[0x7800c000]*26
        words[0x08//4:0x10//4]=[0x91315102,0xa1ddd200]
        words[0x14//4:0x20//4]=[0x5c005818,0x00004002,0x7491e512]
        words[0x28//4:0x30//4]=[0x1040601c,0xfd10f200]
        binary=struct.pack('<26I',*words)
        new_words=struct.unpack('<26I',agent12.restore_aux_notify_callback_packets(binary))
        self.assertEqual(new_words[0x08//4:0x10//4],(0xa1dd5200,0x9131d102))
        self.assertEqual(new_words[0x14//4:0x20//4],(0x00004002,0x74916512,0x5c00d818))
        self.assertEqual(new_words[0x28//4:0x30//4],(0xfd107200,0x1040e01c))
        for offset,changed,reason in [
            (0x08,0x78004000,'opcode classes'),
            (0x0c,0xa1dd5200,'packet markers'),
            (0x14,0x78004000,'ISA classes'),
            (0x18,0x0000c002,'packet markers'),
            (0x1c,0x74916512,'packet markers'),
            (0x28,0x7800401c,'ISA classes'),
            (0x2c,0xfd107200,'packet markers'),
        ]:
            corrupt=words.copy();corrupt[offset//4]=changed
            with self.assertRaisesRegex(ValueError,reason):
                agent12.restore_aux_notify_callback_packets(struct.pack('<26I',*corrupt))
        with self.assertRaisesRegex(ValueError,'104 source bytes'):
            agent12.restore_aux_notify_callback_packets(binary[:-4])

    def test_auxiliary_elf32_symbols_and_generated_sysv_hash(self):
        generated=agent12.emit_auxiliary_elf_symbols()
        self.assertEqual(len(generated),296)
        self.assertEqual((agent12.AUX_DYNAMIC_START,agent12.AUX_DYNAMIC_END),
                         (0x4460d8,0x446200))
        self.assertEqual(generated[:19],b"/usr/lib/libc.so.1\0")
        self.assertEqual(generated[19:36],bytes(17))
        symbol_table=generated[0x24:0x24+7*16]
        strings=generated[0x24+7*16:0x24+7*16+56]
        self.assertEqual(strings,b"\0dlsym\0dladdr\0dlerror\0dlopen\0dlclose\0dlinfo\0_rtld_debug\0")
        for index,(name,address,size,kind,shndx) in enumerate(agent12.AUX_EXPORTS):
            item=symbol_table[index*16:(index+1)*16]
            name_off,decoded_addr,decoded_size,binding,other,section=struct.unpack('<IIIBBH',item)
            self.assertEqual(strings[name_off:].split(b"\0",1)[0].decode(),name)
            self.assertEqual((decoded_addr,decoded_size,binding,other,section),
                             (address,size,0x10|kind,0,shndx))
        self.assertEqual(generated[0x24+7*16+56:0x24+7*16+56+4],bytes(4))
        h=generated[0xd0:0x118]
        nbucket,nchain=struct.unpack_from('<II',h)
        self.assertEqual((nbucket,nchain),(8,8))
        self.assertEqual(struct.unpack_from('<8I',h,8),(0,0,3,0,0,5,4,7))
        self.assertEqual(struct.unpack_from('<8I',h,8+32),(0,0,0,2,0,1,0,6))
        self.assertEqual(h,agent12.make_elf32_sysv_hash(tuple(x[0] for x in agent12.AUX_EXPORTS)))
        self.assertEqual(generated[-16:],bytes(16))
        self.assertEqual(agent12.emit_auxiliary_code_alignment(),bytes(12))

    def test_auxiliary_elf_hash_uses_name_based_collision_chains(self):
        self.assertEqual(agent12.elf32_sysv_hash('dlsym') % 8,5)
        self.assertEqual(agent12.elf32_sysv_hash('dlerror') % 8,2)
        self.assertEqual(agent12.elf32_sysv_hash('_rtld_debug') % 8,7)
        with self.assertRaisesRegex(ValueError,'distinct symbols'):
            agent12.make_elf32_sysv_hash(('hello','hello'))
        with self.assertRaisesRegex(ValueError,'bucket count'):
            agent12.make_elf32_sysv_hash(('x',),0)
        with self.assertRaisesRegex(ValueError,'bucket count'):
            agent12.make_elf32_sysv_hash(('x',),65536)

    def test_regions_are_disjoint_and_bounded_by_executable_segments(self):
        self.assertEqual((agent12.ATOMIC_START, agent12.ATOMIC_END), (0x7C000, 0x7C040))
        self.assertEqual((agent12.CONTROL_START, agent12.CONTROL_END), (0x7C040, 0x7C0A0))
        self.assertEqual((agent12.AUX_START, agent12.AUX_END), (0x446000, 0x4460D8))
        self.assertEqual(agent12.ATOMIC_END - agent12.ATOMIC_START, 64)
        self.assertEqual(agent12.CONTROL_END - agent12.CONTROL_START, 96)
        self.assertEqual(agent12.AUX_END - agent12.AUX_START, 216)
        self.assertEqual(agent12.ATOMIC_END, agent12.CONTROL_START)
        self.assertEqual((agent12.SERVICE_START,agent12.SERVICE_END),(0x7C2EC,0x7C324))
        self.assertLess(agent12.CONTROL_END,agent12.SERVICE_START)
        self.assertEqual((agent12.SERVICE_MMIO_START,agent12.SERVICE_MMIO_END),
                         (0x7C324,0x7C3A8))
        self.assertEqual((agent12.SERVICE_FLAGS_START,agent12.SERVICE_FLAGS_END),
                         (0x7C3A8,0x7C40C))
        self.assertEqual(agent12.SERVICE_END, agent12.SERVICE_MMIO_START)
        self.assertEqual(agent12.SERVICE_MMIO_END,agent12.SERVICE_FLAGS_START)
        self.assertEqual(agent12.SERVICE_FLAGS_END,agent12.SERVICE_ALLOC_START)
        self.assertEqual((agent12.SERVICE_ALLOC_START,agent12.SERVICE_ALLOC_END),
                         (0x7C40C,0x7C4B0))
        self.assertLess(agent12.SERVICE_ALLOC_END,agent12.AUX_START)
        self.assertLessEqual(agent12.AUX_END, 0x446000 + 0x11DA4)
        self.assertEqual(agent12.AUX_INIT_START,0x446200)
        self.assertEqual(agent12.AUX_INIT_END,0x446254)
        self.assertEqual(agent12.AUX_GUARD_START,0x446260)
        self.assertEqual(agent12.AUX_GUARD_END,0x446278)
        self.assertEqual(agent12.AUX_END,0x4460D8)
        self.assertEqual(agent12.AUX_END,agent12.AUX_DYNAMIC_START)
        self.assertEqual(agent12.AUX_DYNAMIC_END,agent12.AUX_INIT_START)
        self.assertEqual(agent12.AUX_INIT_END,agent12.AUX_INIT_ALIGN_START)
        self.assertEqual(agent12.AUX_INIT_ALIGN_END,agent12.AUX_GUARD_START)
        self.assertEqual((agent12.AUX_STAGE_START,agent12.AUX_STAGE_END), (0x446278,0x4462E0))
        self.assertEqual((agent12.AUX_MMIO_START,agent12.AUX_MMIO_END), (0x4462E0,0x4462F8))
        self.assertEqual(agent12.AUX_GUARD_END,agent12.AUX_STAGE_START)
        self.assertEqual(agent12.AUX_STAGE_END,agent12.AUX_MMIO_START)
        self.assertEqual((agent12.AUX_HEAP_START,agent12.AUX_HEAP_END),(0x4462F8,0x4463B4))
        self.assertEqual(agent12.AUX_MMIO_END,agent12.AUX_HEAP_START)
        self.assertEqual(agent12.AUX_HEAP_END,agent12.AUX_WORKER_START)
        self.assertEqual((agent12.AUX_WORKER_START,agent12.AUX_WORKER_END),
                         (0x4463B4,0x446420))
        self.assertEqual((agent12.AUX_LOOKUP_START,agent12.AUX_LOOKUP_END),
                         (0x446420,0x446488))
        self.assertEqual(agent12.AUX_WORKER_END,agent12.AUX_LOOKUP_START)
        self.assertEqual((agent12.AUX_DISPATCH_START,agent12.AUX_DISPATCH_END),
                         (0x446650,0x446694))
        self.assertLess(agent12.AUX_LOOKUP_END,agent12.AUX_DISPATCH_START)
        self.assertEqual((agent12.AUX_TYPE_START,agent12.AUX_TYPE_END),
                         (0x446694,0x4467A0))
        self.assertEqual(agent12.AUX_DISPATCH_END,agent12.AUX_TYPE_START)
        self.assertEqual((agent12.AUX_NOTIFY_START,agent12.AUX_NOTIFY_END),
                         (0x4467A0,0x4467F0))
        self.assertEqual(agent12.AUX_TYPE_END,agent12.AUX_NOTIFY_START)
        self.assertEqual((agent12.AUX_ENTRY_START,agent12.AUX_ENTRY_END),(0x4467F0,0x446804))
        self.assertEqual(agent12.AUX_NOTIFY_END,agent12.AUX_ENTRY_START)
        self.assertEqual((agent12.AUX_VALIDATOR_START,agent12.AUX_VALIDATOR_END),
                         (0x446804,0x446A68))
        self.assertEqual(agent12.AUX_ENTRY_END,agent12.AUX_VALIDATOR_START)
        self.assertEqual((agent12.AUX_RUNTIME_INIT_START,agent12.AUX_RUNTIME_INIT_END),
                         (0x446A68,0x446BDC))
        self.assertEqual(agent12.AUX_VALIDATOR_END,agent12.AUX_RUNTIME_INIT_START)
        self.assertEqual((agent12.AUX_VENEERS_START,agent12.AUX_VENEERS_END),
                         (0x446BDC,0x446C18))
        self.assertEqual(agent12.AUX_RUNTIME_INIT_END,agent12.AUX_VENEERS_START)
        self.assertEqual((agent12.AUX_REGISTRATION_START,agent12.AUX_REGISTRATION_END),
                         (0x446C18,0x446E1C))
        self.assertEqual(agent12.AUX_VENEERS_END,agent12.AUX_REGISTRATION_START)
        self.assertEqual((agent12.AUX_DIAGNOSTIC_HELPERS_START,agent12.AUX_DIAGNOSTIC_HELPERS_END),
                         (0x446E1C,0x446E90))
        self.assertEqual(agent12.AUX_REGISTRATION_END,agent12.AUX_DIAGNOSTIC_HELPERS_START)
        self.assertEqual((agent12.AUX_SESSION_PREFIX_START,agent12.AUX_SESSION_PREFIX_END),
                         (0x446E90,0x446EE8))
        self.assertEqual(agent12.AUX_DIAGNOSTIC_HELPERS_END,agent12.AUX_SESSION_PREFIX_START)
        self.assertEqual((agent12.AUX_FIND_FREE_START,agent12.AUX_FIND_FREE_END),
                         (0x447028,0x447078))
        self.assertEqual((agent12.AUX_RESOLVE_START,agent12.AUX_RESOLVE_END),
                         (0x447078,0x447100))
        self.assertEqual(agent12.AUX_FIND_FREE_END,agent12.AUX_RESOLVE_START)
        self.assertLess(agent12.AUX_SESSION_PREFIX_END,agent12.AUX_FIND_FREE_START)

    def test_atomic_packet_reorder_checks_opcode_class_and_parse_bits(self):
        words = [0x7800C000] * 16  # synthetic A2_tfrsi instruction for surrounding words
        # Architecturally typed synthetic Hexagon instruction classes, not OEM reference data.
        words[4] = 0x10004000  # J4_predicated test+branch: continuation marker
        words[5] = 0x7400C000  # A2_predicated arithmetic: end-of-packet marker
        first = struct.pack("<16I", *words)
        out = agent12.restore_atomic_packet_order(first)
        decoded = list(struct.unpack("<16I", out))
        self.assertEqual(decoded[4] & agent12.MARKER_MASK, agent12.CONTINUE_MARKER)
        self.assertEqual(decoded[5] & agent12.MARKER_MASK, agent12.END_MARKER)
        self.assertEqual(decoded[4] >> 24, 0x74)
        self.assertEqual(decoded[5] >> 24, 0x10)
        self.assertEqual(decoded[0:4], words[0:4])
        self.assertEqual(decoded[6:], words[6:])
        with self.assertRaisesRegex(ValueError, "opcode|comparison-branch"):
            agent12.restore_atomic_packet_order(first[:16] + struct.pack("<I", 0x78004000) + first[20:])
        with self.assertRaisesRegex(ValueError, "parse markers"):
            agent12.restore_atomic_packet_order(first[:16] + struct.pack("<I", 0x1000C000) + first[20:])
        with self.assertRaisesRegex(ValueError, "64 bytes"):
            agent12.restore_atomic_packet_order(first[:60])

    def test_controller_reorder_validates_hexagon_instruction_classes(self):
        # Synthetic packet categories: branch, constant extender, source move.
        words = [0x7800C000] * 24
        words[5:8] = [0x10004000, 0x0C004000, 0x7800C000]
        words[11:14] = [0x0C004000, 0x7800C000, 0x48000000]  # duplex has 00 parse field
        words[19:22] = [0x10004000, 0x0C004000, 0x7E00C000]
        src = struct.pack('<24I', *words)
        built = agent12.restore_controller_packet_order(src)
        got = list(struct.unpack('<24I', built))
        self.assertEqual([x >> 24 for x in got[5:8]], [0x0C,0x78,0x10])
        self.assertEqual([x & agent12.MARKER_MASK for x in got[5:8]],
                         [agent12.CONTINUE_MARKER,agent12.CONTINUE_MARKER,agent12.END_MARKER])
        self.assertEqual([x >> 24 for x in got[19:22]], [0x0C,0x7E,0x10])
        self.assertEqual(got[12] & agent12.MARKER_MASK, agent12.CONTINUE_MARKER)
        self.assertEqual(got[13] & agent12.MARKER_MASK, 0)  # duplex parse format
        with self.assertRaisesRegex(ValueError, 'classes changed'):
            bad=words.copy();bad[19] = 0x78004000
            agent12.restore_controller_packet_order(struct.pack('<24I',*bad))
        with self.assertRaisesRegex(ValueError, 'packet markers'):
            bad=words.copy();bad[13] |= agent12.END_MARKER
            agent12.restore_controller_packet_order(struct.pack('<24I',*bad))

    def test_aux_initializer_packet_rewrite_is_checked(self):
        words = [0x7800C000] * 21
        words[6:8] = [0x7800C002,0x28000000]
        data=struct.pack('<21I',*words)
        rewritten=agent12.restore_aux_init_packet(data)
        rewritten_words=list(struct.unpack('<21I',rewritten))
        self.assertEqual(rewritten_words[6]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(rewritten_words[7]&agent12.MARKER_MASK,0)
        self.assertEqual(rewritten_words[6]>>24,0x78)
        with self.assertRaisesRegex(ValueError,'scalar/duplex'):
            bad=words.copy();bad[7] = 0x7A000000
            agent12.restore_aux_init_packet(struct.pack('<21I',*bad))
        with self.assertRaisesRegex(ValueError,'packet marker'):
            bad=words.copy();bad[6] = 0x78004002
            agent12.restore_aux_init_packet(struct.pack('<21I',*bad))
        with self.assertRaisesRegex(ValueError,'84 bytes'):
            agent12.restore_aux_init_packet(data[:-4])

    def test_auxiliary_stage_packet_rewrite_is_checked(self):
        words=[0x7800C000]*26
        words[7]=0xB000C000
        words[8]=0x28000000  # following Hexagon duplex
        words[10]=0x3C00C000
        words[17]=0x5A00C000
        words[23]=0x5A004000
        words[24]=0x7800C000
        packed=struct.pack('<26I',*words)
        built=agent12.restore_aux_stage_packets(packed)
        got=struct.unpack('<26I',built)
        for off in [0x1c,0x28,0x44,0x60]:
            self.assertEqual(got[off//4]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(got[0x5c//4]&agent12.MARKER_MASK,agent12.END_MARKER)
        self.assertEqual(got[8]&agent12.MARKER_MASK,0)
        bad=words.copy();bad[10]=0x7800C000
        with self.assertRaisesRegex(ValueError,'instruction class'):
            agent12.restore_aux_stage_packets(struct.pack('<26I',*bad))
        bad=words.copy();bad[8]=0x2800C000
        with self.assertRaisesRegex(ValueError,'final duplex'):
            agent12.restore_aux_stage_packets(struct.pack('<26I',*bad))
        with self.assertRaisesRegex(ValueError,'104 bytes'):
            agent12.restore_aux_stage_packets(packed[:-4])

    def test_service_probe_reorder_validates_isa_and_lengths(self):
        words=[0x7800C000]*14
        words[5]=0x5A00C000
        words[6]=0x0C004000
        words[9]=0x7800C000
        words[10]=0x0C004000
        synthetic=struct.pack('<14I',*words)
        actual=agent12.restore_service_probe_packets(synthetic)
        decoded=struct.unpack('<14I',actual)
        self.assertEqual(decoded[5]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(decoded[9]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        bad=words.copy();bad[5]=0x7800C000
        with self.assertRaisesRegex(ValueError,'instruction classes'):
            agent12.restore_service_probe_packets(struct.pack('<14I',*bad))
        bad=words.copy();bad[9]=0x78004000
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_service_probe_packets(struct.pack('<14I',*bad))
        with self.assertRaisesRegex(ValueError,'56 compiled bytes'):
            agent12.restore_service_probe_packets(synthetic[:-4])

    def test_service_mmio_packet_adjustment_rejects_unrelated_opcode_classes(self):
        words=[0x7800C000]*33
        words[5]=0x5A00C000
        words[6]=0x0C004000
        words[8]=0x5B00C000
        words[9]=0x0C004000
        source=struct.pack('<33I',*words)
        decoded=struct.unpack('<33I',agent12.restore_service_mmio_packets(source))
        for i in (5,8):
            self.assertEqual(decoded[i]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
            self.assertEqual(decoded[i+1],words[i+1])
        self.assertEqual(decoded[0:5],tuple(words[0:5]))
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            bad=words.copy();bad[5]=0x7800C000
            agent12.restore_service_mmio_packets(struct.pack('<33I',*bad))
        with self.assertRaisesRegex(ValueError,'packet markers'):
            bad=words.copy();bad[8]=0x5B004000
            agent12.restore_service_mmio_packets(struct.pack('<33I',*bad))
        with self.assertRaisesRegex(ValueError,'132 bytes'):
            agent12.restore_service_mmio_packets(source[:-4])

    def test_service_flag_packet_reorder_checks_source_semantics(self):
        # Synthetic input: cmp/branch moved before its extender/move.
        words=[0x7800C000]*25
        i=(agent12.SERVICE_FLAGS_START + 0x40 - agent12.SERVICE_FLAGS_START)//4
        words[i:i+3]=[0x10004010,0x0eb1410f,0x7e80c702]
        decoded=list(struct.unpack('<25I',agent12.restore_service_flag_packets(
            struct.pack('<25I',*words))))
        self.assertEqual([x>>24 for x in decoded[i:i+3]],[0x0e,0x7e,0x10])
        self.assertEqual([x&agent12.MARKER_MASK for x in decoded[i:i+3]],
                         [agent12.CONTINUE_MARKER,agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[i+1]=0x78004000
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_service_flag_packets(struct.pack('<25I',*bad))
        bad=words.copy();bad[i]=0x1000C010
        with self.assertRaisesRegex(ValueError,'packet parse markers'):
            agent12.restore_service_flag_packets(struct.pack('<25I',*bad))
        with self.assertRaisesRegex(ValueError,'100 source bytes'):
            agent12.restore_service_flag_packets(bytes(96))

    def test_allocator_return_packet_fixup_rejects_other_opcodes(self):
        words=[0x7800C000]*41
        idx=0x98//4
        words[idx:idx+3]=[0x7070C000,0x3e0c0005,0x961ec01e]
        encoded=struct.pack('<41I',*words)
        fixed=struct.unpack('<41I',agent12.restore_service_alloc_return_packet(encoded))
        self.assertEqual(fixed[idx]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(fixed[idx] & ~agent12.MARKER_MASK,
                         words[idx] & ~agent12.MARKER_MASK)
        bad=words.copy();bad[idx]=0x7800C000
        with self.assertRaisesRegex(ValueError,'instruction classes'):
            agent12.restore_service_alloc_return_packet(struct.pack('<41I',*bad))
        bad=words.copy();bad[idx+1]=0x3e0c4005
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_service_alloc_return_packet(struct.pack('<41I',*bad))
        with self.assertRaisesRegex(ValueError,'164 source bytes'):
            agent12.restore_service_alloc_return_packet(encoded[:-4])

    def test_aux_heap_producer_branch_reorder_is_checked(self):
        words=[0x7800C000]*47
        i=0x6c//4
        words[i:i+2]=[0x10426024,0x7800C002]
        repaired=struct.unpack('<47I',agent12.restore_aux_heap_predicate_packet(
            struct.pack('<47I',*words)))
        self.assertEqual([x>>24 for x in repaired[i:i+2]],[0x78,0x10])
        self.assertEqual([x&agent12.MARKER_MASK for x in repaired[i:i+2]],
                         [agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[i]=0x78004024
        with self.assertRaisesRegex(ValueError,'opcode class'):
            agent12.restore_aux_heap_predicate_packet(struct.pack('<47I',*bad))
        bad=words.copy();bad[i+1]=0x78004002
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_heap_predicate_packet(struct.pack('<47I',*bad))
        with self.assertRaisesRegex(ValueError,'188 bytes'):
            agent12.restore_aux_heap_predicate_packet(bytes(184))

    def test_aux_worker_checked_branch_and_packet_adjustments(self):
        words=[0x7800C000]*27
        i=0x24//4
        words[i:i+2]=[0x78dffff0,0x5800c01e]
        j=0x3c//4
        words[j:j+2]=[0x5a00c050,0x73306020]
        rebuilt=struct.unpack('<27I',agent12.restore_aux_worker_packets(
            struct.pack('<27I',*words)))
        self.assertEqual(rebuilt[i]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(rebuilt[i+1]&0x3f,0x20)
        self.assertEqual(rebuilt[j]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        bad=words.copy();bad[i+1]=0x7800c01e
        with self.assertRaisesRegex(ValueError,'classes'):
            agent12.restore_aux_worker_packets(struct.pack('<27I',*bad))
        bad=words.copy();bad[i+1]=0x5800c022
        with self.assertRaisesRegex(ValueError,'PC-relative'):
            agent12.restore_aux_worker_packets(struct.pack('<27I',*bad))
        bad=words.copy();bad[j+1]=0x7330c020
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_worker_packets(struct.pack('<27I',*bad))
        with self.assertRaisesRegex(ValueError,'108 bytes'):
            agent12.restore_aux_worker_packets(bytes(104))

    def test_aux_lookup_epilogue_fixup_rejects_invalid_packet(self):
        words=[0x7800C000]*26
        i=0x60//4
        words[i:i+2]=[0x7070C000,0x3e0c1f40]
        out=struct.unpack('<26I',agent12.restore_aux_lookup_return_packet(
            struct.pack('<26I',*words)))
        self.assertEqual(out[i]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(out[i]&~agent12.MARKER_MASK,words[i]&~agent12.MARKER_MASK)
        broken=words.copy();broken[i]=0x7800C000
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_aux_lookup_return_packet(struct.pack('<26I',*broken))
        broken=words.copy();broken[i+1]|=agent12.CONTINUE_MARKER
        with self.assertRaisesRegex(ValueError,'packet marker'):
            agent12.restore_aux_lookup_return_packet(struct.pack('<26I',*broken))
        with self.assertRaisesRegex(ValueError,'104 source bytes'):
            agent12.restore_aux_lookup_return_packet(bytes(100))

    def test_aux_codec_type_early_return_packet_checks(self):
        words=[0x7800C000]*67
        for i in (0x34//4,0x80//4):
            words[i:i+2]=[0x78dfffe0,0x3e041f40]
        encoded=struct.pack('<67I',*words)
        actual=struct.unpack('<67I',agent12.restore_aux_type_return_packets(encoded))
        for i in (0x34//4,0x80//4):
            self.assertEqual(actual[i]&agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
            self.assertEqual(actual[i]&~agent12.MARKER_MASK,words[i]&~agent12.MARKER_MASK)
        changed=words.copy();changed[0x80//4]=0x5a00c000
        with self.assertRaisesRegex(ValueError,'opcode classes'):
            agent12.restore_aux_type_return_packets(struct.pack('<67I',*changed))
        changed=words.copy();changed[0x34//4]=0x78df7fe0
        with self.assertRaisesRegex(ValueError,'packet markers'):
            agent12.restore_aux_type_return_packets(struct.pack('<67I',*changed))
        with self.assertRaisesRegex(ValueError,'268 bytes'):
            agent12.restore_aux_type_return_packets(bytes(264))

    def test_controller_notification_packet_reorder_is_checked(self):
        words=[0x7800C000]*20
        i=0x10//4
        words[i:i+3]=[0x5c00581c,0xf2034210,0x7060c002]
        j=0x38//4
        words[j:j+3]=[0x91dd4010,0x01e4467e,0x4880d13c]
        packed=struct.pack('<20I',*words)
        out=struct.unpack('<20I',agent12.restore_aux_controller_notify_packets(packed))
        self.assertEqual([x>>24 for x in out[i:i+3]],[0xf2,0x70,0x5c])
        self.assertEqual([x>>24 for x in out[j:j+3]],[0x01,0x48,0x91])
        for offset in (i,j):
            self.assertEqual([x&agent12.MARKER_MASK for x in out[offset:offset+3]],
               [agent12.CONTINUE_MARKER,agent12.CONTINUE_MARKER,agent12.END_MARKER])
        bad=words.copy();bad[i]=0x7800401c
        with self.assertRaisesRegex(ValueError,'packet classes'):
            agent12.restore_aux_controller_notify_packets(struct.pack('<20I',*bad))
        bad=words.copy();bad[j+1]=0x01e4c67e
        with self.assertRaisesRegex(ValueError,'packet marker'):
            agent12.restore_aux_controller_notify_packets(struct.pack('<20I',*bad))
        with self.assertRaisesRegex(ValueError,'80 bytes'):
            agent12.restore_aux_controller_notify_packets(bytes(76))

    def test_sources_use_packets_and_real_control_register_semantics(self):
        a = agent12.ATOMIC_SOURCE.read_text()
        b = agent12.AUX_SOURCE.read_text()
        c = agent12.CONTROL_SOURCE.read_text()
        d = agent12.AUX_INIT_SOURCE.read_text()
        e = agent12.AUX_GUARD_SOURCE.read_text()
        f = agent12.AUX_STAGE_SOURCE.read_text()
        g = agent12.AUX_MMIO_SOURCE.read_text()
        h = agent12.SERVICE_SOURCE.read_text()
        j = agent12.SERVICE_MMIO_SOURCE.read_text()
        k = agent12.SERVICE_FLAGS_SOURCE.read_text()
        l = agent12.SERVICE_ALLOC_SOURCE.read_text()
        m = agent12.AUX_HEAP_SOURCE.read_text()
        n = agent12.AUX_WORKER_SOURCE.read_text()
        o = agent12.AUX_LOOKUP_SOURCE.read_text()
        p = agent12.AUX_DISPATCH_SOURCE.read_text()
        q = agent12.AUX_TYPE_SOURCE.read_text()
        t = agent12.AUX_NOTIFY_SOURCE.read_text()
        self.assertIn("memw_locked(r0)", a)
        self.assertIn("if (!p0.new) r3 = add", a)
        self.assertIn("memb(##3249533708) = r2.new", a)
        self.assertIn("framelimit", b)
        self.assertIn("HEX_WRITE_FRAMELIMIT 0", b)
        self.assertIn("r29 = add(pc,##82752)", b)
        self.assertIn("memd(r0++#8) = r5:4", b)
        self.assertIn(".Lbyte_loop", b)
        self.assertIn(".Lword_loop", b)
        self.assertIn('r5:4 = combine(##260,#0)', c)
        self.assertIn('if (!p0.new) jump:nt', c)
        self.assertIn('conditional_cleanup_callback', c)
        self.assertIn('.Lconstructor_loop',d)
        self.assertIn('r28.h',d)
        self.assertIn('memw(r29+#0) = r27',d)
        self.assertIn('jumpr:nt',e)
        self.assertIn('aux_guard_handoff',e)
        self.assertIn('memw(r16+#40)',f)
        self.assertIn('aux_run_worker',f)
        self.assertIn('HEX_WRITE_FRAMELIMIT',b)
        self.assertIn('memw(r2+#2052)',g)
        self.assertIn('service_load',h)
        self.assertIn('service_flush',h)
        self.assertIn('combine(#0,##3249533728)',h)
        self.assertIn('memw(r2+#-1012)',j)
        self.assertIn('setbit(r3,#2)',j)
        self.assertIn('jumpr r31',j)
        self.assertIn('memw(r3+#0) &= r2',k)
        self.assertIn('if (!p0.new) r2',k)
        self.assertIn('platform_alloc_probe',l)
        self.assertIn('memw(r16+#32)',l)
        self.assertIn('mpyi(r2,r4)',m)
        self.assertIn('worker_register',n)
        self.assertIn('r2 = memw(r2+#0)',o)
        self.assertIn('callr r17',p)
        self.assertIn('bitsplit(r0,#8)',q)
        self.assertIn('r0 = #-1',q)
        self.assertIn('controller_callback',t)

    def test_rejects_bad_executable_elf_magic_and_isa(self):
        with self.assertRaisesRegex(ValueError, "ELF32"):
            agent12.executable_segments(b"not an ELF")
        with self.assertRaisesRegex(ValueError, "Hexagon"):
            obj = bytearray(52)
            obj[:6] = b"\x7fELF\x01\x01"
            struct.pack_into("<H", obj, 18, 40)  # ARM EABI, not Hexagon
            agent12.executable_segments(bytes(obj))

    def test_rejects_unpinned_or_truncated_reference(self):
        with self.assertRaisesRegex(ValueError, "wrong modem firmware revision"):
            agent12.verify_reference(b"short", {agent12.ATOMIC_START: bytes(64)})

    @unittest.skipUnless(os.getenv("NABU_HEXAGON_LLVM_MC"), "set LLVM 6 Hexagon MC path")
    def test_large_controller_entry_exact_symbolic_packet_semantics(self):
        source=agent12.AUX_LARGE_CONTROLLER_PREFIX_SOURCE.read_text()
        self.assertIn('if (!p0) jump:nt aux_normal_mode',source)
        self.assertIn('call aux_logged_error',source)
        self.assertIn('jump aux_controller_continuation',source)
        self.assertNotIn('.word 0x',source)
        blob=agent12.assemble(Path(os.environ["NABU_HEXAGON_LLVM_MC"]),
                              agent12.AUX_LARGE_CONTROLLER_PREFIX_SOURCE)
        self.assertEqual(len(blob),104)
        vals=struct.unpack('<26I',blob)
        self.assertEqual(vals[0] >> 24,0x70)  # r16 = r0
        self.assertEqual(vals[11] & agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        self.assertEqual(vals[22] & agent12.MARKER_MASK,agent12.CONTINUE_MARKER)
        if os.getenv("NABU_MODEM_REFERENCE"):
            modem=Path(os.environ["NABU_MODEM_REFERENCE"]).read_bytes()
            self.assertEqual(blob,modem[agent12.AUX_LARGE_CONTROLLER_PREFIX_START:
                                        agent12.AUX_LARGE_CONTROLLER_PREFIX_END])

    @unittest.skipUnless(os.getenv("NABU_HEXAGON_LLVM_MC"), "set LLVM 6 Hexagon MC path")
    def test_optional_llvm6_exact_source_lengths(self):
        a = agent12.output_fragments(Path(os.environ["NABU_HEXAGON_LLVM_MC"]))
        self.assertEqual({k: len(v) for k, v in a.items()},
                         {0x7C000:160, 0x7C2EC:452,
                          0x446000:agent12.AUX_LARGE_CONTROLLER_PHASE40_END-agent12.AUX_START})
        self.assertEqual(sum(map(len,a.values())),agent12.AUX_LARGE_CONTROLLER_PHASE40_END - agent12.AUX_START + 612)
        self.assertEqual(a[0x446000][0x0d8:0x200],agent12.emit_auxiliary_elf_symbols())
        self.assertEqual(a[0x446000][0x254:0x260],bytes(12))
        self.assertEqual(a[0x446000][0x1448:0x1460],
                         agent12.restore_aux_notify_callback_packets(
                             agent12.assemble(Path(os.environ["NABU_HEXAGON_LLVM_MC"]),
                                              agent12.AUX_NOTIFY_SCAN_SOURCE))[:24])
        self.assertNotIn(agent12.AUX_DISPATCH_START,a)
        self.assertNotIn(agent12.AUX_SESSION_VALIDATION_START,a)
        self.assertNotIn(agent12.AUX_SESSION_REGISTRY_START,a)
        self.assertNotIn(agent12.AUX_INSERT_CONTROLLER_START,a)
        k=agent12.AUX_SESSION_REGISTRY_START-agent12.AUX_START
        self.assertEqual(a[agent12.AUX_START][k:k+4],
                         agent12.assemble(Path(os.environ["NABU_HEXAGON_LLVM_MC"]),
                                          agent12.AUX_SESSION_REGISTRY_SOURCE)[:4])
        self.assertEqual(a[0x446000][0x14b0:0x14bc],
                         agent12.restore_aux_cleanup_compare_packets(
                             agent12.assemble(Path(os.environ["NABU_HEXAGON_LLVM_MC"]),
                                              agent12.AUX_CLEANUP_SOURCE))[:12])
        for location, source, adapter in [
            (0x4475c0,agent12.AUX_FACTORY_SOURCE,lambda b:b),
            (0x447608,agent12.AUX_FIND_FORMAT_SOURCE,agent12.restore_aux_find_format_predicate_packet),
            (0x447664,agent12.AUX_FIND_CLASS_SOURCE,agent12.restore_aux_find_class_new_predicate),
            (0x4476b8,agent12.AUX_CONTROLLER_SLOT_SOURCE,lambda b:b),
            (0x4476bc,agent12.AUX_LARGE_CONTROLLER_PREFIX_SOURCE,lambda b:b),
        ]:
            actual=adapter(agent12.assemble(Path(os.environ["NABU_HEXAGON_LLVM_MC"]),source))
            offset=location-agent12.AUX_START
            self.assertEqual(a[agent12.AUX_START][offset:offset+len(actual)],actual)
        if os.getenv("NABU_MODEM_REFERENCE"):
            agent12.verify_reference(Path(os.environ["NABU_MODEM_REFERENCE"]).read_bytes(), a)


if __name__ == "__main__":
    unittest.main()


class ModemLateControllerContinuationTests(unittest.TestCase):
    def test_initial_session_continuation_is_source_assembled_without_oem_words(self):
        self.assertEqual(agent12.AUX_LARGE_CONTROLLER_MID_START,0x447724)
        self.assertEqual(agent12.AUX_LARGE_CONTROLLER_MID_END,0x4477FC)
        self.assertEqual(agent12.AUX_LARGE_CONTROLLER_MID_START,
                         agent12.AUX_LARGE_CONTROLLER_PREFIX_END)
        asm=agent12.AUX_LARGE_CONTROLLER_MID_SOURCE.read_text()
        self.assertIn('if (!cmp.eq(r17.new,#0))',asm)
        self.assertIn('r21 = r0',asm)
        self.assertIn('call diagnostic_console',asm)
        self.assertIn('jump early_controller_exit',asm)
        self.assertNotIn('.word',asm)
        self.assertNotIn('.incbin',asm)


class ModemLatePhase2Tests(unittest.TestCase):
    def test_phase2_predicated_packet_reconstruction_and_negative_guards(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE2_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE2_END),
                         (0x4477FC,0x4478A4))
        asm=agent12.AUX_LARGE_CONTROLLER_PHASE2_SOURCE.read_text()
        self.assertIn('call controller_log_cb',asm)
        self.assertIn('r2 = ##507534712',asm)
        self.assertIn('if (!p0.new) jump:t',asm)
        self.assertNotIn('.word',asm)
        self.assertNotIn('.incbin',asm)
        words=[0x7800c000]*42
        i=0x70//4
        words[i:i+3]=[0x5c205812,0x01e4440a,0x7e80e0c3]
        src=struct.pack('<42I',*words)
        rebuilt=struct.unpack('<42I',agent12.restore_aux_large_controller_phase2_packet(src))
        self.assertEqual(rebuilt[i:i+3],(0x01e4440a,0x7e8060c3,0x5c20d812))
        for index,modified,pattern in [(i,0x78004012,'classes'),
                                       (i+1,0x01e4c40a,'markers'),
                                       (i+2,0x7e80e0c4,'target or register')]:
            bad=words.copy();bad[index]=modified
            with self.assertRaisesRegex(ValueError,pattern):
                agent12.restore_aux_large_controller_phase2_packet(struct.pack('<42I',*bad))
        with self.assertRaisesRegex(ValueError,'168 bytes'):
            agent12.restore_aux_large_controller_phase2_packet(src[:-4])


class ModemLateSessionLoopTests(unittest.TestCase):
    def test_checked_loop_compare_packet_layout(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE3_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE3_END),
                         (0x4478A4,0x44795C))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE3_SOURCE.read_text()
        self.assertIn('loop0(.Lscan_iter,r3)',src)
        self.assertIn(':endloop0',src)
        self.assertIn('callr r6',src)
        self.assertNotIn('.word',src)
        self.assertNotIn('.incbin',src)
        words=[0x7800c000]*46
        i=0x08//4;j=0x28//4
        words[i:i+2]=[0x100340bc,0x7e006016]
        words[j:j+3]=[0x5c205844,0x00204000,0x7503c000]
        rebuilt=struct.unpack('<46I',agent12.restore_aux_large_controller_phase3_packets(
            struct.pack('<46I',*words)))
        self.assertEqual(rebuilt[i:i+2],(0x7e006016,0x100340bc))
        self.assertEqual(rebuilt[j:j+3],(0x00204000,0x75034000,0x5c20d844))
        for offset,value,msg in [(i,0x780040bc,'opcode classes'),
                                 (i,0x1003c0bc,'packet markers'),
                                 (i+1,0x7e006017,'operands'),
                                 (j,0x78005844,'classes'),
                                 (j+2,0x75034000,'packet markers'),
                                 (j,0x5c205805,'target/compare')]:
            bad=words.copy();bad[offset]=value
            with self.assertRaisesRegex(ValueError,msg):
                agent12.restore_aux_large_controller_phase3_packets(
                    struct.pack('<46I',*bad))
        with self.assertRaisesRegex(ValueError,'184 bytes'):
            agent12.restore_aux_large_controller_phase3_packets(bytes(180))


class ModemLateCallbackAndDuplexTests(unittest.TestCase):
    def test_direct_76_bytes_and_checked_followup_40_bytes(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE4_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE4_END),
                         (0x44795C,0x4479A8))
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE5_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE5_END),
                         (0x4479A8,0x4479D0))
        a=agent12.AUX_LARGE_CONTROLLER_PHASE4_SOURCE.read_text()
        b=agent12.AUX_LARGE_CONTROLLER_PHASE5_SOURCE.read_text()
        self.assertIn('r22 = #0',a)
        self.assertIn('memw(r29+#84)',a)
        self.assertIn('r2 += add(r23,#-1)',b)
        self.assertNotIn('.word',a+b)
        self.assertNotIn('.incbin',a+b)
        words=[0x7800c000]*10
        words[0]=0x5a00fe04
        words[0x18//4]=0xe217dfe2
        words[0x20//4]=0x5c205814
        before=struct.pack('<10I',*words)
        rebuilt=struct.unpack('<10I',agent12.restore_aux_large_controller_phase5_packets(before))
        self.assertEqual((rebuilt[0],rebuilt[6],rebuilt[8]),
                         (0x5a007e04,0xe2175fe2,0x5c205816))
        for offset,new,err in [(0,0x7800fe04,'classes'),
                               (0,0x5a007e04,'markers'),
                               (0x18//4,0xe217dfe3,'relocation'),
                               (0x20//4,0x5c205818,'relocation')]:
            changed=words.copy();changed[offset]=new
            with self.assertRaisesRegex(ValueError,err):
                agent12.restore_aux_large_controller_phase5_packets(struct.pack('<10I',*changed))
        with self.assertRaisesRegex(ValueError,'40 bytes'):
            agent12.restore_aux_large_controller_phase5_packets(before[:-4])


class ModemLateControllerResponseTests(unittest.TestCase):
    def test_source_assembled_callback_paths_and_checked_diagnostic_branch(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE6_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE8_END),
                         (0x4479D0,0x447AE4))
        for asm in (agent12.AUX_LARGE_CONTROLLER_PHASE6_SOURCE,
                    agent12.AUX_LARGE_CONTROLLER_PHASE7_SOURCE,
                    agent12.AUX_LARGE_CONTROLLER_PHASE8_SOURCE):
            text=asm.read_text()
            self.assertNotIn('.word',text)
            self.assertNotIn('.incbin',text)
        self.assertIn('callr r4',agent12.AUX_LARGE_CONTROLLER_PHASE6_SOURCE.read_text())
        self.assertIn('r17 = #0',agent12.AUX_LARGE_CONTROLLER_PHASE7_SOURCE.read_text())
        words=[0x7800c000]*14
        i=0x08//4
        words[i:i+3]=[0x5c205812,0x01e4440d,0x7e80e083]
        packed=struct.pack('<14I',*words)
        result=struct.unpack('<14I',agent12.restore_aux_large_controller_phase8_packet(packed))
        self.assertEqual(result[i:i+3],(0x01e4440d,0x7e806083,0x5c20d812))
        for off,other,msg in [(i,0x78005812,'classes'),
                              (i+1,0x01e4c40d,'markers'),
                              (i+2,0x7e80e084,'target or move register')]:
            bad=words.copy();bad[off]=other
            with self.assertRaisesRegex(ValueError,msg):
                agent12.restore_aux_large_controller_phase8_packet(struct.pack('<14I',*bad))
        with self.assertRaisesRegex(ValueError,'56 bytes'):
            agent12.restore_aux_large_controller_phase8_packet(packed[:-4])


class ModemLateControllerRecordScanTests(unittest.TestCase):
    def test_codec_record_scan_predicate_and_store_packet_are_checked(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE9_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE9_END),
                         (0x447AE4,0x447B5C))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE9_SOURCE.read_text()
        self.assertIn('loop0(',src)
        self.assertIn(':endloop0',src)
        self.assertNotIn('.word',src)
        self.assertNotIn('.incbin',src)
        words=[0x7800c000]*30
        i=0x0c//4;j=0x48//4
        words[i:i+3]=[0x10426008,0x74956286,0x7e80e019]
        words[j:j+2]=[0x97837f97,0xa19dda14]
        result=struct.unpack('<30I',agent12.restore_aux_large_controller_phase9_packets(
            struct.pack('<30I',*words)))
        self.assertEqual(result[i:i+3],(0x74956286,0x7e806019,0x1042e008))
        self.assertEqual(result[j:j+2],(0xa19d5a14,0x9783ff97))
        for off,value,msg in [(i,0x78006008,'opcode classes'),
                              (i+1,0x7495e286,'packet markers'),
                              (i+2,0x7e80e018,'operands/branch'),
                              (j,0x78007f97,'ISA classes'),
                              (j+1,0xa19d5a14,'packet markers')]:
            invalid=words.copy();invalid[off]=value
            with self.assertRaisesRegex(ValueError,msg):
                agent12.restore_aux_large_controller_phase9_packets(
                    struct.pack('<30I',*invalid))
        with self.assertRaisesRegex(ValueError,'120 generated bytes'):
            agent12.restore_aux_large_controller_phase9_packets(bytes(116))


class ModemLateSessionConditionTests(unittest.TestCase):
    def test_controller_opcode_branches_and_packet_boundaries(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE10_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE10_END),
                         (0x447B5C,0x447BC4))
        src=agent12.AUX_LARGE_CONTROLLER_PHASE10_SOURCE.read_text()
        self.assertIn('memw(r29+#60) = r4.new',src)
        self.assertNotIn('.word',src)
        self.assertNotIn('.incbin',src)
        words=[0x7800c000]*26
        words[0x1c//4]=0xf31b5703
        words[0x24//4]=0x20c2e31e
        words[0x30//4:0x3c//4]=[0x5c205876,0x01e4440e,0x7e80e0a3]
        words[0x4c//4:0x58//4]=[0x58004072,0xf3155716,0x7072c01b]
        packed=struct.pack('<26I',*words)
        result=struct.unpack('<26I',agent12.restore_aux_large_controller_phase10_packets(packed))
        self.assertEqual(result[0x1c//4],0xf31bd703)
        self.assertEqual(result[0x24//4],0x20c2e31c)
        self.assertEqual(result[0x30//4:0x3c//4],(0x01e4440e,0x7e8060a3,0x5c20d876))
        self.assertEqual(result[0x4c//4:0x58//4],(0xf3155716,0x7072401b,0x5800c072))
        for off,wrong,err in [(0x1c//4,0x78005703,'instruction/marker'),
                              (0x24//4,0x20c2e31f,'displacement'),
                              (0x30//4,0x78005876,'ISA classes'),
                              (0x38//4,0x7e80e0a4,'target/register'),
                              (0x4c//4,0x78004072,'ISA classes'),
                              (0x54//4,0x7072401b,'markers')]:
            invalid=words.copy();invalid[off]=wrong
            with self.assertRaisesRegex(ValueError,err):
                agent12.restore_aux_large_controller_phase10_packets(
                    struct.pack('<26I',*invalid))
        with self.assertRaisesRegex(ValueError,'104 source bytes'):
            agent12.restore_aux_large_controller_phase10_packets(bytes(100))


class ModemLateResourceCallbackTests(unittest.TestCase):
    def test_callback_resource_path_has_direct_hexagon_assembly(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE11_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE11_END),
                         (0x447BC4,0x447C74))
        asm=agent12.AUX_LARGE_CONTROLLER_PHASE11_SOURCE.read_text()
        self.assertIn('r2 += add(r4,#-1)',asm)
        self.assertIn('r3 = ##507577286',asm)
        self.assertIn('callr r6',asm)
        self.assertIn('jump cleanup',asm)
        self.assertNotIn('.word',asm)
        self.assertNotIn('.incbin',asm)


class ModemLateControllerResultPhase12Tests(unittest.TestCase):
    def test_result_packets_from_symbolic_controller_calls(self):
        self.assertEqual((agent12.AUX_LARGE_CONTROLLER_PHASE12_START,
                          agent12.AUX_LARGE_CONTROLLER_PHASE12_END),
                         (0x447C74,0x447D20))
        source=agent12.AUX_LARGE_CONTROLLER_PHASE12_SOURCE.read_text()
        for snippet in ('{call fallback_log}', '{memw(r17+#8) = #1}',
                        'if (!p0.new) r3 = ##507577346',
                        'p0 = cmp.gtu(r2,r25)', 'scan_loop'):
            self.assertIn(snippet,source)
        self.assertNotIn('.word',source)
        self.assertNotIn('.incbin',source)
        words=[0x7800c000]*43
        words[0x70//4]=0x3c51c101
        words[0x74//4]=0x919dc2a2
        words[0x7c//4:0x88//4]=[0x5c205826,0x01e44410,0x7e80e043]
        words[0xa4//4:0xac//4]=[0x5cdf58f6,0xf262d900]
        packed=struct.pack('<43I',*words)
        after=struct.unpack('<43I',agent12.restore_aux_large_controller_phase12_packets(packed))
        self.assertEqual(after[0x70//4],0x3c514101)
        self.assertEqual(after[0x74//4],0x919dc2a2)
        self.assertEqual(after[0x7c//4:0x88//4],
                         (0x01e44410,0x7e806043,0x5c20d826))
        self.assertEqual(after[0xa4//4:0xac//4],(0xf2625900,0x5cdfd8f6))
        for idx,bad,why in [
            (0x70//4,0x3c51c102,'store opcode/register/marker'),
            (0x70//4,0x3c514101,'store opcode/register/marker'),
            (0x74//4,0x919dc2a3,'frame load'),
            (0x7c//4,0x78005826,'ISA classes'),
            (0x7c//4,0x5c20d826,'marker'),
            (0x84//4,0x7e80e042,'operand or branch'),
            (0xa4//4,0x780058f6,'ISA classes'),
            (0xa8//4,0xf2625900,'packet markers'),
            (0xa4//4,0x5cdf58f5,'predicates/offset'),
        ]:
            other=words.copy();other[idx]=bad
            with self.assertRaisesRegex(ValueError,why):
                agent12.restore_aux_large_controller_phase12_packets(
                    struct.pack('<43I',*other))
        with self.assertRaisesRegex(ValueError,'172 source bytes'):
            agent12.restore_aux_large_controller_phase12_packets(packed[:-4])
