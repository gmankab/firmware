// SPDX-License-Identifier: GPL-3.0-only
// Independently written CDSP callback polling/controller fallback semantics.
.text
.global cdsp_callback_polling
.space 0x230
cdsp_callback_polling:
{ p0=cmp.eq(r0,#-1)
  if(!p0.new) r17=add(r0,#1)
  memd(r29+#-16)=r17:16
  allocframe(#32) }
{ if(p0) jump:nt error_return
  memd(r29+#16)=r19:18
  memd(r29+#8)=r21:20 }
{ call #0x13575c
  r1:0=combine(r17,#-1) }
{ r19=#1
  r16=r0 }
{ r17=mpyi(r16,r17) }
loop:
{ r2=memub(gp+##0xc0364128)
  memw(sp+#4)=#0 }
{ p0=r2
  if(!p0.new) memb(##0xc0364128)=r19 }
{ if(!p0) call #-0x14c
  r20=##0xc028c010 }
callback:
{ r2=memw(r20++#4)
  if(cmp.eq(r2.new,#0)) jump:t callback }
{ callr r2
  r1=#4
  r0=add(r29,#4) }
{ r2=memw(sp+#4)
  if(cmp.eq(r2.new,#0)) jump:t callback }
{ p0=cmp.gtu(r2,r17)
  if(p0.new) jump:t loop }
{ call #0x135708
  r1=r16
  r0=add(r2,#-1) }
{ jump return_all }
error_return:
{ call #-0x74
  r0=#0xffff }
{ call #-0x80
  r0=#0xffff
  r16=r0 }
{ r0+=asl(r16,#16) }
return_all:
{ r17:16=memd(r29+#24)
  r19:18=memd(r29+#16) }
{ r21:20=memd(r29+#8)
  dealloc_return }
