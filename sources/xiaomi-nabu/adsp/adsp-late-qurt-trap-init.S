// SPDX-License-Identifier: GPL-3.0-only
// V66 shared Hexagon source for private ADSP file 0x3f700
.text
.globl adsp_late_qurt_trap_init
adsp_late_qurt_trap_init:
// Seven-instruction QuRT startup trap wrapper, after four NOPs excluded.
{ r0 = #3 }
{ r1 = #0 }
{ r2 = #0 }
{ r3 = #-1 }
{ r4 = #1 }
{ trap0(#103) }
{ jumpr r31 }
