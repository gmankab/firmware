// SPDX-License-Identifier: GPL-3.0-only
// The controller status value and target are one legal Hexagon duplex word.
.text
.globl session_mode_40_duplex
session_mode_40_duplex:
{ r2 = #40; jump 0xb0 }
