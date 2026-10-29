# SPDX-License-Identifier: GPL-3.0-only
"""Controller record reset: source byte identity, privileged ops, and errors."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def imp(name):
    sp=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    return m
m=imp('adsp-agent20-rwe-next-controller-record-reset.py')
previous=imp('adsp-agent20-rwe-next-controller-record-query-tail.py')
first=imp('adsp-agent20-rwe-next-controller-record-query.py')
class ControllerRecordReset(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_and_addresses(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x13250,0x13390,320))
        self.assertEqual(previous.END,m.START)
        src=m.SOURCE.read_text()
        for phrase in ('controller_lookup','linked_loop:','clear_record:',
                       'notify_cleanup:', 'dealloc_return', 'memh(r2+#0)=setbit(#5)'):
            self.assertIn(phrase,src)
        self.assertNotIn('.incbin',src)
        self.assertNotIn('.word',src)
        self.assertEqual((m.SGP0,m.K0_LOCK),(0x1325c,0x132f4))
    def test_exact_code_and_704_byte_combination(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and ADSP reference required')
        ref=Path(self.ref).read_bytes();output=m.render(self.mc)
        m.verify(ref,output)
        self.assertEqual(output[12:16],struct.pack('<I',0x6e80c002))
        self.assertEqual(output[164:168],struct.pack('<I',0x6c20c060))
        combined=first.render(self.mc)+previous.render(self.mc)+output
        self.assertEqual(len(combined),0x13390-first.START)
        self.assertEqual(combined,ref[first.START:m.END])
    def test_wrong_revision_and_corruption_fail_closed(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and ADSP reference required')
        ref=Path(self.ref).read_bytes();output=m.render(self.mc)
        wrong=bytearray(ref);wrong[-1]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,output)
        changed=bytearray(output);changed[64]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,changed)
if __name__=='__main__':unittest.main()
