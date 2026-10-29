# SPDX-License-Identifier: GPL-3.0-only
"""Source-only follow-up when checkpoint 25 was not shipped with the chat."""
import importlib.util
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]
SCRIPT=ROOT/'tools/build-wlanmdsp-agent13-forward.py'


def module():
    spec=importlib.util.spec_from_file_location('agent13forward',SCRIPT)
    m=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


class SourceOnlyForwardTests(unittest.TestCase):
    def test_manifest_exact_boundaries(self):
        m=module()
        self.assertEqual(m.START,0x568c)
        self.assertEqual(m.validate_manifest(),0x739c)
        self.assertEqual(m.FILE_BASE+m.START,0x4568c)
        self.assertEqual(m.FILE_BASE+m.validate_manifest(),0x4739c)
        self.assertEqual(sum(length for _,_,_,length in m.FRAGMENTS),7440)

    def test_validator_forbids_opaque_data_and_layout_holes(self):
        m=module()
        name,path,start,length=m.FRAGMENTS[0]
        with self.assertRaisesRegex(ValueError,'non-adjacent'):
            m.validate_manifest([(name,path,start+4,length)])
        with self.assertRaisesRegex(ValueError,'unaligned'):
            m.validate_manifest([(name,path,start,length+2)])
        with self.assertRaisesRegex(ValueError,'missing independent'):
            m.validate_manifest([(name,ROOT/'missing.S',start,length)])

    def test_source_no_reference_binary_bytes(self):
        m=module()
        for _,source,_,_ in m.FRAGMENTS:
            asm=source.read_text()
            for marker in ('.incbin','.byte','.word','.quad','.fill'):
                self.assertNotIn(marker,asm)
            self.assertIn('{',asm)
            self.assertIn('}',asm)
        controller=m.FRAGMENTS[0][1].read_text()
        self.assertIn('allocframe',controller)
        self.assertIn('call',controller)
        self.assertIn(':mem_noshuf',controller)
        self.assertIn('jump ##',controller)
        loop=m.FRAGMENTS[2][1].read_text()
        self.assertIn('loop0(L_00460,r5)',loop)
        bitmap=m.FRAGMENTS[6][1].read_text()
        self.assertIn('loop0(L_00514,r7)',bitmap)
        self.assertIn('}:endloop0',bitmap)
        self.assertIn('}:endloop0',loop)

    def test_next_controller_split_is_adjacent(self):
        m=module()
        fragments={name:(source,off,size) for name,source,off,size in m.FRAGMENTS}
        assert fragments['controller-snapshot-wrapper'][1:]==(0x5c0c,0x14)
        assert fragments['controller-initialize-stage1'][1:]==(0x5c20,0xd8)
        source=fragments['controller-initialize-stage1'][0].read_text()
        assert ':mem_noshuf' in source
        assert 'loop0(' not in source

    def test_compound_extended_branch_preserves_only_parse_bits(self):
        m=module()
        fragments={name:(source,off,size) for name,source,off,size in m.FRAGMENTS}
        for name in ('compound-branch','compound-memory','controller-initialize-stage2'):
            assert name in fragments
        class ParentStub:
            def assemble(self,source,llvm6,cpu,expected_length):
                assert expected_length==8
                return (0x00004010).to_bytes(4,'little')+(0x0aaaC042).to_bytes(4,'little')
        b=m.assemble_compound_branch(ParentStub(),'llvm6',fragments['compound-branch'][0])
        assert len(b)==8
        assert int.from_bytes(b[4:8],'little')==0x0aaa4042
        assert b[:4]==(0x4010).to_bytes(4,'little')

    def test_status_compact_and_callback_packet_grouping(self):
        m=module()
        frags={name:(path,off,size) for name,path,off,size in m.FRAGMENTS}
        self.assertEqual(frags['record-status-compact'][1:],(0x6fb4,0x5c))
        self.assertEqual(frags['record-callback-thunk'][1:],(0x7010,0x18))
        self.assertEqual(frags['record-state-clear'][1:],(0x7028,0x34))
        status=frags['record-status-compact'][0].read_text()
        thunk=frags['record-callback-thunk'][0].read_text()
        reset=frags['record-state-clear'][0].read_text()
        self.assertIn('memw(r29+#4) = r3.new',status)
        self.assertIn('call -0x5c\n    r16 = r1',thunk)
        self.assertIn('call -0xbdc\n    memw(r16+##4588) = r2',reset)
        self.assertIn('}:mem_noshuf',reset)

    def test_record_validator_preserves_packet_and_branch_source_order(self):
        m=module()
        byname={name:(p,off,size) for name,p,off,size in m.FRAGMENTS}
        p,off,size=byname['controller-record-validate']
        self.assertEqual((off,size),(0x705c,0x12c))
        source=p.read_text()
        self.assertIn('if (cmp.gtu(r19.new,r2)) jump:nt L_198',source)
        self.assertIn('r18 = #20\n    r0 = ##488\n    r2 = r4',source)
        self.assertIn('r18 = #6\n    r0 = ##496\n    r2 = r3',source)
        self.assertIn('call -0x4e40',source)
        self.assertIn('allocframe(#128)',source)
        for opaque in ('.word','.byte','.incbin'):
            self.assertNotIn(opaque,source)

    def test_session_process_prefix_preserves_predicated_memory_order(self):
        m=module()
        p,off,size={n:(path,off,size) for n,path,off,size in m.FRAGMENTS}['session-process-prefix']
        self.assertEqual((off,size),(0x7188,0x8c))
        text=p.read_text()
        self.assertIn('}:mem_noshuf',text)
        self.assertIn('if (p0.new) memb(r16+r18<<#0) = r17',text)
        self.assertIn('if (p0.new) r2 = memw(r22+#12)',text)
        self.assertLess(text.index('if (p0.new) memb'),text.index('if (p0.new) r2 = memw'))
        self.assertIn('loop',m.FRAGMENTS[2][1].read_text())
        self.assertNotIn('.word',text)
        self.assertNotIn('.incbin',text)

    def test_session_record_loop_has_bounded_copy_and_packet_separation(self):
        m=module()
        path,off,size={n:(p,a,b) for n,p,a,b in m.FRAGMENTS}['session-record-loop']
        self.assertEqual((off,size),(0x7214,0xe4))
        text=path.read_text()
        self.assertIn('r20 = add(r25,mpyi(#28,r19))\n}\n{\n    call -0x1bc',text)
        self.assertIn('if (p0.new) jump:nt L_2f4',text)
        self.assertIn('memw(r29+#64) = r2.new',text)
        self.assertIn('memb(r29+#26) = r2.new',text)
        for opaque in ('.incbin','.word','.byte'):
            self.assertNotIn(opaque,text)

    def test_session_record_update_preserves_packed_controller_memops(self):
        m=module()
        path,off,size={n:(p,o,s) for n,p,o,s in m.FRAGMENTS}['session-record-update']
        self.assertEqual((off,size),(0x72f8,0xa4))
        text=path.read_text()
        self.assertIn('memuh(r20+#38)',text)
        self.assertIn('r0 = add(r4,mpyi(#48,r2))\n    r3 = lsl(#1,r3)',text)
        self.assertIn('call -0x5250',text)
        self.assertIn('}:mem_noshuf',text) if ':mem_noshuf' in text else self.assertIn('p0 = !cmp.gtu',text)
        self.assertNotIn('.word',text)

    def test_controller_stage3_preserves_packet_order(self):
        m=module()
        fragments={name:(source,off,size) for name,source,off,size in m.FRAGMENTS}
        assert fragments['controller-initialize-stage3-flags'][1:]==(0x5df0,0xb4)
        assert fragments['controller-initialize-stage3-state'][1:]==(0x5ea4,0x5c)
        assert fragments['controller-initialize-stage3-mask'][1:]==(0x5f00,0x100)
        assert fragments['controller-initialize-stage3-validation'][1:]==(0x6000,0x100)
        assert fragments['controller-initialize-stage3-loop-entry'][1:]==(0x6100,0x48)
        assert ':mem_noshuf' in fragments['controller-initialize-stage3-flags'][0].read_text()
        assert 'jump:nt 0x18' in fragments['controller-initialize-stage3-flags'][0].read_text()
        assert 'if (cmp.gtu(r2.new,r27))' not in fragments['controller-initialize-stage3-loop-entry'][0].read_text()

    def test_controller_loop_register_parser_adjustment_only(self):
        m=module()
        fragments={name:(path,off,size) for name,path,off,size in m.FRAGMENTS}
        assert fragments['controller-loop-start'][1:]==(0x6148,0x18)
        assert fragments['controller-loop-registers'][1:]==(0x6160,0x08)
        assert fragments['controller-loop-duplex'][1:]==(0x6168,0x04)
        assert fragments['controller-loop-tail'][1:]==(0x616c,0x10)
        assert fragments['controller-loop-publish'][1:]==(0x617c,0x44)
        assert fragments['controller-loop-table'][1:]==(0x61c0,0x54)
        class ParentStub:
            def assemble(self,src,mc,cpu,expected_length):
                assert expected_length==8
                return (0x0a004010).to_bytes(4,'little')+(0x0b00c035).to_bytes(4,'little')
        source=fragments['controller-loop-registers'][0]
        encoded=m.assemble_controller_loop_registers(ParentStub(),'llvm6',source)
        assert len(encoded)==8
        assert encoded[:4]==(0x0a004010).to_bytes(4,'little')
        assert int.from_bytes(encoded[4:8],'little')==0x0b004035
        assert ':endloop0' in fragments['controller-loop-tail'][0].read_text()
        assert 'loop0(0x18,#6)' in fragments['controller-loop-start'][0].read_text()

    def test_controller_register_update_fragments_have_exact_boundaries(self):
        m=module()
        fragments={name:(source,offset,size) for name,source,offset,size in m.FRAGMENTS}
        assert fragments['controller-select-planes'][1:]==(0x6214,0xec)
        assert fragments['controller-report-status'][1:]==(0x6300,0x80)
        assert fragments['controller-report-callbacks'][1:]==(0x6380,0x40)
        assert fragments['controller-register-updates'][1:]==(0x63c0,0x44)
        asm=fragments['controller-report-status'][0].read_text()
        assert '}:mem_noshuf' in asm
        assert 'memw(r23+#896)' in asm
        assert 'r18 = memw(r29+#12)' in asm
        assert '##-1342151444' in fragments['controller-register-updates'][0].read_text()

    def test_stage4_loop_and_extended_jump_are_source_defined(self):
        m=module()
        f={name:(path,off,size) for name,path,off,size in m.FRAGMENTS}
        assert f['controller-init-finalize'][1:]==(0x6404,0xfc)
        assert f['session-buffer-loop-start'][1:]==(0x6500,0x100)
        assert f['session-buffer-loop-body'][1:]==(0x6600,0x100)
        assert 'jump ##-0x534c' in f['controller-init-finalize'][0].read_text()
        assert 'jump ##0x253c' in f['controller-init-finalize'][0].read_text()
        asm=f['session-buffer-loop-start'][0].read_text()
        assert 'r23 = #0' in asm and 'r19 = #0' in asm
        assert 'call' in asm
        assert 'if (cmp.eq(r3.new,#0))' in asm

    def test_session_plane_packet_order_and_layout(self):
        m=module()
        fragments={name:(path,start,length) for name,path,start,length in m.FRAGMENTS}
        assert fragments['session-buffer-iteration'][1:]==(0x6700,0x100)
        assert fragments['session-buffer-selection'][1:]==(0x6800,0x100)
        assert fragments['session-buffer-plane-copies'][1:]==(0x6900,0x108)
        assert fragments['session-buffer-selection'][0].read_text().count('}:mem_noshuf')>=3
        assert fragments['session-buffer-plane-copies'][0].read_text().count('}:mem_noshuf')>=2

    def test_session_metadata_completion_uses_extended_jump(self):
        m=module()
        mapping={n:(src,off,size) for n,src,off,size in m.FRAGMENTS}
        asm,off,size=mapping['session-metadata-final']
        self.assertEqual((off,size),(0x6e00,0x11c))
        text=asm.read_text()
        self.assertIn('jump ##0x29e92c',text)
        self.assertIn('}:mem_noshuf',text)
        self.assertIn('L_00508:',text)
        self.assertNotIn('.incbin',text)

    def test_record_flag_probe_preserves_compound_call_packet(self):
        m=module()
        source,offset,size={name:(src,off,length) for name,src,off,length in m.FRAGMENTS}['record-flag-probe']
        self.assertEqual((offset,size),(0x6f1c,0x98))
        code=source.read_text()
        self.assertIn('L_6f9c:',code)
        self.assertIn('call -0x1454',code)
        self.assertIn('r0 = ##-1339397237',code)
        self.assertIn('r1 = #1921',code)
        self.assertNotIn('.word',code)
        self.assertNotIn('.incbin',code)
        # The extended call packet must not be split into two separate blocks.
        self.assertIn('call -0x1454\n    r0 = ##-1339397237\n    r1 = #1921',code)

    def test_partial_session_metadata_preserves_duplex_selection(self):
        m=module()
        fragments={name:(path,off,size) for name,path,off,size in m.FRAGMENTS}
        source,offset,length=fragments['session-metadata-prefix']
        self.assertEqual((offset,length),(0x6b9c,0x264))
        text=source.read_text()
        for value in ('allocframe(#384)', 'r22 = add(r29,#140)',
                      'memb(r29+#200) = r11', 'callr r2'):
            self.assertIn(value,text)
        self.assertNotIn('.word',text)
        self.assertNotIn('.incbin',text)

    def test_session_hardware_loop_tail_packet_order(self):
        m=module()
        parts={name:(source,off,size) for name,source,off,size in m.FRAGMENTS}
        source,offset,size=parts['session-buffer-loop-final']
        self.assertEqual((offset,size),(0x6a08,0x194))
        program=source.read_text()
        self.assertIn('}:endloop0',program)
        self.assertEqual(program.count('}:mem_noshuf'),6)
        self.assertIn('L_00190:',program)
        self.assertIn('dealloc_return',program)
        self.assertNotIn('.word',program)

    def test_structured_predicate_duplex_sources(self):
        m=module()
        names=[fragment[0] for fragment in m.FRAGMENTS]
        self.assertIn('bitmap-predicate-pair',names)
        self.assertIn('bitmap-duplex',names)
        predicate=m.FRAGMENTS[4][1].read_text()
        duplex=m.FRAGMENTS[5][1].read_text()
        self.assertIn('p0 = cmp.eq(r2,#0)',predicate)
        self.assertIn('p1 = cmp.eq(r2,#1)',predicate)
        self.assertIn('r2 = r6; r0 = #-1',duplex)
        self.assertEqual(m.FRAGMENTS[4][3]+m.FRAGMENTS[5][3],12)


if __name__=='__main__':unittest.main()
