// SPDX-License-Identifier: GPL-3.0-only
// Allocate and register a WLAN record context and initial command blocks.
// File [0x43410,0x434a8), exact Hexagon V66 packet encoding.
// Independently assembled instructions, no extracted opcode arrays.
.text
.globl agent13_wlan_record_create
agent13_wlan_record_create:
// Active record metadata, pointer setup, and external factory callbacks.
{
  call 0x2a2368
  allocframe(#0x10)
}
{
  r16 = #0x0
  r18 = r0
  r17 = r1
  if (cmp.eq(r17.new,#0x0)) jump:nt L_034a0
}
{
  call 0x80
  r0 = #0x18
}
{
  r16 = r0
  immext(#0xc0)
  r1:0 = combine(r17,##0xf0)
}
{
  call 0x21e988
  immext(#0x100)
  r3:2 = combine(##0x100,#0x9)
  memw(r16+#0x0) = r18
}
{
  r1 = asl(r17,#0x3)
  r0 = #0x10; 	memw(r16+#0x4) = r0
}
{
  call 0x21e970
  immext(#0x100)
  r3:2 = combine(##0x100,#0x9)
}
{
  r2 = add(r16,#0xc)
  r0 = #0x8; 	memw(r16+#0x8) = r0
}
{
  call 0x40
  memw(r16+#0x10) = r2; 	memw(r16+#0xc) = #0
}
{
  call 0x38
  r17 = r0
  r0 = #0x800
  memw(r16+#0x14) = r17.new
}
{
  // Preserve the original duplex grouping: r2 is extended, r0 pairs with store.
  r0 = r16
  r2 = ##0x400000
  memw(r17+#0x0) = r0
}
{
  call 0x2184
  memw(r17+#0x4) = r2
}
{
  call 0x20ab10
  immext(#0xb0002f40)
  r0 = ##-0x4fffd08c
}
L_034a0:
{
  // Extended packet-relative jump to 0xb02a5854.
  r0 = r16; jump ##0x2a23b4
}
