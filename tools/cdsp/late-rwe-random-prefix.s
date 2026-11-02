// SPDX-License-Identifier: GPL-3.0-only
// Semantic Hexagon V66 input to LLVM6; packet order may differ from OEM.
.text
.global cdsp_rng_entry
.space 0x320
cdsp_rng_entry:
{ r2=memw(gp+##0xfe0ad660)
  allocframe(#48)
  p0=cmp.eq(r2,#0) }
{ r17:16=combine(r0,r3)
  memd(r29+#40)=r17:16
  memd(r29+#32)=r19:18 }
{ if(!p0) r20=##0xc0364158
  memd(r29+#24)=r21:20
  memd(r29+#16)=r23:22 }
{ if(!p0) r23=#1 }
{ if(p0) r0=#0
  if(p0) jump:nt #0xe8
  memd(sp+#8)=r25:24 }
{ r18=##0xc0364150
  r19=##0xc0364148 }
