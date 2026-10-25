// SPDX-License-Identifier: GPL-3.0-only
// Source-authored Hexagon context switch: [0x4e1c,0x4e60).
// HTID and S54 privileged reads are encoded separately from LLVM6's
// supported arithmetic, 64-bit store, literal extender and packet scheduling.
.text
.globl adsp_controller_context_save
adsp_controller_context_save:
{ nop } // V66 r23 = HTID
{ r23 = asl(r23,#2) }
{ r23 = asl(r23,#18) }
{ r23.l = #0xdc00 }
{ r23 |= lsr(r23,#16) }
{ r23.h = #0xf000 }
{ nop } // V66 r22 = S54
{ r23 = add(r23,r22) }
{
 immext(#0xf000d840)
 r23 = add(r22,##-0xfff27a0)
 memd(r23+#0) = r1:0
}
{ nop; r1 = add(r23,#0x68) } // V66 r22 = HTID; the first op continues the packet
{ r1 += mpyi(r22,#0xe0); jump context_continue }
context_continue:
{ r1 = sp; sp = r1 }
