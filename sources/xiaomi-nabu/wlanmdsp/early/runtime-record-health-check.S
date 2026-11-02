// SPDX-License-Identifier: GPL-3.0-only
// Check record health, format flags and guarded platform callbacks. File [0x44a4c,0x44b20).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_health_check
agent13_wlan_record_health_check:
L_04a4c:
{
  call 0x2a0d1c
  allocframe(#0x20)
}
L_04a54:
{
  r17:16 = combine(r3,r2)
  r19 = r1
  r4 = memw(gp+#0x78c)
}
L_04a60:
{
  r4 = addasl(r4,r0,#0x2)
}
L_04a64:
{
  r18 = memw(r4+#0x54)
}
L_04a68:
{
  r0 = r18; 	r4 = memw(r18+#0x28)
}
L_04a6c:
{
  r5 = memub(r18+#0x250)
}
L_04a70:
{
  r4 = addasl(r4,r5,#0x2)
}
L_04a74:
{
  r4 = memw(r4+#0x328)
}
L_04a78:
{
  call -0xa0
  r20 = memw(r4+#0x24)
}
L_04a80:
{
  r1 = #0x4851
  immext(#0x140)
  r0 = ##0x148; 	memw(r29+#0x0) = r0
}
L_04a8c:
{
  call 0x216728
}
L_04a90:
{
  call -0xb8
  r0 = r18
}
L_04a98:
{
  r2 = r0
}
L_04a9c:
{
  call -0x1e54
  immext(#0xb02a6480)
  r1:0 = combine(r2,##-0x4fd59b70)
}
L_04aa8:
{
  r2 = memb(r18+#0x0)
  if (!cmp.eq(r2.new,#0x1)) jump:t L_04b1c
}
L_04ab0:
{
  call -0xd8
  r0 = r18
}
L_04ab8:
{
  if (!p0.new) r1:0 = combine(r19,r20)
  if (p0.new) r3 = add(r17,#0x0)
  p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt L_04ad0
}
L_04ac4:
{
  call 0x20a9f0
  r3:2 = combine(#0x0,r17)
}
L_04acc:
{
  jump L_04afc
}
L_04ad0:
{
  loop0(L_04ae0,#0x8)
  r2 = memw(r18+#0xd40)
  memw(r19+#0x0) = r2.new
}
L_04adc:
{
  r2 = add(r18,#0xd48)
}
L_04ae0:
{
  r4 = memub(r2+#0x3); 	r5 = memub(r2+#0x2)
}
L_04ae4:
{
  r5 |= asl(r4,#0x8)
  r6 = memub(r2+#0x1)
  r7 = memub(r2++#0x4)
}
L_04af0:
{
  r7 |= asl(r6,#0x8)
}
L_04af4:
{
  r7 |= asl(r5,#0x10)
  memw(r3++#0x4) = r7.new
}:endloop0
L_04afc:
{
  loop0(L_04b08,#0x8)
  r3:2 = combine(#0x0,#0x0)
}
L_04b04:
{
  r1 = r2
}
L_04b08:
{
  r0 = memw(r17++#0x4)
}
L_04b0c:
{
  r4 = popcount(r1:0)
}
L_04b10:
{
  r3 = add(r4,r3)
  nop
}:endloop0
L_04b18:
{
  memw(r16+#0x0) = r3
}
L_04b1c:
{
  jump 0x2a0d28
}
