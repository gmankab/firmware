// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r0 = memw(r2+#0)
			r26 = memw(r3+#0)
	}
L_00008:
	{
			call 0x2164e0
			r20 = memw(r4+#0)
	}
L_00010:
	{
			call 0x2164d8
			r18 = r0
			r1:0 = combine(r21,r20)
	}
L_0001c:
	{
			p0 = cmp.gtu(r26,#6)
			if (p0.new) jump:t 0x10
	}
L_00024:
	{
			r26 = memw(r26<<#2+##2955571184)
	}
L_0002c:
	{
			p0 = cmp.eq(r25,#1)
			if (p0.new) r2 = #0
			if (p0.new) jump:nt 0x2c
	}
L_00038:
	{
			r2 = asl(r18,#28)
			p0 = cmp.eq(r25,#0)
			if (!p0.new) jump:nt 0x34
	}
L_00044:
	{
			r19 = or(r2,and(r19,##16715775))
	}
L_0004c:
	{
			r19 = insert(r0,#4,#24)
	}
L_00050:
	{
			r19 = insert(r26,#4,#12)
			jump 0x1c
	}
L_00058:
	{
			r2 = insert(r18,#4,#20)
	}
L_0005c:
	{
			r19 = or(r2,and(r19,##-16715521))
	}
L_00064:
	{
			r19 = insert(r0,#4,#16)
	}
L_00068:
	{
			r19 = insert(r26,#4,#8)
	}
L_0006c:
	{
			r25 = add(r25,#1)
			r24 = add(r24,#240)
			r2 = memw(gp+#1932)
	}
L_00078:
	{
			r2 = memub(r2+#2)
			if (cmp.gtu(r2.new,r25)) jump:t -0xa8
	}
L_00080:
	{
			jump 0x8
	}
L_00084:
	{
			r19 = #0
	}
L_00088:
	{
			r2 = #64
			p0 = cmp.eq(r17,#2); if (p0.new) jump:nt -0x100
	}
L_00090:
	{
			r2 = #128
			p0 = cmp.eq(r17,#1); if (p0.new) jump:t -0x108
	}
L_00098:
	{
			r2 = add(r17,#1)
			r3 = memw(r23+#892)
	}
L_000a0:
	{
			p0 = cmp.eq(r2,#6)
			if (p0.new) r2 = ##-1207959552
			memw(r3+r17<<#2) = r19
	}
L_000b0:
	{
			if (!p0) jump:nt -0x118
			if (p0) r16 = memw(##2956330892)
	}
L_000bc:
	{
			r0 = add(r16,#4168)
			r3 = memw(r16+#1460)
			r4 = memw(r16+#1948)
	}
L_000c8:
	{
			p1 = cmp.eq(r4,#0)
			p0 = cmp.eq(r3,#0)
			if (p0.new) r4 = #0
	}
L_000d0:
	{
			if (!p0) r4 = ##-1744830464
			r3 = mux(p0,#0,r2)
			memw(r23+#900) = r3.new
	}
L_000e0:
	{
			r2 = or(r4,##1610612736)
	}
L_000e8:
	{
			if (!p1) r4 = add(r2,#0)
	}
