// SPDX-License-Identifier: GPL-3.0-only
// Insert indexed record into a linked controller list, preserving tail jumps.
// File [0x446f0,0x44760); the jump at 0x4758 requires `jump ##0x2a10fc`.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_list_insert
agent13_wlan_record_list_insert:
L_046f0:
{
  call 0x2a1088
  allocframe(#0x10)
}
L_046f8:
{
  r17:16 = combine(r2,r0)
  r19:18 = combine(r3,r1)
}
L_04700:
{
  r4 = memw(r16+#0x3c)
  if (cmp.eq(r4.new,#0x0)) jump:nt L_04720
}
L_04708:
{
  r2 = memw(r4+#0x8)
  if (!cmp.eq(r2.new,r18)) jump:t L_04718
}
L_04710:
{
  r2 = memw(r4+#0xc)
  if (cmp.eq(r2.new,r17)) jump:nt L_04754
}
L_04718:
{
  r4 = memw(r4+#0x4)
  if (!cmp.eq(r4.new,#0x0)) jump:t L_04708
}
L_04720:
{
  r2 = memw(r16+#0xc)
}
L_04724:
{
  call 0x21da08
  r0 = memw(r2+#0x8)
}
L_0472c:
{
  r4 = add(r0,#0x4)
  if (p0.new) r2 = #0x2
  p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt L_04758
}
L_04738:
{
  r2 = #0x0
  memb(r0+#0x0) = r19; 	memw(r0+#0x8) = r18
}
L_04740:
{
  memw(r0+#0xc) = r17; 	memw(r0+#0x4) = #0
}
L_04744:
{
  r3 = memw(r16+#0x40)
}
L_04748:
{
  jump L_04758
  memw(r3+#0x0) = r0
  memw(r16+#0x40) = r4
}
L_04754:
{
  r2 = #0x0
}
L_04758:
{
  r0 = r2 ; jump ##0x2a10fc
}
