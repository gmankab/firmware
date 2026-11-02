// SPDX-License-Identifier: GPL-3.0-only
// Source-reencoded original Hexagon V66 shared-ISA executable at ADSP file 0x3f64c.
.text
.globl adsp_late_qurt_trap_103
adsp_late_qurt_trap_103:
// 8-byte QuRT syscall wrapper; trap#103.
{ trap0(#103) }
{ jumpr r31 }
