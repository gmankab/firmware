#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check ELF32/MBNv6 data hashes, OEM RSA-PSS signature and embedded X.509 chain.

This verifies *internal* cryptographic consistency only. An embedded root CA is
not a Qualcomm hardware trust anchor. Never use success as a boot authorization.
Python 3.9+ and OpenSSL 3.x are required; no third-party Python packages.
Certificate validity uses the current time unless an explicit verification
epoch is supplied. Reproducibility manifests should record a fixed epoch.
"""

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile


class VerificationError(ValueError):
    """Malformed input, unsupported profile or failed authentication check."""


def span(data, start, length, label):
    if start < 0 or length < 0 or start + length > len(data):
        raise VerificationError(f"{label} extends beyond its container")
    return data[start : start + length]


def read_elf32(image):
    if len(image) < 52 or image[:7] != b"\x7fELF\x01\x01\x01":
        raise VerificationError("only little-endian ELF32 files are supported")
    elf = struct.unpack_from("<16sHHIIIIIHHHHHH", image)
    offset, stride, count = elf[5], elf[9], elf[10]
    if elf[3] != 1 or elf[8] != 52:
        raise VerificationError("invalid ELF32 version or header size")
    if offset < 52 or count < 2 or count > 256 or stride != 32:
        raise VerificationError("invalid ELF32 program-header table")
    span(image, offset, stride * count, "ELF program headers")
    headers = [struct.unpack_from("<8I", image, offset + stride * i) for i in range(count)]
    matches = [i for i, ph in enumerate(headers) if ph[6] & 0x07000000 == 0x02000000]
    if len(matches) != 1:
        raise VerificationError(f"expected exactly one Qualcomm hash segment, got {len(matches)}")
    auth_idx = matches[0]
    auth = headers[auth_idx]
    auth_start, auth_end = auth[1], auth[1] + auth[4]
    for idx, ph in enumerate(headers):
        if ph[0] == 1 and ph[4] > ph[5]:
            raise VerificationError(f"ELF PT_LOAD {idx} has p_filesz greater than p_memsz")
        if idx == auth_idx or not ph[4]:
            # Memory-only/BSS offsets may legally lie beyond the physical file.
            continue
        span(image, ph[1], ph[4], f"ELF segment {idx}")
        if ph[1] < auth_end and auth_start < ph[1] + ph[4]:
            raise VerificationError(f"ELF segment {idx} overlaps MBNv6 authentication segment")
    # The signature must bind the layout that selects its data hashes. Without
    # this, a valid signature need not authenticate the ELF program headers.
    headers_end = offset + stride * count
    if not any(idx != auth_idx and ph[1] == 0 and ph[4] >= headers_end
               for idx, ph in enumerate(headers)):
        raise VerificationError("ELF header and program-header table are not covered by a hashed segment")
    return headers, auth_idx


def parse_der_chain(field):
    """Separate concatenated DER certificates from 00/FF fixed-field padding."""
    certs = []
    cursor = 0
    while cursor < len(field) and field[cursor] == 0x30:
        if cursor + 2 > len(field):
            raise VerificationError("truncated certificate DER header")
        length_id = field[cursor + 1]
        if length_id < 128:
            size, length_field = length_id, 1
        elif 0x81 <= length_id <= 0x84:
            width = length_id & 0x7F
            n = span(field, cursor + 2, width, "certificate length")
            size, length_field = int.from_bytes(n, "big"), width + 1
            if not n[0] or size < 128:
                raise VerificationError("non-minimal DER certificate length")
        else:
            raise VerificationError("indefinite or oversized DER certificate length")
        end = cursor + 1 + length_field + size
        certs.append(bytes(span(field, cursor, end - cursor, "DER certificate")))
        cursor = end
    if not certs:
        raise VerificationError("certificate field has no DER X.509 certificates")
    if len(set(certs)) != len(certs):
        raise VerificationError("duplicate certificate in embedded DER chain")
    if any(byte not in (0, 0xFF) for byte in field[cursor:]):
        raise VerificationError("unexpected non-padding bytes after DER certificates")
    return certs


def openssl(args, *, input=None):
    if shutil.which("openssl") is None:
        raise VerificationError("OpenSSL executable is required")
    result = subprocess.run(["openssl", *args], input=input, capture_output=True)
    if result.returncode:
        detail = result.stderr.decode(errors="replace").strip() or result.stdout.decode(errors="replace").strip()
        raise VerificationError(f"OpenSSL authentication failed: {detail[:350]}")
    return result.stdout


def verify_image(image, *, expected_root_sha256=None, verification_time=None):
    if expected_root_sha256 is not None:
        if (not isinstance(expected_root_sha256, str) or len(expected_root_sha256) != 64 or
                any(c not in "0123456789abcdefABCDEF" for c in expected_root_sha256)):
            raise VerificationError("root fingerprint must have 64 hexadecimal characters")
    if (verification_time is not None and
            (type(verification_time) is not int or not 0 <= verification_time <= 0x7fff_ffff_ffff_ffff)):
        raise VerificationError("verification time must be a nonnegative signed 64-bit Unix epoch")
    headers, auth_idx = read_elf32(image)
    auth = headers[auth_idx]
    raw = span(image, auth[1], auth[4], "MBNv6 authentication segment")
    if len(raw) < 48:
        raise VerificationError("truncated MBNv6 header")
    (image_id, version, qti_sig_size, qti_cert_size, payload_size, hash_size,
     _oem_sig_pointer, sig_size, _oem_cert_pointer, cert_size, qti_meta_size,
     oem_meta_size) = struct.unpack_from("<12I", raw)
    if version != 6:
        raise VerificationError(f"unsupported MBN header version {version}")
    if qti_sig_size or qti_cert_size:
        raise VerificationError("dual-QTI-signer MBNv6 profile not yet implemented")
    if hash_size != len(headers) * 48 or not sig_size or not cert_size:
        raise VerificationError("MBNv6 signer sizes or hash count are inconsistent")
    if payload_size != hash_size + qti_sig_size + qti_cert_size + sig_size + cert_size:
        raise VerificationError("MBNv6 declared payload size mismatch")
    hash_start = 48 + qti_meta_size + oem_meta_size
    signed_end = hash_start + hash_size
    signature_start = signed_end + qti_sig_size + qti_cert_size
    cert_start = signature_start + sig_size
    if cert_start + cert_size != len(raw):
        raise VerificationError("MBNv6 authentication fields do not exactly fill the declared hash segment")

    actual_hashes = span(raw, hash_start, hash_size, "MBNv6 hash table")
    digest_count = 0
    for idx, ph in enumerate(headers):
        segment = None if idx == auth_idx or ph[4] == 0 else span(
            image, ph[1], ph[4], f"ELF segment {idx}"
        )
        computed = hashlib.sha384(segment).digest() if segment is not None else bytes(48)
        if actual_hashes[idx * 48 : (idx + 1) * 48] != computed:
            raise VerificationError(f"SHA-384 mismatch in ELF segment {idx}")
        digest_count += segment is not None

    certs = parse_der_chain(span(raw, cert_start, cert_size, "OEM certificates"))
    root_digest = hashlib.sha256(certs[-1]).hexdigest()
    if expected_root_sha256 is not None and root_digest.lower() != expected_root_sha256.lower():
        raise VerificationError("OEM embedded root certificate does not match expected SHA-256")

    with tempfile.TemporaryDirectory(prefix="mbnv6-verify-") as tmp:
        folder = Path(tmp)
        for idx, cert in enumerate(certs):
            (folder / f"cert-{idx}.der").write_bytes(cert)
            (folder / f"cert-{idx}.pem").write_bytes(openssl([
                "x509", "-inform", "DER", "-in", str(folder / f"cert-{idx}.der"),
                "-outform", "PEM",
            ]))
        intermediates = b"".join((folder / f"cert-{idx}.pem").read_bytes()
                                 for idx in range(1, len(certs) - 1))
        (folder / "intermediates.pem").write_bytes(intermediates)
        # `-trusted` disables all default CA stores. `-check_ss_sig` also checks
        # the embedded root's self-signature, which OpenSSL otherwise skips.
        # https://docs.openssl.org/3.0/man1/openssl-verification-options/
        common = ["verify", "-purpose", "any", "-check_ss_sig"]
        if verification_time is not None:
            common += ["-attime", str(verification_time)]
        for idx in range(len(certs) - 1):
            # Validate the declared order rather than letting OpenSSL silently
            # ignore an unrelated intermediate from the embedded field. These
            # pair checks supplement, not replace, the full path validation.
            openssl(common + ["-partial_chain", "-trusted", str(folder / f"cert-{idx + 1}.pem"),
                              str(folder / f"cert-{idx}.pem")])
        args = common + ["-trusted", str(folder / f"cert-{len(certs) - 1}.pem")]
        if len(certs) > 2:
            args += ["-untrusted", str(folder / "intermediates.pem")]
        openssl(args + [str(folder / "cert-0.pem")])

        signed_message = raw[:signed_end]
        (folder / "signed-message.bin").write_bytes(signed_message)
        (folder / "signature.bin").write_bytes(span(raw, signature_start, sig_size, "OEM signature"))
        (folder / "leaf-public.pem").write_bytes(openssl([
            "x509", "-in", str(folder / "cert-0.pem"), "-pubkey", "-noout"
        ]))
        openssl([
            "dgst", "-sha256", "-verify", str(folder / "leaf-public.pem"),
            "-signature", str(folder / "signature.bin"),
            "-sigopt", "rsa_padding_mode:pss", "-sigopt", "rsa_pss_saltlen:auto",
            "-sigopt", "rsa_mgf1_md:sha256",
            str(folder / "signed-message.bin"),
        ])
    return {
        "mbn_image_id": image_id,
        "segment_hashes_verified": digest_count,
        "certificate_count": len(certs),
        "embedded_root_sha256": root_digest,
        "oem_signature_verified": True,
        "certificate_chain_verified_to_embedded_root": True,
        "embedded_root_self_signature_verified": True,
        "certificate_order_verified": True,
        "certificate_verification_time_unix": verification_time,
        "certificate_verification_time_source": "explicit" if verification_time is not None else "system_clock",
        "oem_signature_algorithm": "RSA-PSS",
        "oem_signature_digest": "SHA-256",
        "oem_signature_mgf1_digest": "SHA-256",
        "oem_signature_salt_length": "auto-detected",
        "reference_image_sha256": hashlib.sha256(image).hexdigest(),
        "device_hardware_trust_verified": False,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("firmware", type=Path, help="ELF32 MBNv6 firmware file")
    parser.add_argument("--expected-root-sha256", metavar="SHA256",
                        help="optional pinned SHA-256 of embedded root DER certificate")
    parser.add_argument("--verification-time", type=int, metavar="UNIX_EPOCH",
                        help="fixed certificate validity time for reproducible audits (default: current time)")
    args = parser.parse_args()
    if args.expected_root_sha256 is not None:
        try:
            bytes.fromhex(args.expected_root_sha256)
        except ValueError:
            parser.error("root fingerprint is not hexadecimal")
        if len(args.expected_root_sha256) != 64:
            parser.error("root fingerprint must have 64 hexadecimal characters")
    try:
        result = verify_image(args.firmware.read_bytes(),
                              expected_root_sha256=args.expected_root_sha256,
                              verification_time=args.verification_time)
    except (VerificationError, OSError) as exc:
        parser.exit(1, f"FAIL: {exc}\n")
    print(json.dumps(result, indent=2))
    print("NOTICE: Embedded-chain validation does not prove hardware secure-boot trust.")


if __name__ == "__main__":
    main()
