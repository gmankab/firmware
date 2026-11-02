// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r2 = add(r3,#284)
	}
L_00004:
	{
			memw(r2+#0) = setbit(#0)
	}
L_00008:
	{
			r0 = add(r3,#760)
			r1 = ##-1342151444
			r2 = #0
	}
L_00014:
	{
			call 0x215328
			memw(r3+#564) = r18
	}
L_0001c:
	{
			r2 = memw(gp+#1932)
			r5:4 = memd(gp+#3296)
	}
L_00024:
	{
			r3 = add(r2,#284)
			r0 = add(r2,#288)
	}
L_0002c:
	{
			memw(r3+#0) = setbit(#2)
	}
L_00030:
	{
			r2 = add(r2,#292)
			memb(r2+#28) = #0
	}
L_00038:
	{
			memw(r0+#0) |= r4
	}
L_0003c:
	{
			call 0x1f9418
			memw(r2+#0) |= r5
	}
