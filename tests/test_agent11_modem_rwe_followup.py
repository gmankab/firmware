# SPDX-License-Identifier: GPL-3.0-only
"""Independent source reconstruction and bounds for the disjoint RWE lock."""
import importlib.util
import os
from pathlib import Path
import unittest

SCRIPT=Path(__file__).resolve().parents[1]/'tools/assemble-agent11-rwe-followup.py'
spec=importlib.util.spec_from_file_location('modem_rwe_followup',str(SCRIPT))
mod=importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(mod)

class ExceptionFollowup(unittest.TestCase):
    def test_disjoint_bounds_and_no_implicit_vendor_fill(self):
        self.assertEqual((mod.FILE_START,mod.FILE_END),(0x39920,0x39954))
        self.assertEqual(mod.FILE_END-mod.FILE_START,52)
        self.assertGreater(mod.FILE_START,0x390dc)
        self.assertEqual(mod.FILE_START-0x390dc,0x844)

    def test_pinned_revision_fails_closed(self):
        with self.assertRaisesRegex(ValueError,'Wrong modem source revision'):
            mod.verify(bytes(mod.FILE_END),bytes(mod.FILE_END-mod.FILE_START))

    def test_source_assembler_and_optional_byte_reference(self):
        mc=os.environ.get('AGENT11_LLVM_MC')
        if not mc:self.skipTest('Set AGENT11_LLVM_MC for Hexagon source assembly test')
        source=SCRIPT.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_followup.s'
        data=mod.assemble(Path(mc),source)
        self.assertEqual(len(data),52)
        word=lambda off:int.from_bytes(data[off-mod.FILE_START:off-mod.FILE_START+4],'little')
        self.assertEqual(word(0x39920),0x9200c012) # memw_locked(r0)
        self.assertEqual(word(0x39930),0xa0a0d200) # checked memw_locked(r0,p0)=r18
        self.assertEqual(word(0x39944),0x529f4000) # packet-local jumpr lr
        self.assertEqual(word(0x3994c),0x529f4000) # failure-path return
        ref=os.environ.get('AGENT11_MODEM_MBN')
        if ref:mod.verify(Path(ref).read_bytes(),data)

if __name__=='__main__':unittest.main()
