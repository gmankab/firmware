# SPDX-License-Identifier: GPL-3.0-only
"""Instruction-level validation of source-built ADSP controller table init."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
s=importlib.util.spec_from_file_location('controller_table_init',T/'rwe-next-controller-table-init.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class ControllerTableInit(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC')
        cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_exact_layout_and_source_operations(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11ae0,0x11ba0,192))
        self.assertEqual(m.FIELD_AT,0x11b40)
        src=m.SOURCE.read_text()
        for snippet in ('bitsplit(r5,#5)','loop0(clear_words,r7)',
                        'loop0(copy_register_table,r2)','memw(r4++#4)=r2',
                        'jump platform_finalize'):
            self.assertIn(snippet,src)
        self.assertNotIn('.incbin',src)
    def test_typed_packet_parse_field(self):
        if not self.mc:self.skipTest('LLVM6 is unavailable')
        out=m.render(self.mc);self.assertEqual(len(out),m.SIZE)
        word=struct.unpack_from('<I',out,m.FIELD_AT-m.START)[0]
        self.assertEqual(word&m.PARSE_MASK,m.PACKET_CONTINUE)
    def test_exact_reference_and_rejected_revisions(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference not configured')
        ref=Path(self.ref).read_bytes();out=m.render(self.mc);m.verify(ref,out)
        bad=bytearray(out);bad[0]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
        wrong=bytearray(ref);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,out)
    def test_empty_code_and_wrong_layout_fail_closed(self):
        if not self.ref:self.skipTest('reference not configured')
        ref=Path(self.ref).read_bytes()
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,b'')
        corrupted=bytearray(ref);corrupted[0x119c0]=0
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(corrupted,ref[m.START:m.END])
if __name__=='__main__':unittest.main()
