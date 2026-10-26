// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 00-get: original ADSP file [0x3ec80,0x3ec8c); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_00_get
adsp_late_controller_field_00_get:
{ r0 = ##-265506768; jump shared_field_lookup }
.set shared_field_lookup, adsp_late_controller_field_00_get+0xa4c
