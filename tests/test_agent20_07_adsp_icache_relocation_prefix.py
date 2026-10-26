# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 instruction-cache relocation prefix, separate from fill."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
if str(T) not in sys.path:sys.path.insert(0,str(T))
s=importlib.util.spec_from_file_location('cache_reloc_prefix',T/'adsp-agent20-rwe-next-icache-relocation-prefix.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class CacheRelocation(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_address_bounds_and_fill(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x11170,0x111dc,108))
        self.assertEqual(m.FILE_START-m.FILL_START,12)
    def test_checked_evb_read_and_packets(self):
        if not self.mc:self.skipTest('Hexagon LLVM6 unavailable')
        from adsp_hexagon_v66 import read_system_register,SystemRegister
        out=m.render(self.mc)
        self.assertEqual(struct.unpack_from('<I',out,4)[0],read_system_register(2,SystemRegister.EVB))
        self.assertEqual(out[0:4],struct.pack('<I',0xebf41c30))
        if self.ref:m.verify(Path(self.ref).read_bytes(),out)
    def test_byte_and_revision_fail_closed(self):
        if not self.mc or not self.ref:self.skipTest('Hexagon LLVM6 and pinned ADSP required')
        ref=Path(self.ref).read_bytes();out=m.render(self.mc);m.verify(ref,out)
        bad=bytearray(out);bad[-1]^=1
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,bad)
        altered=bytearray(ref);altered[m.FILL_START]=1
        with self.assertRaisesRegex(ValueError,'Wrong ADSP'):m.verify(altered,out)
if __name__=='__main__':unittest.main()
