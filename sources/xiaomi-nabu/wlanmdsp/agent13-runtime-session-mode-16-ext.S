// SPDX-License-Identifier: GPL-3.0-only
// The controller case branch uses a signed upper immediate + short duplex.
// These are Hexagon semantic operands; LLVM6 assembles the extender itself.
.text
.globl session_mode_16_ext
session_mode_16_ext:
{ jump ##0x480 }
