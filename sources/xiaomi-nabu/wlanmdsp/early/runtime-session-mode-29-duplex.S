// SPDX-License-Identifier: GPL-3.0-only
// The controller status value and target are one legal Hexagon duplex word.
.text
.globl session_mode_29_duplex
session_mode_29_duplex:
{ r2 = #29; jump 0x10 }
