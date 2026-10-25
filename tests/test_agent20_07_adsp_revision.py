# SPDX-License-Identifier: GPL-3.0-only
"""10-word source-generated ADSP V66 revision guard [0x65f4,0x661c)."""
import importlib.util
import os
from pathlib import Path
import struct
import unittest
from unittest.mock import patch
import sys
T=Path(__file__).resolve().parents[1]/'tools'
sys.path.insert(0,str(T))
spec=importlib.util.spec_from_file_location('adsp07',T/'adsp-agent20-rwe.py')
assert spec and spec.loader
module=importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

class RevisionGuardTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc=os.environ.get('HEXAGON_LLVM_MC')
        cls.ref=os.environ.get('NABU_ADSP_REFERENCE')

    def test_revision_guard_bounds(self):
        self.assertEqual(module.REVISION_GUARD_SIZE,40)
        self.assertEqual(module.REVISION_GUARD_END,0x661c)
        self.assertGreaterEqual(module.SIZE,1516)

    def test_revision_register_encoder(self):
        self.assertEqual(module.read_system_register(0,module.SystemRegister.REV),0x6e9dc000)

    def test_revision_guard_matches_pinned_reference(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC not set')
        b=module.revision_guard(self.mc,module.baseline())
        self.assertEqual(len(b),40)
        self.assertEqual(struct.unpack_from('<I',b,0)[0],0x529fc000)
        self.assertEqual(struct.unpack_from('<I',b,4)[0],0x6e9dc000)
        if self.ref:
            ref=Path(self.ref).read_bytes()
            self.assertEqual(ref[0x65f4:0x661c],b)
            module.verify(ref,module.render(self.mc))

    def test_marker_must_be_exact(self):
        if not self.mc:self.skipTest('HEXAGON_LLVM_MC not set')
        o=module.baseline()
        data=bytearray(o.assemble(self.mc,module.REVISION_GUARD_SOURCE,40))
        data[4:8]=struct.pack('<I',0x7f00c001)
        fake=type('Fake',(),{'assemble':staticmethod(lambda *args:bytes(data)),
                             'AdspError':ValueError})()
        with self.assertRaisesRegex(ValueError,'sentinel moved'):
            module.revision_guard(self.mc,fake)

if __name__=='__main__':unittest.main()
