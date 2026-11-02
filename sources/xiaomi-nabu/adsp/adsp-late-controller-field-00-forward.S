// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 00-forward: original ADSP file [0x3ec8c,0x3ec9c); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_00_forward
adsp_late_controller_field_00_forward:
{ r2 = r0 }
{ r1:0 = combine(r2,##-265506768); jump shared_field_update }
.set shared_field_update, adsp_late_controller_field_00_forward+0xa2c
