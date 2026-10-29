// SPDX-License-Identifier: GPL-3.0-only
// V66 shared Hexagon source for private ADSP file 0x3f74c
.text
.globl adsp_late_qurt_clear_register_bank
adsp_late_qurt_clear_register_bank:
// Clear 257 indexed controller registers using loop0 packet mechanics.
{ loop0(clear_slot,#257) }
{ immext(#4029460672); r2 = ##-265506564; immext(#4027742400); r1 = ##-267224888 }
clear_slot:
{ r0 = #0; memw(r2++#4) = r1 }:endloop0
{ jumpr r31 }
