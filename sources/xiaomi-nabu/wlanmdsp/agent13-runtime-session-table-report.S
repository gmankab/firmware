// SPDX-License-Identifier: GPL-3.0-only
// Collect record-table and session metadata for diagnostics. File [0x44960,0x449d8).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_session_table_report
agent13_wlan_session_table_report:
L_04960:
{
  r17:16 = combine(r0,r1)
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x18)
}
L_04968:
{
  r1 = #0x4814
  immext(#0x140)
  r0 = ##0x140; 	r2 = memw(r16+#0x4)
}
L_04974:
{
  r4 = memw(r16+#0xc); 	r3 = memw(r16+#0x0)
}
L_04978:
{
  r5 = memw(r16+#0x10); 	r2 = memub(r2+#0x3)
}
L_0497c:
{
  memw(r29+#0xc) = r5; 	memw(r29+#0x8) = r4
}
L_04980:
{
  call 0x216834
  memw(r29+#0x4) = r3; 	memw(r29+#0x0) = r2
}
L_04988:
{
  immext(#0xb02a6480)
  r1:0 = combine(#0x4,##0xb02a6484)
  r2 = memw(r16+#0x4); 	r4 = memw(r16+#0xc)
}
L_04994:
{
  r3 = memw(r16+#0x0); 	r5 = memw(r16+#0x10)
}
L_04998:
{
  r2 = memub(r2+#0x3); 	memw(r29+#0xc) = r5
}
L_0499c:
{
  memw(r29+#0x8) = r4; 	memw(r29+#0x4) = r3
}
L_049a0:
{
  call -0x2690
  memw(r29+#0x0) = r2
}
L_049a8:
{
  jump L_049d0
  r17 = memw(r17+#0x3c)
}
L_049b0:
{
  r2 = memw(r17+#0x8)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_049cc
}
L_049b8:
{
  r3 = memw(r16+#0x0); 	r4 = memub(r17+#0x0)
}
L_049bc:
{
  p0 = bitsclr(r3,r4)
  if (p0.new) jump:nt L_049cc
}
L_049c4:
{
  callr r2
  r1 = r16; 	r0 = memw(r17+#0xc)
}
L_049cc:
{
  r17 = memw(r17+#0x4)
}
L_049d0:
{
  p0 = cmp.eq(r17,#0x0); if (!p0.new) jump:t L_049b0
}
L_049d4:
{
  r17:16 = memd(r29+#0x10); 	dealloc_return
}
