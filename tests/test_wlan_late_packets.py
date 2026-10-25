# SPDX-License-Identifier: GPL-3.0-only
"""Boundary and malformed-packet checks independent of an external toolchain.

Synthetic words exercise only the documented structural format. Passing these
checks is not an assertion that a synthetic word has a legal instruction opcode.
"""
import importlib.util
from pathlib import Path
import struct
import sys
import unittest

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "wlan_late_packets", ROOT / "tools/wlan_late_packets.py")
assert SPEC is not None and SPEC.loader is not None
packet_module = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = packet_module
SPEC.loader.exec_module(packet_module)
parse_packets = packet_module.parse_packets
ParseError = packet_module.ParseError


def encoded(*words):
    return struct.pack("<" + "I" * len(words), *words)


def normal(parse=3):
    return 0x70000000 | (parse << 14)


def extender(parse=1):
    return 0x00000001 | (parse << 14)


def duplex(iclass=3):
    return ((iclass & 0xE) << 28) | ((iclass & 1) << 13)


class WlanLatePacketTests(unittest.TestCase):
    def test_empty_span(self):
        self.assertEqual(parse_packets(b""), [])

    def test_ordinary_packet_sizes_and_addresses(self):
        data = encoded(normal(), normal(1), normal(),
                       normal(1), normal(1), normal(),
                       normal(1), normal(1), normal(1), normal())
        packets = parse_packets(data, 0xB00C0000)
        self.assertEqual([p.word_count for p in packets], [1, 2, 3, 4])
        self.assertEqual([p.offset for p in packets], [0, 4, 12, 24])
        self.assertEqual(packets[-1].end_offset, len(data))
        self.assertEqual(packets[2].vaddr, 0xB00C000C)
        self.assertTrue(all(p.end_kind == "ordinary" for p in packets))

    def test_instruction_word_truncation(self):
        for tail_size in (1, 2, 3):
            with self.subTest(tail_size=tail_size), self.assertRaisesRegex(ParseError, "word"):
                parse_packets(encoded(normal()) + bytes(tail_size))

    def test_unterminated_packet_is_not_accepted_as_partial(self):
        for count in (1, 2, 3):
            with self.subTest(count=count), self.assertRaisesRegex(ParseError, "truncated packet"):
                parse_packets(encoded(*[normal(1)] * count))

    def test_four_continuations_fail_before_a_fifth_word(self):
        for suffix in ((), (normal(),)):
            with self.subTest(suffix=suffix), self.assertRaisesRegex(ParseError, "four encoded"):
                parse_packets(encoded(*([normal(1)] * 4), *suffix))

    def test_start_alignment_and_address_bounds(self):
        for start in (1, 2, 3, -4, 1 << 32, "0"):
            with self.subTest(start=start), self.assertRaises(ParseError):
                parse_packets(encoded(normal()), start)
        self.assertEqual(parse_packets(encoded(normal()), 0xFFFFFFFC)[0].vaddr,
                         0xFFFFFFFC)
        with self.assertRaisesRegex(ParseError, "address space"):
            parse_packets(encoded(normal(), normal()), 0xFFFFFFFC)

    def test_loop_end_flags_and_reset(self):
        data = encoded(normal(2), normal(),
                       normal(1), normal(2), normal(),
                       normal(2), normal(2), normal(), normal())
        self.assertEqual([(p.loop0, p.loop1) for p in parse_packets(data)],
                         [(True, False), (False, True), (True, True), (False, False)])

    def test_loop_marker_in_third_or_fourth_word_rejected(self):
        for index in (2, 3):
            with self.subTest(index=index), self.assertRaisesRegex(ParseError, "word 0 or 1"):
                parse_packets(encoded(*([normal(1)] * index), normal(2), normal()))

    def test_loop_markers_still_require_packet_terminator(self):
        for words in ((normal(2),), (normal(1), normal(2))):
            with self.subTest(words=words), self.assertRaisesRegex(ParseError, "truncated packet"):
                parse_packets(encoded(*words))

    def test_extender_is_counted_when_locating_loop_marker(self):
        p = parse_packets(encoded(extender(2), normal(2), normal()))[0]
        self.assertEqual((p.word_count, p.extender_count, p.loop0, p.loop1),
                         (3, 1, True, True))
        with self.assertRaisesRegex(ParseError, "word 0 or 1"):
            parse_packets(encoded(extender(), normal(1), normal(2), normal()))

    def test_duplex_ends_packet_and_class_zero_is_not_an_extender(self):
        p, following = parse_packets(encoded(duplex(0), normal()))
        self.assertEqual((p.word_count, p.end_kind, p.extender_count, p.duplex_class),
                         (1, "duplex", 0, 0))
        self.assertEqual(following.offset, 4)

    def test_reserved_duplex_class_rejected(self):
        with self.assertRaisesRegex(ParseError, "reserved"):
            parse_packets(encoded(duplex(15)))

    def test_all_nonreserved_duplex_classes_have_structural_shape(self):
        self.assertEqual([p.duplex_class for p in parse_packets(
            encoded(*(duplex(i) for i in range(15))))], list(range(15)))

    def test_duplex_instruction_limit_excludes_extenders(self):
        with self.assertRaisesRegex(ParseError, "more than four instructions"):
            parse_packets(encoded(normal(1), normal(1), normal(1), duplex()))
        for words in ((extender(), normal(1), normal(1), duplex()),
                      (normal(1), normal(1), extender(), duplex())):
            p = parse_packets(encoded(*words))[0]
            self.assertEqual((p.word_count, p.extender_count, p.end_kind), (4, 1, "duplex"))

    def test_extender_may_precede_duplex_or_loop_terminal_packet(self):
        p = parse_packets(encoded(extender(2), duplex()))[0]
        self.assertEqual((p.loop0, p.loop1, p.extender_count), (True, False, 1))
        p = parse_packets(encoded(normal(1), extender(2), duplex()))[0]
        self.assertEqual((p.loop0, p.loop1, p.extender_count), (False, True, 1))

    def test_extender_cannot_terminate_packet(self):
        with self.assertRaisesRegex(ParseError, "extender cannot terminate"):
            parse_packets(encoded(extender(3)))

    def test_consecutive_extenders_rejected(self):
        with self.assertRaisesRegex(ParseError, "consecutive"):
            parse_packets(encoded(extender(), extender(), normal()))
        p = parse_packets(encoded(extender(), normal(1), extender(), normal()))[0]
        self.assertEqual(p.extender_count, 2)

    def test_framing_signature_is_position_independent_but_preserves_markers(self):
        data = encoded(normal(2), duplex())
        self.assertEqual(packet_module.packet_signature(parse_packets(data)),
                         packet_module.packet_signature(parse_packets(data, 0xB00C0000)))
        self.assertNotEqual(packet_module.packet_signature(parse_packets(data)),
                            packet_module.packet_signature(parse_packets(
                                encoded(normal(1), duplex()))))


if __name__ == "__main__":
    unittest.main()
