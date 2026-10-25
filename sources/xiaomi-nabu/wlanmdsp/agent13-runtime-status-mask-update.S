// SPDX-License-Identifier: GPL-3.0-only
// Manipulate two 8-bit feature masks with controller zero/bit-test checks.
// File [0x44804,0x4492c).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_status_mask_update
agent13_wlan_status_mask_update:
L_04804:
{
  jumpr r31
  r0 = #0x0; 	memb(r4+#0x0) = r3
}
L_0480c:
{
  p0 = cmp.eq(r1,#0x2); if (p0.new) jump:nt L_04860
}
L_04810:
{
  p0 = cmp.eq(r1,#0x1); 	if (!p0.new) jumpr:nt r31
}
L_04814:
{
  p0 = cmp.eq(r3,#0x0); 	r4 = memw(r0+#0x10)
}
L_04818:
{
  r3 = r4
  immext(#0x40)
  if (p0) memb(r0+##0x44) = r2
}
L_04824:
{
  r3 = insert(r2,#0x8,#0x0)
  if (p0) jump:nt L_04928
  if (p0) memw(r0+#0x10) = r3.new
}
L_04830:
{
  immext(#0xa000000)
  r3 = and(r4,##0xa000000)
  if (cmp.eq(r3.new,#0x0)) jump:nt L_048ac
}
L_0483c:
{
  p0 = !tstbit(r4,#0x19)
  if (p0.new) jumpr:nt r31
}
L_04844:
{
  p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:t L_04894
}
L_04848:
{
  r2 = memub(r0+#0x44)
}
L_0484c:
{
  p0 = cmp.eq(r2,#0x0)
}
L_04850:
{
  r3 = mux(p0,#0xf,r2)
}
L_04854:
{
  r4 = or(r3,and(r4,#-0x100))
  r2 = aslh(r3)
  jump L_04914
}
L_04860:
{
  p0 = cmp.eq(r3,#0x0); 	r4 = memw(r0+#0x10)
}
L_04864:
{
  r3 = r4
  immext(#0x40)
  if (p0) memb(r0+##0x45) = r2
}
L_04870:
{
  r3 = insert(r2,#0x8,#0x8)
  if (p0) jump:nt L_04928
  if (p0) memw(r0+#0x10) = r3.new
}
L_0487c:
{
  immext(#0xa000000)
  r3 = and(r4,##0xa000000)
  if (cmp.eq(r3.new,#0x0)) jump:nt L_048d0
}
L_04888:
{
  p0 = !tstbit(r4,#0x1b)
  if (p0.new) jumpr:nt r31
}
L_04890:
{
  p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt L_048f4
}
L_04894:
{
  immext(#0xe5ffffc0)
  r4 = and(r4,##-0x1a000001)
  r2 = and(r2,#0xff)
  memw(r0+#0x10) = r4.new
}
L_048a4:
{
  r4 = insert(r3,#0x8,#0x0)
  jump L_0491c
}
L_048ac:
{
  p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:t L_048c4
}
L_048b0:
{
  r4 = insert(r3,#0x9,#0x10)
  r5 = memub(r0+#0x44)
  memw(r0+#0x10) = r4.new
}
L_048bc:
{
  p0 = cmp.eq(r5,#0x0)
}
L_048c0:
{
  r2 = mux(p0,#0xf,r5)
}
L_048c4:
{
  r4 = insert(r2,#0x8,#0x0)
  jumpr r31
  memw(r0+#0x10) = r4.new
}
L_048d0:
{
  p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:t L_048e8
}
L_048d4:
{
  r4 = insert(r3,#0x9,#0x10)
  r5 = memub(r0+#0x45)
  memw(r0+#0x10) = r4.new
}
L_048e0:
{
  p0 = cmp.eq(r5,#0x0)
}
L_048e4:
{
  r2 = mux(p0,#0x2d,r5)
}
L_048e8:
{
  r4 = insert(r2,#0x8,#0x8)
  jumpr r31
  memw(r0+#0x10) = r4.new
}
L_048f4:
{
  r2 = memub(r0+#0x45)
}
L_048f8:
{
  r2 = asl(r2,#0x8)
  p0 = cmp.eq(r2,#0x0)
}
L_04900:
{
  immext(#0x2d00)
  if (p0) r2 = ##0x2d00
}
L_04908:
{
  immext(#0xffff00c0)
  r4 = or(r2,and(r4,##-0xff01))
  r2 = asl(r2,#0x8)
}
L_04914:
{
  r4 = insert(r2,#0x9,#0x10)
  memw(r0+#0x10) = r4
}
L_0491c:
{
  r2 = or(r4,r2)
  jumpr r31
  memw(r0+#0x10) = r2.new
}
L_04928:
{
  jumpr r31
}
