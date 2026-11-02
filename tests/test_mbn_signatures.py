# SPDX-License-Identifier: GPL-3.0-only
"""Synthetic MBNv6 RSA-PSS regression tests, without any OEM binary material."""

import hashlib
import importlib.util
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile
import time
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "tools/verify" / "verify-mbn-signatures.py"
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
    return image_with_chain(folder, key, [der])


def image_with_chain(folder, key, certificates, cert_size=2048, code_offset=0x2000):
    cert_field = b"".join(certificates)
    assert len(cert_field) < cert_size
    hashes_size = 3 * 48
    auth_size = 48 + hashes_size + 256 + cert_size
    b = bytearray(code_offset + 32)
    b[:7] = b"\x7fELF\x01\x01\x01"
    struct.pack_into("<HHIIIIIHHHHHH", b, 16, 2, 164, 1, 0, 52, 0, 0, 52, 32, 3, 0, 0, 0)
    struct.pack_into("<8I", b, 52, 1, 0, 0, 0, 148, 148, 0x07000000, 0x1000)
    struct.pack_into("<8I", b, 84, 1, 0x1000, 0, 0, auth_size, auth_size, 0x02000000, 0x1000)
    struct.pack_into("<8I", b, 116, 1, code_offset, 0, 0, 32, 32, 5, 0x1000)
    b[code_offset:code_offset + 32] = bytes((i * 17 + 5) % 256 for i in range(32))
    h = [hashlib.sha384(b[:148]).digest(), bytes(48), hashlib.sha384(b[code_offset:code_offset + 32]).digest()]
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
    b[cert_start:cert_start + cert_size] = cert_field + b"\xff" * (cert_size - len(cert_field))
    return bytes(b), hashlib.sha256(certificates[-1]).hexdigest(), sig_start


def resign_fixture(image, folder):
    """Sign test-only geometry changes with our disposable fixture key."""
    changed = bytearray(image)
    headers = [struct.unpack_from("<8I", changed, 52 + 32 * i) for i in range(3)]
    table = b"".join(
        bytes(48) if idx == 1 or ph[4] == 0 else hashlib.sha384(changed[ph[1]:ph[1] + ph[4]]).digest()
        for idx, ph in enumerate(headers))
    changed[0x1030:0x10c0] = table
    message = folder / "resign-message.bin"
    message.write_bytes(changed[0x1000:0x10c0])
    signature = run("dgst", "-sha256", "-sign", folder / "test-only-key.pem",
                    "-sigopt", "rsa_padding_mode:pss", "-sigopt", "rsa_pss_saltlen:-1",
                    "-binary", message)
    changed[0x10c0:0x11c0] = signature
    return changed


def synthetic_certificate_chain(folder):
    """Three disposable test certificates with ordinary CA constraints."""
    root_key, root_pem = folder / "root-key.pem", folder / "root.pem"
    run("req", "-new", "-x509", "-newkey", "rsa:2048", "-nodes", "-sha256",
        "-subj", "/CN=Temporary MBN chain root", "-days", "2",
        "-addext", "basicConstraints=critical,CA:TRUE,pathlen:1",
        "-addext", "keyUsage=critical,keyCertSign,cRLSign",
        "-keyout", root_key, "-out", root_pem)
    parent_key, parent_pem = root_key, root_pem
    certificates = [run("x509", "-in", root_pem, "-outform", "DER")]
    for name, serial, constraints, usage in [
            ("intermediate", 2, "CA:TRUE,pathlen:0", "keyCertSign,cRLSign"),
            ("leaf", 3, "CA:FALSE", "digitalSignature")]:
        key, csr, pem = (folder / f"{name}-key.pem", folder / f"{name}.csr", folder / f"{name}.pem")
        ext = folder / f"{name}.cnf"
        ext.write_text(f"basicConstraints=critical,{constraints}\nkeyUsage=critical,{usage}\n"
                       "subjectKeyIdentifier=hash\nauthorityKeyIdentifier=keyid,issuer\n")
        run("req", "-new", "-newkey", "rsa:2048", "-nodes", "-sha256",
            "-subj", f"/CN=Temporary MBN {name}", "-keyout", key, "-out", csr)
        run("x509", "-req", "-in", csr, "-CA", parent_pem, "-CAkey", parent_key,
            "-set_serial", serial, "-days", "2", "-sha256", "-extfile", ext, "-out", pem)
        certificates.insert(0, run("x509", "-in", pem, "-outform", "DER"))
        parent_key, parent_pem = key, pem
    return image_with_chain(folder, parent_key, certificates, cert_size=6144, code_offset=0x3000)


class TestMbnStructure(unittest.TestCase):
    def test_nonminimal_der_long_length_rejected(self):
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "non-minimal DER"):
            mbn_signatures.parse_der_chain(b"\x30\x82\x00\x80" + b"x" * 128)

    def test_truncated_der_length_rejected(self):
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "beyond its container"):
            mbn_signatures.parse_der_chain(b"\x30\x82\x01")

    def test_invalid_api_parameters_rejected(self):
        for pin in ["g" * 64, "a" * 63, "ab " * 21 + "a", 123]:
            with self.subTest(pin=pin), self.assertRaisesRegex(mbn_signatures.VerificationError, "fingerprint"):
                mbn_signatures.verify_image(b"", expected_root_sha256=pin)
        for epoch in [-1, True, "100", 1 << 63]:
            with self.subTest(epoch=epoch), self.assertRaisesRegex(mbn_signatures.VerificationError, "verification time"):
                mbn_signatures.verify_image(b"", verification_time=epoch)


