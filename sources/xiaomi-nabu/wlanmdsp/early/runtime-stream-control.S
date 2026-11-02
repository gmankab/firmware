// SPDX-License-Identifier: GPL-3.0-only
// Evaluate encoded WLAN payload and update record-state control flags.
// File [0x437f0,0x43984), rebuilt Hexagon V66 packet semantics.
.text
.globl agent13_wlan_stream_control
agent13_wlan_stream_control:
{
  call 0x2a1f4c
  allocframe(#0x30)
}
{
  r16 = r2; 	r20 = r0
}
{
  r2 = memw(r16+#0x0)
}
{
  r2 = memw(r2+#0x10)
}
{
  r3 = memub(r2+#0x5a)
  r4 = memub(r2+#0x5b)
}
{
  r3 |= asl(r4,#0x8)
  r5 = memub(r2+#0x59)
  r17 = memub(r2+#0x58)
}
{
  r17 |= asl(r5,#0x8)
}
{
  r17 |= asl(r3,#0x10)
}
{
  r3 = memub(r17+#0x0)
}
{
  r18 = and(r3,#0xf0)
  r1 = and(r3,#0xc)
  if (cmp.eq(r1.new,#0x0)) jump:nt L_03838
}
L_03830:
{
  // Packet-relative far jump to the record-processing fallback.
  r0 = #0x0; jump ##0x2a1fe8
}
L_03838:
{
  p0 = cmp.eq(r18,#0x80)
  if (p0.new) jump:nt L_03850
  r23 = memub(r2+#0x5d)
  r22 = memub(r2+#0x5c)
}
{
  p0 = cmp.eq(r18,#0x50)
  if (!p0.new) jump:nt L_03830
}
L_03850:
{
  r21 = memw(r16+#0x8)
}
{
  call 0x16a1dc
  r0 = r21
}
{
  call 0x16a190
  r19 = r0; 	r0 = r21
}
{
  r20 = memw(r20+#0xc)
  if (cmp.eq(r20.new,#0x0)) jump:nt L_03830
}
{
  r22 |= asl(r23,#0x8)
  r24 = #0x0
  r21 = add(r17,#0xa)
  r25 = sxth(r0)
}
{
  r2 = add(r22,#-0x4)
  r23 = #0x0
}
{
  r22 = zxth(r2)
}
L_03888:
{
  r2 = memub(r20+#0x13)
}
{
  p0 = !tstbit(r2,#0x2)
  if (p0.new) jump:t L_03960
}
{
  r2 = memw(r20+#0x8)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_0396c
}
{
  r3 = memw(r2+#0x14)
  if (cmp.eq(r3.new,#0x0)) jump:nt L_038e0
}
{
  call 0x7cd74
  r1:0 = combine(r18,r17)
  r3:2 = combine(#0x3,r22)
}
{
  r23 = r0; 	r2 = memw(r20+#0x8)
}
{
  r2 = memw(r2+#0x14)
}
{
  call 0x216370
  r0 = add(r2,#0x10)
}
{
  if (p0.new) r1:0 = combine(r18,r17)
  p0 = cmp.eq(r23,#0x0); 	r2 = memw(r20+#0x8)
}
{
  r3 = memw(r2+#0x14)
}
{
  r23 = add(r3,#0x10)
  if (!p0) jump:nt L_038e0
}
{
  call 0x7cd44
  r3:2 = combine(#0x3d,r22)
}
{
  r2 = memw(r20+#0x8)
}
L_038e0:
{
  // This register ordering maintains the original compound duplex.
  r0 = add(r29,#0x2)
  r1 = r21
  r2 = #0x6
  r3 = memw(r2+#0xc)
}
{
  r4 = memw(r3+#0x0)
  memb(r29+#0x2) = r4.new
}
{
  r5 = lsr(r4,#0x8)
  r6 = lsr(r4,#0x10)
  r3 = memw(r3+#0x4)
  memb(r29+#0x6) = r3.new
}
{
  r4 = lsr(r4,#0x18)
  r3 = lsr(r3,#0x8)
  memb(r29+#0x3) = r5
  memb(r29+#0x4) = r6
}
{
  call L_0397c
  memb(r29+#0x5) = r4
  memb(r29+#0x7) = r3
}
{
  if (p0.new) r0 = add(r20,#0x0)
  p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:t L_03960
}
{
  call 0x91c
  memb(r20+#0x4d) = r24
}
{
  p0 = cmp.eq(r23,#0x0); if (p0.new) jump:nt L_0394c
  if (!p0.new) r2 = memh(r23+#0x0)
}
{
  p0 = !cmp.eq(r2,r25)
  if (!p0.new) r1 = add(r19,#0x0)
  if (!p0.new) r0 = memw(r20+#0x8)
}
{
  immext(#0x225940)
  if (!p0) call 0x225970
}
L_0394c:
{
  call 0x1230
  r0 = r20
}
{
  call 0xa90
  r1:0 = combine(#0x4,r20)
  r2 = r16
}
L_03960:
{
  r20 = memw(r20+#0x38)
  if (!cmp.eq(r20.new,#0x0)) jump:t L_03888
}
{
  jump L_03830
}
L_0396c:
{
  call 0x2b4
  immext(#0xb02a63c0)
  r0 = ##-0x4fd59c2c
  r1 = #0x4f4
}
L_0397c:
{
  immext(#0xffffd780)
  jump -0x287c
}
