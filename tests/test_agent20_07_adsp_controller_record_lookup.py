# SPDX-License-Identifier: GPL-3.0-only
"""Controller record allocation uses exact V66 system-register semantics."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
sp=importlib.util.spec_from_file_location('adsp_record_lookup',T/'adsp-agent20-rwe-next-controller-record-lookup.py')
assert sp and sp.loader
m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
class AdspRecordLookup(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_address_ranges_and_noninstruction_gap(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x12e40,0x12f50,272))
        self.assertEqual(m.SGP0_FILE_OFFSET-m.START,0x10)
        asm=m.SOURCE.read_text()
        for token in ('record_loop:', 'memd(r17+#16)=r19:18','if(p0.new) jump:t exit',
                      'r7=#0; r5=#0; r20=add(r2,#24); r0=#0','dealloc_return'):
            self.assertIn(token,asm)
        self.assertNotIn('.word',asm)
        self.assertNotIn('.incbin',asm)
    def test_source_emission_equals_original(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and ADSP reference required')
        original=Path(self.ref).read_bytes();output=m.render(self.mc)
        m.verify(original,output)
        self.assertEqual(output[m.SGP0_FILE_OFFSET-m.START:m.SGP0_FILE_OFFSET-m.START+4],struct.pack('<I',0x6e80c002))
        self.assertEqual(original[0x12e35:0x12e40],bytes(11))
    def test_detect_wrong_revision_or_generated_code_corruption(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and ADSP reference required')
        original=Path(self.ref).read_bytes();code=m.render(self.mc)
        bad=bytearray(original);bad[-100]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(bad,code)
        altered=bytearray(code);altered[16]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(original,altered)
if __name__=='__main__':unittest.main()
