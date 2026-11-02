// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			if (!p0) r3 = add(r2,#0)
			r2 = memw(r22+#-24)
	}
L_00008:
	{
			r2 = or(r2,r3)
			memw(r16+#32) = r2.new
	}
L_00010:
	{
			r3 = memw(r22+#-20)
	}
L_00014:
	{
			r2 |= asl(r3,#13)
			memw(r16+#32) = r2.new
	}
L_0001c:
	{
			r3 = memw(r22+#-16)
	}
L_00020:
	{
			r2 |= asl(r3,#16)
			memw(r16+#32) = r2.new
	}
L_00028:
	{
			r3 = memw(r22+#-28)
	}
L_0002c:
	{
			r2 |= asl(r3,#23)
			memw(r16+#32) = r2.new
	}
L_00034:
	{
			r3 = memw(r22+#-12)
	}
L_00038:
	{
			r3 = setbit(r2,#28)
			p0 = cmp.eq(r3,#0)
			if (!p0.new) memw(r16+#32) = r3.new
	}
L_00044:
	{
			r4 = memw(r22+#-8)
			if (cmp.eq(r4.new,#0)) jump:nt 0x14
	}
L_0004c:
	{
			if (!p0) r2 = add(r3,#0)
	}
L_00050:
	{
			r2 = setbit(r2,#29)
			memw(r16+#32) = r2.new
	}
L_00058:
	{
			call -0x34c
			r0 = memuh(r22+#-50)
	}
L_00060:
	{
			r3 = memw(r16+#12)
			memw(r16+#36) = r0
	}
L_00064:
	{
			r4 = memuh(r22+#-52)
			memw(r16+#48) = r4.new
	}
L_0006c:
	{
			r4 = memuh(r22+#-50)
			memw(r16+#52) = r4.new
	}
L_00074:
	{
			r4 = memw(r22+#-4)
			r2 = memw(r16+#28)
	}
L_0007c:
	{
			r3 = insert(r4,#4,#28)
			r2 = memw(r16+#32)
			memw(r16+#60) = r2
	}
L_00084:
	{
			memw(r16+#64) = r2
			memw(r16+#12) = r3
	}
L_0008c:
	{
			r3 = memub(r22+#-60)
	}
L_00090:
	{
			p0 = !tstbit(r3,#6)
			if (p0.new) jump:t 0x14
	}
L_00098:
	{
			r2 = setbit(r2,#5)
			memw(r16+#64) = r2.new
	}
L_000a0:
	{
			r3 = memub(r22+#-60)
	}
L_000a4:
	{
			r3 = sxtb(r3)
	}
L_000a8:
	{
			r3 = setbit(r2,#6)
			p0 = cmp.gt(r3,#-1)
			if (!p0.new) memw(r16+#64) = r3.new
	}
L_000b4:
	{
			if (p0) r3 = add(r2,#0)
			r4 = memw(r22+#-40)
			if (!cmp.eq(r4.new,#4)) jump:t 0x18
	}
L_000c0:
	{
			r3 = setbit(r3,#3)
			memw(r16+#64) = r3.new
	}
L_000c8:
	{
			r4 = memw(r22+#-40)
	}
L_000cc:
	{
			r2 = setbit(r3,#2)
			p0 = cmp.eq(r4,#3)
			if (p0.new) memw(r16+#64) = r2.new
	}
L_000d8:
	{
			call -0x3cc
			r0 = memuh(r22+#-46)
	}
L_000e0:
	{
			r2 = memw(r16+#52)
			memw(r16+#68) = r0
	}
L_000e8:
	{
			p0 = cmp.eq(r2,#0)
			r3 = memuh(r22+#-48)
			memw(r16+#80) = r3.new
	}
L_000f4:
	{
			if (p0) jump:nt 0x14
			r2 = memuh(r22+#-46)
			memw(r16+#84) = r2.new
	}
