// SPDX-License-Identifier: GPL-3.0-only
// The controller status value and target are one legal Hexagon duplex word.
.text
.globl session_mode_13_duplex
session_mode_13_duplex:
{ r2 = #13; jump 0xe0 }
