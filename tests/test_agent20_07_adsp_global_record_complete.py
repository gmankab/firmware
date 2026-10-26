# SPDX-License-Identifier: GPL-3.0-only
"""Full ADSP global record completion source and K0 unlock checks."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools';s=importlib.util.spec_from_file_location('global_record',T/'adsp-agent20-rwe-next-global-record-complete.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class GlobalRecordCompletion(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_layout_and_privileged_contract(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x11928,0x119c0,152))
        self.assertEqual(m.UNLOCK_AT,0x1198c)
        source=m.SOURCE.read_text()
        self.assertIn('r4=memw(gp+##0xf00250cc)',source)
        self.assertIn('r5=memw(gp+##0xf00250d0)',source)
        self.assertIn('call update_record_status; memw(r16+#232)=r2',source)
        self.assertIn('jump record_trap',source)
        self.assertNotIn('.incbin',source)
    def test_exact_rebuild_and_k0_unlock(self):
        if not self.mc:self.skipTest('LLVM6 not configured')
        b=m.render(self.mc)
        self.assertEqual(struct.unpack_from('<I',b,m.UNLOCK_AT-m.START)[0],0x6c20c080)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)
    def test_reject_wrong_reference_and_changed_code(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and reference required')
        r=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(r,b)
        bad=bytearray(b);bad[4]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(r,bad)
        wrong=bytearray(r);wrong[0]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(wrong,b)
    def test_no_residual_nops(self):
        if not self.mc:self.skipTest('LLVM6 not configured')
        b=m.render(self.mc)
        self.assertNotIn(m.NOP,[b[i:i+4] for i in range(0,len(b),4)])
if __name__=='__main__':unittest.main()
