# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V60 source-built trap entries and callback frame tests."""
import importlib.util,os,struct,unittest
from pathlib import Path
S=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-controller-trap-callbacks.py'
spec=importlib.util.spec_from_file_location('agent11_trap_callbacks',S)
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class TrapCallbacks(unittest.TestCase):
    def test_mapping_and_length(self):
        self.assertEqual((m.START,m.END),(0x3aa1c,0x3aa8c))
        self.assertEqual(m.END-m.START,112)
        self.assertEqual(m.START+0x8e008000,0x8e042a1c)
    def test_trap_number_and_return_mutation_fail_closed(self):
        b=bytearray(112)
        for i,n in enumerate((10,8,9)):
            struct.pack_into('<II',b,i*8,0x5400c0e0 + (n<<2),0x5800c000)
        struct.pack_into('<I',b,24,0xa09dc006)
        struct.pack_into('<I',b,60,0x50a4c000)
        struct.pack_into('<I',b,68,0x50a5c000)
        struct.pack_into('<II',b,104,0x901ec01e,0x529fc000)
        self.assertEqual(len(m.validate(bytes(b))),112)
        for ix in (0,4,8,12,16,20,24,60,68,104,108):
            changed=bytearray(b);changed[ix]^=1
            with self.subTest(ix=ix),self.assertRaises(ValueError):m.validate(changed)
    def test_bad_revision_rejected(self):
        with self.assertRaisesRegex(ValueError,'Wrong pinned modem'):m.verify(bytes(0x3aa8c),bytes(112))
    def test_assembled_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('Hexagon LLVM6 not configured')
        src=S.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_trap_callbacks.s'
        b=m.assemble(Path(mc),src)
        self.assertEqual(len(b),112)
        if ref:=os.environ.get('AGENT11_MODEM_MBN'):m.verify(Path(ref).read_bytes(),b)
if __name__=='__main__':unittest.main()
