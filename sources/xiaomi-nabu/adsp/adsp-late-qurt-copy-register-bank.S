// SPDX-License-Identifier: GPL-3.0-only
// V66 shared Hexagon source for private ADSP file 0x3f76c
.text
.globl adsp_late_qurt_copy_register_bank
adsp_late_qurt_copy_register_bank:
// Copy a variable count of packed controller slots using loop0.
{ loop0(copy_slot,r1); immext(#4029460672); r4 = ##4029460732; r5 = r0 }
copy_slot:
{ r3:2 = memd(r5++#8) }
{ r0 = #0; memw(r4+r2<<#2) = r3 }:endloop0
{ jumpr r31 }
