// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 01-get: original ADSP file [0x3eca0,0x3ecac); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_01_get
adsp_late_controller_field_01_get:
{ r0 = ##-265506720; jump shared_field_lookup }
.set shared_field_lookup, adsp_late_controller_field_01_get+0xa2c
