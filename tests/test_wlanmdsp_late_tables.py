# SPDX-License-Identifier: GPL-3.0-only
"""Symbol-derived WLAN dispatch data and optional real-toolchain checks."""
from dataclasses import replace
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parent.parent
TOOL = ROOT / "tools/build-wlanmdsp-late-tables.py"
REGISTRY = ROOT / "sources/xiaomi-nabu/wlanmdsp/late/regions.json"
SPEC = importlib.util.spec_from_file_location("build_wlanmdsp_late_tables", TOOL)
assert SPEC is not None and SPEC.loader is not None
tables = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = tables
SPEC.loader.exec_module(tables)

TABLE_SHA256 = "f33f4dae122e666e9eefcfd539eb2405bc967d4928e96a72b3897907144f7390"
SYNTHETIC_TEXT = struct.pack("<3I", 0x70004000, 0x7000C000, 0x7000C000)


def elf_object(symbols, text=SYNTHETIC_TEXT):
    """Make a small ELF32 object; entries are (name, section offset, section)."""
    strings = bytearray(b"\0")
    entries = bytearray(16)  # Required undefined symbol zero.
    for name, offset, section in symbols:
        name_index = len(strings)
        strings.extend(name.encode("ascii") + b"\0")
        entries.extend(struct.pack("<IIIBBH", name_index, offset, 0, 0, 0, section))
    section_names = b"\0.text\0.symtab\0.strtab\0.shstrtab\0"
    image = bytearray(52)

    def append(data, alignment=1):
        image.extend(bytes((-len(image)) % alignment))
        offset = len(image)
        image.extend(data)
        return offset

    text_offset = append(text, 4)
    symbols_offset = append(entries, 4)
    strings_offset = append(strings)
    names_offset = append(section_names)
    image.extend(bytes((-len(image)) % 4))
    headers_offset = len(image)
    headers = [
        (0,) * 10,
        (section_names.index(b".text"), 1, 6, 0, text_offset, len(text), 0, 0, 4, 0),
        (section_names.index(b".symtab"), 2, 0, 0, symbols_offset,
         len(entries), 3, len(symbols) + 1, 4, 16),
        (section_names.index(b".strtab"), 3, 0, 0, strings_offset,
         len(strings), 0, 0, 1, 0),
        (section_names.index(b".shstrtab"), 3, 0, 0, names_offset,
         len(section_names), 0, 0, 1, 0),
    ]
    for header in headers:
        image.extend(struct.pack("<10I", *header))
    ident = b"\x7fELF\x01\x01\x01" + bytes(9)
    image[:52] = struct.pack("<16sHHIIIIIHHHHHH", ident, 1, 164, 1, 0, 0,
                             headers_offset, 0, 52, 0, 0, 40, len(headers), 4)
    return bytes(image)


def section_header_offset(image, index):
    return struct.unpack_from("<I", image, 32)[0] + index * 40


