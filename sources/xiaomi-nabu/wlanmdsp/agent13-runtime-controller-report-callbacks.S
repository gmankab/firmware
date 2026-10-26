// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			call -0x3738
			r1:0 = combine(r17,##-1339396848)
	}
L_0000c:
	{
			p0 = cmp.eq(r17,#0); if (p0.new) jump:nt 0xc
			r2 = memw(gp+#1932)
	}
L_00014:
	{
			memw(r2+#200) = setbit(#3)
	}
L_00018:
	{
			r2 = add(r2,#272)
	}
L_0001c:
	{
			call 0x1f93e4
			memw(r2+#0) = setbit(#0)
	}
L_00024:
	{
			p0 = cmp.eq(r0,#0); if (p0.new) jump:nt 0x10
	}
L_00028:
	{
			r2 = memw(gp+#1932)
	}
L_0002c:
	{
			r2 = add(r2,#272)
	}
L_00030:
	{
			memw(r2+#0) = setbit(#2)
	}
L_00034:
	{
			call 0x1f93d0
	}
L_00038:
	{
			p0 = cmp.eq(r0,#0); if (p0.new) jump:nt 0x10
			r3 = memw(gp+#1932)
	}
