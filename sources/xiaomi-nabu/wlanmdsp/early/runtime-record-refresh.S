// SPDX-License-Identifier: GPL-3.0-only
// Packet input refresh and diagnostics, file [0x44314,0x443e4).
// `r1 = #2; jump ##0xc` explicitly extends a packet-local jump to retain
// original Hexagon duplex ordering; packet 0x43d8 needs :mem_noshuf.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_refresh
agent13_wlan_record_refresh:
L_04314:
{
  r1 = #0x2 ; jump ##0xc
  r2 = #0x0
}
L_04320:
{
  r5:4 = combine(#0x0,#0x0)
  r17 = r1
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x18)
}
L_0432c:
{
  r16 = memw(r0+#0x34); 	memd(r29+#0x0) = r5:4
}
L_04330:
{
  r0 = r16
}
L_04334:
{
  call 0xb0
  r2 = #0x0
  memw(r16+#0x48) = r2
}
L_04340:
{
  r0 = add(r29,#0xb)
  p0 = cmp.eq(r17,#0x2); if (!p0.new) jump:t L_043d8
}
L_04348:
{
  r2 = memw(r16+#0x48)
  if (!cmp.eq(r2.new,#0x0)) jump:t L_043d8
}
L_04350:
{
  r2 = memw(r16+#0x8); 	r3 = memw(r16+#0x10)
}
L_04354:
{
  r3 = extractu(r3,#0x8,#0x8)
  r4 = and(r3,#0xff)
}
L_0435c:
{
  r2 = memub(r2+#0x3)
  memb(r29+#0xb) = r2.new
}
L_04364:
{
  r2 = add(r3,r4)
  memb(r29+#0xc) = r2.new
}
L_0436c:
{
  r3 = lsr(r2,#0x8)
  r1 = lsr(r2,#0x18)
  memb(r29+#0xd) = r3.new
}
L_04378:
{
  r2 = lsr(r2,#0x10)
  memb(r29+#0xf) = r1
}
L_04380:
{
  call 0xbc
  memb(r29+#0xe) = r2
}
L_04388:
{
  r2 = memw(r16+#0x8)
}
L_0438c:
{
  r3 = memw(r2+#0x14)
  r2 = memub(r2+#0x250)
}
L_04394:
{
  memb(r29+#0x0) = r2
}
L_04398:
{
  call 0x2157d8
  r0 = memuh(r3+#0x10)
}
L_043a0:
{
  call 0x224e40
  r0 = memw(r16+#0x8)
  memb(r29+#0x1) = r0
}
L_043ac:
{
  r0 = add(r29,#0x0)
  r2 = memw(r16+#0x10)
  memb(r29+#0x2) = r0
}
L_043b8:
{
  r2 += lsr(r2,#0x8)
  r3 = memw(r16+#0x8)
  memb(r29+#0x3) = r2.new
}
L_043c4:
{
  r2 = memub(r3+#0x0)
  memb(r29+#0x4) = r2.new
}
L_043cc:
{
  call 0x80
  r2 = memub(r3+#0x3)
  memb(r29+#0x5) = r2.new
}
L_043d8:
{
  memw(r16+#0x48) = #0x0
  r17:16 = memd(r29+#0x10)
}:mem_noshuf
L_043e0:
{
  dealloc_return
}
