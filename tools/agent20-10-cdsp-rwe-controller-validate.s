// SPDX-License-Identifier: GPL-3.0-only
// Independently assembled Hexagon controller address range validation.
.text
.space 0x650
.global cdsp_controller_validate
cdsp_controller_validate:
{ call #0x7e30
  r3:2=combine(#0,#-1)
  r1=r17; r0=memw(r3+#16) }
{ r18=#0xae
  r16=r0
  if(cmp.eq(r16.new,#0)) jump:nt empty }
{ r22=lsr(r22,#12)
  r25=add(r24,#0x4000) }
{ r23=memw(r16+#0) }
{ p0=cmp.gtu(r23,r24)
  r2=add(r23,r17)
  if(p0.new) jump:nt second_bank }
{ p0=!cmp.gtu(r24,r2)
  if(p0.new) jump:t in_bank }
second_bank:
{ p0=cmp.gtu(r23,r25)
  if(p0.new) jump:nt failed }
{ p0=!cmp.gtu(r25,r2)
  if(!p0.new) jump:nt failed }
in_bank:
{ nop }
// The original fallback is at physical 0x96cc86c0, distinct from the
// in-bank success label at 0x96cc8698.
.space 0x24
failed:
{ nop }
// The null-input exit is at physical 0x96cc88bc.
.space 0x1f8
empty:
{ nop }
