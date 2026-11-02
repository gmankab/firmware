# SPDX-License-Identifier: GPL-3.0-only
"""Second protected Hexagon V60 lookup and IRQ-clear code tests."""
import importlib.util,os,tempfile,unittest
from pathlib import Path
SCRIPT=Path(__file__).resolve().parents[1]/'tools/modem/assemble-rwe-controller-mode-secondary.py'
spec=importlib.util.spec_from_file_location('agent11_mode_secondary',SCRIPT)
mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
class TestSecondaryMode(unittest.TestCase):
 def test_range(self):
  self.assertEqual((mod.START,mod.END),(0x3a5a0,0x3a648))
  self.assertEqual(mod.END-mod.START,168)
 def test_wrong_reference_sha_rejected(self):
  with self.assertRaisesRegex(ValueError,'Wrong modem reference revision'):
   mod.verify(bytes(mod.END),bytes(mod.END-mod.START))
 def test_source_and_original_match(self):
  mc=os.environ.get('AGENT11_LLVM_MC')
  if not mc:self.skipTest('Hexagon llvm-mc not configured')
  src=SCRIPT.parents[1]/'sources/xiaomi-nabu/modem/modem_rwe_controller_mode_secondary.s'
  b=mod.assemble(Path(mc),src)
  self.assertEqual(len(b),168)
  ref=os.environ.get('AGENT11_MODEM_MBN')
  if ref:mod.verify(Path(ref).read_bytes(),b)
 def test_fail_closed_on_privileged_action_mutation(self):
  mc=os.environ.get('AGENT11_LLVM_MC')
  if not mc:self.skipTest('Hexagon llvm-mc not configured')
  src=SCRIPT.parents[1]/'sources/xiaomi-nabu/modem/modem_rwe_controller_mode_secondary.s'
  with tempfile.TemporaryDirectory() as td:
   bad=Path(td)/'bad.s'
   bad.write_text(src.read_text().replace('MODEM_CTRL_ACTION 0x60,3','MODEM_CTRL_ACTION 0x80,3',1))
   with self.assertRaisesRegex(ValueError,'Unexpected source-assembled V60 privileged'):
    mod.assemble(Path(mc),bad)
if __name__=='__main__':unittest.main()
