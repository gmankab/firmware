// SPDX-License-Identifier: GPL-3.0-only
// Independently authored semantic subset of CDSP file 0x56600 onward.
.text
.space 0x600
.global cdsp_large_controller_prologue
cdsp_large_controller_prologue:
{ memd(r29+#-16)=r17:16
  allocframe(#48) }
{ memd(r29+#32)=r19:18
  memd(r29+#24)=r21:20 }
{ memd(sp+#16)=r23:22
  memd(sp+#8)=r25:24 }
{ memd(sp+#0)=r27:26 }
{ r2=memw(r20=##0xc0673260)
  if(cmp.eq(r2.new,#0)) jump:nt return_empty }
{ r22=memw(gp+##0xfe0c6734)
  r2=memw(gp+##0xfe0c6680) }
{ r17=add(r2,##0xfffff)
  r24=memw(gp+##0xc028c0f0) }
{ r17=and(##0xfff00,lsr(r17,#12))
  r3=memw(r19=##0xc0367850) }
.space 0x22c
return_empty:
{ nop }
