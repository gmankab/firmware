// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon source semantics for file 0x565c0..0x565fc.
.text
.space 0x5c0
.global cdsp_copy_or_delegate
cdsp_copy_or_delegate:
{ memd(r29+#-16)=r17:16
  allocframe(#8) }
{ r16=r2
  if(cmp.eq(r16.new,#0)) jump:nt finished }
{ r2=sub(r0,r1)
  p0=cmp.gtu(r1,r0)
  if(p0.new) jump:nt delegate }
{ loop0(copy_loop,r16)
  p0=!cmp.gt(r16,r2)
  if(p0.new) jump:nt delegate }
copy_loop:
{ r2=memub(r1++#1)
  memb(r0++#1)=r2.new }:endloop0
{ jump finished }
delegate:
{ call #0x113154
  r2=r16 }
finished:
{ r0=r16
  r17:16=memd(r29+#0)
  dealloc_return }
{ nop }
