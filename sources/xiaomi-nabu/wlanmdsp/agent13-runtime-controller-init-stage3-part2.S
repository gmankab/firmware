// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r2 = memub(r22+#-32)
	}
L_00004:
	{
			p0 = !tstbit(r2,#4)
			if (p0.new) jump:t 0xb0
			if (p0.new) memw(r16+#32) = #0
	}
L_00010:
	{
			r2 = memw(gp+#1932)
	}
L_00014:
	{
			memw(r2+#168) = setbit(#0)
	}
L_00018:
	{
			r2 = memub(r22+#-60)
	}
L_0001c:
	{
			p0 = tstbit(r2,#0); if (!p0.new) jump:t 0xc
	}
L_00020:
	{
			memw(r16+#32) = setbit(#11)
	}
L_00024:
	{
			r2 = memub(r22+#-60)
	}
L_00028:
	{
			p0 = !tstbit(r2,#1)
			if (p0.new) jump:t 0x10
	}
L_00030:
	{
			memw(r16+#32) = setbit(#12)
	}
L_00034:
	{
			r2 = memub(r22+#-60)
	}
L_00038:
	{
			p0 = !tstbit(r2,#2)
			if (p0.new) jump:t 0x18
			r3 = memw(r16+#32)
	}
L_00044:
	{
			r3 = setbit(r3,#19)
			memw(r16+#32) = r3.new
	}
L_0004c:
	{
			r2 = memub(r22+#-60)
	}
L_00050:
	{
			p0 = !tstbit(r2,#3)
			r2 = setbit(r3,#20)
			if (!p0.new) memw(r16+#32) = r2.new
	}
