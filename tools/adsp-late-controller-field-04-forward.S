// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 04-forward: original ADSP file [0x3ed0c,0x3ed1c); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_04_forward
adsp_late_controller_field_04_forward:
{ r2 = r0 }
{ r1:0 = combine(r2,##-264959712); jump shared_field_update }
.set shared_field_update, adsp_late_controller_field_04_forward+0x9ac
