// SPDX-License-Identifier: GPL-3.0-only
// The jump target displacement is 0x4a0 from the packet at offset 0x28.
// This `##` instruction causes LLVM to emit the signed immediate extender.
// The extender is consumed by the short-duplex jump in the second file.
.text
.globl session_state_extended_jump
session_state_extended_jump:
{ jump ##0x4a0 }
