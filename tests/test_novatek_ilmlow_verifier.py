# SPDX-License-Identifier: GPL-3.0-only
"""Verifier failure-policy regressions using synthetic inputs, without OEM blobs."""

import contextlib
import importlib.util
import io
import json
from pathlib import Path
import struct
import sys
import tempfile
import unittest
from unittest import mock


SCRIPT = Path(__file__).resolve().parents[1] / "tools" / "verify-novatek-ilm-low.py"
SPEC = importlib.util.spec_from_file_location("novatek_ilmlow_verifier", SCRIPT)
verifier = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(verifier)


def listing(*rows):
    """Produce a synthetic contiguous listing, never a reference-derived one."""
    profile = verifier.PROFILES["postmarketos"]
    pc = verifier.FILE_START - verifier.ILM_FILE_BASE
    lines = []
    for size, assembly in rows:
        lines.append(f"{pc:x}\t{size}\t{assembly}")
        pc += size
    header = (f"# profile=postmarketos gp=0x{profile['gp']:x} itb=0x{profile['itb']:x} "
              f"file_start=0x{verifier.FILE_START:x} file_end=0x{pc + verifier.ILM_FILE_BASE:x}")
    return "\n".join([header, *lines]) + "\n"


def parsed(*rows):
    return verifier.parse_listing(listing(*rows), "postmarketos")[1]


# Synthetic instructions assembled with GNU 2.25.1 -march=v3 -mno-all-relax
# -EL -mall-ext. J/JAL fixtures were linked with --no-relax at runtime zero.
# These exercise arbitrary operand values, not firmware instruction samples.
EFFECT_FIXTURES = (
    ("64630403", {"operation": "mtsr", "source_register": 6, "csr": 0xC1}),
    ("58220123", {"operation": "ori", "destination_register": 2, "source_register": 4, "immediate": 0x123}),
    ("54220123", {"operation": "andi", "destination_register": 2, "source_register": 4, "immediate": 0x123}),
    ("447ffffd", {"operation": "movi", "destination_register": 7, "immediate": -3}),
    ("40b81c09", {"operation": "srli", "destination_register": 11, "source_register": 16, "shift": 7}),
    ("48000abb", {"operation": "j", "target_low25": 0x1576}),
    ("4900101a", {"operation": "jal", "target_low25": 0x2034}),
)


class TestListingValidation(unittest.TestCase):
    def test_indexed_memory_shift_and_immediate_syntax_is_accepted(self):
        rows = parsed((4, "lb $r3,[$r0+($r5<<#0x0)]"),
                      (4, "sb $r3,[$r1+($r5<<0)]"),
                      (2, "ifcall9 #16"), (2, "ifret16"))
        self.assertEqual([row["length"] for row in rows], [4, 4, 2, 2])
        self.assertEqual(rows[2]["operands"], "#16")

    def test_data_directives_comments_and_statement_injection_are_rejected(self):
        invalid = (
            ".byte 0x83,0xff", "byte 0x83", ".include firmware",
            "mov55 $r0,$r1; ifret16", "mov55 $r0,$r1 ! comment",
            "mov55 $r0,$r1 # comment", "mov55 $r0,\t$r1",
            "mov55 $r0,$r1\n.word 0", "mov55 $r0,$r1\\n",
            "mov55 $r0,$r1 | 0", 'mov55 $r0,"r1"',
        )
        for assembly in invalid:
            with self.subTest(assembly=assembly), self.assertRaises(verifier.VerificationFailure):
                parsed((2, assembly))

    def test_only_complete_shift_tokens_and_balanced_memory_operands_are_allowed(self):
        for assembly in ("lb $r3,[$r0+($r5<#0)]", "lb $r3,[$r0+($r5<<<0)]",
                         "lb $r3,[$r0+($r5>>0)]", "lb $r3,[$r0+($r5<<0]",
                         "lb $r3,[$r0+$r5<<0)]"):
            with self.subTest(assembly=assembly), self.assertRaises(verifier.VerificationFailure):
                parsed((4, assembly))

    def test_reserved_aliases_cannot_claim_ordinary_move_or_branch_semantics(self):
        for assembly in ("mov55 $sp,$sp", "mov55 $r31,$sp", "mov55 $r31,$r31",
                         "beqs38 $r5,.+2", "bnes38 $r5,.+2"):
            with self.subTest(assembly=assembly), self.assertRaisesRegex(
                    verifier.VerificationFailure, "reserved"):
                parsed((2, assembly))
        self.assertEqual(parsed((2, "ifret16"))[0]["mnemonic"], "ifret16")
        self.assertEqual(parsed((2, "mov55 $r0,$r31"))[0]["mnemonic"], "mov55")
        self.assertEqual(parsed((2, "bnes38 $r4,.+2"))[0]["mnemonic"], "bnes38")

    def test_unsupported_mnemonics_and_false_instruction_widths_fail(self):
        for row in ((2, "unknown"), (4, "mov55 $r0,$r1"), (2, "movi $r0,1")):
            with self.subTest(row=row), self.assertRaisesRegex(
                    verifier.VerificationFailure, "unsupported"):
                parsed(row)

    def test_metadata_and_contiguity_are_checked(self):
        original = listing((2, "mov55 $r0,$r1"))
        for bad in (original.replace("gp=", "gp=0 gp="),
                    original.replace("profile=postmarketos", "profile=map220v"),
                    original.replace("gp=0x2d9e0", "gp=0x2d9e4"),
                    original.replace("file_end=0x66c", "file_end=0x66e"),
                    original.replace("\n46a\t", "\n46c\t"),
                    original.replace("# profile=", "# garbage profile=")):
            with self.subTest(text=bad), self.assertRaises(verifier.VerificationFailure):
                verifier.parse_listing(bad, "postmarketos")


