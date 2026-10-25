// SPDX-License-Identifier: GPL-3.0-only
// Agent 13: process an incoming WLAN session/record; validate field widths,
// check per-format status flags and update record-state bookkeeping.
// File [0x42fac,0x43410), VMA [0xb0002fac,0xb0003410).
//
// The routine uses packet-local new-value predicates, conditional jumps,
// packed byte fields, and QuRT callback slots. All instructions are expressed
// as Hexagon V66 assembly operations; no vendor instruction byte arrays.
//
// Special packet encodings reproduced using independent assembler:
// - The r16/r18/r2/combine packet's source ordering forces the original duplex.
// - The far jump at runtime 0x3394 uses an extended packet-relative target.
// - An original call packet starts at 0x3410 and belongs to the NEXT function.

.text
.globl agent13_wlan_record_processing
agent13_wlan_record_processing:
// Entry: log the request and unpack the incoming record header.
{
  call 0x2a27cc
  allocframe(#0x58)
}
{
  r16 = r0
  r18 = r1
  r2 = add(r29,#0x30)
  r1:0 = combine(#0,#0x0)
}
{
  r1 = #0x4827
  r17 = memw(r16+#0x8); 	memd(r29+#0x28) = r1:0
}
{
  r3 = memw(r16+#0x10); 	memw(r29+#0x30) = r17
}
{
  r0 = extractu(r3,#0x9,#0x10)
  r4 = memub(r16+#0x4d)
  immext(#0xb0004440)
  memw(r2+#0x4) = ##-0x4fffbba4
}
{
  r5 = extractu(r3,#0x1,#0x1c)
  r7 = memub(r17+#0x3); 	memw(r29+#0x4) = r0
}
{
  r6 = extractu(r3,#0x1,#0x19)
  r3 = extractu(r3,#0x1,#0x1b)
  immext(#0x100)
  r0 = ##0x130
}
{
  memw(r29+#0x18) = r4; 	memw(r29+#0x14) = r18
}
{
  memw(r29+#0x10) = r3; 	memw(r29+#0xc) = r6
}
{
  call 0x2181b8
  memw(r29+#0x8) = r5; 	memw(r29+#0x0) = r7
}
{
  immext(#0xb02a6440)
  r1:0 = combine(#0x7,##0xb02a646c)
  r2 = memw(r16+#0x8); 	r3 = memw(r16+#0x10)
}
{
  r5 = extractu(r3,#0x9,#0x10)
  r6 = extractu(r3,#0x1,#0x1c)
  r4 = memub(r16+#0x4d)
}
{
  r7 = extractu(r3,#0x1,#0x19)
  r3 = extractu(r3,#0x1,#0x1b)
  r2 = memub(r2+#0x3); 	memw(r29+#0x18) = r4
}
{
  memw(r29+#0x14) = r18; 	memw(r29+#0x10) = r3
}
{
  memw(r29+#0xc) = r7; 	memw(r29+#0x8) = r6
}
{
  call -0xd20
  memw(r29+#0x4) = r5; 	memw(r29+#0x0) = r2
}
{
  r2 = memw(r16+#0x78)
  r3 = memw(r16+#0x84)
}
{
  r2 |= asl(r3,#0x1)
  r0 = memw(r16+#0x7c)
  r1 = memw(r16+#0xe4)
}
{
  r2 |= asl(r0,#0x2)
  r0 = memw(r16+#0xe8)
}
{
  r2 |= asl(r1,#0x3)
  r1 = #0x4821
}
{
  r2 |= asl(r0,#0x4)
  immext(#0x100)
  r0 = ##0x138
  memw(r29+#0x0) = r2.new
}
{
  call 0x218148
}
{
  r1 = memw(r16+#0x78)
  r2 = memw(r16+#0x84)
}
{
  r1 |= asl(r2,#0x1)
  r0 = memw(r16+#0x7c)
}
{
  r1 |= asl(r0,#0x2)
  r0 = memw(r16+#0xe4)
}
{
  r1 |= asl(r0,#0x3)
  r0 = memw(r16+#0xe8)
}
{
  r1 |= asl(r0,#0x4)
  call -0x448
  immext(#0xb02a6440)
  r0 = ##-0x4fd59b88
}
{
  call 0x1344
  r1:0 = combine(#0x10,r16)
  r2 = #0x0
}
{
  r2 = memw(r17+#0x14)
}
{
  r2 = memuh(r2+#0x10)
}
{
  r2 = lsr(r2,#0x3)
}
{
  p0 = cmp.gtu(r2,#0x176)
  if (p0.new) r19 = #0x3
  if (p0.new) jump:t L_030d4
}
{
  r19 = #0x43
  r2 = memb(r17+#0x1)
}
{
  p0 = cmp.eq(r2,#0x2)
  if (p0.new) r19 = #0x3
}
L_030d4:
{
  call 0x1602f0
  r1 = r19; 	r0 = memw(r17+#0x2c)
}
{
  r18 = r0; 	p0 = cmp.eq(r18,#0x0)
}
{
  r2 = p0
  if (p0) jump:nt L_030fc
  memw(r29+#0x24) = r2.new
}
{
  call -0x1d4
  r0 = memw(r16+#0x8)
}
{
  r2 = add(r16,#0x4d)
}
{
  memb(r2+#0x0) += r0
}
L_030fc:
{
  r3:2 = combine(#0x0,#0x0)
  r1:0 = memd(r16+#0x0)
}
{
  p0 = cmp.eq(r1:0,r3:2)
  if (p0.new) r2 = add(r29,#0x3b)
  if (p0.new) jump:nt L_03114
}
L_03110:
{
  jump 0x2a2744
}
// Validate the record envelope, length and packed flags.
L_03114:
{
  r3 = memw(r16+#0x10); 	r0 = memw(r16+#0x8)
}
{
  r5 = extractu(r3,#0x8,#0x8)
  r6 = extractu(r3,#0x9,#0x10)
  r4 = and(r3,#0xff)
  memb(r29+#0x3c) = r3
}
{
  r4 = add(r5,r4)
  r7 = memub(r0+#0x3); 	memb(r2+#0x2) = #0
}
{
  p0 = !cmp.gtu(r4,r6)
  memb(r29+#0x3b) = r7
  memb(r2+#0x4) = #0x0
}
{
  if (p0) jump:nt L_0318c
  memb(r2+#0x3) = #0x0
}
{
  r2 = memw(r29+#0x24)
}
{
  p0 = r2
  if (p0.new) jump:nt L_0318c
}
{
  immext(#0x4000000)
  r2 = and(r3,##0x4000000)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_0318c
}
{
  call -0x244
}
{
  r2 = memw(r16+#0x10)
}
{
  r3 = r2
}
{
  r3 += asl(r0,#0x10)
  r0 = memw(r16+#0x8)
}
{
  r2 = insert(r3,#0x19,#0x0)
  call -0x258
  memw(r16+#0x10) = r2.new
}
{
  r2 = memw(r16+#0x8)
}
{
  r2 = add(r2,#0x5f0)
}
{
  memw(r2+#0x0) += r0
}
{
  r3 = memw(r16+#0x10)
}
L_0318c:
{
  r19 = insert(r18,#0x18,#0x8)
  p0 = tstbit(r3,#0x1c)
  if (p0.new) jump:t L_03268
}
{
  r2 = extractu(r3,#0x6,#0x2)
  r5 = extractu(r3,#0x9,#0x10)
  r4 = #0x4; 	r0 = and(r3,#255)
}
{
  r2 = maxu(r2,r4)
}
{
  r2 = add(r2,r5)
  if (cmp.gtu(r0,r2.new)) jump:t L_03404
}
{
  r2 = setbit(r3,#0x1c)
  r1:0 = combine(#0x8,r16)
  r3 = #0x0
  memw(r16+#0x10) = r2.new
}
{
  call 0x1224
  r2 = #0x0
  memb(r16+#0x4c) = r3
}
{
  r17 = #0x10
  r0 = add(r29,#0x3b)
  r2 = memub(r16+#0x10)
}
{
  r3 = lsr(r2,#0x2)
  p0 = cmp.gtu(r17,r2)
}
{
  if (p0) r3 = #0x4
}
{
  r2 = sub(r2,r3)
  memb(r29+#0x3c) = r2.new
}
{
  r3 = lsr(r2,#0x18)
  r1 = lsr(r2,#0x10)
  memb(r29+#0x3f) = r3.new
}
{
  r2 = lsr(r2,#0x8)
  memb(r29+#0x3e) = r1
}
{
  call 0x123c
  memb(r29+#0x3d) = r2
}
{
  r2 = memw(r16+#0x8)
}
{
  r3 = memw(r2+#0x14)
  r2 = memub(r2+#0x250)
}
{
  r0 = memuh(r3+#0x10)
  memb(r29+#0x28) = r2
}
{
  call 0x216954
}
{
  call 0x225fc0
  r0 = memw(r16+#0x8)
  memb(r29+#0x29) = r0
}
{
  r0 = add(r29,#0x28)
  r2 = memw(r16+#0x10)
  memb(r29+#0x2a) = r0
}
{
  r3 = and(r2,#255); 	r4 = memw(r16+#0x8)
}
{
  r3 = lsr(r3,#0x2)
  p0 = cmp.gtu(r17,r3)
}
{
  if (p0) r3 = #0x4
}
{
  r2 = sub(r2,r3)
  memb(r29+#0x2b) = r2.new
}
{
  r2 = memub(r4+#0x0)
  memb(r29+#0x2c) = r2.new
}
{
  call 0x11f4
  r2 = memub(r4+#0x3)
  memb(r29+#0x2d) = r2.new
}
{
  jump L_03404
}
L_03268:
{
  p0 = tstbit(r3,#0x19)
  immext(#0x780000)
  r18 = or(r19,##0x780000)
  if (p0.new) r2 = zxtb(r3)
}
{
  if (p0) r0 = add(r16,#0x18)
  if (p0) jump:nt L_03338
}
{
  r2 = extractu(r3,#0x9,#0x10)
}
{
  r4 = and(r3,#0xff)
  if (cmp.gtu(r4.new,r2)) jump:t L_03404
}
{
  r3 = setbit(r3,#0x19)
  r1:0 = combine(#0x1,r16)
  r2 = #0x0
  memw(r16+#0x10) = r3.new
}
{
  call 0x1148
}
{
  r2 = add(r29,#0x3b)
  r0 = add(r29,#0x3b)
  r1 = memub(r16+#0x10)
  memb(r29+#0x3c) = r1.new
}
{
  memb(r2+#0x4) = #0; 	memb(r2+#0x3) = #0
}
{
  call 0x1188
  memb(r2+#0x2) = #0x0
}
{
  r2 = memw(r16+#0x8)
}
{
  r3 = memw(r2+#0x14)
  r2 = memub(r2+#0x250)
}
{
  r0 = memuh(r3+#0x10)
  memb(r29+#0x28) = r2
}
{
  call 0x2168a0
}
{
  call 0x225f0c
  r0 = memw(r16+#0x8)
  memb(r29+#0x29) = r0
}
{
  r0 = add(r29,#0x28)
  r2 = memw(r16+#0x8)
  memb(r29+#0x2a) = r0
}
{
  r3 = memw(r16+#0x10)
  memb(r29+#0x2b) = r3.new
}
{
  r3 = memub(r2+#0x0)
  memb(r29+#0x2c) = r3.new
}
{
  call 0x1150
  r2 = memub(r2+#0x3)
  memb(r29+#0x2d) = r2.new
}
{
  r2 = memub(r16+#0x4d)
}
{
  p0 = cmp.gtu(r2,#0x59)
  if (p0.new) jump:t L_03404
}
{
  r2 = add(r29,#0x30)
  r0 = r17
  r1 = memw(r17+#0xc)
  memw(r29+#0x4) = r2.new
}
{
  immext(#0x1000000)
  r5:4 = combine(#0x1,##0x1000000)
  r3:2 = combine(#0,#0x0); 	memw(r29+#0x0) = r18
}
{
  call 0xbcfc
}
{
  jump L_03404
}
L_03338:
{
  r4 = extractu(r3,#0x8,#0x8)
  p0 = tstbit(r3,#0x1b)
  if (p0.new) jump:t L_03380
}
{
  r5 = extractu(r3,#0x9,#0x10)
}
{
  r2 = add(r4,r2)
  if (cmp.gtu(r2.new,r5)) jump:t L_033c0
}
{
  r4 = #0x5a
  r2 = memub(r16+#0x4d)
}
{
  r2 = setbit(r3,#0x1b)
  p0 = cmp.gtu(r4,r2); if (p0.new) jump:t L_033a0
  memw(r16+#0x10) = r2.new
}
{
  call 0x2185f8
}
{
  call 0xfac
  r0 = memw(r16+#0x8)
}
{
  call 0xed4
  r0 = r16
}
{
  jump L_033c0
  memh(r16+#0x4c) = #0x1
}
L_03380:
{
  call 0x2185dc
}
{
  call 0xf90
  r0 = memw(r16+#0x8)
}
{
  call 0xeb8
  r0 = r16
}
{
  r2 = #0x1; jump ##0x2a24c0
  memb(r16+#0x4c) = r2.new
}
// Finish bounded status accounting and fall through to shared epilogues.
L_033a0:
{
  r2 = add(r29,#0x30)
  r0 = r17
  r1 = memw(r17+#0xc)
  memw(r29+#0x4) = r2.new
}
{
  immext(#0x1000000)
  r5:4 = combine(#0x1,##0x1000000)
  r3:2 = combine(#0,#0x0); 	memw(r29+#0x0) = r18
}
{
  call 0xbc70
}
L_033c0:
{
  r2 = memw(r16+#0x84)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_033fc
}
{
  r2 = memw(r16+#0x78)
  if (!cmp.eq(r2.new,#0x1)) jump:t L_033fc
}
{
  r2 = memw(r16+#0x7c)
  if (!cmp.eq(r2.new,#0x0)) jump:t L_033fc
}
{
  r3 = memub(r16+#0xe2)
  r2 = memw(r16+#0x10)
}
{
  r2 = extractu(r2,#0x9,#0x10)
  r4 = memub(r16+#0xe0)
}
{
  r3 = add(r4,r3)
  if (!cmp.gtu(r2,r3.new)) jump:t L_033fc
}
{
  call 0x11c0
  r0 = memw(r17+#0x2c)
}
{
  memw(r16+#0x78) = #0x0
}
L_033fc:
{
  r2 = memb(r16+#0x4c)
  if (!cmp.eq(r2.new,#0x0)) jump:t L_03110
}
L_03404:
{
  call 0xb84
  r0 = r16
}
{
  jump 0x2a2448
}
