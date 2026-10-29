// SPDX-License-Identifier: GPL-3.0-only
// One-word Hexagon duplex: format case and branch-low operand.
.text
.globl session_mode9_duplex
session_mode9_duplex:
{ r2 = #9; jump 0x30 }
