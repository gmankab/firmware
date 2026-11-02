// SPDX-License-Identifier: GPL-3.0-only
// Independent semantic reconstruction, not the original packet order.
.text
.space 0x6c0
.global cdsp_controller_bitloop
cdsp_controller_bitloop:
{ r25:24=combine(#1,#7)
  r18=#16 }
{ p0=cmp.eq(r17,#0)
  if(p0.new) jump:nt done }
{ r3=cl0(r17)
  r2=add(r18,#-16)
  if(!cmp.gtu(r24,r2.new)) jump:nt fail }
.space 0x5c
done:
{ nop }
.space 0x158
fail:
{ nop }
