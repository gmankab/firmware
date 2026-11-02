// SPDX-License-Identifier: GPL-3.0-only
// Query indexed WLAN record table and route format-specific diagnostics.
// File [0x43a18,0x43af4), rebuilt Hexagon V66 packet semantics.
.text
.globl agent13_wlan_record_query
agent13_wlan_record_query:
// Safely classify the record before calling the registered query callback.
{
  call 0x2a1d60
  allocframe(#0x20)
}
{
  r16 = r2
  r18 = r1
  if (cmp.eq(r18.new,#0x0)) jump:nt L_03af0
}
{
  p0 = cmp.eq(r16,#0x0); if (p0.new) jump:nt L_03af0
}
{
  r2 = memw(r16+#0x84)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03af0
}
{
  r2 = memw(r16+#0x7c)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03af0
}
{
  r17 = add(r16,#0x58)
  r2 = memw(r16+#0x8)
}
{
  r0 = r17
  r2 = memw(r2+#0x2c)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03ac8
}
{
  r1 = #0x4853
  immext(#0x140)
  r0 = ##0x158; 	r19 = memw(r18+#0x8)
}
{
  r2 = memw(r18+#0x0)
  r3 = memw(r16+#0x78)
}
{
  memw(r29+#0x8) = r3; 	memw(r29+#0x4) = r19
}
{
  call 0x217748
  memw(r29+#0x0) = r2
}
{
  immext(#0xb02a6480)
  r0 = ##-0x4fd59b58
  r1 = memw(r18+#0x0)
  r3 = memw(r16+#0x78)
}
{
  call -0x1570
  r2 = r19
}
{
  r2 = memw(r18+#0x0)
}
{
  r3 = #0x400
  if (cmp.eq(r3.new,r2)) jump:nt L_03ad0
}
{
  immext(#0x200)
  p0 = cmp.eq(r2,##0x200)
  if (!p0.new) jump:nt L_03af0
}
{
  r2 = setbit(r19,#0x1)
  if (!cmp.eq(r2.new,#0x3)) jump:nt L_03af0
}
{
  r2 = memw(r16+#0x8)
}
{
  call 0xb08
  r0 = memw(r2+#0x2c)
}
{
  call 0x2a1d34
  r0 = r17
  memw(r16+#0x78) = #0x1
}
{
  jump 0x217e98
}
L_03ac8:
{
  call 0x2a1d24
}
{
  jump 0x217e90
}
L_03ad0:
{
  p0 = cmp.eq(r19,#0x2); if (!p0.new) jump:t L_03af0
  if (p0.new) r2 = memw(r16+#0x8)
}
{
  call 0xad8
  r0 = memw(r2+#0x2c)
}
{
  call 0x2a1d0c
  r0 = r16
  memw(r16+#0x78) = #0x0
}
{
  jump 0x11e8
}
L_03af0:
{
  jump 0x2a1d64
}
