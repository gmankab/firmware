// SPDX-License-Identifier: GPL-3.0-only
// Clear counters and runtime record state on release. File [0x44b20,0x44b7c).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_reset_helper
agent13_wlan_record_reset_helper:
L_04b20:
{
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x8)
}
L_04b24:
{
  r16 = memw(r0+#0x34)
}
L_04b28:
{
  r2 = memw(r16+#0x84)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_04b78
}
L_04b30:
{
  r2 = memw(r16+#0x78)
}
L_04b34:
{
  p0 = cmp.eq(r2,#0x1)
  if (p0.new) r0 = memw(r0+#0x2c)
}
L_04b3c:
{
  immext(#0xfffffa40)
  if (p0) call -0x58c
}
L_04b44:
{
  call 0x216e18
  r0 = add(r16,#0x58)
}
L_04b4c:
{
  r3 = #0xff
  r0 = add(r16,#0x90)
  r1 = #0x0; 	r5:4 = combine(#0,#0x0)
}
L_04b58:
{
  r2 = #0x53
  r3 = #0x0
  memb(r16+#0x80) = r3
  memw(r16+#0x84) = #0x0
}
L_04b68:
{
  memw(r16+#0xe8) = #0x0
  memd(r16+#0x78) = r5:4
}
L_04b70:
{
  call -0x2bc8
  memb(r16+#0x88) = r3
}
L_04b78:
{
  r17:16 = memd(r29+#0x0); 	dealloc_return
}
