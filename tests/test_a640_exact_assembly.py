# SPDX-License-Identifier: GPL-3.0-only
"""Source-only guardrails for exact A640 external-authentication workflow."""
from pathlib import Path
import contextlib
import importlib.util
import io
import tempfile
from types import SimpleNamespace
from unittest import mock
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "tools/build/assemble-a640-exact.py"
SPEC = importlib.util.spec_from_file_location("a640_exact", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class TestA640ExactAssembly(unittest.TestCase):
    def test_pinned_revision_cannot_be_replaced_by_a_different_payload(self):
        self.assertEqual(len(MODULE.REFERENCE_SHA256), 64)
        self.assertEqual(len(MODULE.REFERENCE_EMBEDDED_ROOT_SHA256), 64)
        self.assertEqual(MODULE.EXTERNAL_AUTH_SIZE, 6400)
        self.assertEqual(MODULE.OUTPUT_SIZE, 14256)
        with self.assertRaisesRegex(ValueError, "exactly|expected 6400"):
            MODULE.validate_external_authentication(b"test-only")
        MODULE.validate_external_authentication(bytes(6400))  # only the length is validated here
        with self.assertRaisesRegex(ValueError, "output length"):
            MODULE.validate_exact_rebuilt_image(bytes(14255))
        with self.assertRaisesRegex(ValueError, "not the exact original A640 revision"):
            MODULE.validate_exact_rebuilt_image(bytes(14256))

    def test_atomic_publish_refuses_racing_destination(self):
        # Exercise final publication with *synthetic* bytes: the Rust builder,
        # hash verifier and chain verifier are stubbed, not OEM firmware.
        with tempfile.TemporaryDirectory() as directory:
            auth = Path(directory) / "external-auth.bin"
            auth.write_bytes(bytes(6400))
            output = Path(directory) / "racing-output.mbn"

            def fake_builder(argv, **kwargs):
                Path(argv[-1]).write_bytes(b"synthetic-test-only")
                return SimpleNamespace(returncode=0, stderr="")

            def fake_link(source, destination):
                # Another process creates the destination *after* the early
                # existence check and *before* the atomic hardlink creation.
                Path(destination).write_bytes(b"other-process-data")
                raise FileExistsError("racing destination")

            with (mock.patch.object(MODULE.subprocess, "run", side_effect=fake_builder),
                  mock.patch.object(MODULE, "validate_exact_rebuilt_image"),
                  mock.patch.object(MODULE, "verify_embedded_chain", return_value={"certificate_count": 3}),
                  mock.patch.object(MODULE.os, "link", side_effect=fake_link),
                  contextlib.redirect_stderr(io.StringIO())):
                with self.assertRaises(SystemExit) as raised:
                    MODULE.main([str(auth), str(output), "--firmware-tools", str(SCRIPT)])
            self.assertEqual(raised.exception.code, 1)
            self.assertEqual(output.read_bytes(), b"other-process-data")
            self.assertEqual(sorted(p.name for p in Path(directory).iterdir()),
                             ["external-auth.bin", "racing-output.mbn"])

    def test_rejects_existing_destination_without_clobbering(self):
        with tempfile.TemporaryDirectory() as directory:
            original = Path(directory) / "original.auth"
            original.write_bytes(bytes(6400))  # deliberately invalid OEM signature
            output = Path(directory) / "existing.mbn"
            output.write_bytes(b"keep")
            with contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit) as raised:
                    MODULE.main([str(original), str(output), "--firmware-tools", str(SCRIPT)])
            self.assertEqual(raised.exception.code, 1)
            self.assertEqual(output.read_bytes(), b"keep")


if __name__ == "__main__":
    unittest.main()
