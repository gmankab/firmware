// SPDX-License-Identifier: GPL-3.0-only
// ADSP firmware file [0x394b4,0x394b8): empty helper returning to caller.
// A one-instruction Hexagon function, not an alignment NOP.
.text
.globl adsp_late_return_only
adsp_late_return_only:
{ jumpr lr }
