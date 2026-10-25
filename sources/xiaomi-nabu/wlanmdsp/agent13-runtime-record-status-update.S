// SPDX-License-Identifier: GPL-3.0-only
// Set request status fields and adjust packed readiness bits in a guarded call path.
// File [0x44614,0x446f0).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_status_update
agent13_wlan_record_status_update:
L_04614:
{
  call 0x2a1164
  allocframe(#0x18)
}
L_0461c:
{
  r17:16 = combine(r1,r0)
}
L_04620:
{
  r18 = memw(r16+#0x34); 	r2 = memub(r16+#0x3)
}
L_04624:
{
  p0 = cmp.eq(r18,#0x0); if (p0.new) jump:nt L_04650
}
L_04628:
{
  r1 = #0x1052
  immext(#0x100)
  r0 = ##0x128; 	r3 = memw(r18+#0x10)
}
L_04634:
{
  r2 = extractu(r3,#0x1,#0x1d)
  memw(r29+#0x0) = r2
}
L_0463c:
{
  call 0x216b78
  memw(r29+#0x4) = r2
}
L_04644:
{
  r2 = memw(r18+#0x10); 	r1 = memub(r16+#0x3)
}
L_04648:
{
  r2 = extractu(r2,#0x1,#0x1d)
  jump L_04668
}
L_04650:
{
  r1 = #0x1052
  immext(#0x100)
  r0 = ##0x128; 	memw(r29+#0x0) = r2
}
L_0465c:
{
  call 0x216b58
  memw(r29+#0x4) = #0x0
}
L_04664:
{
  r2 = #0x0; 	r1 = memub(r16+#0x3)
}
L_04668:
{
  call -0x2348
  immext(#0xb02a6440)
  r0 = ##-0x4fd59ba0
}
L_04674:
{
  r2 = memw(r16+#0x34)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_04688
}
L_0467c:
{
  r2 = memub(r2+#0x13)
}
L_04680:
{
  p0 = tstbit(r2,#0x5)
  if (p0.new) jump:t L_046ec
}
L_04688:
{
  r2 = memub(r17+#0x0)
}
L_0468c:
{
  p0 = bitsclr(r2,#0x14)
  if (p0.new) jump:nt L_046ec
  immext(#0xd40)
  if (!p0.new) r2 = memw(r16+##0xd40)
}
L_0469c:
{
  r18 = add(r16,#0xd48)
}
L_046a0:
{
  p0 = cmp.gtu(r2,#0xfe)
  r0 = r18; 	r2 = add(r2,#1)
}
L_046a8:
{
  call -0xe8
  immext(#0xd40)
  if (!p0) memw(r16+##0xd40) = r2
}
L_046b4:
{
  r2 = memw(r17+#0x0)
  if (cmp.eq(r2.new,#0x10)) jump:nt L_046d4
}
L_046bc:
{
  p0 = cmp.eq(r2,#0x4); if (!p0.new) jump:nt L_046ec
}
L_046c0:
{
  r2 = memw(r16+#0xd44)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_046ec
}
L_046c8:
{
  r2 = add(r2,#-0x1)
  jump 0x2a118c
  memw(r16+#0xd44) = r2.new
}
L_046d4:
{
  r2 = memw(r16+#0xd44)
}
L_046d8:
{
  p0 = cmp.gtu(r2,#0xfe)
  r2 = add(r2,#0x1)
  immext(#0xd40)
  if (!p0.new) memw(r16+##0xd44) = r2.new
}
L_046e8:
{
  memw(r18+#0x0) = setbit(#0x0)
}
L_046ec:
{
  jump 0x2a1168
}
