#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify CDSP's 17-instruction Hexagon V66 initialization with Rizin.

This is a *decoder* cross-check, unlike the LLVM 6 assembler comparison of
CDSP's initial nine scalar packets. It checks the meaning of reconstructed
bytes without embedding OEM opcodes or pretending Rizin is an assembler.
"""
import argparse
import os
from pathlib import Path
import re
import subprocess

FILE_OFFSET = 0x3000
PHYSICAL_ADDRESS = 0x96c00000
EXPECTED_MNEMONICS = (
    "jump 0x96c0001c", "jump 0x96c00004", "jump 0x96c00008",
    "jump 0x96c0000c", "jump 0x96c00010", "jump 0x96c00014",
    "jump 0x96c00018", "jump 0x96c00cac", "r0 = ##0x0",
    "ssr = r0", "r0 = syscfg", "r0 = setbit(r0,#0x3)",
    "r0 = setbit(r0,#0x5)", "syscfg = r0", "dckill",
    "ickill", "jump 0x96c00144",
)
SYS_CFG_RESET_MNEMONICS = (
    "r15 = syscfg", "r15 = clrbit(r15,#0x1)", "syscfg = r15",
    "isync", "ickill", "isync", "r15 = or(r15,##0x3)",
    "syscfg = r15", "isync", "lr = sub(lr,r25)", "jumpr lr",
)
PROFILES = {
    "init": (PHYSICAL_ADDRESS, FILE_OFFSET, EXPECTED_MNEMONICS),
    "syscfg-reset": (0x96c00d40, 0x3d40, SYS_CFG_RESET_MNEMONICS),
}


def decode_output(stdout: str) -> list[str]:
    """Parse one 4-byte instruction per line from Rizin's -D output."""
    instructions = []
    for line in stdout.splitlines():
        tokens = line.split()
        if len(tokens) < 5 or not re.fullmatch(r"0x[0-9a-fA-F]+", tokens[0]):
            continue
        if tokens[1] != "4" or not re.fullmatch(r"[0-9a-fA-F]{8}", tokens[2]):
            raise ValueError("unexpected Hexagon packet size or instruction width")
        # The fourth column marks start/continue/end in a Hexagon packet.
        # Reconstructed bytes are all singleton packets marked '['.
        if tokens[3] != "[":
            raise ValueError(f"expected a standalone Hexagon instruction packet: {line}")
        instructions.append(" ".join(tokens[4:]).lower().replace(" ", ""))
    return instructions


def verify(decoder: Path, generated: Path, reference: Path | None,
           profile: str = "init") -> None:
    phys, file_offset, instructions = PROFILES[profile]
    data = generated.read_bytes()
    expected_length = len(instructions) * 4
    if len(data) != expected_length:
        raise ValueError(f"expected {expected_length} CDSP {profile} bytes, got {len(data)}")
    command = [str(decoder), "-a", "hexagon", "-b", "32",
               "-o", f"0x{phys:x}", "-D", data.hex()]
    decoded = subprocess.run(command, text=True, capture_output=True, check=True)
    found = decode_output(decoded.stdout)
    expected = [value.replace(" ", "").lower() for value in instructions]
    if found != expected:
        for i, (seen, wanted) in enumerate(zip(found, expected)):
            if seen != wanted:
                raise ValueError(f"CDSP V66 instruction at +0x{4 * i:x} decoded as {seen}, expected {wanted}")
        raise ValueError(f"CDSP V66 decoded instruction count differs: {len(found)} != {len(expected)}")
    print(f"Rizin Hexagon decoded all {len(expected)} CDSP {profile} scalar packets with expected semantics")
    if reference is not None:
        image = reference.read_bytes()
        if image[file_offset:file_offset + len(data)] != data:
            raise ValueError(f"CDSP reference bytes differ from generated {profile} region")
        print(f"CDSP {profile} reference comparison: {len(data)} / {len(data)} bytes, zero mismatches")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rz-asm", type=Path, required=True)
    parser.add_argument("--rust-output", type=Path, required=True)
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--profile", choices=sorted(PROFILES), default="init")
    arguments = parser.parse_args()
    verify(arguments.rz_asm, arguments.rust_output, arguments.reference, arguments.profile)


if __name__ == "__main__":
    main()
