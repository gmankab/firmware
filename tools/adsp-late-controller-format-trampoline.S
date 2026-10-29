// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3bb40,0x3bb4c): forward controller constant 3 and platform pointer.
// The alignment NOP [0x3bb3c,0x3bb40) is *not* part of this routine.
.text
.globl adsp_late_controller_format_trampoline
adsp_late_controller_format_trampoline:
{ immext(#4028149760); r1:0 = combine(#3,##4028149784); jump format_trampoline_destination }
.set format_trampoline_destination, adsp_late_controller_format_trampoline+0x3c2c
