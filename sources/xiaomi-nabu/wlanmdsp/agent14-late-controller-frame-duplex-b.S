// SPDX-License-Identifier: GPL-3.0-only
// Three independently assembled named Hexagon mini-packets for frame subtype pointer and scratch stack variables.
// Packet kinds must be 11/11/00 before source-level three-word joining.
.text
.globl packet15b
packet15b:
{ r23 = add(r20,#0x28) }
{ r0 = add(r29,#0x38) }
{ r2 = add(r29,#0x38); memw(r29+#0x44) = r22 }
