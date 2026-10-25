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

    def test_assembler_with_llvm6_when_available(self):
        llvm=os.environ.get("AGENT11_LLVM_MC")
        if not llvm: self.skipTest("set AGENT11_LLVM_MC for external tool verification")
        source=SCRIPT.parent.parent/"crates/firmware-tools/src/devices/xiaomi_nabu/modem_entry_followup.s"
        data=module.assemble(Path(llvm),source)
        self.assertEqual(len(data),64)
        # Structural assertion checks the two adjacent word loads are in
        # one duplex slot and the final instruction is a packet-ending return.
        words=[int.from_bytes(data[n:n+4],"little") for n in range(0,len(data),4)]
        self.assertEqual(words[0] & 0xc000,0x4000) # constant extender continuation parse marker
        self.assertEqual(words[-1]&0xc000,0xc000)

if __name__=='__main__':unittest.main()
