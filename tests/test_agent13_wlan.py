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
        self.assertEqual(builder.validate_fragment_layout(), 0x3af4)
        self.assertEqual(sum(length for _, _, _, length in builder.FRAGMENTS), 5100)
        self.assertEqual(builder.FILE_OFFSET + builder.START, 0x42708)

    def test_does_not_embed_opaque_instructions(self):
        for _, source, _, _ in builder.FRAGMENTS:
            data = source.read_text()
            for prohibited in (".incbin", ".byte", ".word", ".long", ".quad", ".fill"):
                self.assertNotIn(prohibited, data)
            self.assertIn(".text", data)
        first=builder.FRAGMENTS[0][1].read_text()
        second=builder.FRAGMENTS[1][1].read_text()
        self.assertIn("loop0(array_entry,#7)",first)
        self.assertIn(":endloop0",first)
        self.assertIn("dealloc_return",first)
        self.assertIn("extractu",second)
        self.assertIn("type_six:",second)
        self.assertIn("record_walk:",builder.FRAGMENTS[2][1].read_text())
        self.assertIn("deallocframe",builder.FRAGMENTS[3][1].read_text())
        self.assertIn("deallocframe",builder.FRAGMENTS[4][1].read_text())
        self.assertIn("jumpr r4",builder.FRAGMENTS[5][1].read_text())
        self.assertIn("dealloc_return",builder.FRAGMENTS[10][1].read_text())
        self.assertIn(":mem_noshuf",builder.FRAGMENTS[13][1].read_text())
        self.assertIn(":endloop0",builder.FRAGMENTS[13][1].read_text())
        guard={name:source for name,source,_,_ in builder.FRAGMENTS}
        self.assertIn("dealloc_return",guard["record-readiness"].read_text())
        self.assertIn("dealloc_return",guard["record-callback-guard"].read_text())
        process=guard["incoming-record-processing"].read_text()
        self.assertIn("agent13_wlan_record_processing:",process)
        self.assertIn("r2 = #0x1; jump ##0x2a24c0",process)
        self.assertIn("if (p0.new) jump:nt",process)
        self.assertIn("memb(r16+#0x4c)",process)
        self.assertNotIn("allocframe(#0x10)",process)  # next function starts at 0x3410
        create=guard["record-context-allocation"].read_text()
        self.assertIn("r0 = r16; jump ##0x2a23b4",create)
        init=guard["record-state-init"].read_text()
        self.assertIn(":mem_noshuf",init)
        self.assertIn("r0 = r16; jump ##0x2a226c",init)
        controller=guard["record-state-controller"].read_text()
        self.assertIn("loop0(",controller)
        self.assertIn(":endloop0",controller)
        self.assertIn("popcount",controller)
        self.assertIn("jump -0x83c",controller)
        stream=guard["stream-record-control"].read_text()
        self.assertIn("r0 = #0x0; jump ##0x2a1fe8",stream)
        self.assertIn("r1 = r21",stream)
        self.assertIn("dealloc_return",guard["bounded-record-counter"].read_text())
        self.assertIn("record-query",str(guard["record-query-dispatch"]))

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
        program=b"".join(parent.assemble(source,os.environ["HEXAGON_LLVM22"],"hexagonv66",expected_length=length) for _,source,_,length in builder.FRAGMENTS)
        self.assertEqual(len(program),5100)
        firmware=os.getenv("NABU_WLAN_REFERENCE")
        if firmware:
            parent.check_reference(program,Path(firmware).read_bytes(),runtime_offset=builder.START)


if __name__=="__main__":unittest.main()
