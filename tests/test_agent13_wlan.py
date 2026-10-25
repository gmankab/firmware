# SPDX-License-Identifier: GPL-3.0-only
"""Agent13 WLAN source structural/coverage tests; private binaries optional."""
import importlib.util
import os
from pathlib import Path
import tempfile
import unittest

BASE = Path(__file__).resolve().parents[1]
SCRIPT = BASE / "tools/build-wlanmdsp-agent13.py"
spec = importlib.util.spec_from_file_location("wlan_agent13", SCRIPT)
assert spec is not None and spec.loader is not None
builder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(builder)


class Agent13WlanTests(unittest.TestCase):
    def test_starts_after_agent07_and_ends_at_full_function_return(self):
        self.assertEqual(builder.START, 0x2708)
        self.assertEqual(builder.validate_fragment_layout(), 0x27a0)
        self.assertEqual(sum(length for _, _, _, length in builder.FRAGMENTS), 152)
        self.assertEqual(builder.FILE_OFFSET + builder.START, 0x42708)

    def test_does_not_embed_opaque_instructions(self):
        for _, source, _, _ in builder.FRAGMENTS:
            data = source.read_text()
            for prohibited in (".incbin", ".byte", ".word", ".long", ".quad", ".fill"):
                self.assertNotIn(prohibited, data)
            self.assertIn("loop0(array_entry,#7)", data)
            self.assertIn(":endloop0", data)
            self.assertIn("dealloc_return", data)

    def test_explicit_gap_rejected(self):
        old = builder.FRAGMENTS[0]
        with self.assertRaisesRegex(ValueError, "not adjacent"):
            builder.validate_fragment_layout(((old[0],old[1],old[2]+4,old[3]),))

    def test_mismatched_reference_revision_rejected(self):
        parent=builder.load_parent()
        with self.assertRaisesRegex(parent.FragmentError, "wrong WLAN firmware revision"):
            parent.check_reference(bytes(152), bytes(0x40000+152), runtime_offset=builder.START)

    @unittest.skipUnless(os.getenv("HEXAGON_LLVM22"), "set HEXAGON_LLVM22 to run real packet-roundtrip")
    def test_optional_independent_assembler(self):
        parent=builder.load_parent()
        program=parent.assemble(builder.FRAGMENTS[0][1],os.environ["HEXAGON_LLVM22"],"hexagonv66",expected_length=152)
        self.assertEqual(len(program),152)
        firmware=os.getenv("NABU_WLAN_REFERENCE")
        if firmware:
            parent.check_reference(program,Path(firmware).read_bytes(),runtime_offset=builder.START)


if __name__=="__main__":unittest.main()
