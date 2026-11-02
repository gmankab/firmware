// SPDX-License-Identifier: GPL-3.0-only
// One Hexagon duplex (two compact scalar immediates in one encoded word).
// Independently assembled by LLVM6 and used only after checking the original
// two-word non-duplex placeholder packet and preserving parse-marker bits.
.text
.globl scalar_pair
scalar_pair:
{ r0 = #24; r1 = #-1 }