class TestControlFlow(unittest.TestCase):
    def test_ifcall9_literal_is_a_displacement_from_the_instruction_pc(self):
        rows = parsed((2, "ifcall9 0x10"))
        result = verifier.control_flow(rows, 0x46A, 0x46C)
        edge = result["direct_edges"][0]
        self.assertEqual(edge["target_runtime_address"], 0x47A)
        self.assertEqual(edge["target_file_offset"], 0x67A)
        self.assertEqual(edge["byte_displacement"], 16)
        self.assertTrue(edge["outside_emitted_interval"])
        self.assertIn("IFC-state-dependent", edge["return_semantics"])

    def test_ifcall9_rejects_absolute_expression_odd_negative_and_oversize_operands(self):
        for operand in (".+16", "1", "-2", "1024", "010", "symbol"):
            with self.subTest(operand=operand), self.assertRaises(verifier.VerificationFailure):
                parsed((2, "ifcall9 " + operand))
        for operand in ("0", "1022"):
            self.assertEqual(parsed((2, "ifcall9 " + operand))[0]["mnemonic"], "ifcall9")

    def test_regular_branches_require_explicit_relative_targets_and_instruction_boundaries(self):
        rows = parsed((2, "beqz38 $r0,.+2"), (4, "movi $r1,3"))
        self.assertEqual(verifier.control_flow(rows, 0x46A, 0x470)["direct_edges"][0]
                         ["target_runtime_address"], 0x46C)
        for operand in ("0x46c", ".+4", ".+1"):
            rows = parsed((2, "beqz38 $r0," + operand), (4, "movi $r1,3"))
            with self.subTest(operand=operand), self.assertRaises(verifier.VerificationFailure):
                verifier.control_flow(rows, 0x46A, 0x470)

    def test_ex9_jump_uses_call_site_region_and_unsigned_low25_bits(self):
        self.assertEqual(verifier.ex9_jump_target(0x4900101A, 0xA2030000), 0xA2002034)
        self.assertEqual(verifier.ex9_jump_target(0x49C00001, 0xA2030000), 0xA3800002)
        self.assertTrue(verifier.verify_effect(bytes.fromhex("49c00001"),
                                              {"operation": "jal", "target_low25": 0x1800002}))
        with self.assertRaises(verifier.VerificationFailure):
            verifier.ex9_jump_target(0x447FFFFD, 0x46A)
        rows = parsed((2, "ifret16"))
        flow = verifier.control_flow(rows, 0x46A, 0x46C)
        self.assertFalse(flow["direct_edges"])
        self.assertEqual(len(flow["ifc_return_sites"]), 1)


