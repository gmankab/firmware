# SPDX-License-Identifier: GPL-3.0-only
"""Fail-closed checks for Hexagon V60 protected controller bitfield routine."""
import importlib.util,os,struct,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-mode-word-ops.py'
spec=importlib.util.spec_from_file_location('agent11_mode_word_ops',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)

class Agent11ModeWordOps(unittest.TestCase):
    def test_bounds_and_protected_controller_location(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x3a47c,0x3a4c4))
        self.assertEqual(mod.FILE_END-mod.FILE_START,72)
        self.assertEqual(mod.FILE_START+0x8e008000,0x8e04247c)
    def test_reordering_check_requires_exact_opcode_and_marker(self):
        buf=bytearray(72)
        struct.pack_into('<II',buf,0x14,mod.LLVM_BRANCH,mod.LLVM_PREDICATE)
        out=mod.normalize_llvm6(bytes(buf))
        self.assertEqual(struct.unpack_from('<II',out,0x14),
                         (mod.LLVM_PREDICATE,mod.LLVM_BRANCH))
        for bad in (mod.LLVM_BRANCH+1,mod.LLVM_BRANCH|(1<<15)):
            struct.pack_into('<II',buf,0x14,bad,mod.LLVM_PREDICATE)
            with self.assertRaisesRegex(ValueError,'Unexpected source-assembled'):
                mod.normalize_llvm6(bytes(buf))
        with self.assertRaisesRegex(ValueError,'Invalid mode-bitfield instruction count'):
            mod.normalize_llvm6(bytes(68))
    def test_wrong_modem_revision_fails_closed(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem reference revision'):
            mod.verify(bytes(mod.FILE_END),bytes(72))
    def test_source_assembly_optional_exact_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC absent')
        src=SCRIPT.parents[1]/'sources/xiaomi-nabu/modem/modem_rwe_mode_word_ops.s'
        b=mod.assemble(Path(mc),src)
        self.assertEqual(len(b),72)
        self.assertEqual(struct.unpack_from('<I',b,0x30)[0],0x838058e2)
        self.assertEqual(struct.unpack_from('<I',b,0x44)[0],0x57e0c000)
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),b)
if __name__=='__main__':unittest.main()
