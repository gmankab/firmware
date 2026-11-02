// SPDX-License-Identifier: GPL-3.0-only
// Validate active session readiness and encoded plane status. File [0x449d8,0x44a4c).
// Extended packet-local tail jump at 0x4a44 is explicitly source-encoded.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_session_state_guard
agent13_wlan_session_state_guard:
L_049d8:
{
  call 0x2a0da0
  allocframe(#0x10)
}
L_049e0:
{
  r16 = r0
}
L_049e4:
{
  call 0x7e298
  r17 = memw(r16+#0x2c)
}
L_049ec:
{
  call 0x208e88
  r18 = r0; 	r0 = r17
}
L_049f4:
{
  p0 = cmp.eq(r18,#0x0); if (p0.new) jump:nt L_04a44
}
L_049f8:
{
  p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:nt L_04a44
  if (p0.new) r2 = memw(r16+#0x28)
}
L_04a00:
{
  r3 = memub(r16+#0x250)
}
L_04a04:
{
  r2 = addasl(r2,r3,#0x2)
}
L_04a08:
{
  r2 = memw(r2+#0x328)
}
L_04a0c:
{
  r2 = memub(r2+#0x5)
  if (!cmp.eq(r2.new,#0x1)) jump:nt L_04a44
}
L_04a14:
{
  call 0x1d3e88
  r0 = r17
}
L_04a1c:
{
  p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt L_04a44
  if (!p0.new) r2 = memuh(r16+#0x1c)
}
L_04a24:
{
  p0 = tstbit(r2,#0); if (!p0.new) jump:t L_04a44
}
L_04a28:
{
  r2 = memb(r16+#0x0)
  if (!cmp.eq(r2.new,#0x1)) jump:t L_04a44
}
L_04a30:
{
  r0 = #0x1; 	r2 = memw(r16+#0x34)
}
L_04a34:
{
  r2 = memub(r2+#0x13)
}
L_04a38:
{
  p0 = !tstbit(r2,#0x6)
  if (!p0.new) r0 = #0x0
  jump 0x2a0e1c
}
L_04a44:
{
  r0 = #0x0 ; jump ##0x2a0e10
}
