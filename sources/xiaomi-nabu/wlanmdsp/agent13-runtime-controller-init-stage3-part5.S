// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r22 = add(r22,#80)
			r16 = add(r16,#240)
			r2 = memub(r23+#2)
			if (!cmp.gtu(r2.new,r27)) jump:nt 0x2c
	}
L_00010:
	{
			r27 = add(r27,#1)
			r2 = memw(r26+#4)
	}
L_00018:
	{
			jump -0x3f0
	}
L_0001c:
	{
			p0 = cmp.eq(r3,#4); if (p0.new) jump:t -0x330
	}
L_00020:
	{
			r2 = cmp.eq(r3,#1)
			jump -0x330
			memw(r16+#24) = r2.new
	}
L_0002c:
	{
			r24 = add(r24,#1)
			r2 = #6
			r17 = add(r17,#168)
	}
L_00038:
	{
			p0 = cmp.gtu(r2,r24)
			if (!p0.new) r3 = add(r23,##972)
			if (p0.new) jump:t -0x45c
	}
