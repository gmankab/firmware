# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon RWE vector table layout, source-level branch encoding and RTE."""
from pathlib import Path
import importlib.util,os,unittest
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-vectors.py'
spec=importlib.util.spec_from_file_location('agent11_vectors',str(SCRIPT))
module=importlib.util.module_from_spec(spec);assert spec.loader is not None;spec.loader.exec_module(module)
class Agent11ModemVectors(unittest.TestCase):
    def test_segment_mapping_and_count(self):
        self.assertEqual(module.FILE_END-module.FILE_START,256)
        self.assertEqual((module.BRANCH_BYTES,module.PADDING_BYTES,module.RTE_BYTES),(196,4,56))
        self.assertEqual(module.ELF_VADDR-module.FILE_START,0xc0808000)
        self.assertEqual(module.PHYSICAL_START-module.FILE_START,0x8e008000)
    def test_wrong_reference_rejected(self):
        with self.assertRaisesRegex(ValueError,'wrong modem.mbn'):
            module.verify_pinned(bytes(0x38100),bytes(256))
    def test_llvm6_vector_assembler_when_available(self):
        llvm=os.getenv('AGENT11_LLVM_MC')
        if not llvm:self.skipTest('set AGENT11_LLVM_MC for assembler test')
        code=module.assemble(Path(llvm),SCRIPT.parent.parent/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_irq_vectors.s')
        self.assertEqual(len(code),256)
        self.assertEqual(code[49*4:50*4],b'\0'*4)
        self.assertEqual(code[50*4:54*4],(0x57e0c000).to_bytes(4,'little')*4)
        self.assertEqual(len(code[50*4:]),56)
if __name__=='__main__':unittest.main()
