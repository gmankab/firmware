// SPDX-License-Identifier: GPL-3.0-only
// Small allocator wrapper dispatching allocation mode 3 to the QuRT helper.
// File [0x434a8,0x434b4), exact Hexagon V66 packet encoding.
// Independently assembled instructions, no extracted opcode arrays.
.text
.globl agent13_wlan_record_allocator_tail
agent13_wlan_record_allocator_tail:
{
  r2 = r0
}
{
  r1:0 = combine(r2,#0x3)
  jump 0x219484
}
