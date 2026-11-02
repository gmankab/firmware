// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r22 = +mpyi(r17,##488)
			r19 = #0
			r25 = #0
	}
L_00010:
	{
			r22 += add(r2,##968)
			r24 = r22
	}
L_0001c:
	{
			r24 += add(r2,##1064)
	}
L_00024:
	{
			r2 = memw(r22+#4)
	}
L_00028:
	{
			p0 = !tstbit(r2,r25)
			if (!p0.new) r1 = add(r16,#0)
			if (p0.new) jump:nt 0x98
	}
L_00034:
	{
			r2 = memw(r24+#-68)
	}
L_00038:
	{
			p0 = !tstbit(r2,#1)
			if (p0.new) r2 = add(r24,#-36)
			if (p0.new) r3 = add(r24,#-60)
			if (p0.new) r4 = add(r24,#-32)
	}
L_00048:
	{
			if (!p0) r2 = add(r24,#-4)
			if (!p0) r3 = add(r24,#-28)
			if (!p0) r4 = add(r24,#0)
	}
