// SPDX-License-Identifier: GPL-3.0-only
// Publish record statistics and optional control callbacks. File [0x44b7c,0x44cb4).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_statistics_publish
agent13_wlan_record_statistics_publish:
L_04b7c:
{
  call 0x2a0bd8
  allocframe(#0x48)
}
L_04b84:
{
  r16 = r0
}
L_04b88:
{
  r2 = memw(r16+#0x84)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_04bc4
}
L_04b90:
{
  r2 = memw(r16+#0x78)
  if (!cmp.eq(r2.new,#0x1)) jump:t L_04bc4
}
L_04b98:
{
  r2 = memw(r16+#0x7c)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_04bc8
}
L_04ba0:
{
  call 0x216dbc
  r0 = add(r16,#0x58)
}
L_04ba8:
{
  r2 = memw(r16+#0x8)
}
L_04bac:
{
  call -0x5fc
  r0 = memw(r2+#0x2c)
}
L_04bb4:
{
  call 0x2a0c0c
  r0 = r16
  memw(r16+#0x78) = #0x0
}
L_04bc0:
{
  jump 0x114
}
L_04bc4:
{
  jump 0x2a0c6c
}
L_04bc8:
{
  call 0x204
  r0 = r16
}
L_04bd0:
{
  r17 = r0
  if (cmp.gtu(r17.new,#0x9)) jump:t L_04bec
}
L_04bd8:
{
  r2 = memw(r16+#0x8)
}
L_04bdc:
{
  call 0x1fd448
  r0 = memw(r2+#0x2c)
}
L_04be4:
{
  r2 = addasl(r16,r17,#0x3)
}
L_04be8:
{
  memd(r2+#0x90) = r1:0
}
L_04bec:
{
  r2 = add(r16,#0x80)
}
L_04bf0:
{
  memb(r2+#0x0) += #0x1
}
L_04bf4:
{
  r2 = memw(r16+#0x8)
}
L_04bf8:
{
  call -0x648
  r0 = memw(r2+#0x2c)
}
L_04c00:
{
  r20 = memub(r16+#0x80)
  memw(r16+#0x78) = #0x0
}
L_04c08:
{
  r21 = memub(r16+#0x88)
  if (cmp.gtu(r21.new,r20)) jump:t L_04c24
}
L_04c10:
{
  call 0xc4
  r0 = r16
  memw(r16+#0x7c) = #0x1
}
L_04c1c:
{
  r20 = memub(r16+#0x80)
  r21 = memub(r16+#0x88)
}
L_04c24:
{
  r2 = memw(r16+#0x8); 	r3 = memw(r16+#0x10)
}
L_04c28:
{
  r22 = extractu(r3,#0x9,#0x10)
  r23 = memw(r16+#0x7c)
  r0 = memw(r2+#0x2c)
}
L_04c34:
{
  call 0x1fd3f0
}
L_04c38:
{
  r19:18 = combine(r1,r0)
  r2 = memw(r16+#0x8)
}
L_04c40:
{
  call 0x1fd3e4
  r0 = memw(r2+#0x2c)
}
L_04c48:
{
  r1 = #0x4837
  immext(#0x140)
  r0 = ##0x160; 	r3:2 = combine(#0,r1)
}
L_04c54:
{
  memd(r29+#0x18) = r19:18; 	memd(r29+#0x20) = r3:2
}
L_04c58:
{
  memw(r29+#0x10) = r17; 	memw(r29+#0xc) = r23
}
L_04c5c:
{
  memw(r29+#0x8) = r21; 	memw(r29+#0x4) = r22
}
L_04c60:
{
  call 0x216554
  memw(r29+#0x0) = r20
}
L_04c68:
{
  r2 = memw(r16+#0x8); 	r3 = memw(r16+#0x10)
}
L_04c6c:
{
  r21 = extractu(r3,#0x9,#0x10)
  r20 = memub(r16+#0x80)
  r22 = memub(r16+#0x88)
}
L_04c78:
{
  call 0x1fd3ac
  r0 = memw(r2+#0x2c)
  r23 = memw(r16+#0x7c)
}
L_04c84:
{
  r19:18 = combine(r1,r0)
  r2 = memw(r16+#0x8)
}
L_04c8c:
{
  call 0x1fd398
  r0 = memw(r2+#0x2c)
}
L_04c94:
{
  immext(#0xb02a6480)
  r1:0 = combine(#0x7,##0xb02a64b4)
  memw(r29+#0x18) = r1; 	memw(r29+#0x14) = r18
}
L_04ca0:
{
  memw(r29+#0x10) = r17; 	memw(r29+#0xc) = r23
}
L_04ca4:
{
  memw(r29+#0x8) = r22; 	memw(r29+#0x4) = r21
}
L_04ca8:
{
  call -0x2998
  memw(r29+#0x0) = r20
}
L_04cb0:
{
  jump 0x2a0b80
}
