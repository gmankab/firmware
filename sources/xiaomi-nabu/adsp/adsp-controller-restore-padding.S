// SPDX-License-Identifier: GPL-3.0-only
// Hexagon reset-vector packet alignment, file [0x4d1c, 0x4e1c).
// 57 semantically explicit no-ops followed by seven error-handler jumps.
.text
.globl adsp_controller_restore_padding
adsp_controller_restore_padding:
{ nop }
.rept 14
{ nop; nop; nop; nop }
.endr
{ jump context_init }
reset_trap_1: { jump reset_trap_1 }
reset_trap_2: { jump reset_trap_2 }
reset_trap_3: { jump reset_trap_3 }
reset_trap_4: { jump reset_trap_4 }
reset_trap_5: { jump reset_trap_5 }
reset_trap_6: { jump reset_trap_6 }
.set context_init, adsp_controller_restore_padding + 0x100
