# SPDX-License-Identifier: GPL-3.0-only
"""Standalone Hexagon V60 source-verification for the second modem PT_LOAD."""
from pathlib import Path
import importlib.util
import os
import unittest
SCRIPT=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe.py'
spec=importlib.util.spec_from_file_location('agent11_modem_rwe',str(SCRIPT))
module=importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)

class TestAgent11ModemRwe(unittest.TestCase):
    def test_elf_rwe_address_is_distinct_from_boot(self):
        self.assertEqual((module.FILE_START,module.FILE_END),(0xcb00,0xcc1c))
        self.assertEqual(module.ELF_VADDR-module.FILE_START,0xc0808000)
        self.assertEqual(module.PHYSICAL_START-module.FILE_START,0x8e008000)
        self.assertLess(module.FILE_END,0x436a8)
        self.assertGreater(module.FILE_START,0xa000)

    def test_refuses_wrong_revision_and_unknown_bytes(self):
        with self.assertRaisesRegex(ValueError,'wrong modem.mbn'):
            module.verify_pinned(bytes(50000),bytes(100))
        self.assertEqual(len(module.PINNED_REFERENCE_SHA256),64)

    def test_asm_produces_source_encoded_284_bytes(self):
        mc=os.getenv('AGENT11_LLVM_MC')
        if not mc:self.skipTest('Set AGENT11_LLVM_MC for external LLVM6 assembler test')
        output=module.assemble(Path(mc),SCRIPT.parent.parent/'sources/xiaomi-nabu/modem/modem_rwe_early_controller.s')
        self.assertEqual(len(output),284)
        words=[int.from_bytes(output[i:i+4],'little') for i in range(0,len(output),4)]
        # Source-built immediate extension precedes a long GPREL load.
        self.assertEqual(words[1]&0xc000,0x4000)
        self.assertEqual(words[2]&0xc000,0xc000)
        self.assertEqual(words[12]&0xc000,0x4000)
        self.assertEqual(words[-1]&0xc000,0xc000)
        # Three packet-order fixes ensure true `.new` semantics, including
        # the compact r16-is-zero branch in a 3-word predicate packet.
        for a in (0xcb78,0xcb98,0xcbc4):
            self.assertEqual(words[(a-module.FILE_START)//4]&0xc000,0x4000)
        self.assertEqual([words[(0xcbc4-module.FILE_START)//4+i]&0xc000 for i in range(3)],
                         [0x4000,0x4000,0xc000])
        self.assertEqual(words[(0xcbb4-module.FILE_START)//4],
                         0x64800020|(1<<16)|(3<<14)|(3<<8))
if __name__=='__main__':unittest.main()
