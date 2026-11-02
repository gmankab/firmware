// SPDX-License-Identifier: GPL-3.0-only
// The controller status value and target are one legal Hexagon duplex word.
.text
.globl session_mode_51_duplex
session_mode_51_duplex:
{ r2 = #51; jump 0x60 }