class WlanLateTableSymbolTests(unittest.TestCase):
    def setUp(self):
        base = tables.TABLES[0]
        self.spec = replace(base, function_file_end=base.function_file_start + 12,
                            first_type=0, entry_count=2, case_types=(0,))
        self.entries = [(self.spec.entry_symbol, 0, 1),
                        ("type_0000", 0, 1), ("restore_status", 8, 1)]
        self.obj = elf_object(self.entries)
        self.symbols, self.text_section = tables.read_text_symbols(self.obj)

    def test_symbol_offsets_generate_runtime_targets_in_type_order(self):
        generated, mappings = tables.derive_table(
            self.spec, SYNTHETIC_TEXT, self.symbols, self.text_section)
        self.assertEqual(struct.unpack("<2I", generated),
                         (self.spec.function_vaddr, self.spec.function_vaddr + 8))
        self.assertEqual([m["type"] for m in mappings], [0, 1])
        self.assertEqual([m["label"] for m in mappings], ["type_0000", "restore_status"])
        self.assertEqual([m["target_code_offset"] for m in mappings], [0, 8])
        self.assertEqual([m["is_default"] for m in mappings], [False, True])
        self.assertEqual([m["target_vaddr"] for m in mappings],
                         list(struct.unpack("<2I", generated)))

    def test_missing_explicit_case_does_not_silently_become_default(self):
        del self.symbols["type_0000"]
        with self.assertRaises(tables.BuildError):
            tables.derive_table(self.spec, SYNTHETIC_TEXT, self.symbols, self.text_section)

    def test_missing_default_and_entry_symbols_are_rejected(self):
        for name in ("restore_status", self.spec.entry_symbol):
            with self.subTest(name=name):
                changed = dict(self.symbols)
                del changed[name]
                with self.assertRaises(tables.BuildError):
                    tables.derive_table(self.spec, SYNTHETIC_TEXT, changed, self.text_section)

    def test_undefined_and_other_section_targets_are_rejected(self):
        for section in (0, 3, 0xFFF1):
            for name in ("type_0000", "restore_status", self.spec.entry_symbol):
                with self.subTest(section=section, name=name):
                    changed = dict(self.symbols)
                    changed[name] = tables.Symbol(self.symbols[name].offset, section)
                    with self.assertRaises(tables.BuildError):
                        tables.derive_table(self.spec, SYNTHETIC_TEXT, changed,
                                            self.text_section)

    def test_function_entry_must_begin_at_registered_base(self):
        changed = dict(self.symbols)
        changed[self.spec.entry_symbol] = tables.Symbol(8, self.text_section)
        with self.assertRaises(tables.BuildError):
            tables.derive_table(self.spec, SYNTHETIC_TEXT, changed, self.text_section)

    def test_targets_must_be_inside_function_excluding_end(self):
        for offset in (-4, len(SYNTHETIC_TEXT), len(SYNTHETIC_TEXT) + 4):
            with self.subTest(offset=offset):
                changed = dict(self.symbols)
                changed["type_0000"] = tables.Symbol(offset, self.text_section)
                with self.assertRaises(tables.BuildError):
                    tables.derive_table(self.spec, SYNTHETIC_TEXT, changed, self.text_section)

    def test_aligned_inside_packet_and_unaligned_targets_are_rejected(self):
        # Offset four is an encoded instruction, but its packet begins at zero.
        for offset in (2, 4):
            with self.subTest(offset=offset):
                changed = dict(self.symbols)
                changed["type_0000"] = tables.Symbol(offset, self.text_section)
                with self.assertRaises(tables.BuildError):
                    tables.derive_table(self.spec, SYNTHETIC_TEXT, changed, self.text_section)

    def test_truncated_or_unterminated_source_packets_are_rejected(self):
        malformed = struct.pack("<3I", 0x70004000, 0x70004000, 0x70004000)
        for text in (SYNTHETIC_TEXT[:-4], malformed):
            with self.subTest(text=text.hex()):
                with self.assertRaises(tables.BuildError):
                    tables.derive_table(self.spec, text, self.symbols, self.text_section)

    def test_duplicate_symbol_names_are_ambiguous(self):
        obj = elf_object(self.entries + [("type_0000", 8, 1)])
        with self.assertRaises(tables.BuildError):
            tables.read_text_symbols(obj)

    def test_symbol_names_must_be_bounded_and_terminated(self):
        symbol_offset = struct.unpack_from("<I", self.obj,
                                          section_header_offset(self.obj, 2) + 16)[0]
        strings_header = section_header_offset(self.obj, 3)
        strings_offset, strings_size = struct.unpack_from("<II", self.obj,
                                                          strings_header + 16)
        outside = bytearray(self.obj)
        struct.pack_into("<I", outside, symbol_offset + 16, strings_size)
        unterminated = bytearray(self.obj)
        unterminated[strings_offset + strings_size - 1] = ord("X")
        for image in (outside, unterminated):
            with self.subTest(image=bytes(image).hex()[-60:]):
                with self.assertRaises(tables.BuildError):
                    tables.read_text_symbols(bytes(image))

    def test_invalid_symbol_table_link_and_entry_size_are_rejected(self):
        header = section_header_offset(self.obj, 2)
        for field, value in ((24, 9), (24, 1), (36, 0), (36, 12), (20, 17)):
            with self.subTest(field=field, value=value):
                changed = bytearray(self.obj)
                struct.pack_into("<I", changed, header + field, value)
                with self.assertRaises(tables.BuildError):
                    tables.read_text_symbols(bytes(changed))

    def test_only_bounded_hexagon_elf32_relocatable_objects_are_accepted(self):
        wrong_type = bytearray(self.obj)
        struct.pack_into("<H", wrong_type, 16, 2)
        wrong_machine = bytearray(self.obj)
        struct.pack_into("<H", wrong_machine, 18, 3)
        wrong_class = bytearray(self.obj)
        wrong_class[4] = 2
        for image in (self.obj[:51], self.obj[:-1], wrong_type,
                      wrong_machine, wrong_class):
            with self.subTest(length=len(image)):
                with self.assertRaises(tables.BuildError):
                    tables.read_text_symbols(bytes(image))


