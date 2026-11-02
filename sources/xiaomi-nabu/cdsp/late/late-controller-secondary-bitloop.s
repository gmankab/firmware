// SPDX-License-Identifier: GPL-3.0-only
// Independently authored Hexagon V60 assembly. LLVM6 is free to choose its
// own packet markers; Rizin validates the original V66 parse structure.
.text
.space 0x7a8
loop:
{ r3=cl0(r23) }
{ r2=add(r17,#-31)
  if(!cmp.gtu(r25,r2.new)) jump:nt capacity_exit }
controller_secondary_scan:
{ r2=or(r19,r24) }
{ r2=setbit(r2,#12) }
{ r3=sub(#31,r3) }
{ r2=ct0(r2) }
{ r4=minu(r3,r2) }
{ r2=lsr(r4,#1) }
{ r27=clrbit(r4,#0) }
{ r23 -= asl(r26,r27) }
{ r0=lsl(#1,r2) }
{ r2=#8 }
{ r2=insert(r19,#21,#1) }
{ r3=and(r0,#0xff) }
{ r1:0=combine(r17,#1) }
{ r2 |= or(r3,##0x77000000) }
{ r3=or(r24,##-0x40000000) }
{ call #0xf82b4 }
{ r17=add(r17,#1) }
{ r24 += asl(r26,r27) }
{ r19 += asl(r26,r27) }
{ p0=cmp.eq(r23,#0)
  if(!p0.new) jump:t loop }
.space 0x9c
capacity_exit:
{ nop }
