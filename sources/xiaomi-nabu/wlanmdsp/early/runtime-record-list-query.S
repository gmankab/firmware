// SPDX-License-Identifier: GPL-3.0-only
// Test whether a linked record with given key is present; fast return.
// File [0x447dc,0x44804).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_list_query
agent13_wlan_record_list_query:
L_047dc:
{
  r0 = #-0x1; 	r4 = memw(r0+#0x3c)
}
L_047e0:
{
  p0 = cmp.eq(r4,#0x0); 	if (p0.new) jumpr:nt r31
}
L_047e4:
{
  r5 = memw(r4+#0x8)
}
L_047e8:
{
  p0 = cmp.eq(r5,r1); if (!p0.new) jump:t L_047f8
  r5 = memw(r4+#0x4)
}
L_047f0:
{
  r6 = memw(r4+#0xc)
  if (cmp.eq(r6.new,r2)) jump:nt 0x14
}
L_047f8:
{
  r4 = r5
  p0 = cmp.eq(r5,#0x0); if (!p0.new) jump:t L_047e4
}
L_04800:
{
  jumpr r31
}
