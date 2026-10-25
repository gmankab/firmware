#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build modem TLB setup from independent Hexagon assembly and ISA templates.

The historical LLVM 6 assembler recognizes most Hexagon v60 instructions but
neither its privileged system/TLB instructions nor the original scheduling of
one predicate/jump packet. The .s macros encode those missing ISA forms via
checked field formulas; this driver restores the *semantic* packet order and
its in-word parse-bit markers, NOT copied opcode bytes.
"""
import argparse
import hashlib
from pathlib import Path
import struct
import subprocess
import sys
import tempfile

START = 0x3500
END = 0x35C4
ELF_LOAD_START = 0x3000
PREFIX_LEN = START - ELF_LOAD_START
PART_LEN = END - START
EXPECTED_MODEM_SHA256 = "6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782"
MARKER_MASK = 0b11 << 14
CONTINUE = 0b01 << 14
END_PACKET = 0b11 << 14
JUMP_GROUP_VMA = 0x3558


def text_from_object(obj: bytes) -> bytes:
    """Extract the one executable .text section; fail closed on malformed ELF."""
    if obj[:6] != b"\x7fELF\x01\x01" or len(obj) < 52:
        raise ValueError("expected ELF32 little-endian Hexagon assembler object")
    _, machine = struct.unpack_from("<HH", obj, 16)
    if machine != 164:
        raise ValueError(f"assembler object machine is {machine}, expected Hexagon 164")
    shoff = struct.unpack_from("<I", obj, 32)[0]
    shentsize, shcount = struct.unpack_from("<HH", obj, 46)
    if shentsize != 40 or shcount == 0 or shoff + shcount * shentsize > len(obj):
        raise ValueError("invalid ELF section table")
    sections = [struct.unpack_from("<IIIIIIIIII", obj, shoff + i * shentsize)
                for i in range(shcount)]
    text = [s for s in sections if s[1] == 1 and (s[2] & 4) and s[5]]
    if len(text) != 1:
        raise ValueError(f"expected exactly one executable PROGBITS section, got {len(text)}")
    start, length = text[0][4:6]
    if start + length > len(obj):
        raise ValueError(".text section exceeds input file")
    return obj[start:start+length]


def restore_predicated_packet(program: bytes) -> bytes:
    """Restore original instruction order in one legal four-instruction packet.

    LLVM's scheduler moves the conditional branch ahead of two predicated
    arithmetic instructions. In this modem boot routine the packet instead
    executes insert, conditional add-not, conditional add, conditional jump.
    Hexagon parse bits are instruction-local and updated after ordering.
    """
    if len(program) != PART_LEN:
        raise ValueError(f"expected exactly {PART_LEN} TLB init bytes, got {len(program)}")
    words = list(struct.unpack("<" + "I"*(len(program)//4),program))
    pos=(JUMP_GROUP_VMA-START)//4
    initial=words[pos:pos+4]
    if len(initial) != 4 or [w & MARKER_MASK for w in initial] != [CONTINUE,CONTINUE,CONTINUE,END_PACKET]:
        raise ValueError("unexpected 4-instruction packet layout; refuse to reorder")
    first, jump, add_not, add_yes = initial
    # Class checking from the open Hexagon ISA masks catches incompatible
    # toolchain scheduling changes before reordering any bytes.
    if (jump & 0xff201c01) != 0x5c200000 or \
       (add_not & 0xff802000) != 0x74800000 or \
       (add_yes & 0xff802000) != 0x74000000:
        raise ValueError("predicated arithmetic/jump packet classes differ from expected")
    words[pos:pos+4] = [first,
        (add_not & ~MARKER_MASK)|CONTINUE,
        (add_yes & ~MARKER_MASK)|CONTINUE,
        (jump & ~MARKER_MASK)|END_PACKET]
    return struct.pack("<"+"I"*len(words),*words)


def main(argv=None):
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--llvm-mc", required=True, type=Path, help="Hexagon-enabled LLVM 6 llvm-mc executable")
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--prefix", type=Path, help="1,280-byte typed modem layout from modem-rebuild emit-layout")
    parser.add_argument("--reference", type=Path, help="Private unmodified modem.mbn, for optional exact-byte comparison")
    opts=parser.parse_args(argv)
    source=Path(__file__).resolve().parents[1]/"crates/firmware-tools/src/devices/xiaomi_nabu/modem_tlb_init.s"
    with tempfile.TemporaryDirectory(prefix="hexagon-tlb-") as tmp:
        generated=Path(tmp)/"modem-tlb.o"
        proc=subprocess.run([str(opts.llvm_mc),"-triple=hexagon","-mcpu=hexagonv60", "-filetype=obj",str(source),"-o",str(generated)],capture_output=True,text=True)
        if proc.returncode:
            raise RuntimeError("Hexagon assembler failed: " + proc.stderr)
        assembled=restore_predicated_packet(text_from_object(generated.read_bytes()))
    if opts.prefix is None:
        out=assembled
    else:
        prefix=opts.prefix.read_bytes()
        if len(prefix)!=PREFIX_LEN:
            raise ValueError(f"prefix is {len(prefix)} bytes; expected {PREFIX_LEN}")
        out=prefix+assembled
    if opts.reference is not None:
        reference=opts.reference.read_bytes()
        reference_digest=hashlib.sha256(reference).hexdigest()
        if reference_digest!=EXPECTED_MODEM_SHA256:
            raise ValueError(f"wrong modem.mbn reference revision: {reference_digest}")
        start=ELF_LOAD_START if opts.prefix else START
        if len(reference)<start+len(out):raise ValueError("truncated private reference")
        expected=reference[start:start+len(out)]
        if out!=expected:
            offset=next(i for i,(got,want) in enumerate(zip(out,expected)) if got!=want)
            raise ValueError(f"byte mismatch at modem file 0x{start+offset:x}: rebuilt 0x{out[offset]:02x}, reference 0x{expected[offset]:02x}")
        print(f"Reference verified: {len(out)} bytes at file [{start:#x},{start+len(out):#x}); code 68 + 196, metadata 412, fill 800 when prefix provided")
    opts.output.write_bytes(out)
    print(f"Emitted {len(out)} bytes; sha256 {hashlib.sha256(out).hexdigest()} (PARTIAL modem only)")

if __name__ == "__main__":
    try:main()
    except (ValueError,RuntimeError,OSError) as e:
        print(f"ERROR: {e}",file=sys.stderr)
        sys.exit(1)
