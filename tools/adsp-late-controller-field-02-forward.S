// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 02-forward: original ADSP file [0x3eccc,0x3ecdc); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_02_forward
adsp_late_controller_field_02_forward:
{ r2 = r0 }
{ r1:0 = combine(r2,##-265506672); jump shared_field_update }
.set shared_field_update, adsp_late_controller_field_02_forward+0x9ec
