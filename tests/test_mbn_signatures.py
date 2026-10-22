# SPDX-License-Identifier: GPL-3.0-only
"""Synthetic MBNv6 RSA-PSS regression tests, without any OEM binary material."""

import hashlib
import importlib.util
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "tools" / "verify-mbn-signatures.py"
SPEC = importlib.util.spec_from_file_location("mbn_signatures", SCRIPT)
mbn_signatures = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(mbn_signatures)


def run(*args):
    return subprocess.check_output(["openssl", *map(str, args)], stderr=subprocess.DEVNULL)


def synthetic_image(folder):
    key = folder / "test-only-key.pem"
    pem = folder / "test-only-cert.pem"
    run("req", "-new", "-x509", "-newkey", "rsa:2048", "-nodes", "-sha256",
        "-subj", "/CN=Temporary MBN verifier fixture", "-days", "2",
        "-keyout", key, "-out", pem)
    der = run("x509", "-in", pem, "-outform", "DER")
    cert_size = 2048
    assert len(der) < cert_size
    hashes_size = 3 * 48
    auth_size = 48 + hashes_size + 256 + cert_size
    b = bytearray(0x2020)
    b[:7] = b"\x7fELF\x01\x01\x01"
    struct.pack_into("<HHIIIIIHHHHHH", b, 16, 2, 164, 1, 0, 52, 0, 0, 52, 32, 3, 0, 0, 0)
    struct.pack_into("<8I", b, 52, 1, 0, 0, 0, 148, 148, 0x07000000, 0x1000)
    struct.pack_into("<8I", b, 84, 1, 0x1000, 0, 0, auth_size, auth_size, 0x02000000, 0x1000)
    struct.pack_into("<8I", b, 116, 1, 0x2000, 0, 0, 32, 32, 5, 0x1000)
    b[0x2000:0x2020] = bytes((i * 17 + 5) % 256 for i in range(32))
    h = [hashlib.sha384(b[:148]).digest(), bytes(48), hashlib.sha384(b[0x2000:0x2020]).digest()]
    header = struct.pack("<12I", 0, 6, 0, 0, hashes_size + 256 + cert_size,
                         hashes_size, 0xffffffff, 256, 0xffffffff, cert_size, 0, 0)
    message = header + b"".join(h)
    b[0x1000:0x1000 + len(message)] = message
    message_file = folder / "message.bin"
    message_file.write_bytes(message)
    signature = run("dgst", "-sha256", "-sign", key, "-sigopt", "rsa_padding_mode:pss",
                    "-sigopt", "rsa_pss_saltlen:-1", "-binary", message_file)
    assert len(signature) == 256
    sig_start = 0x1000 + len(message)
    b[sig_start:sig_start + 256] = signature
    cert_start = sig_start + 256
    b[cert_start:cert_start + cert_size] = der + b"\xff" * (cert_size - len(der))
    return bytes(b), hashlib.sha256(der).hexdigest(), sig_start


@unittest.skipUnless(shutil.which("openssl"), "OpenSSL needed for RSA-PSS fixtures")
class TestMbnSignatureVerifier(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="firmware-mbnv6-test-")
        cls.original, cls.root, cls.sig_offset = synthetic_image(Path(cls.temp.name))

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def test_signed_synthetic_firmware_succeeds_and_root_pin_matches(self):
        result = mbn_signatures.verify_image(self.original, expected_root_sha256=self.root)
        self.assertEqual(result["segment_hashes_verified"], 2)
        self.assertEqual(result["certificate_count"], 1)
        self.assertTrue(result["oem_signature_verified"])
        self.assertFalse(result["device_hardware_trust_verified"])

    def test_changed_code_fails_sha384(self):
        changed = bytearray(self.original)
        changed[0x2000] ^= 0x40
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "SHA-384 mismatch"):
            mbn_signatures.verify_image(changed)

    def test_changed_signature_fails_rsa_pss(self):
        changed = bytearray(self.original)
        changed[self.sig_offset + 7] ^= 0x40
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "OpenSSL authentication failed"):
            mbn_signatures.verify_image(changed)

    def test_changed_signed_metadata_fails_rsa_pss(self):
        changed = bytearray(self.original)
        changed[0x1000] ^= 1   # image ID, inside the signed MBNv6 header
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "OpenSSL authentication failed"):
            mbn_signatures.verify_image(changed)

    def test_unexpected_root_fails(self):
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "root certificate"):
            mbn_signatures.verify_image(self.original, expected_root_sha256="0" * 64)

    def test_bad_certificate_padding_fails(self):
        changed = bytearray(self.original)
        changed[0x1000 + 48 + 144 + 256 + 1500] = 0x40
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "non-padding"):
            mbn_signatures.verify_image(changed)

    def test_bad_elf_rejected(self):
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "ELF32"):
            mbn_signatures.verify_image(b"not an ELF")


if __name__ == "__main__":
    unittest.main()
