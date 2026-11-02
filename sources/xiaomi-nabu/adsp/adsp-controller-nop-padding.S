// SPDX-License-Identifier: GPL-3.0-only
// 0x4b08-0x4c00: exact 248-byte executable NOP alignment/reserved block.
// No firmware bytes are imported: this is an explicit count of harmless
// single-word Hexagon NOP packets, not recovered controller logic.
.text
.globl adsp_controller_alignment
adsp_controller_alignment:
.rept 62
{ nop }
.endr
