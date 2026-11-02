// SPDX-License-Identifier: GPL-3.0-only
// Agent13 independently reassembled WLAN session chain iteration and
// predicate-driven resource cleanup. File [0x43c8c,0x43e04).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_session_walk
agent13_wlan_session_walk:
L_03c8c:
{
  call 0x2a1ac8
  allocframe(#0x28)
}
L_03c94:
{
  r1 = #0x4811
  immext(#0xc0)
  r0 = ##0xe8; 	r16 = r0
}
L_03ca0:
{
  r18 = memw(r16+#0x28); 	r2 = memub(r16+#0x0)
}
L_03ca4:
{
  call 0x217510
  memw(r29+#0x0) = r2
}
L_03cac:
{
  call -0x1064
  immext(#0xb02a6400)
  r0 = ##0xb02a6400; 	r1 = memub(r16+#0x0)
}
L_03cb8:
{
  r20 = #0x0
  r19 = add(r18,#0x54)
  r2 = memw(r18+#0x140)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03dd4
}
L_03cc8:
{
  p1 = and(p1,!p1)
  r23:22 = combine(#0x0,#0x0)
}
L_03cd0:
{
  r17 = memw(r19+#0x0)
  if (cmp.eq(r17.new,#0x0)) jump:nt L_03dc8
}
L_03cd8:
{
  r3 = memb(r17+#0x250)
}
L_03cdc:
{
  r4 = memb(r16+#0x0)
  if (!cmp.eq(r4.new,r3)) jump:t L_03dc8
}
L_03ce4:
{
  r3 = memuh(r17+#0x1c)
}
L_03ce8:
{
  p0 = tstbit(r3,#0); if (!p0.new) jump:t L_03dc8
}
L_03cec:
{
  r3 = memb(r17+#0x0)
  if (!cmp.eq(r3.new,#0x1)) jump:t L_03dc8
}
L_03cf4:
{
  r3 = memw(r17+#0x34)
}
L_03cf8:
{
  r1:0 = memd(r3+#0x0)
}
L_03cfc:
{
  p0 = cmp.eq(r1:0,r23:22)
  if (!p0.new) jump:t L_03dc8
  if (p0.new) r2 = memub(r3+#0x13)
}
L_03d08:
{
  p0 = tstbit(r2,#0x5)
  if (!p0.new) r0 = add(r17,#0x0)
  if (p0.new) jump:t L_03d34
}
L_03d14:
{
  call 0x177c
}
L_03d18:
{
  p1 = and(p1,!p1)
  p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt L_03d2c
}
L_03d20:
{
  call -0x17c
  r0 = memw(r17+#0x34)
}
L_03d28:
{
  jump L_03d64
}
L_03d2c:
{
  r3 = memw(r17+#0x34)
  if (cmp.eq(r3.new,#0x0)) jump:nt L_03d7c
}
L_03d34:
{
  r2 = memub(r3+#0x13)
}
L_03d38:
{
  p0 = !tstbit(r2,#0x5)
  if (!p0.new) r0 = add(r17,#0x0)
  if (p0.new) jump:t L_03d68
}
L_03d44:
{
  call 0x174c
}
L_03d48:
{
  p1 = and(p1,!p1)
  p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:t L_03d68
}
L_03d50:
{
  call 0x20bc54
  r0 = memw(r16+#0x24)
}
L_03d58:
{
  r2 = r0; 	r0 = memw(r17+#0x34)
}
L_03d5c:
{
  call 0x15e4
  r1 = r2
}
L_03d64:
{
  p1 = and(p1,!p1)
}
L_03d68:
{
  r3 = memw(r17+#0x34); 	r2 = memub(r17+#0x3)
}
L_03d6c:
{
  p0 = cmp.eq(r3,#0x0); if (p0.new) jump:nt L_03d80
  if (!p0.new) r3 = memub(r3+#0x13)
}
L_03d74:
{
  p0 = tstbit(r3,#0x5)
  jump L_03d84
}
L_03d7c:
{
  r2 = memub(r17+#0x3)
}
L_03d80:
{
  p0 = or(p1,p1)
}
L_03d84:
{
  r1 = #0x4852
  immext(#0xc0)
  r0 = ##0xf0; 	memw(r29+#0x0) = r2
}
L_03d90:
{
  call 0x217424
  r3 = mux(p0,#0x1,#0x0)
  memw(r29+#0x4) = r3.new
}
L_03d9c:
{
  p0 = and(p0,!p0)
  r2 = memw(r17+#0x34); 	r1 = memub(r17+#0x3)
}
L_03da4:
{
  p1 = cmp.eq(r2,#0x0); if (p1.new) jump:nt L_03db0
  if (!p1.new) r2 = memub(r2+#0x13)
}
L_03dac:
{
  p0 = tstbit(r2,#0x5)
}
L_03db0:
{
  call -0x1a90
  immext(#0xb02a6400)
  r0 = ##-0x4fd59bf4
  r2 = mux(p0,#0x1,#0x0)
}
L_03dc0:
{
  p1 = and(p1,!p1)
  r2 = memw(r18+#0x140)
}
L_03dc8:
{
  r19 = add(r19,#0x4)
  r20 = add(r20,#0x1)
  if (cmp.gtu(r2,r20.new)) jump:t L_03cd0
}
L_03dd4:
{
  jump 0x2a1a5c
}
L_03dd8:
{
  r0 = #0x0; 	r2 = r0
}
L_03ddc:
{
  r3 = memuh(r2+#0x1c)
}
L_03de0:
{
  p0 = tstbit(r3,#0x0)
  if (!p0.new) jumpr:nt r31
}
L_03de8:
{
  r3 = memb(r2+#0x0)
  if (!cmp.eq(r3.new,#0x1)) jump:t L_03e00
}
L_03df0:
{
  r5:4 = combine(#0,#0x0); 	r2 = memw(r2+#0x34)
}
L_03df4:
{
  r3:2 = memd(r2+#0x0)
}
L_03df8:
{
  p0 = cmp.eq(r3:2,r5:4)
}
L_03dfc:
{
  r0 = mux(p0,#0x1,#0x0)
}
L_03e00:
{
  jumpr r31
}
