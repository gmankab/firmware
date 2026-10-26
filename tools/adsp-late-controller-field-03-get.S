// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 03-get: original ADSP file [0x3ece0,0x3ecec); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_03_get
adsp_late_controller_field_03_get:
{ r0 = ##-265506624; jump shared_field_lookup }
.set shared_field_lookup, adsp_late_controller_field_03_get+0x9ec
