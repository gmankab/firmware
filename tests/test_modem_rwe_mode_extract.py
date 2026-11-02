# SPDX-License-Identifier: GPL-3.0-only
"""Byte-exact source-built EXTRACTU/RTE packet and contiguous 64-byte dispatcher."""
import importlib.util,os,struct,tempfile,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-mode-extract.py'
spec=importlib.util.spec_from_file_location('agent11_mode_extract',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)

class Agent11ModeExtract(unittest.TestCase):
    def test_exact_bounds_and_packet_markers(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x3a40c,0x3a414))
        source=struct.pack('<II',mod.LLVM_STANDALONE_EXTRACTU,mod.RTE_END)
        result=mod.normalize_packet(source)
        self.assertEqual(len(result),8)
        self.assertEqual(struct.unpack('<II',result),(0x8d2f4700,0x57e0c000))
        self.assertEqual((result[1]&0xc0),0x40) # EXTRACTU continuation
    def test_wrong_operands_or_parse_marker_fail_closed(self):
        for w in (0x8d2fc701,0x8d2f8700,0x8f2fc700,0x8d2f4700):
            with self.assertRaisesRegex(ValueError,'Unrecognized source-assembled'):
                mod.normalize_packet(struct.pack('<II',w,mod.RTE_END))
        with self.assertRaisesRegex(ValueError,'Invalid controller-mode'):
            mod.normalize_packet(bytes(12))
    def test_wrong_modem_revision_rejected(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem source revision'):
            mod.verify(bytes(0x3a414),bytes(8))
    def test_llvm6_assembler_and_contiguous_64_optional_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC required for original ISA assembly')
        reference=os.environ.get('AGENT11_MODEM_MBN')
        src=SCRIPT.parents[1]/'sources/xiaomi-nabu/modem/modem_rwe_mode_extract_rte.s'
        b=mod.assemble(Path(mc),src)
        self.assertEqual(struct.unpack('<II',b),(0x8d2f4700,0x57e0c000))
        ref=Path(reference).read_bytes() if reference else None
        if ref:mod.verify(ref,b)
        with tempfile.TemporaryDirectory() as d:
            result=Path(d)/'mode-64.bin'
            mod.emit_contiguous_64(Path(mc),ref,b,result)
            self.assertEqual(result.stat().st_size,64)
            if ref:self.assertEqual(result.read_bytes(),ref[0x3a404:0x3a444])
if __name__=='__main__':unittest.main()
