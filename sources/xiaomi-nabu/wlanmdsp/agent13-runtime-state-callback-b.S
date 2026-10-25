// SPDX-License-Identifier: GPL-3.0-only
// State callback thunk: alternate selector for extended jump to 0x1d2f30.
// File [0x445b8,0x445c0).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_state_callback_b
agent13_wlan_state_callback_b:
L_045b8:
{
  r1 = #1; jump ##0x1ce978
}
