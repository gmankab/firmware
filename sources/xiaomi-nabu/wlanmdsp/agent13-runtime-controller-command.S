// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 controller mode classification, state callbacks and guarded record updates.
// File [0x4445c,0x445b0).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_controller_command
agent13_wlan_controller_command:
L_0445c:
{
  call 0x2a130c
  allocframe(#0x48)
}
L_04464:
{
  r17 = r0; 	r18 = r3
}
L_04468:
{
  r20 = cmp.eq(r18,#0x0)
  r16 = memw(r17+#0x34)
}
L_04470:
{
  r2 = memw(r16+#0x10); 	r1 = memw(r16+#0x8)
}
L_04474:
{
  r2 = extractu(r2,#0x1,#0x1a)
  immext(#0xa000000)
  r19 = and(r2,##0xa000000)
  r4 = memub(r16+#0x4c)
}
L_04484:
{
  r0 = extractu(r19,#0x1,#0x1b)
  r1 = #0x4828
  r3 = memub(r1+#0x3)
  r5 = memub(r16+#0x4d)
}
L_04494:
{
  r0 = extractu(r19,#0x1,#0x19)
  memw(r29+#0x10) = r0
}
L_0449c:
{
  immext(#0xa000000)
  r21 = cmp.eq(r19,##0xa000000)
  memw(r29+#0xc) = r0
}
L_044a8:
{
  immext(#0x100)
  r0 = ##0x120
  memw(r29+#0x1c) = r5; 	memw(r29+#0x18) = r2
}
L_044b4:
{
  memw(r29+#0x8) = r21; 	memw(r29+#0x4) = r20
}
L_044b8:
{
  call 0x216cfc
  memw(r29+#0x14) = r4; 	memw(r29+#0x0) = r3
}
L_044c0:
{
  r2 = memw(r16+#0x8); 	r3 = memw(r16+#0x10)
}
L_044c4:
{
  r1 = extractu(r3,#0x1,#0x1a)
  r0 = extractu(r3,#0x1,#0x19)
  r2 = memub(r2+#0x3)
  r4 = memub(r16+#0x4c)
}
L_044d4:
{
  r3 = extractu(r3,#0x1,#0x1b)
  r5 = memub(r16+#0x4d)
}
L_044dc:
{
  immext(#0xb02a6440)
  r1:0 = combine(#0x8,##0xb02a6454)
  memw(r29+#0x18) = r1; 	memw(r29+#0xc) = r0
}
L_044e8:
{
  memw(r29+#0x1c) = r5; 	memw(r29+#0x14) = r4
}
L_044ec:
{
  memw(r29+#0x10) = r3; 	memw(r29+#0x8) = r21
}
L_044f0:
{
  call -0x21e0
  memw(r29+#0x4) = r20; 	memw(r29+#0x0) = r2
}
L_044f8:
{
  r2 = memb(r16+#0x4c)
  if (cmp.eq(r2.new,#0x1)) jump:t L_04510
}
L_04500:
{
  r2 = memub(r16+#0x13)
}
L_04504:
{
  p0 = !tstbit(r2,#0x2)
  if (!p0.new) r0 = add(r16,#0x18)
  if (!p0.new) jump:t L_04514
}
L_04510:
{
  jump 0x2a1334
}
L_04514:
{
  immext(#0xa000000)
  p1 = cmp.eq(r19,##0xa000000)
  if (!p1.new) jump:t L_04534
}
L_04520:
{
  r2 = p1
}
L_04524:
{
  call 0x217438
  r20 = r2
}
L_0452c:
{
  r2 = r20
}
L_04530:
{
  p1 = r2
}
L_04534:
{
  p0 = cmp.eq(r18,#0x0); if (p0.new) jump:nt L_04550
  if (!p1) jump:nt L_04580
}
L_0453c:
{
  call -0x228
  r0 = r17
}
L_04544:
{
  call 0x2a1294
  r0 = r16
}
L_0454c:
{
  jump -0x308
}
L_04550:
{
  immext(#0xa000000)
  r2 = ##0xa000000; 	r0 = r16
}
L_04558:
{
  p0 = cmp.eq(r19,r2)
}
L_0455c:
{
  r2 = p0
}
L_04560:
{
  call -0x31c
  r16 = r2
}
L_04568:
{
  r2 = r16; 	r0 = r17
}
L_0456c:
{
  p0 = r2
  r2 = #0x1
}
L_04574:
{
  call 0x2a1264
  r1 = mux(p0,#0x2,#0x4)
}
L_0457c:
{
  jump -0x25c
}
L_04580:
{
  r2 = memw(r16+#0x84)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_04510
}
L_04588:
{
  r2 = memw(r16+#0x78)
  if (!cmp.eq(r2.new,#0x0)) jump:t L_04510
}
L_04590:
{
  r2 = memw(r16+#0x7c)
  if (!cmp.eq(r2.new,#0x0)) jump:t L_04510
}
L_04598:
{
  call 0x20
  r0 = memw(r17+#0x2c)
}
L_045a0:
{
  r2 = memuh(r16+#0x12)
  memw(r16+#0x78) = #0x1
}
L_045a8:
{
  jump 0x2a129c
  memb(r16+#0xe2) = r2
}
