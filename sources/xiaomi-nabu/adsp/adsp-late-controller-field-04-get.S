// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller field 04-get: original ADSP file [0x3ecf0,0x3ecfc); validated symbolic field address and platform tail dispatch.
// This routine is distinct from adjacent alignment NOPs. No opcode words.
.text
.globl adsp_late_controller_field_04_get
adsp_late_controller_field_04_get:
{ r0 = ##-264959712; jump shared_field_lookup }
.set shared_field_lookup, adsp_late_controller_field_04_get+0x9dc
