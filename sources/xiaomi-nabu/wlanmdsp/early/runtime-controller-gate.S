// SPDX-License-Identifier: GPL-3.0-only
// Controller readiness/cleanup callback gate, file [0x4417c,0x441cc).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_controller_gate
agent13_wlan_controller_gate:
L_0417c:
{
  r16 = r0
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x8)
}
L_04184:
{
  r2 = memw(r16+#0x84)
  if (!cmp.eq(r2.new,#0x0)) jump:t L_04194
}
L_0418c:
{
  r2 = memw(r16+#0xe8)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_041c8
}
L_04194:
{
  r2 = memw(r16+#0x7c)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_041b0
}
L_0419c:
{
  call 0x2177c0
  r0 = add(r16,#0x58)
}
L_041a4:
{
  jump L_041bc
  r2 = memw(r16+#0x8)
  memw(r16+#0x78) = #0x0
}
L_041b0:
{
  r2 = memw(r16+#0x78)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_041c8
}
L_041b8:
{
  r2 = memw(r16+#0x8)
}
L_041bc:
{
  r0 = memw(r2+#0x2c); 	r17:16 = memd(r29+#0x0)
}
L_041c0:
{
  jump 0x3f0
  deallocframe
}
L_041c8:
{
  r17:16 = memd(r29+#0x0); 	dealloc_return
}
