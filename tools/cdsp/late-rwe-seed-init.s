// SPDX-License-Identifier: GPL-3.0-only
// Independent semantic Hexagon V66 source for callback seed initialization.
.text
.global cdsp_seed_setup
.space 0x364
cdsp_seed_setup:
{ r22=upcyclelo }
{ call #0xf81d8 }
{ r0=##0xc0364130 }
{ r2=memub(gp+##0xc0364140) }
{ p0=r2
  if(!p0.new) r0=#-2
  if(p0.new) jump:nt skip_input
  if(p0.new) r2=memw(r20+#0) }
{ call #-0x15c }
{ call #-0x160 }
{ r2=add(r0,#1)
  r0=#-2
  memw(r19+#0)=r2.new }
{ call #-0x170 }
{ r2=add(r0,#1)
  r0=#-2
  memw(r18+#0)=r2.new }
{ r2=add(r0,#1)
  memw(r20+#0)=r2.new }
{ memb(gp+##0xc0364140)=r23 }
skip_input:
