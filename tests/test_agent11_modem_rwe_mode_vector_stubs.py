# SPDX-License-Identifier: GPL-3.0-only
"""Fail-closed Hexagon V60 tests for tiny disjoint controller mode stubs."""
import importlib.util, os, struct, unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-mode-vector-stubs.py'
spec=importlib.util.spec_from_file_location('agent11_mode_vector_stubs',SCRIPT)
mod=importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)

class Agent11ModeVectorStubs(unittest.TestCase):
    def test_disjoint_source_ranges_not_counting_gap(self):
        self.assertEqual(mod.FILE_RANGES,((0x3a404,0x3a40c),(0x3a414,0x3a444)))
        self.assertEqual(sum(b-a for a,b in mod.FILE_RANGES),56)
        self.assertEqual(mod.FILE_RANGES[1][0]-mod.FILE_RANGES[0][1],8)
        for start,end in mod.FILE_RANGES:
            self.assertEqual((start+0xc0808000,end+0xc0808000),
                             (0xc0808000+start,0xc0808000+end))
    def test_wrong_firmware_revision_rejected(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem source revision'):
            mod.verify(bytes(mod.FILE_RANGES[-1][-1]),(bytes(8),bytes(48)))
    def test_size_and_opcode_tampering_rejected(self):
        with self.assertRaisesRegex(ValueError,'Invalid Hexagon mode-vector region sizes'):
            mod.validate_instruction_fields((bytes(12),bytes(48)))
        first=bytearray(8);second=bytearray(48)
        with self.assertRaisesRegex(ValueError,'Invalid architecture operand'):
            mod.validate_instruction_fields((bytes(first),bytes(second)))
    def test_llvm6_source_and_optional_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC required to test Hexagon LLVM6')
        src=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_mode_vector_stubs.s'
        parts=mod.assemble(Path(mc),src)
        self.assertEqual(tuple(len(x) for x in parts),(8,48))
        self.assertEqual(struct.unpack_from('<I',parts[1],0x24)[0],0x1230e3ea)
        self.assertEqual(struct.unpack_from('<I',parts[1],0x28)[0],0x78004000)
        self.assertEqual(struct.unpack_from('<I',parts[1],0x2c)[0],0x57e0c000)
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),parts)
if __name__=='__main__':unittest.main()
