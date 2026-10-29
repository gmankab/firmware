# SPDX-License-Identifier: GPL-3.0-only
"""Fail-closed tests for the independent modem RWE controller-mode release."""
import importlib.util,os,struct,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-mode-release.py'
spec=importlib.util.spec_from_file_location('agent11_mode_release',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)

class Agent11ModeRelease(unittest.TestCase):
    def test_exact_disjoint_bounds_and_addresses(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x3a3d8,0x3a404))
        self.assertEqual(mod.VIRTUAL_START-mod.FILE_START,0xc0808000)
        self.assertEqual(mod.PHYSICAL_START-mod.FILE_START,0x8e008000)
        self.assertEqual(mod.FILE_END-mod.FILE_START,44)
    def test_wrong_reference_is_rejected(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem source revision'):
            mod.verify(bytes(mod.FILE_END),bytes(44))
    def test_assembled_operands_and_optional_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('AGENT11_LLVM_MC required for source assembly')
        p=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_early_mode_release.s'
        data=mod.assemble(Path(mc),p)
        get=lambda off:struct.unpack_from('<I',data,off-mod.FILE_START)[0]
        self.assertEqual(get(0x3a3d8),0x6e83c007)  # special S3 read
        self.assertEqual(get(0x3a3e4),0x0c0850aa)  # architecture immediate extender
        self.assertEqual(get(0x3a3e8),0x7e20c187)  # conditionally assigned r7
        self.assertEqual(get(0x3a3f0),0x0c0850aa)  # second extender
        self.assertEqual(get(0x3a3f4),0x7e20c407)  # conditionally assigned r7
        self.assertEqual(get(0x3a3f8),0x670fc006)  # write S6 from r15
        self.assertEqual(get(0x3a3fc),0x6707c003)  # write S3 from r7
        self.assertEqual(get(0x3a400),0x57e0c000)  # architectural RTE
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),data)
if __name__=='__main__':unittest.main()
