// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r0 = asl(r2,#2)
			r16 = #0
			memw(r23+#888) = r2
	}
L_0000c:
	{
			call 0x2dc
			memw(r23+#884) = r16
	}
L_00014:
	{
			r2 = #0 ; jump 0x1c
			r21 = r16
			memw(r23+#892) = r0
	}
L_00020:
	{
			r2 = or(r2,r19)
			r3 = memw(r23+#892)
	}
L_00028:
	{
			r2 = add(r17,#1)
			memw(r3+r17<<#2) = r2
	}
L_00030:
	{
			r2 = #4
			r17 = r2
	}
L_00034:
	{
			p0 = cmp.eq(r17,#3); if (p0.new) jump:t -0x4
	}
L_00038:
	{
			r2 = memw(gp+#1932)
	}
L_0003c:
	{
			r3 = memb(r2+#2)
			if (cmp.eq(r3.new,#0)) jump:nt 0xe0
	}
