// SPDX-License-Identifier: GPL-3.0-only
// Independent expression of the 0x561e0 callback-table invocation wrapper.
.text
.global cdsp_callback_registry_sweeper
.space 0x1e0
cdsp_callback_registry_sweeper:
{ memd(r29+#-16)=r17:16
  allocframe(#16) }
{ r3=memub(gp+##0xc0364128)
  memw(sp+#4)=#0 }
{ p0=r3
  if(!p0.new) r3=#1
  if(!p0.new) memb(##0xc0364128)=r3.new }
{ if(!p0) call #-0xe0
  r16=#0 }
loop0:
{ r16=add(r16,#1)
  r2=memw(r16<<#2+##0xc028c010) }
{ p0=cmp.eq(r2,#0)
  if(p0.new) jump:t loop0 }
{ callr r2
  r1=#4
  r0=add(r29,#4) }
{ r0=memw(sp+#4)
  if(cmp.eq(r0.new,#0)) jump:t loop0 }
{ r17:16=memd(r29+#8)
  dealloc_return }
