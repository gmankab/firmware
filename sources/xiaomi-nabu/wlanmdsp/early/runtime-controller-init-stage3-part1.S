// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V65/V66 WLAN controller stream property encoder
.text
.globl agent13_controller_initialize_stage3
agent13_controller_initialize_stage3:
L_00000:
	{
			memw(r16+#56) = r2
			r2 = memub(r22+#-32)
	}:mem_noshuf
L_00008:
	{
			p0 = !tstbit(r2,#3)
			if (p0.new) jump:t 0xac
	}
L_00010:
	{
			r3 = #1
			memw(r16+#28) = #1
			r2 = memub(r22+#-60)
	}:mem_noshuf
L_0001c:
	{
			p0 = !tstbit(r2,#4)
			if (!p0.new) memw(r16+#28) = #3
			if (!p0.new) r3 = #3
	}
L_00028:
	{
			if (!p0) r2 = memub(r22+##4294967236)
	}
L_00030:
	{
			p0 = !tstbit(r2,#5)
			r2 = setbit(r3,#11)
			if (!p0.new) memw(r16+#28) = r2.new
	}
L_0003c:
	{
			if (p0) r2 = add(r3,#0)
			r4 = memw(r22+#-64)
			if (cmp.eq(r4.new,#0)) jump:nt 0x18
	}
L_00048:
	{
			r2 = setbit(r2,#3)
			memw(r16+#28) = r2.new
	}
L_00050:
	{
			memw(r16+#32) = setbit(#7)
	}
L_00054:
	{
			r3 = memw(r22+#-68)
	}
L_00058:
	{
			r3 = add(r3,#-1)
			if (cmp.gtu(r3.new,#2)) jump:t 0x24
	}
L_00060:
	{
			r3 = add(##256,asl(r3,#8))
			r4 = r3
	}
L_0006c:
	{
			r4 = add(#16,asl(r4,#4))
			memw(r16+#32) |= r3
	}
L_00074:
	{
			r2 = or(r2,r4)
			memw(r16+#28) = r2.new
	}
L_0007c:
	{
			r3 = memw(r22+#-72)
			if (cmp.eq(r3.new,#0)) jump:nt 0x14
	}
L_00084:
	{
			r2 = setbit(r2,#13)
			memw(r16+#28) = r2.new
	}
L_0008c:
	{
			memw(r16+#32) = setbit(#30)
	}
L_00090:
	{
			r3 = memw(r22+#-76)
			if (cmp.eq(r3.new,#0)) jump:nt 0x14
	}
L_00098:
	{
			r2 = setbit(r2,#12)
			memw(r16+#28) = r2.new
	}
L_000a0:
	{
			memw(r16+#32) = setbit(#4)
	}
L_000a4:
	{
			r3 = memw(r22+#-56)
			if (!cmp.eq(r3.new,#1)) jump:t 0x10
	}
L_000ac:
	{
			r2 = setbit(r2,#2)
			memw(r16+#28) = r2.new
	}
