// SPDX-License-Identifier: GPL-3.0-only
// Independently expressed Hexagon callback dispatcher semantics.
.text
.globl cdsp_extended_callbacks
.space 0x120
cdsp_extended_callbacks:
{ memd(r29+#-16) = r17:16
  allocframe(#16) }
{ memd(sp+#0) = r19:18 }
{ r2 = memw(gp+##0xc0363e00)
  if(!cmp.eq(r2.new,#0)) jump:t callback_exit }
{ r17 = ##0xc0363e08
  r19 = #1 }
loop:
{ r2 = memw(r19<<#2+##0xc028c010)
  if(cmp.eq(r2.new,#0)) jump:nt fallback }
{ r0 = ##0xc0363e08
  callr r2 }
{ r1 = ##0x320 }
{ r2 = memw(r16+#0) }
{ p0 = cmp.gtu(r2,#0xc7)
  if(!p0.new) jump:t counter_start }
{ jump callback_exit }
fallback:
{ r2 = #0 }
counter_start:
{ r3 = addasl(r17,r2,#2)
  r4 = sub(#0xc8,r2) }
{ loop0(inner_loop,r4) }
inner_loop:
{ r4 = memw(r3+#0)
  if(cmp.eq(r4.new,#0)) jump:nt bump_other }
{ r2 = add(r2,#1)
  r3 = add(r3,#4)
  memw(r16+#0) = r2.new }:endloop0
{ jump callback_exit }
bump_other:
{ r19 = add(r19,#1)
  p0 = cmp.eq(r2,#0)
  if(p0.new) jump:t loop }
callback_exit:
{ r17:16 = memd(r29+#8)
  r19:18 = memd(r29+#0) }
{ dealloc_return }
