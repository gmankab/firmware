// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r4 = memw(r16+#48)
			if (!cmp.eq(r4.new,#0)) jump:t 0x10
	}
L_00008:
	{
			memw(r16+#36) = #0
			memw(r16+#32) = #0
	}
L_0000c:
	{
			memw(r16+#28) = #0
			memw(r16+#24) = #0
	}
L_00010:
	{
			p0 = cmp.eq(r3,#0); if (p0.new) jump:nt 0x8
	}
L_00014:
	{
			p0 = cmp.eq(r2,#0); if (!p0.new) jump:t 0x10
	}
L_00018:
	{
			memw(r16+#68) = #0
			memw(r16+#64) = #0
	}
L_00020:
	{
			memw(r16+#60) = #0
			memw(r16+#56) = #0
	}
L_00024:
	{
			r2 = memw(r16+#16)
	}
L_00028:
	{
			p0 = tstbit(r2,#0); if (!p0.new) jump:t 0x10
	}
L_0002c:
	{
			r2 = memw(gp+#1932)
	}
L_00030:
	{
			memb(r2+#0) = setbit(#1)
	}
L_00034:
	{
			r2 = memw(r16+#16)
	}
L_00038:
	{
			p0 = !tstbit(r2,#1)
			if (p0.new) jump:t 0x14
			if (!p0.new) r2 = memw(##2956330892)
	}
L_00048:
	{
			memb(r2+#0) = setbit(#0)
	}
L_0004c:
	{
			r18 = memw(r16+#52)
			r2 = memw(gp+#1932)
	}
L_00054:
	{
			call 0x2166a8
			r1:0 = combine(r19,r18)
			r18 = memub(r2+#943)
	}
L_00060:
	{
			p0 = !cmp.gtu(r18,r0)
			if (p0.new) jump:t 0x18
			if (!p0.new) r2 = memw(##2956330892)
	}
L_00070:
	{
			jump 0x14
			r0 = memub(r2+#943)
	}
L_00078:
	{
			call 0x216684
			r1 = r19
			r0 = memw(r16+#52)
	}
L_00080:
	{
			r2 = memw(gp+#1932)
	}
L_00084:
	{
			r1 = r19
			r18 = memub(r2+#944)
			memb(r2+#943) = r0
	}
L_00090:
	{
			call 0x21666c
			r0 = memw(r16+#84)
	}
L_00098:
	{
			p0 = !cmp.gtu(r18,r0)
			if (p0.new) jump:t 0x18
			if (p0.new) r0 = memw(r16+#84)
	}
L_000a4:
	{
			r23 = memw(gp+#1932)
	}
L_000a8:
	{
			jump 0x14
			r0 = memub(r23+#944)
	}
L_000b0:
	{
			call 0x21664c
			r1 = r19
	}
L_000b8:
	{
			r23 = memw(gp+#1932)
	}
L_000bc:
	{
			r2 = add(r23,#956)
			r3 = add(r23,#960)
			r0 = add(r23,#964)
			memb(r23+#944) = r0
	}
L_000cc:
	{
			r4 = memw(r16+#28)
			r5 = memw(r16+#60)
	}
L_000d0:
	{
			r4 = or(r5,r4)
	}
L_000d4:
	{
			memw(r2+#0) |= r4
	}
L_000d8:
	{
			r2 = memw(r16+#32)
			r4 = memw(r16+#64)
	}
L_000e0:
	{
			r2 = or(r4,r2)
	}
L_000e4:
	{
			memw(r3+#0) |= r2
	}
L_000e8:
	{
			r2 = memw(r16+#36)
			r3 = memw(r16+#68)
	}
L_000f0:
	{
			r2 = or(r3,r2)
	}
L_000f4:
	{
			memw(r0+#0) |= r2
	}
L_000f8:
	{
			r2 = memw(r22+#0)
			memw(r16+#192) = r2.new
	}
