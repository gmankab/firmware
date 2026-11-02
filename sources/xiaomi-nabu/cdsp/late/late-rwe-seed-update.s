// SPDX-License-Identifier: GPL-3.0-only
// Independent semantic Hexagon V66 source for original `0x563c0..0x56448`.
.text
.global cdsp_seed_update_loop
.space 0x3c0
cdsp_seed_update_loop:
{ r2 ^= asl(r2,#3) }
{ r6=asl(r2,#17)
  r3=memw(r19+#0)
  r4=memw(r18+#0) }
{ r3 ^= asl(r3,#13) }
{ r24=asl(r3,#12)
  r5=#-128 }
{ r3=lsr(r3,#19)
  r2=lsr(r2,#11) }
{ r0=##0xc0364130
  r5 &= asl(r4,#4) }
{ r24=insert(r3,#13,#0)
  memw(r19+#0)=r24.new }
{ r6=insert(r2,#21,#0)
  r4 ^= asl(r4,#2)
  r3=r5
  memw(r20+#0)=r6.new }
{ r24 ^= xor(r6,r22) }
{ r2=lsr(r4,#25) }
{ r3=insert(r2,#7,#0)
  r24 ^= or(r2,r5)
  memw(r18+#0)=r3.new }
{ call #0xf8228 }
{ p0=cmp.eq(r24,#0)
  if(p0.new) jump:t #-0xb8
  if(!p0.new) memw(sp+#4)=r24 }
{ r3:2=combine(r16,#0x53) }
{ r4=add(sp,#4)
  r5=#4
  r1:0=combine(r17,#0) }
{ call #0xfc698 }
cdsp_seed_early_exit:
{ r17:16=memd(r29+#40)
  r19:18=memd(r29+#32) }
{ r21:20=memd(r29+#24)
  r23:22=memd(r29+#16) }
{ r25:24=memd(sp+#8) }
{ dealloc_return }
// External label branch to preceding function prologue. Hexagon absolute
// assembly resolves the physical displacement from our `.space` anchor.
cdsp_seed_restart:
