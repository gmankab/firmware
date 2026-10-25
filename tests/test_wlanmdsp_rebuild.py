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

    def test_loader_hooks_independent_source_and_assembler_bytes(self):
        asm = fw.HOOK_SOURCE.read_text()
        for forbidden in (".incbin", ".word", ".byte", ".long", ".quad"):
            self.assertNotIn(forbidden, asm)
        self.assertIn("memw(##2959748528) = r0", asm)
        result = fw.assemble(fw.HOOK_SOURCE, self.mc,
                             expected_length=fw.LOADER_HOOKS_LENGTH)
        self.assertEqual(len(result), 100)
        self.assertEqual(result[:8], bytes.fromhex("5257030000d0496a"))
        self.assertEqual(result[0x24:0x40], bytes.fromhex("00c0007f")*7)
        self.assertEqual(result[-4:], bytes.fromhex("00c09f52"))
        for revision in ("hexagonv62",):
            self.assertEqual(result, fw.assemble(fw.HOOK_SOURCE, self.mc,
                cpu=revision, expected_length=fw.LOADER_HOOKS_LENGTH))

    def test_combined_loader_hooks_are_bit_identical_when_reference_supplied(self):
        startup = fw.assemble(fw.SOURCE, self.mc)
        prefix = fw.emit_with_plt(startup, self.mc, "hexagonv65")
        complete = fw.emit_with_loader_hooks(prefix, self.mc, "hexagonv65")
        self.assertEqual(len(complete), fw.THROUGH_LOADER_HOOKS_LENGTH)
        self.assertEqual(complete[:len(prefix)], prefix)
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            fw.check_reference(complete, Path(ref).read_bytes())

    def test_configuration_table_lookup_reconstructs_exact_packets(self):
        src = fw.LOOKUP_SOURCE.read_text()
        for token in (".incbin", ".word", ".byte", ".long", ".quad"):
            self.assertNotIn(token, src)
        self.assertIn("r6 = mux(p0,r4,##1183); if (p0)", src)
        self.assertIn("memw(r3+#16) = r2.new", src)
        output = fw.assemble(fw.LOOKUP_SOURCE, self.mc,
                             expected_length=fw.CONFIG_LOOKUP_LENGTH)
        self.assertEqual(len(output), 156)
        self.assertEqual(output[8:16], bytes.fromhex("12400000e6430473"))
        for cpu in ("hexagonv62",):
            self.assertEqual(output, fw.assemble(fw.LOOKUP_SOURCE, self.mc, cpu,
                                 expected_length=fw.CONFIG_LOOKUP_LENGTH))
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            prefix = fw.emit_with_plt(fw.assemble(fw.SOURCE, self.mc), self.mc, "hexagonv65")
            hooks = fw.emit_with_loader_hooks(prefix, self.mc, "hexagonv65")
            image = fw.emit_with_config_lookup(hooks, self.mc, "hexagonv65")
            self.assertEqual(len(image), fw.THROUGH_CONFIG_LOOKUP_LENGTH)
            fw.check_reference(image, Path(ref).read_bytes())

    def test_record_consumer_entry_is_explicit_partial_function(self):
        src = fw.RECORD_PREFIX_SOURCE.read_text()
        self.assertIn("verified function ENTRY ONLY", src)
        for token in (".incbin", ".word", ".byte", ".long", ".quad"):
            self.assertNotIn(token, src)
        self.assertIn("if (p0.new) jump:nt fail_exit", src)
        encoded = fw.assemble(fw.RECORD_PREFIX_SOURCE, self.mc,
                              expected_length=fw.RECORD_CONSUMER_PREFIX_LENGTH)
        self.assertEqual(len(encoded), 148)
        self.assertEqual(encoded[:8], bytes.fromhex("505d545a0ec09da0"))
        self.assertEqual(encoded[-16:],bytes.fromhex("1140007816400078035a9da105d49da1"))
        for cpu in ("hexagonv62",):
            self.assertEqual(encoded, fw.assemble(fw.RECORD_PREFIX_SOURCE, self.mc, cpu,
                             expected_length=fw.RECORD_CONSUMER_PREFIX_LENGTH))
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            startup = fw.assemble(fw.SOURCE, self.mc)
            plt = fw.emit_with_plt(startup, self.mc,"hexagonv65")
            hooks = fw.emit_with_loader_hooks(plt, self.mc,"hexagonv65")
            lookup = fw.emit_with_config_lookup(hooks, self.mc,"hexagonv65")
            full = fw.emit_with_record_prefix(lookup, self.mc,"hexagonv65")
            self.assertEqual(len(full), fw.THROUGH_RECORD_PREFIX_LENGTH)
            fw.check_reference(full, Path(ref).read_bytes())

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
