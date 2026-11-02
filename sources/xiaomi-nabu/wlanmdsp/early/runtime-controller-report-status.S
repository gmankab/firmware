// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r2 = #32
			memw(r23+#896) = r4
			r18 = memw(r29+#12)
	}:mem_noshuf
L_0000c:
	{
			r1 = add(r18,#1072)
	}
L_00010:
	{
			call 0x14c
			r3 = memw(r18+#36)
			memw(r16+#952) = r3.new
	}
L_0001c:
	{
			r1 = #853
			r2 = memw(r29+#8)
			if (!cmp.eq(r2.new,#0)) jump:nt 0x134
	}
L_00028:
	{
			call 0x134
			r0 = add(r16,#4608)
			r1 = add(r18,#1104)
			r2 = #32
	}
L_00038:
	{
			r4 = #6
			r5 = add(r16,#288)
			r2 = memw(r18+#4)
			r3 = memuh(r18+#8)
	}
L_00044:
	{
			r2 = #9
			r3 = add(r16,#264)
			memh(r16+#16) = r3
			memw(r16+#12) = r2
	}
L_00054:
	{
			memw(r16+#188) |= r4
	}
L_00058:
	{
			memw(r16+#192) = setbit(#3)
	}
L_0005c:
	{
			memw(r5+#0) = setbit(#27)
	}
L_00060:
	{
			memw(r16+#224) = setbit(#1)
	}
L_00064:
	{
			call 0x2003d8
			memw(r3+#0) |= r2
	}
L_0006c:
	{
			r1 = #593
			r0 = ##464
			r17 = r0
	}
L_00078:
	{
			call 0x214e3c
			memw(r29+#0) = r17
	}
