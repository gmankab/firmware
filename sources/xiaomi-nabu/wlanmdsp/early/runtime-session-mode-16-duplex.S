// SPDX-License-Identifier: GPL-3.0-only
// The controller status value and target are one legal Hexagon duplex word.
.text
.globl session_mode_16_duplex
session_mode_16_duplex:
{ r2 = #16; jump 0xc0 }
