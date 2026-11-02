# SPDX-License-Identifier: GPL-3.0-only
"""Verify source-assembled controller state packets and independently built duplex."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
if str(T) not in sys.path:sys.path.insert(0,str(T))
s=importlib.util.spec_from_file_location('adsp_controller_state',T/'rwe-next-controller-state.py')
assert s and s.loader
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)

class ControllerState(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC');cls.ref=os.getenv('NABU_ADSP_REFERENCE')

    def test_exact_original_address_map(self):
        self.assertEqual((m.FILE_START,m.FILE_END,m.SIZE),(0x10f64,0x10fe8,132))
        self.assertEqual(m.PHYSICAL_START,0x8be0df64)

    def test_source_emitter_preserves_checked_duplex(self):
        if not self.mc:self.skipTest('LLVM6 Hexagon assembler not supplied')
        from adsp_hexagon_v66 import packet_continuation
        raw=m.previous().assemble(self.mc,m.SOURCE,m.SIZE)
        built=m.render(self.mc)
        self.assertEqual(struct.unpack_from('<I',raw,0x6c)[0],0x78004480)
        src_zero=struct.unpack('<I',m.previous().assemble(self.mc,m.ZERO_SCALAR,4))[0]
        self.assertEqual(struct.unpack_from('<I',built,0x6c)[0],packet_continuation(src_zero))
        src_duplex=m.previous().assemble(self.mc,m.RESTORE_DUPLEX,4)
        self.assertEqual(built[0x70:0x74],src_duplex)
        self.assertEqual(built[0x38:0x3c],raw[0x38:0x3c])  # regular callback packet
        if self.ref:m.verify(Path(self.ref).read_bytes(),built)

    def test_both_reference_and_output_corruption_are_rejected(self):
        if not self.ref or not self.mc:self.skipTest('pinned reference + LLVM6 required')
        original=Path(self.ref).read_bytes();built=m.render(self.mc)
        m.verify(original,built)
        incorrect=bytearray(built);incorrect[24]^=1
        with self.assertRaisesRegex(ValueError,'differ'):
            m.verify(original,incorrect)
        bad_ref=bytearray(original);bad_ref[0x10f74]^=1
        with self.assertRaisesRegex(ValueError,'Wrong ADSP reference'):
            m.verify(bad_ref,built)
if __name__=='__main__':unittest.main()
