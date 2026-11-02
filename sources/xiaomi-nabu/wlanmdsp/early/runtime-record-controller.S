// SPDX-License-Identifier: GPL-3.0-only
// Agent13 WLAN record controller: select format-specific state transitions,
// account for recorded status bits and run a bounded eight-slot summarizer.
// Firmware file [0x435e0,0x437f0), VMA [0xb00035e0,0xb00037f0).
// Packet-local .new predicates and an explicit loop0 end marker are retained.
.text
.globl agent13_wlan_record_controller
agent13_wlan_record_controller:
// Entry: load incoming status and dispatch per-session handlers.
{
  call 0x2a2188
  allocframe(#0x38)
}
{
  r19:18 = combine(r1,r2)
  r5 = #0x0
  r20 = #0x0
  r17 = r0
}
{
  p0 = cmp.eq(r18,#0x0)
  r3 = memw(r17+#0x28)
  r4 = memub(r17+#0x250)
}
{
  r3 = addasl(r3,r4,#0x2)
  r6 = p0
  r2 = memub(r17+#0x3)
}
{
  r21 = memw(r3+#0x328)
  r3 = memw(r19+#0x0)
}
{
  r16 = memw(r21+#0x24); 	r4 = memub(r21+#0x0)
}
{
  if (p0) jump:nt L_03628
  memw(r29+#0x14) = r6
}
{
  r5 = memw(r18+#0x10)
}
{
  r5 = extractu(r5,#0x1,#0x1d)
}
L_03628:
{
  r1 = #0x4804
  immext(#0x140)
  r0 = ##0x150; 	memw(r29+#0xc) = r5
}
{
  memw(r29+#0x8) = r3; 	memw(r29+#0x4) = r4
}
{
  call 0x217b7c
  memw(r29+#0x0) = r2
}
{
  r2 = memub(r17+#0x3); 	r5 = memw(r29+#0x14)
}
{
  p0 = r5
  if (p0.new) jump:nt L_03658
  r4 = memw(r19+#0x0); 	r3 = memub(r21+#0x0)
}
{
  r5 = memw(r18+#0x10)
}
{
  r20 = extractu(r5,#0x1,#0x1d)
}
L_03658:
{
  immext(#0xb02a6480)
  r1:0 = combine(#0x4,##0xb02a649c)
  memw(r29+#0xc) = r20; 	memw(r29+#0x8) = r4
}
{
  call -0x1354
  memw(r29+#0x4) = r3; 	memw(r29+#0x0) = r2
}
{
  r3 = add(r17,#0xd48)
  r2 = memw(r19+#0x0)
  if (cmp.gt(r2.new,#0x6)) jump:nt L_036d4
}
{
  r3 = add(r2,#-0x2)
}
{
  r4 = #0x2
  if (cmp.gtu(r4.new,r3)) jump:nt L_03738
}
{
  p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt L_03794
}
{
  p0 = cmp.eq(r2,#0x1); if (!p0.new) jump:nt L_03734
}
{
  r1:0 = combine(#0x24,r18)
  r2 = memw(r17+#0x14)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_037d8
}
{
  r2 = memw(r18+#0x10)
}
{
  immext(#0xfa00ffc0)
  r2 = and(r2,##-0x5ff0001)
}
{
  call 0x760
  r2 = setbit(r2,#0x1a)
  memw(r18+#0x10) = r2.new
}
{
  call 0x580
  r0 = r17
}
{
  r2 = memw(r29+#0x14)
}
{
  p0 = r2
  if (p0.new) jump:nt L_03734
  if (!p0.new) r2 = memub(r18+#0x13)
}
{
  p0 = !tstbit(r2,#0x5)
  if (p0.new) jump:t L_03734
}
{
  jump L_03788
}
L_036d4:
{
  p0 = cmp.eq(r2,#0x7); if (p0.new) jump:nt L_0372c
  if (p0.new) r2 = memw(r19+#0x4)
}
{
  p0 = cmp.eq(r2,#0x12); if (p0.new) jump:nt L_0379c
  if (p0.new) r2 = memw(r18+#0x10)
}
{
  p0 = cmp.eq(r2,#0x13); if (!p0.new) jump:nt L_03734
  if (p0.new) r2 = memw(r18+#0x10)
}
{
  p0 = !tstbit(r2,#0x1a)
  r2 = clrbit(r2,#0x1e)
  if (p0.new) jump:t L_03734
  memw(r18+#0x10) = r2.new
}
{
  loop0(L_03708,#0x8)
  r2 = #0x0; 	r5:4 = combine(r3,#0)
}
{
  r1 = r4
}
L_03708:
{
  r0 = memw(r5++#0x4)
}
{
  r6 = popcount(r1:0)
}
{
  r2 = add(r6,r2)
  nop
}:endloop0
{
  call 0x2a20c0
  r0 = r16
  r1 = memw(r17+#0xd40)
  memw(r17+#0xd44) = r2
}
{
  jump 0x20bea8
}
L_0372c:
{
  r3 = memw(r17+#0xc)
  if (cmp.eq(r3.new,r2)) jump:nt L_03738
}
L_03734:
{
  jump 0x2a2110
}
L_03738:
{
  r2 = memw(r18+#0x10)
}
{
  p0 = !tstbit(r2,#0x1a)
  if (p0.new) jump:t L_03774
}
{
  call 0x900
  r1:0 = combine(#0x24,r18)
}
{
  r2 = ##0xe000ffff
  r0 = r17
  r5:4 = combine(#0,#0x0)
}
{
  memw(r18+#0x48) = #0x0
  memd(r18+#0x50) = r5:4
}
{
  memw(r18+#0x10) &= r2
}
{
  memh(r18+#0x4c) = #0x0
  memw(r18+#0xe4) = #0x0
}
{
  call 0x13b4
}
{
  r2 = memw(r18+#0x10)
}
L_03774:
{
  p0 = tstbit(r2,#0x1d)
  r0 = r17
  if (p0.new) jump:t L_03788
}
{
  call 0x2a2058
}
{
  jump 0x4ac
}
L_03788:
{
  call 0x2a2050
  r3:2 = combine(#1,#0x0); 	r1:0 = combine(#0,r16)
}
{
  jump 0x20bd24
}
L_03794:
{
  jump 0x2a20b0
  memw(r18+#0x10) = clrbit(#0x1d)
}
L_0379c:
{
  p0 = !tstbit(r2,#0x1d)
  if (p0.new) jump:nt L_037cc
  if (!p0.new) r4 = memw(r29+#0x14)
}
{
  p0 = r4
  if (p0.new) jump:nt L_037cc
}
{
  immext(#0x4000000)
  r4 = and(r2,##0x4000000)
  if (cmp.eq(r4.new,#0x0)) jump:nt L_037cc
}
{
  call 0x20bcf8
  r1 = add(r17,#0xd40)
  r0 = r16; 	r3:2 = combine(#0,r3)
}
{
  r2 = memw(r18+#0x10)
}
L_037cc:
{
  r2 = setbit(r2,#0x1e)
  jump 0x2a2078
  memw(r18+#0x10) = r2.new
}
// Tail path: callback and jump into existing record processor.
L_037d8:
{
  call 0x448
  immext(#0xb02a6380)
  r0 = ##-0x4fd59c4a
  r1 = #0x479
}
{
  r1:0 = combine(#0x1,r1)
  jump -0x83c
}
