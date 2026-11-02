// SPDX-License-Identifier: GPL-3.0-only
// Record state marking and bounded release, file [0x44244,0x44314).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_mark_release
agent13_wlan_record_mark_release:
L_04244:
{
  r17:16 = combine(#0x0,r0)
  r3 = #0x0
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x20)
}
L_04250:
{
  p0 = cmp.eq(r16,#0x0); 	r2 = memw(r16+#0x8)
}
L_04254:
{
  r4 = p0
}
L_04258:
{
  if (p0) jump:nt L_04268
  r2 = memub(r2+#0x3); 	memw(r29+#0xc) = r4
}
L_04260:
{
  r3 = memw(r16+#0x10)
}
L_04264:
{
  r3 = extractu(r3,#0x1,#0x1d)
}
L_04268:
{
  r1 = #0x4812
  immext(#0x100)
  r0 = ##0x110; 	memw(r29+#0x4) = r3
}
L_04274:
{
  call 0x216f40
  memw(r29+#0x0) = r2
}
L_0427c:
{
  r2 = memw(r16+#0x8)
}
L_04280:
{
  r1 = memub(r2+#0x3); 	r2 = memw(r29+#0xc)
}
L_04284:
{
  p0 = r2
  if (p0.new) jump:nt L_04294
  if (!p0.new) r2 = memw(r16+#0x10)
}
L_04290:
{
  r17 = extractu(r2,#0x1,#0x1d)
}
L_04294:
{
  call -0x1f74
  immext(#0xb02a6400)
  r0 = ##0xb02a643c; 	r2 = r17
}
L_042a0:
{
  call 0x2176bc
  r0 = add(r16,#0x18)
}
L_042a8:
{
  r0 = #0x0
  r2 = memw(r16+#0x10)
  memb(r16+#0x4c) = r0.new
}
L_042b4:
{
  p0 = !tstbit(r2,#0x1d)
  immext(#0xe400ffc0)
  r2 = and(r2,##-0x1bff0001)
  memw(r16+#0x10) = r2.new
}
L_042c4:
{
  if (p0) jump:nt L_042d4
  if (!p0) r2 = memw(r29+#0xc)
}
L_042cc:
{
  p0 = r2
  if (!p0.new) jump:t L_042f0
}
L_042d4:
{
  r0 = memw(r16+#0x8)
}
L_042d8:
{
  r2 = memw(r0+#0x14)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_042f0
}
L_042e0:
{
  r5:4 = combine(#0x0,#0x0)
  r3:2 = memd(r16+#0x0)
}
L_042e8:
{
  p0 = cmp.eq(r3:2,r5:4)
  if (p0.new) jump:nt L_042f4
}
L_042f0:
{
  r17:16 = memd(r29+#0x18); 	dealloc_return
}
L_042f4:
{
  call 0x119c
}
L_042f8:
{
  p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt L_04308
}
L_042fc:
{
  r0 = r16
  r17:16 = memd(r29+#0x18); 	deallocframe
}
L_04304:
{
  jump 0xdd0
}
L_04308:
{
  r0 = r16
  r17:16 = memd(r29+#0x18); 	deallocframe
}
L_04310:
{
  jump -0x388
}
