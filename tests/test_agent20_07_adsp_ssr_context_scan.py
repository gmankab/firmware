# SPDX-License-Identifier: GPL-3.0-only
"""Bounded Hexagon V66 SSR controller scanner with checked endloop duplex."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools'
if str(T) not in sys.path:sys.path.insert(0,str(T))
s=importlib.util.spec_from_file_location('ssr',T/'adsp-agent20-rwe-next-ssr-context-scan.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class SSRContext(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')

    def test_strict_bounds_and_disjointness(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x11090,0x11108,120))
        self.assertEqual(m.FILE_START-m.FILL_START,8)

    def test_loop_source_order_and_canonical_reference(self):
        if not self.mc:self.skipTest('Hexagon LLVM6 required')
        from adsp_hexagon_v66 import SystemRegister,read_system_register
        b=m.render(self.mc)
        self.assertEqual(struct.unpack_from('<I',b,0)[0],read_system_register(2,SystemRegister.SSR))
        at=0x110f4-m.FILE_START
        nopair=m.independent_nopair(self.mc,m.base())
        self.assertEqual(b[at:at+8],nopair[4:12])
        self.assertEqual(b[at+8:at+12],m.base().assemble(self.mc,m.LOOP_DUPLEX,12)[-4:])
        if self.ref:m.verify(Path(self.ref).read_bytes(),b)

    def test_output_corruption_and_reference_revision_fail_closed(self):
        if not(self.mc and self.ref):self.skipTest('Pinned reference and LLVM6 required')
        ref=Path(self.ref).read_bytes();b=m.render(self.mc);m.verify(ref,b)
        corrupted=bytearray(b);corrupted[19]^=0x20
        with self.assertRaisesRegex(ValueError,'differs'):m.verify(ref,corrupted)
        ref=bytearray(ref);ref[m.FILL_START]=1
        with self.assertRaisesRegex(ValueError,'Wrong ADSP'):m.verify(ref,b)
if __name__=='__main__':unittest.main()