class TestExternalEffects(unittest.TestCase):
    def test_independent_synthetic_instruction_fixtures_and_each_bit_mutation(self):
        for raw, effect in EFFECT_FIXTURES:
            original = bytes.fromhex(raw)
            with self.subTest(effect=effect):
                self.assertTrue(verifier.verify_effect(original, effect))
            word = int.from_bytes(original, "big")
            for bit in range(32):
                changed = (word ^ (1 << bit)).to_bytes(4, "big")
                with self.subTest(operation=effect["operation"], mutated_bit=bit):
                    self.assertFalse(verifier.verify_effect(changed, effect))

    def test_unknown_or_truncated_effects_fail(self):
        with self.assertRaisesRegex(verifier.VerificationFailure, "truncated"):
            verifier.verify_effect(b"\0\0", {"operation": "movi"})
        with self.assertRaisesRegex(verifier.VerificationFailure, "unsupported"):
            verifier.verify_effect(b"\0" * 4, {"operation": "unknown"})

    def test_ex9_dependencies_are_separate_and_their_calls_reach_control_flow(self):
        effect = {"operation": "jal", "target_low25": 0x2034}
        profile = {**verifier.PROFILES["postmarketos"], "ex9": {7: effect}}
        data = bytearray(0x300)
        table_file = 0x40 + 4 * 7 + verifier.ILM_FILE_BASE
        data[table_file:table_file + 4] = bytes.fromhex("4900101a")
        rows = parsed((2, "ex9.it 7"), (2, "ex9.it 7"))
        with mock.patch.dict(verifier.PROFILES, {"postmarketos": profile}):
            result = verifier.inspect_ex9(data, "postmarketos", {"itb": 0x40}, rows)
            self.assertEqual((result["uses"], result["distinct_entries"],
                              result["external_reference_bytes"]), (2, 1, 4))
            self.assertEqual(result["counted_as_agent17_reconstructed_bytes"], 0)
            flow = verifier.control_flow(rows, 0x46A, 0x46E, result)
            self.assertEqual(len(flow["external_targets"]), 2)
            self.assertEqual(flow["external_targets"][0]["target_runtime_address"], 0x2034)
            self.assertEqual(flow["external_targets"][1]["target_runtime_address"], 0x2034)
            self.assertEqual(flow["external_targets"][0]["via_ex9_index"], 7)
            data[table_file] ^= 1  # Changes JAL into J: target equality alone must not pass.
            with self.assertRaisesRegex(verifier.VerificationFailure, "does not match"):
                verifier.inspect_ex9(data, "postmarketos", {"itb": 0x40}, rows)
            with self.assertRaisesRegex(verifier.VerificationFailure, "lacks"):
                verifier.inspect_ex9(data, "postmarketos", {"itb": 0x40},
                                     parsed((2, "ex9.it 8")))
        with self.assertRaisesRegex(verifier.VerificationFailure, "lacks a verified"):
            verifier.control_flow(rows, 0x46A, 0x46E)


class TestCliFailurePolicy(unittest.TestCase):
    def synthetic_run(self, directory, skip, wrong_hash=False):
        """Exercise real main/report policy with a fake source emitter and pinned synthetic image."""
        image = bytearray(0x1200)
        struct.pack_into("<III", image, 0, 0x200, 0, 0xFFF)
        emitted = bytes.fromhex("8001")  # Synthetic MOV55 r0,r1.
        image[0x66A:0x66C] = emitted
        reference = directory / "synthetic.bin"
        reference.write_bytes(image)
        expected = {**verifier.PROFILES["postmarketos"], "size": len(image),
                    "sha256": "0" * 64 if wrong_hash else verifier.sha256(image),
                    "ilm_length": 0x1000}
        source_listing = listing((2, "mov55 $r0,$r1"))

        def emitter_only(command, report):
            report["commands"].append({"argv": list(map(str, command))})
            if command[1] == "listing":
                return source_listing
            self.assertEqual(command[1], "emit", "an external assembler must not run under --skip")
            Path(command[3]).write_bytes(emitted)
            return ""

        output = directory / "report"
        args = ["--reference", str(reference), "--profile", "postmarketos",
                "--emitter", sys.executable, "--output-directory", str(output),
                "--as", str(directory / "intentionally-missing-assembler")]
        if skip:
            args.append("--skip-assembler")
        with mock.patch.dict(verifier.PROFILES, {"postmarketos": expected}), \
                mock.patch.object(verifier, "run", side_effect=emitter_only) as commands, \
                contextlib.redirect_stdout(io.StringIO()):
            code = verifier.main(args)
        return code, json.loads((output / "verification.json").read_text()), commands.call_count

    def test_explicit_skip_produces_partial_report_and_exit_two(self):
        with tempfile.TemporaryDirectory() as directory:
            code, report, calls = self.synthetic_run(Path(directory), skip=True)
        self.assertEqual(code, 2)
        self.assertEqual(report["overall_status"], "partial")
        self.assertEqual(report["independent_assembly"]["status"], "skipped")
        self.assertFalse(report["independent_assembly"]["binary_data_fallback_used"])
        self.assertEqual(calls, 2)

    def test_missing_assembler_cannot_silently_become_a_skipped_or_passed_gate(self):
        with tempfile.TemporaryDirectory() as directory:
            code, report, calls = self.synthetic_run(Path(directory), skip=False)
        self.assertEqual(code, 1)
        self.assertEqual(report["overall_status"], "failed")
        self.assertIn("required executable not found", report["error"])
        self.assertEqual(calls, 2)

    def test_wrong_whole_image_hash_stops_before_emitter_execution(self):
        with tempfile.TemporaryDirectory() as directory:
            code, report, calls = self.synthetic_run(Path(directory), skip=True, wrong_hash=True)
        self.assertEqual(code, 1)
        self.assertEqual(report["overall_status"], "failed")
        self.assertIn("whole-firmware", report["error"])
        self.assertEqual(calls, 0)


if __name__ == "__main__":
    unittest.main()
