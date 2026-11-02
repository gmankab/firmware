// SPDX-License-Identifier: GPL-3.0-only
// State notification thunk, file [0x4443c,0x4444c).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_state_notify_a
agent13_wlan_state_notify_a:
L_0443c:
{
  r2 = r0
  immext(#0xa00)
  r1:0 = combine(#0x5,##0xa03)
}
L_04448:
{
  jump 0x216520
}
