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
        self.assertEqual(m.validate_manifest(),0x5df0)
        self.assertEqual(m.FILE_BASE+m.START,0x4568c)
        self.assertEqual(m.FILE_BASE+m.validate_manifest(),0x45df0)
        self.assertEqual(sum(length for _,_,_,length in m.FRAGMENTS),1892)

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
        names=[x[0] for x in m.FRAGMENTS]
        assert names[-5:-3]==['controller-snapshot-wrapper','controller-initialize-stage1']
        assert m.FRAGMENTS[-5][2:]==(0x5c0c,0x14)
        assert m.FRAGMENTS[-4][2:]==(0x5c20,0xd8)
        source=m.FRAGMENTS[-4][1].read_text()
        assert ':mem_noshuf' in source
        assert 'loop0(' not in source

    def test_compound_extended_branch_preserves_only_parse_bits(self):
        m=module()
        assert [frag[0] for frag in m.FRAGMENTS[-3:]]==[
            'compound-branch','compound-memory','controller-initialize-stage2']
        class ParentStub:
            def assemble(self,source,llvm6,cpu,expected_length):
                assert expected_length==8
                # A synthetic LLVM6 packet: zero parse bits in first word,
                # last word has the Hexagon end marker. No OEM instruction bits.
                return (0x00004010).to_bytes(4,'little')+(0x0aaaC042).to_bytes(4,'little')
        b=m.assemble_compound_branch(ParentStub(),'llvm6',m.FRAGMENTS[-3][1])
        assert len(b)==8
        assert int.from_bytes(b[4:8],'little')==0x0aaa4042
        assert b[:4]==(0x4010).to_bytes(4,'little')

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
