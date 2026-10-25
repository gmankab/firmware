// SPDX-License-Identifier: GPL-3.0-only
// Count outstanding WLAN record conditions with checked callback exit.
// File [0x43984,0x43a18), rebuilt Hexagon V66 packet semantics.
.text
.globl agent13_wlan_record_counter
agent13_wlan_record_counter:
// Normalize record totals while preserving conditional callback execution.
{
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x10)
}
{
  r16 = r1
  if (cmp.eq(r16.new,#0x0)) jump:nt L_03a14
}
{
  r2 = memw(r16+#0x84)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03a14
}
{
  r2 = memw(r16+#0x7c)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03a14
}
{
  r2 = memw(r16+#0x8)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03a14
}
{
  r1 = #0x4821
  immext(#0x140)
  r0 = ##0x170; 	r17 = memw(r2+#0x2c)
}
{
  call 0x217800
  r2 = memw(r16+#0x78)
  memw(r29+#0x0) = r2.new
}
{
  call -0xd78
  immext(#0xb02a64c0)
  r0 = ##-0x4fd59b34
  r1 = memw(r16+#0x78)
}
{
  r0 = r17
  r2 = memw(r16+#0x78)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_039ec
}
{
  call 0xbd4
}
{
  call 0x12f4
  r0 = r16
}
{
  r2 = #0x0 ; jump L_03a10
}
L_039ec:
{
  call 0xbcc
  r17 = add(r16,#0x58)
}
{
  call 0x217f68
  r0 = r17
}
{
  call 0x217ee4
  r0 = r17
  r4 = #0x0
  r3:2 = memd(gp+#0xcd0)
}
{
  r2 = #0x1
}
L_03a10:
{
  memw(r16+#0x78) = r2
}
L_03a14:
{
  r17:16 = memd(r29+#0x8); 	dealloc_return
}
