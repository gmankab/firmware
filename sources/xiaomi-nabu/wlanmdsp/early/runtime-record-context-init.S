// SPDX-License-Identifier: GPL-3.0-only
// Allocate record-state metadata, check live status, initialize byte fields.
// File [0x434b4,0x435e0), exact Hexagon V66 packet encoding.
// Independently assembled instructions, no extracted opcode arrays.
.text
.globl agent13_wlan_record_context_init
agent13_wlan_record_context_init:
{
  call 0x2a22b4
  allocframe(#0x18)
}
{
  r18 = r0; 	r17 = r1
}
{
  call 0x21ec6c
  r0 = memw(r18+#0x4)
}
{
  r16 = r0
  if (cmp.eq(r16.new,#0x0)) jump:nt L_035d8
}
{
  call 0x35004
  immext(#0xb00035c0)
  r1:0 = combine(##-0x4fffca20,r17)
  r2 = r16
}
{
  p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt L_034f4
}
{
  call 0x21eb60
  r1 = r16
  r16 = #0x0
  r0 = memw(r18+#0x4)
}
{
  jump L_035d8
}
L_034f4:
{
  r1:0 = combine(#0x10,#0x0)
  r4 = #0x2d0f
  r2 = r16; 	r3 = memw(r16+#0x10)
}
{
  r0 = add(r16,#0x18)
  immext(#0xb00037c0)
  r1 = ##-0x4fffc818
  memd(r16+#0x0) = r1:0
}
{
  r3 = insert(r4,#0x1d,#0x0)
  r21:20 = combine(#0x0,#0x0)
  memh(r16+#0x44) = r4
  memw(r16+#0x8) = r17
}
{
  memw(r16+#0x10) = r3; 	memw(r16+#0xc) = r18
}
{
  call 0x2181d8
}
{
  r2 = add(r16,#0x3c)
  r0 = add(r16,#0x38)
  memw(r16+#0x40) = r2.new
}
{
  r4 = #0x200
  memd(r16+#0x38) = r21:20
  r2 = memw(r18+#0x10)
}:mem_noshuf // Preserve ordered 64-bit store before r2 load.
{
  r1:0 = combine(#0x0,#0x0)
  memw(r2+#0x0) = r16; 	memw(r18+#0x10) = r0
}
{
  immext(#0xb00037c0)
  r2 = ##-0x4fffc810
  r3 = memw(gp+#0x78c)
  memw(r16+#0x48) = #0x0
}
{
  r18 = #0x0
  memh(r16+#0x4c) = #0x0
}
{
  call 0xf584
  r3 = memw(r3+#0x98)
}
{
  call 0x2020
  r0 = r16
}
{
  immext(#0xc0)
  r3:2 = combine(##0xff,#0x53)
  r0 = add(r16,#0x90)
  r1 = #0x0
}
{
  memb(r16+#0x80) = r18
  memd(r16+#0x78) = r21:20
}
{
  r18 = add(r16,#0x58)
  memw(r16+#0x84) = #0x0
  memb(r16+#0x88) = r3
}
{
  call -0x15ec
  memw(r16+#0xe4) = #0x0
  memw(r16+#0xe8) = #0x0
}
{
  call 0x21815c
  immext(#0xb0003980)
  r1:0 = combine(##-0x4fffc67c,r18)
  r2 = r16
}
{
  immext(#0xb0003a00)
  r1 = ##-0x4fffc5e8
  r2 = memw(r17+#0x28)
  r3 = memub(r17+#0x250)
}
{
  r2 = addasl(r2,r3,#0x2)
}
{
  r2 = memw(r2+#0x328)
}
{
  call 0x1fa30c
  immext(#0x600)
  r3:2 = combine(##0x600,r16)
  r0 = memw(r2+#0x24)
}
L_035d8:
{
  // Far tail jump packet 0x35d8 -> 0xb02a5844.
  r0 = r16; jump ##0x2a226c
}