@unittest.skipUnless(shutil.which("openssl"), "OpenSSL needed for RSA-PSS fixtures")
class TestMbnSignatureVerifier(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="firmware-mbnv6-test-")
        cls.original, cls.root, cls.sig_offset = synthetic_image(Path(cls.temp.name))
        chain_dir = Path(cls.temp.name) / "chain"
        chain_dir.mkdir()
        cls.chain, cls.chain_root, cls.chain_sig_offset = synthetic_certificate_chain(chain_dir)

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def test_signed_synthetic_firmware_succeeds_and_root_pin_matches(self):
        result = mbn_signatures.verify_image(self.original, expected_root_sha256=self.root)
        self.assertEqual(result["segment_hashes_verified"], 2)
        self.assertEqual(result["certificate_count"], 1)
        self.assertTrue(result["oem_signature_verified"])
        self.assertTrue(result["embedded_root_self_signature_verified"])
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

    def test_valid_three_certificate_chain_and_fixed_time_succeed(self):
        epoch = int(time.time())
        result = mbn_signatures.verify_image(self.chain, expected_root_sha256=self.chain_root,
                                             verification_time=epoch)
        self.assertEqual(result["certificate_count"], 3)
        self.assertTrue(result["certificate_order_verified"])
        self.assertEqual(result["certificate_verification_time_unix"], epoch)
        self.assertEqual(result["certificate_verification_time_source"], "explicit")
        self.assertEqual(result["reference_image_sha256"], hashlib.sha256(self.chain).hexdigest())

    def test_fixed_time_outside_certificate_validity_fails(self):
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "not yet valid"):
            mbn_signatures.verify_image(self.original, verification_time=1)

    def test_corrupted_embedded_root_self_signature_rejected(self):
        # OpenSSL trusts anchor keys without checking their self-signatures by
        # default, so this mutation used to pass when no root pin was supplied.
        changed = bytearray(self.original)
        cert_start = self.sig_offset + 256
        der = mbn_signatures.parse_der_chain(changed[cert_start:cert_start + 2048])[0]
        changed[cert_start + len(der) - 1] ^= 1
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "OpenSSL authentication failed"):
            mbn_signatures.verify_image(changed)

    def test_duplicate_embedded_certificate_rejected(self):
        changed = bytearray(self.original)
        cert_start = self.sig_offset + 256
        der = mbn_signatures.parse_der_chain(changed[cert_start:cert_start + 2048])[0]
        changed[cert_start:cert_start + 2048] = der * 2 + b"\xff" * (2048 - 2 * len(der))
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "duplicate certificate"):
            mbn_signatures.verify_image(changed)

    def test_unrelated_embedded_intermediate_rejected(self):
        changed = bytearray(self.chain)
        cert_start = self.chain_sig_offset + 256
        certs = mbn_signatures.parse_der_chain(changed[cert_start:cert_start + 6144])
        unrelated = mbn_signatures.parse_der_chain(
            self.original[self.sig_offset + 256:self.sig_offset + 256 + 2048])[0]
        # The real path is still present. A simple -untrusted bag would ignore
        # this extra certificate and report success for the whole field.
        field = certs[0] + unrelated + certs[1] + certs[2]
        changed[cert_start:cert_start + 6144] = field + b"\xff" * (6144 - len(field))
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "OpenSSL authentication failed"):
            mbn_signatures.verify_image(changed)

    def test_signed_impossible_pt_load_size_rejected(self):
        changed = bytearray(self.original)
        struct.pack_into("<I", changed, 116 + 20, 1)
        changed = resign_fixture(changed, Path(self.temp.name))
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "p_filesz greater than p_memsz"):
            mbn_signatures.verify_image(changed)

    def test_overlapping_hashed_and_authentication_segments_rejected(self):
        changed = bytearray(self.original)
        struct.pack_into("<I", changed, 116 + 4, 0x1100)
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "overlaps MBNv6"):
            mbn_signatures.verify_image(changed)

    def test_unbound_program_header_table_rejected(self):
        changed = bytearray(self.original)
        struct.pack_into("<II", changed, 52 + 16, 52, 52)
        changed = resign_fixture(changed, Path(self.temp.name))
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "not covered by a hashed segment"):
            mbn_signatures.verify_image(changed)

    def test_trailing_undeclared_authentication_bytes_rejected(self):
        changed = bytearray(self.original)
        size = struct.unpack_from("<I", changed, 84 + 16)[0]
        struct.pack_into("<II", changed, 84 + 16, size + 1, size + 1)
        changed = resign_fixture(changed, Path(self.temp.name))
        with self.assertRaisesRegex(mbn_signatures.VerificationError, "exactly fill"):
            mbn_signatures.verify_image(changed)

    def test_memory_only_bss_with_out_of_file_offset_still_validates(self):
        changed = bytearray(self.original)
        struct.pack_into("<I", changed, 116 + 4, 0xffff0000)
        struct.pack_into("<II", changed, 116 + 16, 0, 4096)
        changed = resign_fixture(changed, Path(self.temp.name))
        result = mbn_signatures.verify_image(changed, expected_root_sha256=self.root)
        self.assertEqual(result["segment_hashes_verified"], 1)


if __name__ == "__main__":
    unittest.main()
