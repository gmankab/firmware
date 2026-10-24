#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently assemble CDSP's named nine-instruction entry vector.

This verifier never writes the original firmware, and it contains no
reference instruction-word arrays. It assembles fresh Hexagon assembly and
compares just the independently emitted code. LLVM 6 Hexagon v60 accepts
these nine scalar packets; this does NOT decode later V66-only instructions.
"""

import argparse
from pathlib import Path
import struct
import subprocess
import tempfile

# Typed per-packet assembly is kept independent of the Rust Hexagon encoder.
PACKETS = (
    "jump 0x1c",     # initial selector to the final vector slot
    "jump 0", "jump 0", "jump 0", "jump 0", "jump 0", "jump 0",
    "jump 0xc90",    # branch to physical CDSP entry + 0xcac
    "r0 = #0",       # initialize first argument before controller setup
)
FILE_OFFSET = 0x3000


def parse_elf32_text(binary: bytes) -> bytes:
    """Extract .text from a little-endian 32-bit assembler output object."""
    if len(binary) < 52 or binary[:6] != b"\x7fELF\x01\x01":
        raise ValueError("Hexagon assembler did not produce ELF32 little-endian")
    shoff = struct.unpack_from("<I", binary, 32)[0]
    shentsize = struct.unpack_from("<H", binary, 46)[0]
    shnum = struct.unpack_from("<H", binary, 48)[0]
    shstrndx = struct.unpack_from("<H", binary, 50)[0]
    if not shentsize or shentsize < 40 or shnum <= shstrndx:
        raise ValueError("malformed ELF section table")
    if shoff + shentsize * shnum > len(binary):
        raise ValueError("truncated ELF section table")

    def section(i: int) -> tuple[int, int, int]:
        at = shoff + shentsize * i
        name, _, _, _, off, size = struct.unpack_from("<IIIIII", binary, at)
        if off + size > len(binary):
            raise ValueError("out-of-bounds assembler section")
        return name, off, size

    _, off, size = section(shstrndx)
    names = binary[off:off + size]
    for i in range(shnum):
        name, off, size = section(i)
        end = names.find(b"\0", name)
        if name < len(names) and end >= 0 and names[name:end] == b".text":
            return binary[off:off + size]
    raise ValueError("assembler .text section not found")


def make_asm() -> str:
    return (".text\n.global cdsp_boot_entry\ncdsp_boot_entry:\n" +
            "".join("{ " + inst + " }\n" for inst in PACKETS))


def verify(llvm_mc: Path, rust_output: Path, reference: Path | None) -> None:
    with tempfile.TemporaryDirectory(prefix="nabu-cdsp-assembler-") as directory:
        asm = Path(directory) / "entry.s"
        obj = Path(directory) / "entry.o"
        asm.write_text(make_asm())
        subprocess.run([str(llvm_mc), "-triple=hexagon", "-mcpu=hexagonv60",
                        "-filetype=obj", "-o", str(obj), str(asm)], check=True)
        constructed = parse_elf32_text(obj.read_bytes())
    generated = rust_output.read_bytes()
    if len(constructed) != 36 or constructed != generated:
        raise ValueError("Rust and independent LLVM Hexagon entry encoders disagree")
    print(f"LLVM and Rust independently match: {len(constructed)} instruction bytes")
    if reference is not None:
        image = reference.read_bytes()
        end = FILE_OFFSET + len(constructed)
        if len(image) < end or image[FILE_OFFSET:end] != constructed:
            raise ValueError("CDSP firmware entry does not match independent Hexagon code")
        print(f"Reference match: file [0x{FILE_OFFSET:x}, 0x{end:x}), 36 / 36 bytes")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--llvm-mc", required=True, type=Path,
                        help="llvm-mc binary with Hexagon backend (LLVM 6 or newer)")
    parser.add_argument("--rust-output", required=True, type=Path,
                        help="Output of cargo run -p firmware-tools --bin cdsp-rebuild -- emit-entry ...")
    parser.add_argument("--reference", type=Path,
                        help="private CDSP MBN, checked against the assembled entry only")
    args = parser.parse_args()
    verify(args.llvm_mc, args.rust_output, args.reference)


if __name__ == "__main__":
    main()
