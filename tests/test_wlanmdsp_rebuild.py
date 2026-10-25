# SPDX-License-Identifier: GPL-3.0-only
"""Independent-source WLAN DSP startup and safe Hexagon ELF checks."""
import importlib.util
import os
import shutil
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOOL = ROOT / "tools/build-wlanmdsp-startup.py"
SPEC = importlib.util.spec_from_file_location("build_wlanmdsp_startup", TOOL)
assert SPEC is not None and SPEC.loader is not None
fw = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = fw
SPEC.loader.exec_module(fw)


class WlanElfHeaderTests(unittest.TestCase):
    def test_source_is_an_implementation_not_embedded_opcode_words(self):
        body = fw.SOURCE.read_text()
        for forbidden in (".incbin", ".word", ".byte", ".long", ".quad"):
            self.assertNotIn(forbidden, body)
        self.assertIn("clear_runtime_data:", body)
        self.assertIn("wlan_init:", body)
        self.assertIn("jump first_byte_loop; memb", body)

    def test_requires_elf32_hexagon_object(self):
        with self.assertRaisesRegex(fw.FragmentError, "ELF32"):
            fw.read_resolved_text(b"not an ELF")
        fake = bytearray(52)
        fake[:6] = b"\x7fELF\x01\x01"
        with self.assertRaisesRegex(fw.FragmentError, "Hexagon"):
            fw.elf32_sections(bytes(fake))

    def test_bad_section_table_rejected(self):
        fake = bytearray(52)
        fake[:6] = b"\x7fELF\x01\x01"
        fake[18:20] = (164).to_bytes(2, "little")
        with self.assertRaisesRegex(fw.FragmentError, "section-header"):
            fw.read_resolved_text(bytes(fake))

    def test_wrong_reference_revision_rejected(self):
        with self.assertRaisesRegex(fw.FragmentError, "revision"):
            fw.check_reference(b"", b"wrong MBN bytes")


class WlanHexagonAssemblyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc = os.environ.get("WLAN_LLVM_MC") or shutil.which("llvm-mc")
        if not cls.mc:
            raise unittest.SkipTest("Hexagon-capable llvm-mc is not installed")

    def test_rebuilt_text_has_no_unresolved_relocations(self):
        text = fw.assemble(fw.SOURCE, self.mc)
        self.assertEqual(len(text), fw.TEXT_LENGTH)
        self.assertEqual(text[:4], bytes.fromhex("af6a0700"))
        # Verify packet ordering: the memory-clear loops use a backward jump
        # and a store in one packet, not two serial packets.
        self.assertEqual(text[0xc4:0xc8], bytes.fromhex("fe7fff59"))
        self.assertEqual(text[0x118:0x120], bytes.fromhex("0240007801280328"))

    def test_source_encodes_identically_for_supported_llvm6_cpus(self):
        baseline = fw.assemble(fw.SOURCE, self.mc, "hexagonv65")
        for revision in ("hexagonv62",):
            self.assertEqual(baseline, fw.assemble(fw.SOURCE, self.mc, revision))

    def test_unsupported_v60_framelimit_is_reported(self):
        # Decoding v60-compatible instructions does not imply every original
        # instruction can be assembled for v60: frame-limit setup needs v62+.
        with self.assertRaises(fw.FragmentError):
            fw.assemble(fw.SOURCE, self.mc, "hexagonv60")

    def test_plt_has_linker_derived_got_slots(self):
        asm = fw.plt_assembly()
        self.assertEqual(asm.count('wlan_plt_'), 181)
        self.assertIn('r14 = add(pc,##3403768)', asm)
        self.assertIn('.space 20,0', asm)
        self.assertEqual(fw.PLT_FIRST_ENTRY + fw.PLT_COUNT * fw.PLT_ENTRY_BYTES,
                         fw.CODE_AND_PLT_LENGTH)

    def test_complete_first_region_rebuild_is_deterministic(self):
        boot = fw.assemble(fw.SOURCE, self.mc)
        all_bytes = fw.emit_with_plt(boot, self.mc, 'hexagonv65')
        self.assertEqual(len(all_bytes), 0x1b80)
        pad = all_bytes[0x154:0x1000]
        self.assertEqual(pad,
                         fw.LINKER_GAP_WORD.to_bytes(4, 'little') * 939)
        self.assertEqual(all_bytes[0x101c:0x1030], bytes(20))
        self.assertEqual(all_bytes[0x1030:0x1040],
                         fw.assemble_plt(self.mc, 'hexagonv65')[0x30:0x40])
        ref = os.environ.get('WLANMDSP_REFERENCE')
        if ref:
            fw.check_reference(all_bytes, Path(ref).read_bytes())

    def test_unresolved_reference_is_rejected(self):
        with tempfile.TemporaryDirectory() as d:
            bad = Path(d) / "unresolved.S"
            bad.write_text(".text\n.globl entry\nentry:\n{ call absent_symbol }\n")
            with self.assertRaises(fw.FragmentError):
                fw.assemble(bad, self.mc)

    def test_matches_exact_reference_when_supplied(self):
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if not ref:
            self.skipTest("set WLANMDSP_REFERENCE to the manifest-matching private OEM MBN")
        text = fw.assemble(fw.SOURCE, self.mc)
        fw.check_reference(text, Path(ref).read_bytes())


if __name__ == "__main__":
    unittest.main()
