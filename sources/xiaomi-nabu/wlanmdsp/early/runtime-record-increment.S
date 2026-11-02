// SPDX-License-Identifier: GPL-3.0-only
// Advance a record-ring slot and refresh 64-bit plane metadata, using original
// memory order for the four :mem_noshuf packet groups. File 0x43e04–0x43f54.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_increment
agent13_wlan_record_increment:
L_03e04:
{
  call 0x2a1974
  allocframe(#0x28)
}
L_03e0c:
{
  r17:16 = combine(r1,r0)
}
L_03e10:
{
  r2 = #0x59
  if (!cmp.gtu(r2.new,r17)) jump:nt L_03f44
}
L_03e18:
{
  r2 = memw(r16+#0xc)
}
L_03e1c:
{
  r2 = memw(r2+#0x14)
}
L_03e20:
{
  r18 = memuh(r2+#0x4)
}
L_03e24:
{
  r3 = add(r18,#0x1)
  r4 = memuh(r2+#0x6)
  memh(r2+#0x4) = r3.new
}
L_03e30:
{
  r5 = zxth(r3); 	r19 = memw(r2+#0x0)
}
L_03e34:
{
  call 0x217870
  p0 = cmp.gtu(r4,r5)
  if (!p0.new) memh(r2+#0x4) = #0x0
}
L_03e40:
{
  r4 = addasl(r19,r18,#0x5)
  r5 = asl(r18,#0x5)
  r3:2 = combine(#0,#0x1); 	r7:6 = combine(#0,#0x0)
}
L_03e4c:
{
  memd(r19+r5<<#0x0) = r1:0
}
L_03e50:
{
  memw(r4+#0x8) = #0x1
  r1:0 = memd(r16+#0x0)
}:mem_noshuf
L_03e58:
{
  memd(r4+#0x10) = r1:0
  r5 = memw(r16+#0x10)
}:mem_noshuf
L_03e60:
{
  r5 = extractu(r5,#0x1e,#0x0)
  memb(r4+#0x1c) = r17
}
L_03e68:
{
  memw(r4+#0x18) = r5
  r1:0 = memd(r16+#0x0)
}:mem_noshuf
L_03e70:
{
  r5:4 = combine(r1,r0)
}
L_03e74:
{
  r5:4 &= asl(r3:2,r17)
  r3:2 = asl(r3:2,r17)
}
L_03e7c:
{
  p0 = cmp.eq(r5:4,r7:6)
  if (p0.new) jump:nt L_03f10
}
L_03e84:
{
  r4 = memw(r16+#0x8); 	r5 = memw(r16+#0x10)
}
L_03e88:
{
  r2 = extractu(r5,#0x9,#0x10)
  r1:0 = and(r1:0,~r3:2)
}
L_03e90:
{
  r3 = extractu(r5,#0x1,#0x1b)
  memd(r16+#0x0) = r1:0
  r4 = memub(r4+#0x3)
}:mem_noshuf
L_03e9c:
{
  immext(#0xc0)
  r0 = ##0xf8
  memw(r29+#0xc) = r0; 	memw(r29+#0x8) = r1
}
L_03ea8:
{
  r1 = #0x4816
  memw(r29+#0x14) = r3; 	memw(r29+#0x10) = r2
}
L_03eb0:
{
  call 0x217304
  memw(r29+#0x4) = r17; 	memw(r29+#0x0) = r4
}
L_03eb8:
{
  r2 = memw(r16+#0x8)
  r1:0 = memd(r16+#0x0)
}
L_03ec0:
{
  r3 = memw(r16+#0x10)
}
L_03ec4:
{
  r4 = extractu(r3,#0x9,#0x10)
  r3 = extractu(r3,#0x1,#0x1b)
  r2 = memub(r2+#0x3); 	memw(r29+#0xc) = r0
}
L_03ed0:
{
  immext(#0xb02a6400)
  r1:0 = combine(#0x6,##0xb02a6418)
  memw(r29+#0x8) = r1; 	memw(r29+#0x14) = r3
}
L_03edc:
{
  memw(r29+#0x10) = r4; 	memw(r29+#0x4) = r17
}
L_03ee0:
{
  call -0x1bd0
  memw(r29+#0x0) = r2
}
L_03ee8:
{
  r2 = memw(r16+#0x8)
}
L_03eec:
{
  r3 = memuh(r2+#0x1c)
}
L_03ef0:
{
  p0 = tstbit(r3,#0); if (!p0.new) jump:t L_03f10
}
L_03ef4:
{
  r3 = memb(r2+#0x0)
  if (!cmp.eq(r3.new,#0x1)) jump:t L_03f10
}
L_03efc:
{
  r1:0 = combine(#0,#0x0); 	r2 = memw(r2+#0x34)
}
L_03f00:
{
  r3:2 = memd(r2+#0x0)
}
L_03f04:
{
  p0 = cmp.eq(r3:2,r1:0)
  if (p0.new) r0 = add(r16,#0x0)
  if (p0.new) jump:nt L_03f14
}
L_03f10:
{
  jump 0x2a1944
}
L_03f14:
{
  call 0x40
}
L_03f18:
{
  call 0x1578
  r0 = memw(r16+#0x8)
}
L_03f20:
{
  if (!p0.new) r0 = add(r16,#0x0)
  p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt L_03f30
}
L_03f28:
{
  call 0x2a18c4
}
L_03f2c:
{
  jump 0x11a8
}
L_03f30:
{
  call 0x7e8e8
  r1 = #0x2; 	r0 = memw(r16+#0x8)
}
L_03f38:
{
  call 0x2a18b4
  r0 = r16
}
L_03f40:
{
  jump 0x48
}
L_03f44:
{
  call -0x324
  immext(#0xb02a6380)
  r0 = ##-0x4fd59c73
  r1 = #0x15e
}
