// SPDX-License-Identifier: GPL-3.0-only
// Source-reencoded original Hexagon V66 shared-ISA executable at ADSP file 0x3f694.
.text
.globl adsp_late_qurt_trap_4
adsp_late_qurt_trap_4:
// QuRT platform syscall 4 with mode argument 3.
{ r0 = #3 }
{ trap0(#4) }
{ jumpr r31 }
