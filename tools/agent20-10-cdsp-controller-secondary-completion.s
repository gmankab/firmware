// SPDX-License-Identifier: GPL-3.0-only
// Independently authored Hexagon-V60 source for the second plane guard and
// controller command (LLVM packet choices need not match OEM V66 scheduler).
.text
.space 0x80c
second_plane_bounds:
{ r2=add(r17,#-31) }
{ p0=cmp.gtu(r22,r20)
  if(p0.new) jump:nt next_packet }
{ r3=add(r21,r22)
  if(cmp.gtu(r3.new,r20)) jump:t controller_call_end }
next_packet:
{ r3=#8
  if(!cmp.gtu(r3.new,r2)) jump:nt insufficient }
{ r2=asl(r18,#1)
  r3=##0xc0000000
  r1:0=combine(r17,#1) }
{ r3 |= lsr(r20,#12)
  r4=##0x77000001 }
{ r2=or(r4,and(r2,##0x1ffffe)) }
{ call #0xf8264 }
controller_call_end:
{ r1:0=combine(#0,#1) }
.space 0x79
insufficient:
{ nop }
