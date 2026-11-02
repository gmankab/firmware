// SPDX-License-Identifier: GPL-3.0-only
// The register assignment and jump are packed as a single duplex word.
// LLVM6 cannot pair these with an extender in the same input packet, hence
// assemble the two semantic instructions separately from their extender.
.text
.globl session_state_duplex
session_state_duplex:
{ r1 = #63; jump 0x80 }
