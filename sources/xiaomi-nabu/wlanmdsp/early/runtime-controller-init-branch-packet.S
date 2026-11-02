// SPDX-License-Identifier: GPL-3.0-only
// First two semantic operations of a four-word packet with extended branch.
// Hexagon V66 packet parser's final/continue marker is updated in checked
// packaging code; all instruction bits originate from LLVM6 assembly.
.text
.globl agent13_controller_compound_branch
agent13_controller_compound_branch:
{
  p0 = cmp.eq(r2,#0); if (p0.new) jump:nt ##0x434
}
