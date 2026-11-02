// SPDX-License-Identifier: GPL-3.0-only
// Agent13 Hexagon V66 helper wrappers and eight-slot bit counter.
// File [0x43c30,0x43c8c); the r1/r2 source order at 0x3c58
// selects the original duplex packet encoding without opaque opcode words.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_popcount
agent13_wlan_record_popcount:
L_03c30:
{
  r16 = r0
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x10)
}
L_03c38:
{
  r1 = #0x4851
  immext(#0xc0)
  r0 = ##0xe0; 	r2 = memub(r16+#0x3)
}
L_03c44:
{
  call 0x217570
  memw(r29+#0x0) = r2
}
L_03c4c:
{
  call -0x1004
  immext(#0xb02a63c0)
  r0 = ##0xb02a63f4; 	r1 = memub(r16+#0x3)
}
L_03c58:
{
  r0 = add(r16,#0xd40)
  r1 = #0x0; r17:16 = memd(r29+#0x8)
  r2 = #0x28
}
L_03c64:
{
  jump -0x1cbc
  deallocframe
}
L_03c6c:
{
  loop0(L_03c78,#0x8)
  r3:2 = combine(#0x0,#0x0)
}
L_03c74:
{
  r5 = r3
}
L_03c78:
{
  r4 = memw(r0++#0x4)
}
L_03c7c:
{
  r4 = popcount(r5:4)
}
L_03c80:
{
  r2 = add(r4,r2)
  nop
}:endloop0
L_03c88:
{
  r0 = r2; 	jumpr r31
}
