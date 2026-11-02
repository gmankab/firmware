// SPDX-License-Identifier: GPL-3.0-only
// Remove one session list element without reordering the saved-register frame.
// File [0x44760,0x447dc); source ordering at 0x4760 forces correct duplex.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_list_unlink
agent13_wlan_record_list_unlink:
L_04760:
{
  r4 = r1; allocframe(#0x0)
  r3 = #-0x1
  r6 = add(r0,#0x3c)
}
L_0476c:
{
  r5 = memw(r0+#0x3c)
  if (cmp.eq(r5.new,#0x0)) jump:nt L_047d8
}
L_04774:
{
  r1 = r5
}
L_04778:
{
  r7 = memw(r1+#0x8)
}
L_0477c:
{
  p0 = cmp.eq(r7,r4); if (!p0.new) jump:t L_0478c
  r7 = memw(r1+#0x4)
}
L_04784:
{
  r8 = memw(r1+#0xc)
  if (cmp.eq(r8.new,r2)) jump:nt L_04798
}
L_0478c:
{
  r1 = r7
  p0 = cmp.eq(r7,#0x0); if (!p0.new) jump:t L_04778
}
L_04794:
{
  jump L_047d8
}
L_04798:
{
  p0 = cmp.eq(r5,r1); if (p0.new) jump:nt L_047b8
}
L_0479c:
{
  r2 = r5
}
L_047a0:
{
  r5 = memw(r2+#0x4)
  if (!cmp.eq(r5.new,r1)) jump:t L_0479c
}
L_047a8:
{
  if (p0.new) r2 = add(r2,#0x4)
  p0 = cmp.eq(r7,#0x0); 	memw(r2+#0x4) = r7
}
L_047b0:
{
  jump L_047c8
  if (p0) memw(r0+#0x40) = r2
}
L_047b8:
{
  r2 = memw(r5+#0x4)
  memw(r0+#0x3c) = r2.new
}
L_047c0:
{
  p0 = cmp.eq(r2,#0x0)
  if (p0.new) memw(r0+#0x40) = r6
}
L_047c8:
{
  r2 = memw(r0+#0xc)
}
L_047cc:
{
  call 0x21d878
  r0 = memw(r2+#0x8)
}
L_047d4:
{
  r3 = #0x0
}
L_047d8:
{
  r0 = r3; 	dealloc_return
}
