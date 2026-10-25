#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Audit six exact Nabu MBN references for source-only reconstruction coverage.

Requires six original ELF32/MBNv6 files in a private directory. No OEM
firmware, executable bytes, signatures or certificates are written to the
checkout or to the JSONL report. The report accounts for source-generated
bytes and missing file spans; it must never be treated as a flashable image.

The optional embedded-chain verification checks an RSA-PSS signature against a
pinned *embedded* X.509 root, NOT against a Qualcomm hardware trust anchor.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
REVISIONS = ROOT / "reference" / "xiaomi-nabu" / "firmware.sha256"
IMAGES = ("a640_zap.mbn", "adsp.mbn", "cdsp.mbn", "modem.mbn", "venus.mbn", "wlanmdsp.mbn")
EMBEDDED_ROOT_SHA256 = "c924a35f39ce1cdd1b8d5a9f3b8e3c51317930431d7a9dd5a55028cf6965fe65"


def pinned_sha256s(path=REVISIONS):
    expected = {}
    for line in path.read_text().splitlines():
        fields = line.split(maxsplit=1)
        if len(fields) != 2:
            raise ValueError("invalid original firmware SHA-256 manifest row")
        digest, name = fields
        name = Path(name.removeprefix("*").removeprefix("./")).name
        if name in IMAGES:
            if name in expected or len(digest) != 64 or any(c not in "0123456789abcdef" for c in digest):
                raise ValueError(f"invalid/duplicate MBN revision digest: {name}")
            expected[name] = digest
    if set(expected) != set(IMAGES):
        raise ValueError("pinned reference manifest is missing an expected MBN revision")
    return expected


def digest_file(path):
    hashing = hashlib.sha256()
    with path.open("rb") as fd:
        for chunk in iter(lambda: fd.read(1024 * 1024), b""):
            hashing.update(chunk)
    return hashing.hexdigest()


def tool(args):
    result = subprocess.run(args, text=True, capture_output=True, check=False)
    if result.returncode:
        why = (result.stderr or result.stdout).strip()
        raise ValueError(f"command {Path(args[0]).name} failed: {why[:600]}")
    return result.stdout


def audit(root, firmware_tools, verify_signatures=False):
    root = Path(root)
    firmware_tools = Path(firmware_tools)
    if not root.is_dir():
        raise ValueError("the directory containing six original MBN files does not exist")
    if not firmware_tools.is_file():
        raise ValueError("firmware-tools executable does not exist; build the offline release binary")
    revisions = pinned_sha256s()
    reports = []
    for name in IMAGES:
        src = root / name
        if not src.is_file():
            raise ValueError(f"missing original Nabu MBN: {name}")
        actual = digest_file(src)
        if actual != revisions[name]:
            raise ValueError(f"wrong original revision for {name}: SHA-256 {actual}")
        output = tool([str(firmware_tools), "audit-mbn-coverage-json", name, str(src)])
        try:
            report = json.loads(output)
        except json.JSONDecodeError as exc:
            raise ValueError(f"{name}: malformed coverage JSON") from exc
        if (report.get("image") != name or
                report.get("file_bytes") != src.stat().st_size or
                report.get("externally_supplied_authentication_bytes") != 0 or
                report.get("hardware_trust_verified") is not False or
                report.get("missing_file_bytes", 0) <= 0):
            raise ValueError(f"{name}: source-coverage invariants not met")
        # Embedded-chain RSA-PSS consistency is independent of source coverage.
        if verify_signatures:
            tool([sys.executable, str(ROOT / "tools" / "verify-mbn-signatures.py"),
                  str(src), "--expected-root-sha256", EMBEDDED_ROOT_SHA256])
        reports.append(report)
    return reports


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("original_mbn_directory", type=Path, help="directory containing all six original *.mbn files")
    ap.add_argument("--firmware-tools", type=Path,
                    default=ROOT / "target" / "release" / "firmware-tools",
                    help="path to compiled source-only firmware-tools binary")
    ap.add_argument("--verify-embedded-signatures", action="store_true",
                    help="also check pinned embedded root chain and OEM RSA-PSS signature")
    args = ap.parse_args(argv)
    try:
        # Collect every result before printing: no partial success report.
        reports = audit(args.original_mbn_directory, args.firmware_tools,
                        verify_signatures=args.verify_embedded_signatures)
    except (OSError, ValueError) as exc:
        ap.exit(1, f"FAIL: {exc}\n")
    for report in reports:
        print(json.dumps(report, separators=(",", ":")))
    print("NOTICE: matching reference revisions, SHA-384 and optional embedded-chain signatures do NOT mean source-complete or hardware-trusted images.",file=sys.stderr)


if __name__ == "__main__":
    main()
