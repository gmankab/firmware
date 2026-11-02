// SPDX-License-Identifier: GPL-3.0-only
// First two predicate instructions of the original three-word Hexagon packet.
// LLVM 6 with -mno-pairing preserves both predicate slots; its following two
// non-duplex instructions are a compiler artifact and are NOT emitted by the
// source builder. The packet is completed by source-built duplex instruction.
.text
.globl agent13_wlan_bitmap_scan_predicate_group
agent13_wlan_bitmap_scan_predicate_group:
{
  p0 = cmp.eq(r2,#0)
  p1 = cmp.eq(r2,#1)
  r2 = r6
  r0 = #-1
}
