// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon V66 cycle-accumulator leaf [file 0x139c8,0x139dc).
.text
.globl adsp_controller_cycle_helper
adsp_controller_cycle_helper:
{ nop } // source-checked R1:0=PCYCLE
{ immext(#0xf0010e00); r3:2=memd(gp+##0xf0010e20) }
{ r1:0=add(r3:2,r1:0); jump controller_accumulate }
.set controller_accumulate,adsp_controller_cycle_helper+(0x8be248fc-0x8be139c8)
