// SPDX-License-Identifier: GPL-3.0-only
// Complementary state notification thunk, file [0x4444c,0x4445c).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_state_notify_b
agent13_wlan_state_notify_b:
L_0444c:
{
  r2 = r0
  immext(#0xac0)
  r1:0 = combine(#0x8,##0xac4)
}
L_04458:
{
  jump 0x216510
}