class WlanLateTableDependencyTests(unittest.TestCase):
    def setUp(self):
        self.regions = tables.late.load_registry(REGISTRY)

    def test_tables_are_adjacent_data_ranges_with_fixed_case_domains(self):
        self.assertEqual([(spec.file_start, spec.file_end, spec.entry_count)
                          for spec in tables.TABLES],
                         [(0x2F6CD0, 0x2F6D1C, 19), (0x2F6D1C, 0x2F6EC0, 105)])
        self.assertEqual([spec.first_type for spec in tables.TABLES], [0, 0x1002])
        self.assertEqual([len(spec.case_types) for spec in tables.TABLES], [14, 53])
        self.assertTrue(all(spec.default_label == "restore_status" for spec in tables.TABLES))
        self.assertEqual(sum(spec.size_bytes for spec in tables.TABLES), 496)

    def test_dependencies_match_registered_authored_function_ranges(self):
        for spec in tables.TABLES:
            with self.subTest(name=spec.name):
                region = tables.dependency_for(spec, self.regions)
                self.assertEqual((region.file_start, region.file_end),
                                 (spec.function_file_start, spec.function_file_end))
                self.assertEqual(region.source, spec.source)
                self.assertEqual(region.symbol, spec.entry_symbol)
                self.assertEqual(region.classification, "executable")

    def test_missing_and_changed_dependency_identity_is_rejected(self):
        spec = tables.TABLES[0]
        dependency = tables.dependency_for(spec, self.regions)
        missing = [r for r in self.regions if r != dependency]
        with self.assertRaises(tables.BuildError):
            tables.dependency_for(spec, missing)
        for field, value in (("file_start", dependency.file_start + 4),
                             ("file_end", dependency.file_end - 4),
                             ("source", "sources/xiaomi-nabu/wlanmdsp/late/other.S"),
                             ("symbol", "other_entry"),
                             ("classification", "data")):
            with self.subTest(field=field):
                changed = [replace(r, **{field: value}) if r == dependency else r
                           for r in self.regions]
                with self.assertRaises(tables.BuildError):
                    tables.dependency_for(spec, changed)


@unittest.skipUnless(os.environ.get("WLAN_LATE_LLVM_MC"),
                     "set WLAN_LATE_LLVM_MC to a Hexagon v66-capable llvm-mc")
class WlanLateTableAssemblyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mc = os.environ["WLAN_LATE_LLVM_MC"]
        cls.independent_mc = os.environ.get("WLAN_LATE_SECOND_LLVM_MC",
                                            os.environ.get("WLAN_LATE_INDEPENDENT_LLVM_MC"))
        cls.data, cls.manifest = tables.build(cls.mc,
                                             independent_llvm_mc=cls.independent_mc)

    def test_no_reference_build_has_exact_data_size_range_and_digest(self):
        self.assertEqual(len(self.data), 496)
        self.assertEqual(self.manifest["generated_bytes"], 496)
        self.assertEqual((self.manifest["file_start"], self.manifest["file_end"]),
                         (0x2F6CD0, 0x2F6EC0))
        self.assertEqual((self.manifest["vaddr_start"], self.manifest["vaddr_end"]),
                         (0xB02B6CD0, 0xB02B6EC0))
        self.assertEqual(hashlib.sha256(self.data).hexdigest(), TABLE_SHA256)
        self.assertEqual(self.manifest["sha256"], TABLE_SHA256)
        self.assertFalse(self.manifest["reference_checked"])
        self.assertIsNone(self.manifest["reference_byte_mismatches"])

    def test_far_away_tables_add_only_data_without_inventing_code_or_fill(self):
        self.assertFalse(self.manifest["complete_firmware"])
        self.assertEqual(self.manifest["data_bytes"], 496)
        self.assertEqual(self.manifest["source_derived_data_bytes"], 496)
        for field in ("source_derived_executable_bytes", "layout_fill_bytes",
                      "external_reference_bytes_in_output"):
            self.assertEqual(self.manifest[field], 0)
        self.assertEqual([t["classification"] for t in self.manifest["tables"]],
                         ["data", "data"])
        self.assertEqual([t["bytes"] for t in self.manifest["tables"]], [76, 420])

    def test_all_emitted_targets_have_explicit_type_and_authored_label_provenance(self):
        cursor = 0
        for spec, report in zip(tables.TABLES, self.manifest["tables"]):
            with self.subTest(name=spec.name):
                mappings = report["mappings"]
                targets = struct.unpack(f"<{spec.entry_count}I",
                                        self.data[cursor:cursor + spec.size_bytes])
                self.assertEqual(len(mappings), spec.entry_count)
                self.assertEqual([m["type"] for m in mappings],
                                 list(range(spec.first_type, spec.first_type + spec.entry_count)))
                self.assertEqual([m["target_vaddr"] for m in mappings], list(targets))
                for mapping in mappings:
                    explicit = mapping["type"] in spec.case_types
                    label = f"type_{mapping['type']:04x}" if explicit else "restore_status"
                    self.assertEqual(mapping["label"], label)
                    self.assertEqual(mapping["is_default"], not explicit)
                    self.assertGreaterEqual(mapping["target_code_offset"], 0)
                    self.assertLess(mapping["target_code_offset"],
                                    spec.function_file_end - spec.function_file_start)
                    self.assertEqual(mapping["target_vaddr"],
                                     spec.function_vaddr + mapping["target_code_offset"])
                cursor += spec.size_bytes
        self.assertEqual(cursor, len(self.data))

    def test_default_cases_skip_the_already_applied_success_assignment(self):
        short, wide = self.manifest["tables"]
        short_defaults = [m for m in short["mappings"] if m["is_default"]]
        wide_defaults = [m for m in wide["mappings"] if m["is_default"]]
        self.assertEqual([m["type"] for m in short_defaults], [1, 7, 8, 10, 16])
        self.assertEqual(len(wide_defaults), 52)
        self.assertEqual({m["target_vaddr"] for m in short_defaults}, {0xB00C0D6C})
        self.assertEqual({m["target_vaddr"] for m in wide_defaults}, {0xB00C125C})

    def test_dependency_fingerprints_and_table_digests_are_recorded_separately(self):
        regions = tables.late.load_registry(REGISTRY)
        cursor = 0
        for spec, report in zip(tables.TABLES, self.manifest["tables"]):
            with self.subTest(name=spec.name):
                region = tables.dependency_for(spec, regions)
                self.assertEqual(report["source_code_sha256"], region.sha256)
                self.assertEqual(report["source_sha256"],
                                 hashlib.sha256((ROOT / spec.source).read_bytes()).hexdigest())
                self.assertEqual(report["sha256"], spec.sha256)
                self.assertEqual(report["sha256"],
                                 hashlib.sha256(self.data[cursor:cursor + spec.size_bytes]).hexdigest())
                self.assertEqual(report["source_function_file_start"], region.file_start)
                self.assertEqual(report["source_function_file_end"], region.file_end)
                self.assertEqual(report["source_function_vaddr"], spec.function_vaddr)
                cursor += spec.size_bytes

    def test_independent_toolchain_result_is_only_claimed_when_run(self):
        self.assertEqual(self.manifest["independent_assembler_checked"], bool(self.independent_mc))
        self.assertEqual(self.manifest["independent_assembler_byte_mismatches"],
                         0 if self.independent_mc else None)

    def test_changed_registered_source_code_digest_rejects_build(self):
        payload = json.loads(REGISTRY.read_text())
        for region in payload["regions"]:
            if region["source"] == tables.TABLES[0].source:
                region["sha256"] = "0" * 64
        with tempfile.TemporaryDirectory() as temporary:
            registry = Path(temporary) / "changed-registry.json"
            registry.write_text(json.dumps(payload))
            with self.assertRaises(tables.BuildError):
                tables.build(self.mc, registry_path=registry)

    @unittest.skipUnless(os.environ.get("WLANMDSP_REFERENCE"),
                         "set WLANMDSP_REFERENCE to the pinned private OEM MBN")
    def test_every_reference_table_entry_matches_symbolic_generation(self):
        reference = Path(os.environ["WLANMDSP_REFERENCE"])
        generated, report = tables.build(self.mc, reference=reference,
                                          independent_llvm_mc=self.independent_mc)
        expected = reference.read_bytes()[0x2F6CD0:0x2F6EC0]
        self.assertEqual(generated, self.data)
        self.assertEqual(struct.unpack("<124I", generated), struct.unpack("<124I", expected))
        self.assertTrue(report["reference_checked"])
        self.assertEqual(report["reference_byte_mismatches"], 0)

    @unittest.skipUnless(os.environ.get("WLANMDSP_REFERENCE"),
                         "set WLANMDSP_REFERENCE for whole-file identity rejection")
    def test_reference_mutation_outside_the_table_range_is_rejected(self):
        changed = bytearray(Path(os.environ["WLANMDSP_REFERENCE"]).read_bytes())
        changed[-1] ^= 1
        with tempfile.TemporaryDirectory() as temporary:
            reference = Path(temporary) / "changed.mbn"
            reference.write_bytes(changed)
            with self.assertRaises(tables.BuildError):
                tables.build(self.mc, reference=reference)

    def test_cli_emits_a_separate_table_artifact_without_reference(self):
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary) / "dispatch-tables.bin"
            manifest = Path(temporary) / "dispatch-tables.json"
            command = [sys.executable, str(TOOL), str(output), "--llvm-mc", self.mc,
                       "--manifest", str(manifest)]
            if self.independent_mc:
                command.extend(["--independent-llvm-mc", self.independent_mc])
            subprocess.run(command, check=True, capture_output=True, text=True)
            self.assertEqual(output.read_bytes(), self.data)
            report = json.loads(manifest.read_text())
            self.assertEqual(report["generated_bytes"], 496)
            self.assertEqual(report["source_derived_executable_bytes"], 0)
            self.assertEqual(report["layout_fill_bytes"], 0)
            self.assertFalse(report["reference_checked"])


if __name__ == "__main__":
    unittest.main()
