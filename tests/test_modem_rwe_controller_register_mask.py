# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V60 duplex shims, controller system writes and trap checks."""
import importlib.util,os,tempfile,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-controller-register-mask.py'
spec=importlib.util.spec_from_file_location('agent11_ctrl_reg_mask',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class TestControllerRegisterMask(unittest.TestCase):
 def test_bounds(self):
  self.assertEqual((mod.START,mod.END),(0x3a72c,0x3a778))
  self.assertEqual(mod.END-mod.START,76)
 def test_reject_wrong_revision(self):
  with self.assertRaisesRegex(ValueError,'Wrong pinned modem revision'):
   mod.verify(bytes(mod.END),bytes(76))
 def test_exact_llvm_and_pinned_compare(self):
  mc=os.environ.get('AGENT11_LLVM_MC')
  if not mc:self.skipTest('Hexagon LLVM6 unavailable')
  src=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_register_mask.s'
  b=mod.assemble(Path(mc),src)
  self.assertEqual(len(b),76)
  ref=os.environ.get('AGENT11_MODEM_MBN')
  if ref:mod.verify(Path(ref).read_bytes(),b)
 def test_changed_sysreg_rejected(self):
  mc=os.environ.get('AGENT11_LLVM_MC')
  if not mc:self.skipTest('Hexagon LLVM6 unavailable')
  src=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_register_mask.s'
  with tempfile.TemporaryDirectory() as td:
   bad=Path(td)/'changed.s'
   bad.write_text(src.read_text().replace('MODEM_SYS_READ 25,0,3','MODEM_SYS_READ 25,1,3'))
   with self.assertRaisesRegex(ValueError,'Unexpected protected V60 SYS encoding'):
    mod.assemble(Path(mc),bad)
if __name__=='__main__':unittest.main()
