// SPDX-License-Identifier: GPL-3.0-only
// Select and mask one status field according to request class.
// File [0x4492c,0x44960).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_status_mask_query
agent13_wlan_status_mask_query:
L_0492c:
{
  p0 = cmp.eq(r1,#0x3); if (!p0.new) jump:t L_0493c
}
L_04930:
{
  immext(#0x1c0)
  r3:2 = combine(#0x10,##0x1ff)
  jump L_04954
}
L_0493c:
{
  p0 = cmp.eq(r1,#0x2); if (p0.new) jump:nt L_0494c
}
L_04940:
{
  r2 = #0x0; 	p0 = cmp.eq(r1,#0x1)
}
L_04944:
{
  jump L_0495c
  if (p0) r2 = memub(r0+#0x10)
}
L_0494c:
{
  immext(#0xc0)
  r3:2 = combine(#0x8,##0xff)
}
L_04954:
{
  r4 = memw(r0+#0x10)
}
L_04958:
{
  r2 &= lsr(r4,r3)
}
L_0495c:
{
  r0 = r2; 	jumpr r31
}
