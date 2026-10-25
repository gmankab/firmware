// SPDX-License-Identifier: GPL-3.0-only
// Independent semantic Hexagon assembly for atomic controller counter.
.text
.space 0x464
.global cdsp_controller_atomic_counter
cdsp_controller_atomic_counter:
{ r2=##0xc0363e00
  r3=memw(r2)
  if(cmp.eq(r3.new,#0)) jump:nt zero_return }
{ r4=#-1 }
minus_loop:
{ r3=memw_locked(r2) }
{ r3=add(r3,r4) }
{ memw_locked(r2,p0)=r3 }
{ if(!p0) jump:nt minus_loop }
{ if(!p0.new) r3=#1
  p0=cmp.gt(r3,#-1)
  if(!p0.new) jump:nt plus_loop }
{ jumpr lr }
{ r2=memw(r3<<#2+##0xc0363e08)
  memw(r0+#0)=r2.new }
zero_return:
{ jumpr lr }
plus_loop:
{ r4=memw_locked(r2) }
{ r4=add(r4,r3) }
{ memw_locked(r2,p0)=r4 }
{ if(!p0) jump:nt plus_loop }
{ jumpr lr }
{ jumpr lr }
