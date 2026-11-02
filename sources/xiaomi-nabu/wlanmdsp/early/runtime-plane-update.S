// SPDX-License-Identifier: GPL-3.0-only
// Controller plane-byte accounting, original conditional memory order via
// :mem_noshuf in packets 0x409c/0x40a8/0x40dc. File 0x43f88–0x4417c.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_plane_update
agent13_wlan_plane_update:
L_03f88:
{
  call 0x2a17e0
  allocframe(#0x20)
}
L_03f90:
{
  r16 = r0
}
L_03f94:
{
  r2 = memub(r16+#0x13)
}
L_03f98:
{
  p0 = !tstbit(r2,#0x2)
  if (!p0.new) jump:t L_03fa8
  if (!p0.new) r2 = memw(r16+#0x8)
}
L_03fa4:
{
  jump 0x2a18a0
}
L_03fa8:
{
  r19 = #0x0
  r3 = memw(r2+#0x14); 	r0 = memw(r2+#0x2c)
}
L_03fb0:
{
  r17 = memuh(r3+#0x44)
}
L_03fb4:
{
  call 0x1fe070
  r18 = asl(r17,#0xa)
}
L_03fbc:
{
  call L_0403c
  r3:2 = combine(r19,r18)
}
L_03fc4:
{
  call -0x10ac
  r21:20 = combine(r1,r0)
  r0 = memw(r16+#0x8)
}
L_03fd0:
{
  immext(#0x600)
  r2 = +mpyi(r17,##0x600)
  r3 = add(r0,#-0x1)
  r17 = add(r16,#0x18)
}
L_03fe0:
{
  immext(#0x10624dc0)
  r0 = ##0x10624dd3
  r2 = sub(r2,r20)
}
L_03fec:
{
  r3 = add(r2,mpyi(r3,r18))
}
L_03ff0:
{
  r3:2 = mpyu(r3,r0)
  r0 = r17
}
L_03ff8:
{
  call 0x217964
  r18 = lsr(r3,#0x6)
}
L_04000:
{
  r2 = #0x3e8
  r4 = #0x0; 	r0 = r17
}
L_04008:
{
  r3:2 = mpyu(r18,r2)
  call 0x2178d8
}
L_04010:
{
  r1 = #0x4812
  immext(#0x100)
  r0 = ##0x118; 	r2 = memw(r16+#0x8)
}
L_0401c:
{
  r2 = memub(r2+#0x3); 	memw(r29+#0x4) = r18
}
L_04020:
{
  call 0x217194
  memw(r29+#0x0) = r2
}
L_04028:
{
  immext(#0xb02a6440)
  r0 = ##0xb02a6448; 	r2 = memw(r16+#0x8)
}
L_04030:
{
  call 0x2a17a8
  r2 = r18; 	r1 = memub(r2+#0x3)
}
L_04038:
{
  jump -0x1d18
}
L_0403c:
{
  immext(#0xffffd0c0)
  jump -0x2f1c
}
L_04044:
{
  call 0x2a1724
  allocframe(#0x38)
}
L_0404c:
{
  r17:16 = combine(r1,r0)
}
L_04050:
{
  r2 = #0x59
  if (!cmp.gtu(r2.new,r17)) jump:nt L_0416c
}
L_04058:
{
  r2 = memw(r16+#0xc)
  r19:18 = memd(r16+#0x0)
}
L_04060:
{
  r2 = memw(r2+#0x14)
}
L_04064:
{
  r20 = memuh(r2+#0x4)
}
L_04068:
{
  r3 = add(r20,#0x1)
  r4 = memuh(r2+#0x6)
  memh(r2+#0x4) = r3.new
}
L_04074:
{
  r5 = zxth(r3); 	r21 = memw(r2+#0x0)
}
L_04078:
{
  call 0x21762c
  p0 = cmp.gtu(r4,r5)
  if (!p0.new) memh(r2+#0x4) = #0x0
}
L_04084:
{
  r2 = addasl(r21,r20,#0x5)
  r3 = asl(r20,#0x5)
  r7:6 = combine(r19,r18)
}
L_04090:
{
  r1:0 = combine(#0x0,#0x1)
  r21:20 = combine(#0x0,#0x0)
  memd(r21+r3<<#0x0) = r1:0
}
L_0409c:
{
  r7:6 &= asl(r1:0,r17)
  memw(r2+#0x8) = #0x2
  r5:4 = memd(r16+#0x0)
}:mem_noshuf
L_040a8:
{
  p0 = cmp.eq(r7:6,r21:20)
  memd(r2+#0x10) = r5:4
  r3 = memw(r16+#0x10)
}:mem_noshuf
L_040b4:
{
  r3 = extractu(r3,#0x1e,#0x0)
  memb(r2+#0x1c) = r17
}
L_040bc:
{
  if (!p0) jump:nt L_0413c
  memw(r2+#0x18) = r3
}
L_040c4:
{
  r1:0 = asl(r1:0,r17)
  r3:2 = memd(r16+#0x0)
  r4 = memw(r16+#0x8)
}
L_040d0:
{
  r0 = or(r2,r0)
  r1 = or(r3,r1)
  r5 = memw(r16+#0x10)
}
L_040dc:
{
  r2 = extractu(r5,#0x9,#0x10)
  memd(r16+#0x0) = r1:0
  r3 = memub(r4+#0x3)
}:mem_noshuf
L_040e8:
{
  immext(#0x100)
  r0 = ##0x100
  memw(r29+#0xc) = r0; 	memw(r29+#0x8) = r1
}
L_040f4:
{
  r1 = #0x4816
  memd(r29+#0x18) = r19:18; 	memw(r29+#0x10) = r2
}
L_040fc:
{
  call 0x2170b8
  memw(r29+#0x4) = r17; 	memw(r29+#0x0) = r3
}
L_04104:
{
  r2 = memw(r16+#0x8)
  r1:0 = memd(r16+#0x0)
}
L_0410c:
{
  r3 = memw(r16+#0x10)
}
L_04110:
{
  r3 = extractu(r3,#0x9,#0x10)
  r2 = memub(r2+#0x3); 	memw(r29+#0xc) = r0
}
L_04118:
{
  immext(#0xb02a6400)
  r1:0 = combine(#0x6,##0xb02a6424)
  memw(r29+#0x8) = r1; 	memw(r29+#0x14) = r18
}
L_04124:
{
  memw(r29+#0x10) = r3; 	memw(r29+#0x4) = r17
}
L_04128:
{
  call -0x1e18
  memw(r29+#0x0) = r2
}
L_04130:
{
  p0 = cmp.eq(r19:18,r21:20)
  if (p0.new) r0 = add(r16,#0x18)
  if (p0.new) jump:nt L_04140
}
L_0413c:
{
  jump 0x2a1708
}
L_04140:
{
  call 0x21781c
}
L_04144:
{
  r2 = memub(r16+#0x13)
}
L_04148:
{
  p0 = !tstbit(r2,#0x5)
  if (!p0.new) r0 = add(r16,#0x0)
  if (p0.new) jump:t L_04160
}
L_04154:
{
  r3:2 = combine(#0x0,#0x0)
}
L_04158:
{
  call 0x111c
  memd(r16+#0x50) = r3:2
}
L_04160:
{
  call 0x2a1678
  r0 = r16
}
L_04168:
{
  jump 0x14
}
L_0416c:
{
  call -0x54c
  immext(#0xb02a6380)
  r0 = ##-0x4fd59c5f
  r1 = #0x18a
}
