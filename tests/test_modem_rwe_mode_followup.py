# SPDX-License-Identifier: GPL-3.0-only
"""Strict source-assembled mode follow-up packet checks and reference match."""
import importlib.util,os,struct,tempfile,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-mode-followup.py'
spec=importlib.util.spec_from_file_location('agent11_mode_followup',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)

class Agent11Followup(unittest.TestCase):
    def test_controller_followup_bounds(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x3a444,0x3a47c))
        self.assertEqual(mod.FILE_END-mod.FILE_START,56)
        self.assertEqual((mod.FILE_START+0xc0808000,mod.FILE_START+0x8e008000),
                         (0xc0842444,0x8e042444))
    def test_predicate_branch_marker_validation(self):
        words=bytearray(56)
        struct.pack_into('<II',words,0x20,mod.BRANCH_ASSEMBLED,mod.COMPARE_ASSEMBLED)
        result=mod.normalize_llvm6_packet(bytes(words))
        self.assertEqual(struct.unpack_from('<II',result,0x20),(0x75627ff0,0x5c00c806))
        for bad in (mod.BRANCH_ASSEMBLED|0x8000,mod.BRANCH_ASSEMBLED+1):
            struct.pack_into('<II',words,0x20,bad,mod.COMPARE_ASSEMBLED)
            with self.assertRaisesRegex(ValueError,'Unexpected LLVM6'):
                mod.normalize_llvm6_packet(bytes(words))
        with self.assertRaisesRegex(ValueError,'instruction-length mismatch'):
            mod.normalize_llvm6_packet(bytes(52))
    def test_wrong_firmware_is_rejected(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem source revision'):
            mod.verify(bytes(mod.FILE_END),bytes(56))
    def test_original_isa_source_matches_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC required for independent Hexagon assembler')
        src=SCRIPT.parents[1]/'sources/xiaomi-nabu/modem/modem_rwe_mode_followup.s'
        generated=mod.assemble(Path(mc),src)
        self.assertEqual(len(generated),56)
        self.assertEqual(struct.unpack_from('<I',generated,0x30)[0],0x57e0c000)
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),generated)
if __name__=='__main__':unittest.main()
