# SPDX-License-Identifier: GPL-3.0-only
"""37 codec selector function pointers are metadata, not Hexagon opcodes."""
from __future__ import annotations
import importlib.util,os,struct,unittest
from pathlib import Path
T=Path(__file__).resolve().parents[1]/'tools/adsp'
def imp(name):
    spec=importlib.util.spec_from_file_location(name.replace('-','_').replace('.','_'),T/name)
    assert spec and spec.loader
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
m=imp('rwe-next-controller-format-handler-table.py')
code=imp('rwe-next-controller-format-selector-reports.py')
class HandlerTable(unittest.TestCase):
    def test_layout_address_mapping_and_duplicates(self):
        self.assertEqual((m.START,m.END,m.SIZE),(0x129bc,0x12a50,148))
        self.assertEqual(code.END,m.START)
        self.assertEqual(len(m.FORMATS_TO_HANDLER),37)
        self.assertEqual(m.FORMATS_TO_HANDLER.count(0x12828),5)
        self.assertEqual(m.FORMATS_TO_HANDLER.count(0x1297c),5)
        self.assertEqual(m.virtual_handler(0x126dc),0xf000f6dc)
        self.assertEqual(m.virtual_handler(0x129ac),0xf000f9ac)
        self.assertEqual(m.render()[:4],struct.pack('<I',0xf000f6dc))
        self.assertEqual(m.render()[-4:],struct.pack('<I',0xf000f9ac))
        self.assertEqual(len(m.render()),148)
        for x in (0x129bc,0x126aa,0x6000,-1,0xffffffff):
            with self.assertRaises(ValueError):m.virtual_handler(x)
    def test_reference_exact_data_not_code(self):
        ref=os.getenv('NABU_ADSP_REFERENCE')
        if not ref:self.skipTest('Pinned original ADSP unavailable')
        original=Path(ref).read_bytes()
        m.verify(original,m.render())
        self.assertEqual(original[code.START:m.END], original[code.START:code.END]+m.render())
    def test_reject_wrong_revision_and_corrupt_pointer(self):
        ref=os.getenv('NABU_ADSP_REFERENCE')
        if not ref:self.skipTest('Pinned original ADSP unavailable')
        original=Path(ref).read_bytes();data=m.render()
        damaged=bytearray(original);damaged[11]^=1
        with self.assertRaisesRegex(ValueError,'revision'):m.verify(damaged,data)
        bad=bytearray(data);bad[4]^=1
        with self.assertRaisesRegex(ValueError,'differ'):m.verify(original,bad)
if __name__=='__main__':unittest.main()
