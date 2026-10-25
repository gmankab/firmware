// SPDX-License-Identifier: GPL-3.0-only
// Semantic CDSP V66 source for controller callback and trap protocol.
.text
.space 0x4b8
.global cdsp_controller_callback_service
cdsp_controller_callback_service:
{ r17:16=combine(r0,r1) }
{ memd(r29+#-16)=r17:16
  allocframe(#40) }
{ memd(sp+#24)=r19:18 }
{ r2=memw(gp+##0xc028c028)
  if(cmp.eq(r2.new,#0)) jump:nt done }
{ r19=add(sp,#8) }
{ r0=r2
  memw(r29+#8)=r2 }
{ call #0x116ee0 }
{ memw(r19+#4)=#0 }
{ memw(sp+#16)=r0 }
{ r0=#1 }
{ r1=r19 }
{ trap0(#0) }
{ r2=r0 }
{ memw(sp+#4)=r2 }
{ r2=memw(sp+#4)
  if(cmp.gt(r2.new,#1)) jump:t post_reset }
{ r2=#0
  jump done
  memw(gp+##0xc028c028)=r2.new }
post_reset:
{ r2=memw(sp+#4)
  memw(sp+#8)=r2.new }
{ memw(r29+#12)=r17
  memw(r29+#16)=r16 }
{ r0=#6 }
{ r1=r19 }
{ trap0(#0) }
{ r2=memw(sp+#4)
  memw(sp+#8)=r2.new }
{ r0=#2 }
{ r1=r19 }
{ trap0(#0) }
done:
{ r17:16=memd(r29+#32)
  r19:18=memd(r29+#24) }
{ dealloc_return }
