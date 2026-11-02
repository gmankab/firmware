# SPDX-License-Identifier: GPL-3.0-only
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
s=importlib.util.spec_from_file_location('adsp_global',T/'rwe-next-global-record-prefix.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class GlobalRecordPrefix(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_register_and_packet_contract(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x118c0,0x11928,104))
        self.assertEqual(m.PRIVILEGED,{
            0x118c4:0x6e80c002,0x118d0:0x6e9dc001,
            0x118d4:0x6a0bc002,0x11914:0x6e9ec004,
        })
        src=m.SOURCE.read_text()
        self.assertIn('r4=memub(gp+##0xf02ca2c4); memb(r16+#227)=r4.new',src)
        self.assertIn('if(cmp.eq(r16.new,#0)) jump:nt no_record',src)
        self.assertNotIn('.incbin',src)
    def test_rebuild_and_reference_match(self):
        if not self.mc:self.skipTest('Hexagon LLVM6 required')
        b=m.render(self.mc)
        self.assertEqual(len(b),104)
        for addr,word in m.PRIVILEGED.items():self.assertEqual(struct.unpack_from('<I',b,addr-m.START)[0],word)
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)
    def test_invalid_reference_and_changed_bytes_rejected(self):
        if not self.mc or not self.ref:self.skipTest('LLVM6 and original reference required')
        r=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(r,b)
        wrong=bytearray(b);wrong[-1]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(r,wrong)
        corrupt=bytearray(r);corrupt[0]^=1
        with self.assertRaisesRegex(ValueError,'reference SHA'):m.verify(corrupt,b)
    def test_no_unresolved_v66_placeholders(self):
        if not self.mc:self.skipTest('LLVM6 required')
        b=m.render(self.mc)
        self.assertNotIn(m.NOP,[b[i:i+4] for i in range(0,len(b),4)])
if __name__=='__main__':unittest.main()
