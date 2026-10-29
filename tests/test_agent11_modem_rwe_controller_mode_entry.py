# SPDX-License-Identifier: GPL-3.0-only
"""Checked architecture encodings for agent11 controller mode entry."""
import importlib.util,os,struct,tempfile,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-controller-mode-entry.py'
spec=importlib.util.spec_from_file_location('agent11_controller_mode_entry',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class TestControllerEntry(unittest.TestCase):
    def test_bounds_and_pinned_revision(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x3a4c4,0x3a50c))
        self.assertEqual(mod.FILE_END-mod.FILE_START,72)
        self.assertEqual(mod.FILE_START+0x8e008000,0x8e0424c4)
        self.assertEqual(len(mod.PINNED_SHA256),64)
    def test_bad_reference_rejected(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem reference'):
            mod.verify(bytes(mod.FILE_END),bytes(72))
    def test_source_compiles_and_ref_match_when_tools_provided(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC not set')
        source=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_mode_entry.s'
        b=mod.assemble(Path(mc),source)
        self.assertEqual(len(b),72)
        self.assertEqual(struct.unpack_from('<I',b,0x38)[0],0x6c20c080)
        self.assertEqual(struct.unpack_from('<I',b,0x44)[0],0x528dc000)
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),b)
    def test_incorrect_privileged_field_fails(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC not set')
        source=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_mode_entry.s'
        with tempfile.TemporaryDirectory() as td:
            p=Path(td)/'malformed.s'
            p.write_text(source.read_text().replace('MODEM_SYS_READ 13,16,1','MODEM_SYS_READ 13,17,1'))
            with self.assertRaisesRegex(ValueError,'Invalid privileged instruction'):
                mod.assemble(Path(mc),p)
if __name__=='__main__':unittest.main()
