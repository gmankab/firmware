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
        self.assertEqual(m.validate_manifest(),0x5b58)
        self.assertEqual(m.FILE_BASE+m.START,0x4568c)
        self.assertEqual(m.FILE_BASE+m.validate_manifest(),0x45b58)
        self.assertEqual(sum(length for _,_,_,length in m.FRAGMENTS),1228)

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
        self.assertIn('}:endloop0',loop)


if __name__=='__main__':unittest.main()
