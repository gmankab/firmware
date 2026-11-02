// SPDX-License-Identifier: GPL-3.0-only
// The controller status value and target are one legal Hexagon duplex word.
.text
.globl session_mode_10_duplex
session_mode_10_duplex:
{ r2 = #10; jump 0x0 }
