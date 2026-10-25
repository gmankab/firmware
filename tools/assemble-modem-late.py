#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build two independent Hexagon v60 modem *later segment* function groups.

The generated fragments are source-derived, not copied from the OEM firmware.
The original reference is only needed for optional SHA-pinned verification.
All offsets below are ELF file offsets; virtual and physical addresses differ.
"""
from __future__ import annotations

import argparse
import hashlib
from pathlib import Path
import struct
import subprocess
import sys
import tempfile

EXPECTED_REFERENCE_SHA256 = "6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782"
ROOT = Path(__file__).resolve().parents[1]
ATOMIC_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_atomic.s"
AUX_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_boot.s"
CONTROL_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_control.s"
ATOMIC_START = 0x7C000
ATOMIC_END = 0x7C040
CONTROL_START = 0x7C040
CONTROL_END = 0x7C0A0
AUX_START = 0x446000
AUX_END = 0x4460D8
MARKER_MASK = 3 << 14
CONTINUE_MARKER = 1 << 14
END_MARKER = 3 << 14


def extract_text(obj: bytes) -> bytes:
    """Return exactly one executable Hexagon ELF32 little-endian PROGBITS."""
    if not obj.startswith(b"\x7fELF\x01\x01") or len(obj) < 52:
        raise ValueError("expected little-endian ELF32 assembler object")
    machine = struct.unpack_from("<H", obj, 18)[0]
    if machine != 164:
        raise ValueError(f"assembler object e_machine={machine}, expected Hexagon 164")
    shoff = struct.unpack_from("<I", obj, 32)[0]
    shentsize, shnum = struct.unpack_from("<HH", obj, 46)
    if shentsize != 40 or not shnum or shoff + shnum * shentsize > len(obj):
        raise ValueError("invalid assembler section directory")
    sections = [struct.unpack_from("<10I", obj, shoff + i * shentsize) for i in range(shnum)]
    text = [s for s in sections if s[1] == 1 and (s[2] & 4) and s[5]]
    if len(text) != 1:
        raise ValueError(f"expected one nonempty executable PROGBITS section, got {len(text)}")
    start, length = text[0][4:6]
    if start + length > len(obj):
        raise ValueError("assembler .text section exceeds object file")
    return obj[start : start + length]


def restore_atomic_packet_order(assembled: bytes) -> bytes:
    """Correct LLVM6's legal, but non-byte-identical scheduling at 0x7c010.

    Original packet source states the predicated ADD *before* the compound
    `cmp.eq; if(p0.new)jump`. LLVM6 places that compound instruction first.
    Class, marker and register checks reject changes before restoring only
    the original source order and packet-end marker bits. No reference opcode
    bytes are inserted here.
    """
    if len(assembled) != ATOMIC_END - ATOMIC_START or len(assembled) % 4:
        raise ValueError(f"late modem atomics require {ATOMIC_END - ATOMIC_START} bytes")
    words = list(struct.unpack("<" + "I" * (len(assembled) // 4), assembled))
    branch, predicated_add = words[4:6]
    if branch >> 24 != 0x10 or predicated_add >> 24 != 0x74:
        raise ValueError("expected compound comparison-branch followed by predicated ADD")
    if branch & MARKER_MASK != CONTINUE_MARKER or predicated_add & MARKER_MASK != END_MARKER:
        raise ValueError("unexpected Hexagon packet parse markers at modem 0x7c010")
    words[4:6] = [(predicated_add & ~MARKER_MASK) | CONTINUE_MARKER,
                  (branch & ~MARKER_MASK) | END_MARKER]
    return struct.pack("<" + "I" * len(words), *words)


def restore_controller_packet_order(assembled: bytes) -> bytes:
    """Restore two compare+immediate packets and a three-word control packet.

    LLVM6 schedulers move new-value branch consumers ahead of the preceding
    extended immediate/assignment and terminate the nearby parameter move
    prematurely. Each rewrite is checked against documented Hexagon opcode
    *classes* and parse markers; no OEM opcode data is used.
    """
    if len(assembled) != CONTROL_END - CONTROL_START:
        raise ValueError("late modem controller requires 96 source-built bytes")
    words = list(struct.unpack("<" + "I" * (len(assembled)//4),assembled))
    for first, value_class in [(0x14//4, 0x78), (0x4c//4, 0x7e)]:
        pred, extender, value = words[first:first+3]
        if [w >> 24 for w in (pred,extender,value)] != [0x10,0x0c,value_class]:
            raise ValueError(f"controller conditional/extended packet classes changed at {CONTROL_START + first*4:#x}")
        if [w & MARKER_MASK for w in (pred,extender,value)] != [CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
            raise ValueError(f"controller conditional/extended packet markers changed at {CONTROL_START + first*4:#x}")
        words[first:first+3] = [extender,
                               (value & ~MARKER_MASK) | CONTINUE_MARKER,
                               (pred & ~MARKER_MASK) | END_MARKER]
    # With a valid *source-assembled* three-word sequence, change only the
    # MOV's end-of-packet to continue so it shares the following duplex
    # memory-load/immediate instruction, as the original producer ordering.
    extender, move, duplex = words[0x2c//4:0x38//4]
    if [w >> 24 for w in (extender,move,duplex)] != [0x0c,0x78,0x48]:
        raise ValueError("controller extended constant and duplex instruction classes changed")
    if [w & MARKER_MASK for w in (extender,move,duplex)] != [CONTINUE_MARKER,END_MARKER,0]:
        raise ValueError("unexpected controller param-load packet markers")
    words[0x30//4]=(move & ~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<"+"I"*len(words),*words)


def assemble(llvm_mc: Path, source: Path) -> bytes:
    with tempfile.TemporaryDirectory(prefix="nabu-modem-late-") as tmp:
        obj = Path(tmp) / "late.o"
        command = [str(llvm_mc), "-triple=hexagon", "-mcpu=hexagonv60",
                   "-filetype=obj", str(source), "-o", str(obj)]
        result = subprocess.run(command, text=True, capture_output=True)
        if result.returncode:
            raise RuntimeError(f"Hexagon LLVM MC assembly failed for {source.name}: {result.stderr}")
        return extract_text(obj.read_bytes())


def output_fragments(llvm_mc: Path) -> dict[int, bytes]:
    atomic = restore_atomic_packet_order(assemble(llvm_mc, ATOMIC_SOURCE))
    controller = restore_controller_packet_order(assemble(llvm_mc, CONTROL_SOURCE))
    auxiliary = assemble(llvm_mc, AUX_SOURCE)
    if len(auxiliary) != AUX_END - AUX_START:
        raise ValueError(f"auxiliary boot source emitted {len(auxiliary)} instead of {AUX_END-AUX_START} bytes")
    return {ATOMIC_START: atomic + controller, AUX_START: auxiliary}


def executable_segments(reference: bytes) -> dict[int, tuple[int, int, int, int]]:
    if reference[:6] != b"\x7fELF\x01\x01" or len(reference) < 52:
        raise ValueError("expected ELF32 little-endian original modem MBN")
    if struct.unpack_from("<H", reference, 18)[0] != 164:
        raise ValueError("original MBN is not Hexagon ELF")
    phoff = struct.unpack_from("<I", reference, 28)[0]
    phentsize, phnum = struct.unpack_from("<HH", reference, 42)
    if phentsize != 32 or phoff + phnum * phentsize > len(reference):
        raise ValueError("invalid original modem ELF program headers")
    segments = {}
    for i in range(phnum):
        ptype, offset, vma, pma, filesz, memsz, flags, align = struct.unpack_from(
            "<8I", reference, phoff + i * phentsize
        )
        if ptype == 1 and flags & 1:
            if offset in segments:
                raise ValueError(f"duplicate executable load at file offset {offset:#x}")
            if filesz > memsz or offset + filesz > len(reference):
                raise ValueError(f"invalid executable load at {offset:#x}")
            segments[offset] = (vma, pma, filesz, memsz)
    return segments


def verify_reference(original: bytes, fragments: dict[int, bytes]) -> None:
    digest = hashlib.sha256(original).hexdigest()
    if digest != EXPECTED_REFERENCE_SHA256:
        raise ValueError(f"wrong modem firmware revision, SHA256={digest}")
    segments = executable_segments(original)
    expected_map = {
        0x7C000: (0xC0890000, 0x8E090000, 0x183CF0),
        0x446000: (0x1E400000, 0x90000000, 0x11DA4),
    }
    for start, (vma, pma, min_filesz) in expected_map.items():
        actual = segments.get(start)
        if actual is None or actual[:3] != (vma, pma, min_filesz):
            raise ValueError(f"unexpected mapped executable segment {start:#x}: {actual}")
    for start, rebuilt in fragments.items():
        vma, pma, filesz, memsz = segments[start]
        if len(rebuilt) > filesz:
            raise ValueError(f"generated fragment at {start:#x} exceeds executable p_filesz")
        expected = original[start : start + len(rebuilt)]
        if expected != rebuilt:
            i = next(i for i, (a, b) in enumerate(zip(expected, rebuilt)) if a != b)
            raise ValueError(f"Hexagon mismatch at modem file {start+i:#x}: expected {expected[i]:02x}, generated {rebuilt[i]:02x}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--llvm-mc", required=True, type=Path)
    ap.add_argument("--reference", type=Path, help="original pinned modem.mbn stored outside Git")
    ap.add_argument("--output-dir", type=Path, help="directory for the generated, *partial* executable fragments")
    args = ap.parse_args()
    fragments = output_fragments(args.llvm_mc)
    if args.reference:
        verify_reference(args.reference.read_bytes(), fragments)
    if args.output_dir:
        args.output_dir.mkdir(parents=True, exist_ok=True)
        for start, blob in fragments.items():
            path = args.output_dir / f"modem-late-file-{start:06x}-{start+len(blob):06x}.bin"
            path.write_bytes(blob)
    for start, blob in fragments.items():
        print(f"Hexagon V60 independent source: file [{start:#x},{start+len(blob):#x}) "
              f"= {len(blob)} bytes, SHA256={hashlib.sha256(blob).hexdigest()}")
    print(f"Total generated {sum(len(x) for x in fragments.values())} DISJOINT partial modem bytes; "
          f"original reference {'matched' if args.reference else 'not supplied'}; "
          "NOT complete, not new OEM authentication")
    return 0

if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, RuntimeError, OSError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        sys.exit(1)
