# SPDX-License-Identifier: GPL-3.0-only
"""Structural checks for little-endian Hexagon instruction packets.

This is deliberately not an opcode decoder. A successful parse does not prove
that words are code, that opcodes/slot assignments/dataflow are legal, or that
the supplied start is a packet or function entry. Callers must establish code
boundaries independently and use a Hexagon v66 assembler/decoder as well.
Do not pass data tables or linker fill to this code-only validator.

Primary references (common packet format used by LLVM's Hexagon targets):
* https://llvm.org/doxygen/HexagonBaseInfo_8h_source.html
  InstParseBits and InstIClassBits.
* https://llvm.org/doxygen/HexagonDisassembler_8cpp_source.html
  getSingleInstruction: loop-marker positions and duplex classes.
* https://llvm.org/doxygen/HexagonMCCodeEmitter_8cpp_source.html
  parseBits: final duplex/normal word and encoded-word loop positions.
* https://llvm.org/doxygen/HexagonMCTargetDesc_8h_source.html
  HEXAGON_PACKET_SIZE and end-loop minimum word counts.
* https://docs.qualcomm.com/bundle/publicresource/80-N2040-53_REV_AB_Qualcomm_Hexagon_V73_Programmers_Reference_Manual.pdf
  Sections 10.3-10.6 and 10.9 corroborate the common format, duplex classes,
  four-word bound, and constant-extender adjacency. This later manual is not
  used as an opcode whitelist for v66.
"""
from __future__ import annotations

from dataclasses import dataclass
import struct


class ParseError(ValueError):
    """A span violates a structural packet encoding requirement."""


@dataclass(frozen=True)
class Packet:
    """Packet framing; offset is in bytes relative to the supplied span.

    word_count includes constant extenders and counts a duplex as one word.
    The loop flags describe implicit end-loop operations on the whole packet.
    end_kind is either ``ordinary`` (parse 11) or ``duplex`` (parse 00).
    extender_count does not imply the following opcode can accept an extender.
    """

    offset: int
    vaddr: int
    word_count: int
    end_kind: str
    loop0: bool
    loop1: bool
    extender_count: int
    duplex_class: int | None = None

    @property
    def size_bytes(self) -> int:
        return 4 * self.word_count

    @property
    def end_offset(self) -> int:
        return self.offset + self.size_bytes


def parse_packets(data: bytes, start_vaddr: int = 0) -> list[Packet]:
    """Parse a complete code span starting at an independently known boundary.

    Parse bits are word[15:14]. Values 01 and 10 continue a packet; 10 also
    marks loop0 at word index 0 or loop1 at index 1. Values 11 and 00 end a
    packet, with 00 identifying the final duplex word. Every encoded word,
    including an extender, contributes to the index and the four-word bound.
    """
    if not isinstance(start_vaddr, int) or not 0 <= start_vaddr < (1 << 32):
        raise ParseError("start_vaddr must be an unsigned 32-bit address")
    if start_vaddr % 4:
        raise ParseError("start_vaddr must be four-byte aligned")
    if len(data) % 4:
        raise ParseError("truncated instruction word: span length is not a multiple of four")
    if start_vaddr + len(data) > (1 << 32):
        raise ParseError("code span exceeds the 32-bit address space")

    packets: list[Packet] = []
    packet_start = 0
    word_count = 0
    extender_count = 0
    previous_extender = False
    loop0 = loop1 = False

    def fail(at: int, reason: str) -> None:
        raise ParseError(f"offset 0x{at:x}, vaddr 0x{start_vaddr + at:x}: {reason}")

    for (word,) in struct.iter_unpack("<I", data):
        at = packet_start + 4 * word_count
        index = word_count
        word_count += 1
        parse = (word >> 14) & 3

        if parse == 2:
            if index == 0:
                loop0 = True
            elif index == 1:
                loop1 = True
            else:
                fail(at, "loop marker is only valid in encoded word 0 or 1")

        # Parse bits take precedence: a class-0 duplex is not an extender.
        is_extender = parse != 0 and (word >> 28) == 0
        if is_extender:
            if previous_extender:
                fail(at, "consecutive constant extenders have no intervening instruction")
            if parse == 3:
                fail(at, "constant extender cannot terminate a packet")
            extender_count += 1
        previous_extender = is_extender

        if parse in (0, 3):
            duplex_class = None
            if parse == 0:
                duplex_class = ((word >> 28) & 0xE) | ((word >> 13) & 1)
                if duplex_class == 15:
                    fail(at, "duplex class 15 is reserved")
                # The duplex contains two instructions. Extenders occupy
                # encoded words but no execution slots, so four-word packets
                # ending in a duplex can be legal when one word is an extender.
                if word_count - extender_count + 1 > 4:
                    fail(at, "duplex packet contains more than four instructions")
            packets.append(Packet(packet_start, start_vaddr + packet_start,
                                  word_count, "duplex" if parse == 0 else "ordinary",
                                  loop0, loop1, extender_count, duplex_class))
            packet_start = at + 4
            word_count = extender_count = 0
            previous_extender = False
            loop0 = loop1 = False
        elif word_count == 4:
            fail(at, "packet continues beyond the maximum of four encoded words")

    if word_count:
        fail(packet_start, "truncated packet: final word does not terminate it")
    return packets


def packet_signature(packets: list[Packet]) -> tuple[tuple, ...]:
    """Position-independent framing for comparing two generated/reference spans.

    This supplements, and never replaces, full byte equality and ISA decoding.
    """
    return tuple((p.offset, p.word_count, p.end_kind, p.loop0, p.loop1,
                  p.extender_count, p.duplex_class) for p in packets)
