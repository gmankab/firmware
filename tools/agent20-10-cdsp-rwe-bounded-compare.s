// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon semantics for bounded byte comparison [0x56590,0x565c0).
.text
.space 0x590
.global cdsp_bounded_compare
cdsp_bounded_compare:
{ r0=#0; r3=r0 }
initial:
{ p0=!cmp.gtu(r2,r0)
  if(p0.new) jump:nt zero }
{ r4=memb(r1+#0)
  if(cmp.eq(r4.new,#0)) jump:nt zero2 }
loop:
{ r0=add(r0,#1)
  r1=add(r1,#1)
  r5=memb(r3++#1)
  if(cmp.eq(r5.new,r4)) jump:t initial }
{ jumpr lr }
zero:
{ r0=#0; jumpr r31 }
zero2:
{ r0=#0; jumpr r31 }
