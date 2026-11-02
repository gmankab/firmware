// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 02-get: original ADSP file [0x3ecc0,0x3eccc); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_02_get
adsp_late_controller_field_02_get:
{ r0 = ##-265506672; jump shared_field_lookup }
.set shared_field_lookup, adsp_late_controller_field_02_get+0xa0c
