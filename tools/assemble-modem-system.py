#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build modem post-TLB Hexagon code using LLVM 6 and checked ISA macros.

The 128-byte region is file [0x35c4,0x3644), physical address
[0x8e0005c4,0x8e000644). The optional prefix is the separately generated,
1,476-byte bootstrap from checkpoint 04. No OEM binary data is embedded.
"""
import argparse
import hashlib
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile

FILE_START = 0x35c4
FILE_END = 0x3644
PROGRAM_LENGTH = FILE_END - FILE_START
ORIGIN = 0x3000
PREFIX_LENGTH = FILE_START - ORIGIN
EXPECTED_SHA256 = "6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782"

# Reuse the validated ELF32 section reader in the existing companion tool.
get_text = runpy.run_path(str(Path(__file__).with_name("assemble-modem-tlb.py")))["text_from_object"]


def validate_reference(reference: bytes, expected: bytes, start: int) -> None:
    digest = hashlib.sha256(reference).hexdigest()
    if digest != EXPECTED_SHA256:
        raise ValueError(f"modem reference SHA-256 mismatch: {digest}")
    end = start + len(expected)
    if end > len(reference):
        raise ValueError(f"truncated reference at file offset 0x{end:x}")
    if reference[start:end] != expected:
        pos = next(i for i, (want, got) in enumerate(zip(reference[start:end], expected)) if want != got)
        raise ValueError(f"reconstruction mismatch at modem file offset 0x{start+pos:x}: "
                         f"want 0x{reference[start+pos]:02x}, got 0x{expected[pos]:02x}")


def assemble(llvm_mc: Path) -> bytes:
    source = Path(__file__).resolve().parents[1]/"crates/firmware-tools/src/devices/xiaomi_nabu/modem_sys_boot.s"
    with tempfile.TemporaryDirectory(prefix="hexagon-system-") as tmp:
        object_file = Path(tmp)/"system.o"
        proc = subprocess.run([str(llvm_mc), "-triple=hexagon", "-mcpu=hexagonv60", "-filetype=obj",
                               str(source), "-o", str(object_file)], capture_output=True, text=True)
        if proc.returncode:
            raise RuntimeError(f"Hexagon ISA assembler failed: {proc.stderr}")
        result = get_text(object_file.read_bytes())
    if len(result) != PROGRAM_LENGTH:
        raise ValueError(f"Hexagon system bootstrap length {len(result)} != {PROGRAM_LENGTH}")
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--llvm-mc", required=True, type=Path)
    parser.add_argument("--prefix", type=Path, help="prior 1,476-byte independently generated prefix")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--reference", type=Path, help="original unchanged modem.mbn for strict revision-specific comparison")
    opts = parser.parse_args()
    program = assemble(opts.llvm_mc)
    if opts.prefix is None:
        built, start = program, FILE_START
    else:
        prefix = opts.prefix.read_bytes()
        if len(prefix) != PREFIX_LENGTH:
            raise ValueError(f"prefix has {len(prefix)} bytes, expected {PREFIX_LENGTH}")
        built, start = prefix + program, ORIGIN
    if opts.reference is not None:
        validate_reference(opts.reference.read_bytes(), built, start)
        print(f"Exact modem reference comparison passed at [{start:#x},{start+len(built):#x})")
    opts.output.write_bytes(built)
    print(f"Produced {len(built)} bytes, SHA-256 {hashlib.sha256(built).hexdigest()} (partial modem firmware)")


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, RuntimeError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        sys.exit(1)
