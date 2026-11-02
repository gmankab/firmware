// SPDX-License-Identifier: GPL-3.0-only
// State callback thunk: explicitly extended packet-relative tail jump to 0x1d2f30.
// File [0x445b0,0x445b8).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_state_callback_a
agent13_wlan_state_callback_a:
L_045b0:
{
  r1 = #0; jump ##0x1ce980
}
