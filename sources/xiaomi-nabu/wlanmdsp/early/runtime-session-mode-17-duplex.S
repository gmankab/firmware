// SPDX-License-Identifier: GPL-3.0-only
// The controller status value and target are one legal Hexagon duplex word.
.text
.globl session_mode_17_duplex
session_mode_17_duplex:
{ r2 = #17; jump 0xa0 }
