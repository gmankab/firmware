# SPDX-License-Identifier: GPL-3.0-only
"""Agent11 early modem packets: standalone emission and fail-closed checks."""
from pathlib import Path
import importlib.util
import unittest
import os

SCRIPT=Path(__file__).resolve().parents[1]/"tools/assemble-agent11-modem.py"
spec=importlib.util.spec_from_file_location("agent11_modem",str(SCRIPT))
module=importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)

class ModemAgent11Tests(unittest.TestCase):
    def test_mapped_entry_boundaries(self):
        self.assertEqual(module.FILE_START,0x4058)
        self.assertEqual(module.PHYSICAL_START,0x8e001058)
        self.assertEqual(module.ELF_VADDR,0xc0801058)
        self.assertEqual(module.PHYSICAL_START-module.FILE_START,0x8dffd000)

    def test_missing_or_wrong_reference_rejected(self):
        with self.assertRaisesRegex(ValueError,"incorrect revision"):
            module.verify_reference(bytes(0x5000),bytes(64))
        self.assertEqual(len(module.EXPECTED_SHA256),64)

    def test_packet_order_normalizer_rejects_unknown_encoding(self):
        size=0x42c8-module.FILE_START
        with self.assertRaisesRegex(ValueError,"unexpected Hexagon"):
            module.restore_original_legal_hexagon_packet_order(bytes(size))

    def test_assembler_with_llvm6_when_available(self):
        llvm=os.environ.get("AGENT11_LLVM_MC")
        if not llvm: self.skipTest("set AGENT11_LLVM_MC for external tool verification")
        source=SCRIPT.parent.parent/"crates/firmware-tools/src/devices/xiaomi_nabu/modem_entry_followup.s"
        data=module.assemble(Path(llvm),source)
        self.assertEqual(len(data),1276)
        # Structural assertion checks the two adjacent word loads are in
        # one duplex slot and the final instruction is a packet-ending return.
        words=[int.from_bytes(data[n:n+4],"little") for n in range(0,len(data),4)]
        self.assertEqual(words[0] & 0xc000,0x4000) # extended immediate packet
        self.assertEqual(words[16] & 0xc000,0xc000) # S27 = r0, end packet
        self.assertEqual(words[30] & 0xc000,0xc000) # :endloop0 at 0x40d0
        self.assertEqual(words[31]&0xc000,0xc000) # function return at 0x40d4
        self.assertEqual((words[16]>>16)&0x3f,27) # S27 read in first table-copy
        self.assertEqual((words[32]>>16)&0x3f,27) # S27 read in descriptor update
        self.assertEqual(words[36]&0xc000,0x4000) # predicate in 4-word packet
        self.assertEqual(words[39]&0xc000,0xc000) # predicated update closes packet
        self.assertEqual(words[46]&0xc000,0xc000) # protected S-register setup
        self.assertEqual(words[81]&0xc000,0xc000) # fault brkpt at 0x419c
        self.assertEqual(words[-1]&0xc000,0xc000) # status shim jumpr r13
        self.assertEqual(words[95]&0xc000,0xc000) # controller register barrier
        i=(0x42bc-module.FILE_START)//4
        self.assertEqual(words[i]&0xc000,0x4000) # extender of compare immediate
        self.assertEqual(words[i+1]&0xc000,0x4000) # comparison before .new branch
        self.assertEqual(words[i+2]&0xc000,0xc000) # branch ends packet
        for a in (0x4318,0x4340,0x4368):
            self.assertEqual(words[(a-module.FILE_START)//4]&0xc000,0x4000,
                "r7 copy and r5 halfword initialization share one packet")
        self.assertEqual(words[(0x44e8-module.FILE_START)//4]&0xc000,0xc000) # diagnostic brkpt
        self.assertEqual(words[-1]&0xc000,0xc000) # synchronized final cache return
        self.assertEqual(words[(0x444c-module.FILE_START)//4]&0xc000,0x4000)
        self.assertEqual(words[(0x4454-module.FILE_START)//4]&0xc000,0xc000)

if __name__=='__main__':unittest.main()
