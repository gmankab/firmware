// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon semantic assembly of bit-plane controller loop.
.text
.space 0x6cc
loop:
{ nop }
.space 0x8
.global controller_bitplane
controller_bitplane:
{ r2=or(r22,r23) }
{ r2=setbit(r2,#12) }
{ r3=sub(#31,r3) }
{ r2=ct0(r2) }
{ r4=minu(r3,r2) }
{ r2=lsr(r4,#1) }
{ r26=clrbit(r4,#0) }
{ r17 -= asl(r25,r26) }
{ r0=lsl(#1,r2) }
{ r2=r22 }
{ r2=and(##0x3ffffe,asl(r2,#1)) }
{ r3=and(r0,#0xff) }
{ r1:0=combine(r18,#1) }
{ r2 |= or(r3,##0x70000000) }
{ r3=or(r23,##-0x40000000) }
{ call #0xf838c }
{ r18=add(r18,#1) }
{ r23 += asl(r25,r26) }
{ r22 += asl(r25,r26) }
{ p0=cmp.eq(r17,#0)
  if(!p0.new) jump:t loop }
