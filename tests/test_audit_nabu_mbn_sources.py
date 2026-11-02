# SPDX-License-Identifier: GPL-3.0-only
"""Tests for six-original-revision source-coverage audit (no OEM bytes)."""
import contextlib
import importlib.util
import io
from pathlib import Path
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "tools/verify" / "audit-nabu-mbn-sources.py"
SPEC = importlib.util.spec_from_file_location("mbn_source_auditor", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class TestNabuSixMbnSourceAudit(unittest.TestCase):
    def test_exact_six_known_revision_digests(self):
        pinned = MODULE.pinned_sha256s()
        self.assertEqual(tuple(MODULE.IMAGES), tuple(pinned))
        self.assertEqual(pinned["a640_zap.mbn"],
                         "bb1b61bae977aab5e5a68bd88848f4479c8d156911d96c7de73f7195b674db06")
        self.assertTrue(all(len(hash_value) == 64 for hash_value in pinned.values()))

    def test_bad_revision_or_missing_image_refused_before_tool_invocation(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            dummy_executable = root / "firmware-tools"
            dummy_executable.write_bytes(b"test-only-not-a-program")
            with self.assertRaisesRegex(ValueError, "missing original"):
                MODULE.audit(root, dummy_executable)
            (root / "a640_zap.mbn").write_bytes(b"not-the-original-reference")
            with self.assertRaisesRegex(ValueError, "wrong original revision"):
                MODULE.audit(root, dummy_executable)

    def test_failure_does_not_print_partial_jsonl(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            executable = root / "dummy-bin"
            executable.write_bytes(b"stub")
            out, err = io.StringIO(), io.StringIO()
            with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
                with self.assertRaises(SystemExit) as raised:
                    MODULE.main([str(root), "--firmware-tools", str(executable)])
            self.assertEqual(raised.exception.code, 1)
            self.assertEqual(out.getvalue(), "")
            self.assertIn("missing original", err.getvalue())


if __name__ == "__main__":
    unittest.main()
