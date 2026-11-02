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
TOOL = ROOT / "tools/wlanmdsp/build-wlanmdsp-startup.py"
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
        self.assertIn("verified INITIAL PATH ONLY", src)
        for token in (".incbin", ".word", ".byte", ".long", ".quad"):
            self.assertNotIn(token, src)
        self.assertIn("if (p0.new) jump:nt fail_exit", src)
        encoded = fw.assemble(fw.RECORD_PREFIX_SOURCE, self.mc,
                              expected_length=fw.RECORD_CONSUMER_PREFIX_LENGTH)
        self.assertEqual(len(encoded), 300)
        self.assertEqual(encoded[:8], bytes.fromhex("505d545a0ec09da0"))
        self.assertEqual(encoded[-4:],bytes.fromhex("04c1005a"))
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

    def test_disconnected_import_aliases_are_typed_and_verified_at_own_offset(self):
        source = fw.IMPORT_ALIASES_SOURCE.read_text()
        self.assertIn("disjoint executable interval", source)
        for directive in (".incbin", ".byte", ".word", ".long", ".quad"):
            self.assertNotIn(directive, source)
        aliases = fw.assemble(fw.IMPORT_ALIASES_SOURCE, self.mc,
                              expected_length=fw.IMPORT_ALIASES_LENGTH)
        self.assertEqual(len(aliases), 24)
        self.assertEqual(aliases[:4],bytes.fromhex("c37fff0f"))
        self.assertEqual(aliases[-4:],bytes.fromhex("30c00058"))
        for cpu in ("hexagonv62",):
            self.assertEqual(aliases, fw.assemble(fw.IMPORT_ALIASES_SOURCE,
                self.mc, cpu=cpu, expected_length=fw.IMPORT_ALIASES_LENGTH))
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            fw.check_reference(aliases, Path(ref).read_bytes(),
                               runtime_offset=fw.IMPORT_ALIASES_OFFSET)
        if ref:
            with self.assertRaisesRegex(fw.FragmentError,"negative"):
                fw.check_reference(b"", Path(ref).read_bytes(), runtime_offset=-1)

    def test_postaliases_retry_branch_and_fieldwise_record_comparator(self):
        source = fw.POSTALIASES_COMPARE_SOURCE.read_text()
        for directive in (".incbin", ".byte", ".word", ".long", ".quad"):
            self.assertNotIn(directive, source)
        self.assertIn("jump ##-832", source)
        self.assertIn("r2 = r0; r0 = #0", source)
        self.assertIn("if (!cmp.eq(r4.new,r3))", source)
        encoded = fw.assemble(fw.POSTALIASES_COMPARE_SOURCE, self.mc,
                              expected_length=fw.POSTALIASES_COMPARE_LENGTH)
        self.assertEqual(len(encoded), 80)
        self.assertEqual(encoded[12:20], bytes.fromhex("0040007814020250"))
        self.assertEqual(encoded[-4:], bytes.fromhex("00c09f52"))
        for cpu in ("hexagonv62",):
            self.assertEqual(encoded, fw.assemble(fw.POSTALIASES_COMPARE_SOURCE,
                             self.mc, cpu=cpu,
                             expected_length=fw.POSTALIASES_COMPARE_LENGTH))
        together = fw.assemble_import_aliases_through_compare(self.mc, "hexagonv65")
        self.assertEqual(len(together), fw.THROUGH_POSTALIASES_COMPARE_LENGTH)
        self.assertEqual(together[:fw.IMPORT_ALIASES_LENGTH],
                         fw.assemble(fw.IMPORT_ALIASES_SOURCE, self.mc,
                            expected_length=fw.IMPORT_ALIASES_LENGTH))
        self.assertEqual(together[-80:], encoded)
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            fw.check_reference(together, Path(ref).read_bytes(),
                               runtime_offset=fw.IMPORT_ALIASES_OFFSET)

    def test_hexagon_hardware_loop_array_comparator_matches_reference(self):
        src = fw.ARRAY_COMPARE_SOURCE.read_text()
        self.assertIn("loop1(inner_first,r12)", src)
        self.assertIn("loop0(inner_iteration,r0)", src)
        self.assertIn(":endloop0", src)
        self.assertIn(":endloop1", src)
        for opcode_blob in (".incbin", ".word", ".byte", ".long", ".quad"):
            self.assertNotIn(opcode_blob, src)
        rebuilt = fw.assemble(fw.ARRAY_COMPARE_SOURCE, self.mc,
                              expected_length=fw.ARRAY_COMPARE_LENGTH)
        self.assertEqual(len(rebuilt), 296)
        self.assertEqual(rebuilt[0x6c:0x70], bytes.fromhex("08c02c60"))
        self.assertEqual(rebuilt[0x7c:0x80], bytes.fromhex("08c00060"))
        for cpu in ("hexagonv62",):
            self.assertEqual(rebuilt, fw.assemble(fw.ARRAY_COMPARE_SOURCE, self.mc,
                cpu=cpu, expected_length=fw.ARRAY_COMPARE_LENGTH))
        joined = fw.assemble_import_aliases_through_array_compare(self.mc, "hexagonv65")
        self.assertEqual(len(joined), fw.THROUGH_ARRAY_COMPARE_LENGTH)
        self.assertEqual(joined[-296:], rebuilt)
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            fw.check_reference(joined, Path(ref).read_bytes(),
                               runtime_offset=fw.IMPORT_ALIASES_OFFSET)

    def test_postalias_record_wrapper_and_runtime_guards(self):
        pieces = [
            (fw.RECORD_WRAPPER_SOURCE, fw.RECORD_WRAPPER_LENGTH),
            (fw.IRQ_STATE_GUARD_SOURCE, fw.IRQ_STATE_GUARD_LENGTH),
            (fw.RUNTIME_TABLE_INIT_SOURCE, fw.RUNTIME_TABLE_INIT_LENGTH),
        ]
        for src, length in pieces:
            body = src.read_text()
            for forbidden in (".incbin", ".byte", ".word", ".long", ".quad"):
                self.assertNotIn(forbidden, body)
            encoded = fw.assemble(src, self.mc, "hexagonv65", length)
            self.assertEqual(len(encoded), length)
        self.assertIn(":mem_noshuf", fw.RUNTIME_TABLE_INIT_SOURCE.read_text())
        # LLVM6 only accepts this no-memory-shuffle flag on Hexagon v65.
        with self.assertRaises(fw.FragmentError):
            fw.assemble(fw.RUNTIME_TABLE_INIT_SOURCE, self.mc,
                        "hexagonv62", fw.RUNTIME_TABLE_INIT_LENGTH)
        for src, length in pieces[:2]:
            self.assertEqual(fw.assemble(src, self.mc, "hexagonv65", length),
                             fw.assemble(src, self.mc, "hexagonv62", length))
        joined = fw.assemble_import_aliases_through_runtime_helpers(self.mc,
                                                                      "hexagonv65")
        self.assertEqual(len(joined), fw.THROUGH_RUNTIME_HELPERS_LENGTH)
        self.assertEqual(joined[fw.THROUGH_ARRAY_COMPARE_LENGTH:fw.THROUGH_ARRAY_COMPARE_LENGTH + 8],
                         bytes.fromhex("185b545a03c09da0"))
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            fw.check_reference(joined, Path(ref).read_bytes(),
                               runtime_offset=fw.IMPORT_ALIASES_OFFSET)

    def test_hexagonv66_record_consumer_gap_and_full_contiguous_prefix(self):
        source = fw.RECORD_CONSUMER_BODY_SOURCE.read_text()
        for directive in (".incbin", ".byte", ".word", ".long", ".quad"):
            self.assertNotIn(directive, source)
        self.assertIn("r20 -= mpyi(r17,r16)", source)
        self.assertIn("memw(r6+r4<<#0) = r2", source)
        self.assertIn("jump write_block", source)
        mc_v66 = os.environ.get("WLAN_LLVM_MC_V66")
        if not mc_v66:
            self.skipTest("set WLAN_LLVM_MC_V66 to LLVM22 Hexagon v66 assembler")
        v66 = fw.assemble(fw.RECORD_CONSUMER_BODY_SOURCE, mc_v66,
                          "hexagonv66", fw.RECORD_CONSUMER_BODY_LENGTH)
        self.assertEqual(len(v66), 508)
        # The v66 multiply/subtract instruction is precisely where LLVM6
        # decoding of the exact firmware stopped in this function.
        self.assertEqual(v66[0x34:0x38], bytes.fromhex("145091ef"))
        self.assertEqual(fw.THROUGH_COMPLETE_RECORD_CONSUMER_LENGTH, 0x1fa8)
        prefix = fw.assemble(fw.SOURCE, self.mc)
        prefix = fw.emit_with_plt(prefix, self.mc, "hexagonv65")
        prefix = fw.emit_with_loader_hooks(prefix, self.mc, "hexagonv65")
        prefix = fw.emit_with_config_lookup(prefix, self.mc, "hexagonv65")
        prefix = fw.emit_with_record_prefix(prefix, self.mc, "hexagonv65")
        contiguous = fw.emit_with_complete_record_consumer(prefix, mc_v66)
        self.assertEqual(contiguous[-508:], v66)
        self.assertEqual(len(contiguous), 0x1fa8)
        complete = fw.emit_with_full_runtime_helpers(contiguous, self.mc,
                                                     "hexagonv65")
        self.assertEqual(len(complete), 0x2208)
        self.assertEqual(complete[0x1fa8:0x1fa8+24],
                         fw.assemble(fw.IMPORT_ALIASES_SOURCE, self.mc,
                                     expected_length=fw.IMPORT_ALIASES_LENGTH))
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            fw.check_reference(complete, Path(ref).read_bytes())

    def test_hexagonv66_runtime_format_handler_and_extended_prefix(self):
        mc_v66 = os.environ.get("WLAN_LLVM_MC_V66")
        if not mc_v66:
            self.skipTest("set WLAN_LLVM_MC_V66 for Hexagon v66 assembler")
        asm = fw.RUNTIME_FORMAT_HANDLER_SOURCE.read_text()
        for forbidden in (".incbin", ".word", ".byte", ".long", ".quad"):
            self.assertNotIn(forbidden, asm)
        self.assertIn("r3 = memw(r18=##0xb06a2dc4)", asm)
        self.assertIn("jump ##-0x1270", asm)
        self.assertEqual(asm.count("{ nop }"), 1)
        rebuilt = fw.assemble(fw.RUNTIME_FORMAT_HANDLER_SOURCE, mc_v66,
                              "hexagonv66", fw.RUNTIME_FORMAT_HANDLER_LENGTH)
        self.assertEqual(len(rebuilt), 0x138)
        self.assertEqual(rebuilt[-24:], bytes.fromhex("00c0007f") * 6)
        self.assertEqual(rebuilt[:8], bytes.fromhex("b85a545a04c09da0"))
        self.assertEqual(fw.THROUGH_RUNTIME_FORMAT_HANDLER_LENGTH, 0x2340)
        with self.assertRaisesRegex(fw.FragmentError, "prefix through 0x2208"):
            fw.emit_with_runtime_format_handler(b"too short", mc_v66)
        if os.environ.get("WLANMDSP_REFERENCE"):
            fw.check_reference(rebuilt,
                               Path(os.environ["WLANMDSP_REFERENCE"]).read_bytes(),
                               runtime_offset=0x2208)

    def test_runtime_format_handler_full_prefix_cli_manifest(self):
        mc_v66 = os.environ.get("WLAN_LLVM_MC_V66")
        if not mc_v66:
            self.skipTest("set WLAN_LLVM_MC_V66 for Hexagon v66 assembler")
        import subprocess, json
        with tempfile.TemporaryDirectory() as work:
            output, manifest = Path(work) / "wlan-through-format.bin", Path(work) / "format.json"
            cmd = [sys.executable, str(fw.ROOT / "tools/wlanmdsp/build-wlanmdsp-startup.py"),
                   str(output), "--with-runtime-format-handler",
                   "--llvm-mc", self.mc, "--llvm-mc-v66", mc_v66,
                   "--manifest", str(manifest)]
            ref = os.environ.get("WLANMDSP_REFERENCE")
            if ref:
                cmd.extend(["--reference", ref])
            subprocess.run(cmd, check=True, capture_output=True, text=True)
            self.assertEqual(output.stat().st_size, 0x2340)
            self.assertEqual(output.read_bytes()[-24:], bytes.fromhex("00c0007f")*6)
            m = json.loads(manifest.read_text())
            self.assertEqual(m["independently_encoded_runtime_format_handler_bytes"], 312)
            self.assertEqual(m["linker_layout_fill_bytes"], 3776)
            self.assertFalse(m["complete_firmware"])
            if ref:
                fw.check_reference(output.read_bytes(), Path(ref).read_bytes())

    def test_hexagonv66_runtime_session_init_and_contiguous_layout(self):
        mc_v66 = os.environ.get("WLAN_LLVM_MC_V66")
        if not mc_v66:
            self.skipTest("set WLAN_LLVM_MC_V66 for Hexagon v66 assembler")
        source = fw.RUNTIME_SESSION_INIT_SOURCE.read_text()
        for marker in (".incbin", ".word", ".byte", ".long", ".quad"):
            self.assertNotIn(marker, source)
        self.assertIn(":mem_noshuf", source)
        self.assertIn("loop0(seed_step,#2)", source)
        self.assertIn("memb(r9++#2)", source)
        encoded = fw.assemble(fw.RUNTIME_SESSION_INIT_SOURCE, mc_v66,
                              "hexagonv66", fw.RUNTIME_SESSION_INIT_LENGTH)
        self.assertEqual(len(encoded), 340)
        self.assertEqual(encoded[0xb0:0xb8], bytes.fromhex("ff60093c29c02691"))
        self.assertEqual(fw.THROUGH_RUNTIME_SESSION_INIT_LENGTH, 0x2494)
        with self.assertRaisesRegex(fw.FragmentError,"prefix through 0x2340"):
            fw.emit_with_runtime_session_init(b"short", mc_v66)
        ref = os.environ.get("WLANMDSP_REFERENCE")
        if ref:
            fw.check_reference(encoded, Path(ref).read_bytes(), runtime_offset=0x2340)

    def test_runtime_session_init_full_prefix_cli_manifest(self):
        mc_v66 = os.environ.get("WLAN_LLVM_MC_V66")
        if not mc_v66:
            self.skipTest("set WLAN_LLVM_MC_V66 for Hexagon v66 assembler")
        import json, subprocess
        with tempfile.TemporaryDirectory() as d:
            output,manifest=Path(d)/"wlan-9364.bin",Path(d)/"manifest.json"
            cmd=[sys.executable,str(fw.ROOT/"tools/wlanmdsp/build-wlanmdsp-startup.py"),
                 str(output),"--with-runtime-session-init","--llvm-mc",self.mc,
                 "--llvm-mc-v66",mc_v66,"--manifest",str(manifest)]
            reference=os.environ.get("WLANMDSP_REFERENCE")
            if reference:cmd += ["--reference",reference]
            subprocess.run(cmd,check=True,capture_output=True,text=True)
            self.assertEqual(output.stat().st_size,0x2494)
            info=json.loads(manifest.read_text())
            self.assertEqual(info["independently_encoded_runtime_session_init_bytes"],340)
            self.assertEqual(info["linker_layout_fill_bytes"],3776)
            self.assertFalse(info["complete_firmware"])
            if reference:fw.check_reference(output.read_bytes(),Path(reference).read_bytes())

    def test_hexagon_v66_reporting_and_tail_trampolines(self):
        v66 = os.environ.get("WLAN_LLVM_MC_V66")
        if not v66:
            self.skipTest("set WLAN_LLVM_MC_V66 for Hexagon v66 assembler")
        asm = fw.RUNTIME_REPORTING_SOURCE.read_text()
        for forb in (".incbin", ".byte", ".word", ".long", ".quad"):
            self.assertNotIn(forb,asm)
        self.assertIn("{ r16 = r1; r0 = ##48; memw(r29+#0) = r0 }",asm)
        self.assertIn("memh(r19+#0) += #1",asm)
        self.assertIn("jump 0x1d5b04",asm)
        encoded=fw.assemble(fw.RUNTIME_REPORTING_SOURCE,v66,"hexagonv66",fw.RUNTIME_REPORTING_LENGTH)
        self.assertEqual(len(encoded),268)
        self.assertEqual(encoded[0x18:0x24],bytes.fromhex("004000000046007800281870"))
        reference=os.environ.get("WLANMDSP_REFERENCE")
        if reference:
            fw.check_reference(encoded,Path(reference).read_bytes(),runtime_offset=0x2494)
        with self.assertRaisesRegex(fw.FragmentError,"prefix through 0x2494"):
            fw.emit_with_runtime_reporting(b"incomplete",v66)
        self.assertEqual(fw.THROUGH_RUNTIME_REPORTING_LENGTH,0x25a0)

    def test_wlan_reporting_contiguous_build_cli(self):
        v66=os.environ.get("WLAN_LLVM_MC_V66")
        if not v66:
            self.skipTest("set WLAN_LLVM_MC_V66 for Hexagon v66 assembler")
        import json, subprocess
        with tempfile.TemporaryDirectory() as work:
            artifact=Path(work)/"wlan-9632.bin"
            manifest=Path(work)/"wlan-report.json"
            cmd=[sys.executable,str(fw.ROOT/"tools/wlanmdsp/build-wlanmdsp-startup.py"),
                 str(artifact),"--with-runtime-reporting", "--llvm-mc",self.mc,
                 "--llvm-mc-v66",v66,"--manifest",str(manifest)]
            ref=os.environ.get("WLANMDSP_REFERENCE")
            if ref:cmd += ["--reference",ref]
            subprocess.run(cmd,check=True,capture_output=True,text=True)
            self.assertEqual(artifact.stat().st_size,9632)
            m=json.loads(manifest.read_text())
            self.assertEqual(m["independently_encoded_runtime_reporting_bytes"],268)
            self.assertEqual(m["linker_layout_fill_bytes"],3776)
            self.assertFalse(m["complete_firmware"])
            if ref:
                fw.check_reference(artifact.read_bytes(),Path(ref).read_bytes())

    def test_hexagon_v66_registration_and_table_seeding(self):
        v66=os.environ.get("WLAN_LLVM_MC_V66")
        if not v66:
            self.skipTest("set WLAN_LLVM_MC_V66 for Hexagon v66 assembler")
        asm=fw.RUNTIME_REGISTRATION_SOURCE.read_text()
        for forb in (".incbin", ".byte", ".word", ".long", ".quad"):
            self.assertNotIn(forb,asm)
        for token in ("loop0(table_seed_body,#2)",":mem_noshuf","memb(r13++#1) = r7"):
            self.assertIn(token,asm)
        encoded=fw.assemble(fw.RUNTIME_REGISTRATION_SOURCE,v66,"hexagonv66",
                            fw.RUNTIME_REGISTRATION_LENGTH)
        self.assertEqual(len(encoded),360)
        self.assertEqual(encoded[0xcc:0xd8],bytes.fromhex("c5400078b868060b17284628"))
        self.assertEqual(fw.THROUGH_RUNTIME_REGISTRATION_LENGTH,0x2708)
        with self.assertRaisesRegex(fw.FragmentError,"prefix through 0x25a0"):
            fw.emit_with_runtime_registration(b"incomplete",v66)
        ref=os.environ.get("WLANMDSP_REFERENCE")
        if ref:fw.check_reference(encoded,Path(ref).read_bytes(),runtime_offset=0x25a0)

    def test_runtime_registration_full_cli_manifest(self):
        v66=os.environ.get("WLAN_LLVM_MC_V66")
        if not v66:
            self.skipTest("set WLAN_LLVM_MC_V66 for Hexagon v66 assembler")
        import json,subprocess
        with tempfile.TemporaryDirectory() as temp:
            out,manifest=Path(temp)/"wlan-9992.bin",Path(temp)/"manifest.json"
            cmd=[sys.executable,str(fw.ROOT/"tools/wlanmdsp/build-wlanmdsp-startup.py"),
                 str(out),"--with-runtime-registration","--llvm-mc",self.mc,
                 "--llvm-mc-v66",v66,"--manifest",str(manifest)]
            ref=os.environ.get("WLANMDSP_REFERENCE")
            if ref:cmd += ["--reference",ref]
            subprocess.run(cmd,check=True,capture_output=True,text=True)
            self.assertEqual(out.stat().st_size,0x2708)
            result=json.loads(manifest.read_text())
            self.assertEqual(result["independently_encoded_runtime_registration_bytes"],360)
            self.assertEqual(result["linker_layout_fill_bytes"],3776)
            self.assertFalse(result["complete_firmware"])
            if ref:fw.check_reference(out.read_bytes(),Path(ref).read_bytes())

    def test_full_joined_prefix_cli_manifest_counts_code_and_linker_fill(self):
        mc_v66 = os.environ.get("WLAN_LLVM_MC_V66")
        if not mc_v66:
            self.skipTest("set WLAN_LLVM_MC_V66 to LLVM22 Hexagon v66 assembler")
        import json
        import subprocess
        with tempfile.TemporaryDirectory() as scratch:
            dest = Path(scratch) / "full-first-region.bin"
            manifest = Path(scratch) / "full-first-region.json"
            cmd = [sys.executable, str(TOOL), str(dest),
                   "--with-full-initial-region", "--llvm-mc", self.mc,
                   "--llvm-mc-v66", mc_v66, "--manifest", str(manifest)]
            if os.environ.get("WLANMDSP_REFERENCE"):
                cmd.extend(("--reference", os.environ["WLANMDSP_REFERENCE"]))
            subprocess.run(cmd, check=True, capture_output=True, text=True)
            report = json.loads(manifest.read_text())
            self.assertEqual(dest.stat().st_size, 0x2208)
            self.assertEqual(report["generated_bytes"], 0x2208)
            self.assertEqual(report["independently_encoded_record_body_bytes"], 508)
            self.assertEqual(report["independently_encoded_record_comparison_bytes"], 80)
            code_bytes = sum(v for k, v in report.items()
                             if k.startswith("independently_encoded_") and k.endswith("_bytes"))
            self.assertEqual(code_bytes, 4936)
            self.assertEqual(report["linker_layout_fill_bytes"], 3776)
            self.assertFalse(report["separate_disconnected_executable_region"])
            self.assertFalse(report["complete_firmware"])

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
