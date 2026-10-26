// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 01-forward: original ADSP file [0x3ecac,0x3ecbc); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_01_forward
adsp_late_controller_field_01_forward:
{ r2 = r0 }
{ r1:0 = combine(r2,##-265506720); jump shared_field_update }
.set shared_field_update, adsp_late_controller_field_01_forward+0xa0c
