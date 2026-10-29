# SPDX-License-Identifier: GPL-3.0-only
"""V60 mode lookup conditional branch packet and privileged ISA checks."""
import importlib.util,os,unittest,tempfile,hashlib
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-controller-mode-lookup.py'
spec=importlib.util.spec_from_file_location('agent11_mode_lookup',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class TestModeLookup(unittest.TestCase):
    def test_expected_interval(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x3a50c,0x3a5a0))
        self.assertEqual(mod.FILE_END-mod.FILE_START,148)
        self.assertEqual(mod.FILE_START+0x8e008000,0x8e04250c)
    def test_wrong_reference_rejected(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem revision'):
            mod.verify(bytes(mod.FILE_END),bytes(148))
    def test_source_assembly_and_exact_bytes(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC missing')
        src=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_mode_lookup.s'
        b=mod.assemble(Path(mc),src)
        self.assertEqual(len(b),148)
        orig=os.environ.get('AGENT11_MODEM_MBN')
        if orig:mod.verify(Path(orig).read_bytes(),b)
    def test_wrong_privileged_action_fails_closed(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC missing')
        src=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_mode_lookup.s'
        with tempfile.TemporaryDirectory() as td:
            mutated=Path(td)/'wrong.s'
            mutated.write_text(src.read_text().replace('MODEM_CTRL_ACTION 0x60,3','MODEM_CTRL_ACTION 0x80,3'))
            with self.assertRaisesRegex(ValueError,'Unexpected privileged mode instruction'):
                mod.assemble(Path(mc),mutated)
if __name__=='__main__':unittest.main()
