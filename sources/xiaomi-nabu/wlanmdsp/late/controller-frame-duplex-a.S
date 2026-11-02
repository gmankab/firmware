// SPDX-License-Identifier: GPL-3.0-only
// Three independently assembled named Hexagon mini-packets for controller input pointer, stack pointer and zeroed descriptor pair.
// Packet kinds must be 11/11/00 before source-level three-word joining.
.text
.globl packet15a
packet15a:
{ r20 = add(r0,r2) }
{ r22 = add(r29,#0x10) }
{ r2 = #0x10; r5:4 = combine(#0,#0x0) }
