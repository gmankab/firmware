#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Reassemble exact known A640 revision with borrowed OEM authentication.

Fails closed: no final file is written unless the new binary matches the
**pinned original-image SHA-256**, and its existing RSA-PSS signature and
embedded certificate chain pass OpenSSL verification against the pinned
embedded root fingerprint. This CANNOT issue a new Qualcomm signing key or
prove device hardware trust. OEM material is loaded from a user-owned file,
never bundled in this repository.
"""
import argparse
import hashlib
import importlib.util
import os
from pathlib import Path
import subprocess
import sys
import tempfile

REFERENCE_SHA256 = "bb1b61bae977aab5e5a68bd88848f4479c8d156911d96c7de73f7195b674db06"
REFERENCE_EMBEDDED_ROOT_SHA256 = "c924a35f39ce1cdd1b8d5a9f3b8e3c51317930431d7a9dd5a55028cf6965fe65"
EXTERNAL_AUTH_SIZE = 0x1900
OUTPUT_SIZE = 0x37B0
ROOT = Path(__file__).resolve().parents[1]


def validate_external_authentication(auth: bytes):
    if len(auth) != EXTERNAL_AUTH_SIZE:
        raise ValueError(f"expected {EXTERNAL_AUTH_SIZE} original OEM signature+certificate bytes, got {len(auth)}")


def validate_exact_rebuilt_image(data: bytes):
    if len(data) != OUTPUT_SIZE:
        raise ValueError(f"A640 output length {len(data)} does not match {OUTPUT_SIZE}")
    observed = hashlib.sha256(data).hexdigest()
    if observed != REFERENCE_SHA256:
        raise ValueError(f"not the exact original A640 revision: SHA-256 {observed} != {REFERENCE_SHA256}")


def verify_embedded_chain(data: bytes):
    spec = importlib.util.spec_from_file_location("nabu_mbn_verifier", ROOT / "tools" / "verify-mbn-signatures.py")
    if spec is None or spec.loader is None:
        raise ValueError("cannot load the independent OpenSSL MBN verifier")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    result = module.verify_image(data, expected_root_sha256=REFERENCE_EMBEDDED_ROOT_SHA256)
    if not result["oem_signature_verified"] or not result["certificate_chain_verified_to_embedded_root"]:
        raise ValueError("OEM RSA-PSS or certificate chain has not been verified")
    if result["device_hardware_trust_verified"]:
        raise ValueError("unexpected unsupported hardware trust claim")
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("original_auth", type=Path, help="original 6,400-byte OEM field, stored outside Git")
    parser.add_argument("output", type=Path, help="destination, must not already exist")
    parser.add_argument("--firmware-tools", type=Path, default=ROOT / "target" / "release" / "firmware-tools",
                        help="built firmware-tools executable (default: target/release/firmware-tools)")
    args = parser.parse_args(argv)
    if not args.original_auth.is_file():
        parser.error("OEM authentication input does not exist")
    try:
        original_auth = args.original_auth.read_bytes()
        validate_external_authentication(original_auth)
        if args.output.exists():
            raise ValueError("refusing to overwrite existing output")
        if not args.output.parent.is_dir():
            raise ValueError("destination directory does not exist")
        if not args.firmware_tools.is_file():
            raise ValueError("firmware-tools executable does not exist")
        # The temporary file stays outside the Git source. Failed verification
        # exits without moving a file into the user-selected destination.
        with tempfile.TemporaryDirectory(prefix="a640-original-verify-", dir=args.output.parent) as tmp:
            private_output = Path(tmp) / "candidate.mbn"
            process = subprocess.run([
                str(args.firmware_tools.resolve()), "assemble-a640-with-external-auth",
                str(args.original_auth.resolve()), str(private_output),
            ], capture_output=True, text=True, check=False)
            if process.returncode:
                raise ValueError(f"Rust builder failed: {process.stderr.strip()[:250]}")
            built = private_output.read_bytes()
            validate_exact_rebuilt_image(built)
            result = verify_embedded_chain(built)
            # An atomic hard-link creation refuses an output that appeared
            # after our earlier existence check (no TOCTOU overwrite).
            # Same-filesystem placement is guaranteed by dir=output.parent.
            try:
                os.link(private_output, args.output)
            except FileExistsError as exc:
                raise ValueError("refusing to overwrite an output created during verification") from exc
        print(f"Exact original A640: {len(built)} bytes, SHA-256 {REFERENCE_SHA256}")
        print("Generated from source: 7856 bytes; original borrowed authentication: 6400 bytes")
        print(f"Verified embedded certificates: {result['certificate_count']}; OEM RSA-PSS: true")
        print("NO new OEM signature created; embedded-chain consistency does not prove hardware trust")
    except (OSError, ValueError) as exc:
        parser.exit(1, f"FAIL: {exc}\n")


if __name__ == "__main__":
    main()
