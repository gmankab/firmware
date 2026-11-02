// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			p0 = cmp.eq(r0,#0)
			r5 = ##528400
			r2 = memw(gp+#1932)
	}
L_00010:
	{
			r0 = r2
			r6 = add(r2,#304)
	}
L_00018:
	{
			r3 = memw(r2+#296)
	}
L_0001c:
	{
			r4 = setbit(r3,#13)
			if (!p0) memw(r2+##296) = r4.new
	}
L_00028:
	{
			if (p0) r4 = add(r3,#0)
			r3 = add(r2,#300)
	}
L_00030:
	{
			r4 = or(r4,##1610645504)
			memw(r2+#296) = r4.new
	}
L_0003c:
	{
			memw(r3+#0) |= r5
	}
L_00040:
	{
			jump 0x29f3b8
			memw(r6+#0) = setbit(#2)
	}
L_00048:
	{
			r1 = #687
	}
L_0004c:
	{
			call -0x908
			r0 = ##-1339397370
	}
L_00058:
	{
			jump ##-0x534c
	}
L_00060:
	{
			r2 = r0
	}
L_00064:
	{
			r1:0 = combine(r2,#3)
			jump 0x2164c8
	}
L_0006c:
	{
			r2 = #291
			if (!cmp.gtu(r2.new,r1)) jump:nt 0x6c
	}
L_00074:
	{
			r3 = and(r1,#3)
			r2 = and(r1,#-4)
			p0 = cmp.gtu(r1,#127)
			if (p0.new) jump:t 0x1c
	}
L_00084:
	{
			r2 += add(r0,##160)
			jump 0x48
	}
L_00090:
	{
			r2 = add(r1,#-256)
			p0 = cmp.gtu(r1,#255)
			if (p0.new) jump:t 0x28
	}
L_0009c:
	{
			r2 = add(r1,#-128)
	}
L_000a0:
	{
			r2 = and(##536870908,lsr(r2,#3))
			r3 = and(r2,#31)
	}
L_000ac:
	{
			r2 += add(r0,##288)
			jump 0x20
	}
L_000b8:
	{
			r2 = and(##536870908,lsr(r2,#3))
			r3 = and(r2,#31)
	}
L_000c4:
	{
			r2 += add(r0,##304)
	}
L_000cc:
	{
			r3 = lsl(#1,r3)
	}
L_000d0:
	{
			jumpr r31
			memw(r2+#0) |= r3
	}
L_000d8:
	{
			r1 = #1938
			r0 = ##-1339397223
			allocframe(#0)
	}
L_000e4:
	{
			call -0x9a0
	}
L_000e8:
	{
			r0 = #1; jump ##0x253c
	}
L_000f0:
	{
			nop
	}
L_000f4:
	{
			nop
	}
L_000f8:
	{
			nop
	}
