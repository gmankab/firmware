# SPDX-License-Identifier: GPL-3.0-only
"""Disjoint ADSP V66 sparse-layout source assembly and gap accounting."""
from __future__ import annotations
import importlib.util,os,struct,sys,unittest
from pathlib import Path
TOOLS=Path(__file__).resolve().parents[1]/'tools'
spec=importlib.util.spec_from_file_location('rwe_sparse',TOOLS/'adsp-agent20-rwe-sparse.py')
assert spec and spec.loader
sparse=importlib.util.module_from_spec(spec);spec.loader.exec_module(sparse)
class SparseRwe(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.getenv('HEXAGON_LLVM_MC')
        cls.ref=os.getenv('NABU_ADSP_REFERENCE')
    def test_file_layout_is_explicit_and_disjoint(self):
        self.assertEqual((sparse.FILE_START,sparse.FILE_END,sparse.SIZE),(0x10330,0x10534,516))
        self.assertEqual(sparse.FILL_END-sparse.FILL_START,0xff20-0x6c6c)
        self.assertEqual(sparse.UNKNOWN_RECORD_END-sparse.UNKNOWN_RECORD_START,0x90)
        self.assertEqual(sparse.FILL2_END-sparse.FILL2_START,0x380)
    def test_source_packets_and_strict_reference(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC required')
        result=sparse.render(self.mc)
        self.assertEqual(len(result),516)
        # Function return is not a packet boundary: packet continues with
        # extended GP load from the next function, so no artificial split.
        self.assertEqual((struct.unpack_from('<I',result,0x10)[0]>>14)&3,1)
        self.assertEqual((struct.unpack_from('<I',result,0x18)[0]>>14)&3,3)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x1c)[0],0x6e88c006)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x20)[0],0x6f1ec004)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x40)[0],0x6e88400c)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x4c)[0],0x6f1e4004)
        self.assertEqual((struct.unpack_from('<I',result,0x1c+0x4c)[0]>>14)&3,1)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x58)[0],0x6e80401d)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x60)[0],0x6eaac000)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x68)[0],0x6e92c001)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x70)[0],0x6701c012)
        self.assertEqual(struct.unpack_from('<I',result,0x1c+0x78)[0],0x6eaac000)
        if self.ref:
            original=Path(self.ref).read_bytes()
            sparse.verify(original,result)
    def test_sram_clone_loop_packet_and_reference(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC required')
        clone=sparse.base().assemble(self.mc,sparse.SRAM_CLONE_SOURCE,0x60)
        self.assertEqual(len(clone),96)
        # Two original controller loops: 50 word copies and ten cache cleans.
        self.assertEqual(struct.unpack_from('<I',clone,0x40)[0],0x6901c08a)
        self.assertEqual(struct.unpack_from('<I',clone,0x54)[0],0x6000c008)
        self.assertEqual((struct.unpack_from('<I',clone,0x34)[0]>>14)&3,3)
        self.assertEqual((struct.unpack_from('<I',clone,0x38)[0]>>14)&3,1)
        if self.ref:
            self.assertEqual(clone,Path(self.ref).read_bytes()[0x103d0:0x10430])

    def test_controller_status_poll_uses_source_packet_pairs(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC required')
        source=sparse.base().assemble(self.mc,sparse.CONTROLLER_POLL_SOURCE,0x30)
        self.assertEqual(len(source),48)
        self.assertEqual(struct.unpack_from('<I',source,0x08)[0],0x2402e018)
        self.assertEqual(struct.unpack_from('<I',source,0x20)[0],0x2582c000)
        self.assertEqual(struct.unpack_from('<I',source,0x2c)[0],0x2582c000)
        if self.ref:
            self.assertEqual(source,Path(self.ref).read_bytes()[0x10430:0x10460])

    def test_privileged_debugger_register_snapshot(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC required')
        source=sparse.base().assemble(self.mc,sparse.DEBUG_SNAPSHOT_SOURCE,0x58)
        for off in range(0x08,0x40,8):
            self.assertEqual(struct.unpack_from('<I',source,off)[0],0xb0004000)
        generated=sparse.debug_snapshot(self.mc,sparse.base())
        self.assertEqual(len(generated),0x58)
        from adsp_hexagon_v66 import SystemRegister,read_system_register,packet_continuation
        for i,reg in enumerate((SystemRegister.ISDBCFG0,SystemRegister.ISDBCFG1,
             SystemRegister.BRKPTPC0,SystemRegister.BRKPTCFG0,
             SystemRegister.BRKPTPC1,SystemRegister.BRKPTCFG1,SystemRegister.ISDBGPR)):
            self.assertEqual(struct.unpack_from('<I',generated,0x08+i*8)[0],
                             packet_continuation(read_system_register(0,reg)))
        if self.ref:
            self.assertEqual(generated,Path(self.ref).read_bytes()[0x10460:0x104b8])

    def test_next_controller_phase_uses_checked_isdben_write(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC required')
        source=sparse.base().assemble(self.mc,sparse.CONTROLLER_PHASE_SOURCE,
                                       sparse.CONTROLLER_PHASE_SIZE)
        self.assertEqual(len(source),124)
        self.assertEqual(struct.unpack_from('<I',source,0x74)[0],0x7f00c000)
        from adsp_hexagon_v66 import SystemRegister,write_system_register
        compiled=sparse.controller_phase(self.mc,sparse.base())
        self.assertEqual(struct.unpack_from('<I',compiled,0x74)[0],
                         write_system_register(SystemRegister.ISDBEN,0))
        self.assertEqual(compiled[-4:],source[-4:])
        if self.ref:
            self.assertEqual(compiled,
                Path(self.ref).read_bytes()[0x104b8:0x10534])

    def test_reject_reference_mutation(self):
        if not self.mc or not self.ref:self.skipTest('reference and LLVM6 required')
        b=sparse.render(self.mc)
        ref=bytearray(Path(self.ref).read_bytes())
        ref[sparse.FILE_START]^=1
        with self.assertRaises(ValueError):sparse.verify(bytes(ref),b)
