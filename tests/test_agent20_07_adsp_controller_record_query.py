# SPDX-License-Identifier: GPL-3.0-only
"""V66 indexed controller record query and conditional bitmap paths."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
def imp(name):
    sp=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=imp('adsp-agent20-rwe-next-controller-record-query.py')
prior=imp('adsp-agent20-rwe-next-controller-record-recovery.py')
class RecordQuery(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_function_seams_and_error_edges(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x130d0,0x131e0,272))
        self.assertEqual(prior.END+12,m.START)
        s=m.SOURCE.read_text()
        for fragment in ('r2=asl(r16,#5)', 'memw(r2<<#0+##0xf02d1258)',
                         'if(!cmp.gtu(r3.new,r16)) jump:t finish',
                         'p0=!tstbit(r2,#19)','if(!cmp.eq(r2.new,#0)) jump:t rescan',
                         'dealloc_return'):
            self.assertIn(fragment,s)
        self.assertNotIn('.word',s)
        self.assertNotIn('.incbin',s)
    def test_source_build_exact_and_privileged_read(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
        original=Path(self.ref).read_bytes();b=m.render(self.mc)
        m.verify(original,b)
        self.assertEqual(b[m.SGP0-m.START:m.SGP0-m.START+4],struct.pack('<I',0x6e80c002))
        self.assertEqual(original[prior.END:m.START],bytes(12))
    def test_revision_and_corrupt_packet_rejection(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and pinned ADSP required')
        original=Path(self.ref).read_bytes();b=m.render(self.mc)
        badref=bytearray(original);badref[35]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(badref,b)
        corrupted=bytearray(b);corrupted[0]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(original,corrupted)
if __name__=='__main__':unittest.main()
