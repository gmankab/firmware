# SPDX-License-Identifier: GPL-3.0-only
import importlib.util
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location('bridge',ROOT/'tools/cdsp'/'verify-cdsp-supervisor-bridge.py')
BRIDGE=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BRIDGE)

class SupervisorBridgeTests(unittest.TestCase):
    def test_rel_descriptor_preserves_extended_immediate_packet(self):
        assembler=BRIDGE.make_asm()
        self.assertIn('r30 = ##-0x1fff280',assembler)
        self.assertIn('r31 = add(r31,r25)',assembler)
        self.assertIn('r30 = add(r30,r25)',assembler)
        self.assertIn('jumpr r30',assembler)
        self.assertEqual(BRIDGE.TRAMPOLINE_SIZE,20)
        self.assertEqual(BRIDGE.ENTRY_OFFSET,0x3d6c)

if __name__=='__main__':unittest.main()
