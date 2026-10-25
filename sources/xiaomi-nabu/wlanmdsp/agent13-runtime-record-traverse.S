// SPDX-License-Identifier: GPL-3.0-only
// Agent13 independently reassembled Hexagon V66 record-chain unlink, callbacks and cleanup.
// File [0x43af4,0x43c30). Conditional store/load at runtime 0x3b1c
// deliberately uses :mem_noshuf to match original packet order.
// At 0x3b88 source scheduling packs r2 move before immext and duplex load.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_traverse
agent13_wlan_record_traverse:
L_03af4:
{
  call 0x2a1c84
  allocframe(#0x10)
}
L_03afc:
{
  r0 = #0x0; 	r16 = r0
}
L_03b00:
{
  call 0xf2bc
  r17 = memw(r16+#0xc)
}
L_03b08:
{
  r18 = add(r16,#0x3c)
  r1 = memw(r16+#0x3c)
  if (cmp.eq(r1.new,#0x0)) jump:nt L_03b34
}
L_03b14:
{
  r2 = memw(r1+#0x4)
  memw(r16+#0x3c) = r2.new
}
L_03b1c:
{
  call 0x21e528
  p0 = cmp.eq(r2,#0x0)
  if (p0.new) memw(r16+#0x40) = r18
  r0 = memw(r17+#0x8)
}:mem_noshuf
L_03b2c:
{
  r1 = memw(r16+#0x3c)
  if (!cmp.eq(r1.new,#0x0)) jump:t L_03b14
}
L_03b34:
{
  r3 = memw(r17+#0xc)
  if (cmp.eq(r3.new,r16)) jump:nt L_03b5c
}
L_03b3c:
{
  r2 = r3
  r3 = memw(r3+#0x38)
  if (!cmp.eq(r3.new,r16)) jump:t L_03b3c
}
L_03b48:
{
  r3 = memw(r16+#0x38)
  memw(r2+#0x38) = r3.new
}
L_03b50:
{
  p0 = cmp.eq(r3,#0x0); if (!p0.new) jump:t L_03b70
  jump L_03b6c
  r2 = add(r2,#0x38)
}
L_03b5c:
{
  r2 = memw(r16+#0x38)
  memw(r17+#0xc) = r2.new
}
L_03b64:
{
  if (p0.new) r2 = add(r17,#0xc)
  p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:t L_03b70
}
L_03b6c:
{
  memw(r17+#0x10) = r2
}
L_03b70:
{
  call 0x217dec
  r0 = add(r16,#0x18)
}
L_03b78:
{
  call 0x217de4
  r0 = add(r16,#0x58)
}
L_03b80:
{
  call 0x1a4c
  r0 = r16
}
L_03b88:
{
  r1 = ##0xb00035e0
  r2 = r16
  r0 = memw(r16+#0x8)
}
L_03b94:
{
  call 0x35234
}
L_03b98:
{
  call 0x2a1c54
  r1 = r16; 	r0 = memw(r17+#0x4)
}
L_03ba0:
{
  jump 0x21e4a4
}
L_03ba4:
{
  r16 = r0
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x10)
}
L_03bac:
{
  r1 = #0x4811
  immext(#0xc0)
  r0 = ##0xd8; 	r2 = memw(r16+#0x8)
}
L_03bb8:
{
  call 0x2175fc
  r2 = memub(r2+#0x3)
  memw(r29+#0x0) = r2.new
}
L_03bc4:
{
  immext(#0xb02a6340)
  r0 = ##0xb02a634c; 	r2 = memw(r16+#0x8)
}
L_03bcc:
{
  call -0xf84
  r1 = memub(r2+#0x3)
}
L_03bd4:
{
  if (!p0.new) r1 = #0x10c
  p0 = cmp.eq(r16,#0x0); if (p0.new) jump:nt L_03bec
  r2 = memw(r16+#0x10)
}
L_03be0:
{
  if (!p0.new) jump:t L_03bf4
  p0 = tstbit(r2,#0x1d)
  jump L_03c14
}
L_03bec:
{
  immext(#0x0)
  r2 = memw(##0x10)
}
L_03bf4:
{
  p0 = !tstbit(r2,#0x1c)
  r1 = #0x10d
  if (p0.new) r0 = add(r16,#0x18)
  if (!p0.new) jump:nt L_03c14
}
L_03c04:
{
  call 0x217d58
}
L_03c08:
{
  r0 = r16
  r17:16 = memd(r29+#0x8); 	deallocframe
}
L_03c10:
{
  jump 0x14c4
}
L_03c14:
{
  call L_03c20
  immext(#0xb02a6340)
  r0 = ##-0x4fd59c91
}
L_03c20:
{
  r3:2 = combine(r0,r1)
}
L_03c24:
{
  immext(#0xb02a6340)
  r1:0 = combine(r3,##-0x4fd59ca8)
  jump 0x2193a0
}
