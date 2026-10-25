#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Pin the reference and compare typed Rust with independent GNU NDS32 output.

Neither source emitter consumes the firmware. The private image is read only
by this verification process; it is never copied into generated source.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

PINNED_SHA256 = "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5"
IMAGE_SIZE = 196608
START, END = 0x10016, 0x101B6
EXPECTED_SHA256 = "2dc59e3cd4cff852e54bf53392bc6fe4853ded0a49c9b530d430b3ea37d2a3b4"


def run(*args):
    return subprocess.check_output([str(a) for a in args], text=True).strip()


def checked_reference(data):
    digest = hashlib.sha256(data).hexdigest()
    if len(data) != IMAGE_SIZE or digest != PINNED_SHA256:
        raise ValueError(f"wrong reference: size={len(data)}, sha256={digest}")
    return data


def compare(expected, actual, label):
    if len(actual) != len(expected):
        raise ValueError(f"{label}: length {len(actual)} != {len(expected)}")
    mismatches = [(i, a, b) for i, (a, b) in enumerate(zip(expected, actual)) if a != b]
    if mismatches:
        i, a, b = mismatches[0]
        raise ValueError(f"{label}: {len(mismatches)} mismatches; first file {START+i:#x}: reference={a:02x}, generated={b:02x}")
    return 0


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("reference", type=Path)
    p.add_argument("--emitter", type=Path, required=True)
    p.add_argument("--as", dest="assembler", default="nds32le-elf-as")
    p.add_argument("--ld", dest="linker", default="nds32le-elf-ld")
    p.add_argument("--objcopy", default="nds32le-elf-objcopy")
    p.add_argument("--report", type=Path)
    args = p.parse_args()
    root = Path(__file__).resolve().parents[3]
    reference = checked_reference(args.reference.read_bytes())
    expected = reference[START:END]
    with tempfile.TemporaryDirectory(prefix="agent20-18-verify-") as temp:
        d = Path(temp)
        run(args.emitter.resolve(), "emit", d/"rust.bin")
        run(args.assembler, "-mno-all-relax", "-o", d/"gnu.o",
            root/"reference/xiaomi-nabu/agent20-18/upper-ilm-checkpoint01.S")
        run(args.linker, "--no-relax", f"-Ttext={START-0x200:#x}", "-e", "agent18_upper_start",
            "-o", d/"gnu.elf", d/"gnu.o")
        run(args.objcopy, "-O", "binary", "--only-section=.text", d/"gnu.elf", d/"gnu.bin")
        rust, gnu = (d/"rust.bin").read_bytes(), (d/"gnu.bin").read_bytes()
        compare(expected, rust, "typed Rust")
        compare(expected, gnu, "independent GNU assembler")
        digest = hashlib.sha256(rust).hexdigest()
        if digest != EXPECTED_SHA256:
            raise ValueError(f"unexpected span digest {digest}")
        report = {
            "agent": "20-18", "checkpoint": 1,
            "git_head": run("git", "-C", root, "rev-parse", "HEAD"),
            "actual_parent": "51b1f91f32d32542aa03e114f5a2f4d94b559e2a",
            "checkpoint50_integration": False,
            "reference_sha256": PINNED_SHA256, "reference_size": len(reference),
            "file_start": START, "file_end": END,
            "runtime_start": START-0x200, "runtime_end": END-0x200,
            "typed_instructions": 155, "executable_bytes": len(rust),
            "data_bytes": 0, "layout_fill_bytes": 0,
            "reference_payload_bytes_embedded": 0, "byte_mismatches": 0,
            "span_sha256": digest,
            "assembler_version": run(args.assembler, "--version").splitlines()[0],
            "leading_seam": {"start": 0x10000, "end": START, "bytes": START-0x10000},
            "first_uncovered_file": 0x10000, "next_forward_file": END,
            "behavior_verified_on_hardware": False,
        }
    rendered = json.dumps(report, indent=2) + "\n"
    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(rendered)
    print(rendered, end="")


if __name__ == "__main__":
    main()
