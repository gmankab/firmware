// SPDX-License-Identifier: GPL-3.0-only
// Hexagon v66 encoding helpers for compressed controller offset formats.
// These are two separate leaf routines with no referenced OEM code words.
// ADSP file [0x11108,0x11164) physical [0x8be0e108,0x8be0e164).
.text
.globl adsp_pack_controller_range
adsp_pack_controller_range:
{ immext(#0x800000); r3=add(r1,sub(##0x800000,r0)); r2=sub(r1,r0) }
{ r0=extractu(r2,#14,#1); immext(#0x1ff0000); r2=and(##0x1ff0000,asl(r2,#1)) }
{ immext(#0xffffc0); p0=cmp.gtu(r3,##0xfffffc); r3=#0 }
{ immext(#0x5800c000); r0 |= or(r2,##0x5800c000) }
{ if(p0) r0=add(r3,#0); jumpr lr }
.globl adsp_pack_plane_fields
adsp_pack_plane_fields:
{ r2=sub(r1,r0); immext(#0x5800c000); r1=##0x5800c000 }
{ r3=extractu(r2,#26,#6); r4=extractu(r2,#14,#6) }
{ r1=insert(r2,#6,#1); r3=asl(r3,#2) }
{ r3=insert(r4,#16,#0) }
{ r0=setbit(r3,#14); jumpr lr }
