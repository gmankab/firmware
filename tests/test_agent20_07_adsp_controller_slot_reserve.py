# SPDX-License-Identifier: GPL-3.0-only
"""Exact byte/packet/revision tests for source-built controller slot reserve."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools'
def load(name):
    p=TOOLS/name;sp=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),p)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
m=load('adsp-agent20-rwe-next-controller-slot-reserve.py')
prior=load('adsp-agent20-rwe-next-controller-slot-bitmap-toggle.py')
class Reserve(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_source_and_boundaries(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11fd8,0x12098,192))
        self.assertEqual(prior.END,m.START)
        src=m.SOURCE.read_text()
        for op in ('memw_locked(r17)','memw_locked(r17,p0)=r3',
                   'r1:0=bitsplit(r16,#5)','memw(r17+#0)|=r19',
                   'if(p0) call trigger_slot_callback','dealloc_return'):
            self.assertIn(op,src.replace('if (p0)','if(p0)'))
        self.assertNotIn('.incbin',src)
    def test_opcode_fields_and_reference(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        b=m.render(self.mc)
        self.assertEqual(struct.unpack_from('<I',b,m.SGP0_AT-m.START)[0],0x6e80c002)
        self.assertEqual(struct.unpack_from('<I',b,m.PACKET_AT-m.START)[0],0x7c004022)
        ref=Path(self.ref).read_bytes();m.verify(ref,b)
        self.assertEqual(prior.render(self.mc)+b,ref[prior.START:m.END])
    def test_fail_closed_on_bad_revision_and_mutation(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6/reference unavailable')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc)
        bad=bytearray(b);bad[30]^=0x04
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
        badref=bytearray(ref);badref[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(badref,b)
if __name__=='__main__':unittest.main()
