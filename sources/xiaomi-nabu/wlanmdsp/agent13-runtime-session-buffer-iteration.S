// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r1 = #0
			r2 = memw(gp+#1316)
	}
L_00008:
	{
			r2 = memw(r2+#52)
	}
L_0000c:
	{
			callr r2
	}
L_00010:
	{
			dealloc_return
	}
L_00014:
	{
			call -0xbcc
			r0 = ##-1339397356
	}
L_00020:
	{
			r5 = +mpyi(r1,##488)
			r4 = add(r0,#12)
			allocframe(#0)
	}
L_00030:
	{
			r5 += add(r0,##968)
			r6 = memub(r0+#1)
	}
L_0003c:
	{
			p0 = cmp.eq(r6,#1); if (p0.new) jump:nt 0x48
	}
L_00040:
	{
			p0 = cmp.eq(r6,#2); if (!p0.new) jump:nt 0xcc
	}
L_00044:
	{
			r6 = memw(r5+#268)
			memw(r3+#32) = #5
	}
L_0004c:
	{
			p0 = !tstbit(r6,#0)
			r6 = and(r6,#1)
			memw(r3+#28) = r6.new
	}
L_00058:
	{
			if (p0) r6 = ##5220
			memb(r3+#36) = #10
	}
L_00064:
	{
			if (!p0) r6 = ##2412
	}
L_0006c:
	{
			r6 = add(r3,#20)
			memh(r3+#20) = r6
			memh(r3+#22) = r6
	}
L_00078:
	{
			memb(r2+#28) = #1
			memw(r2+#36) = r4
	}
L_00080:
	{
			memw(r2+#32) = r6
	}
L_00084:
	{
			r5 = memw(r5+#28)
			memw(r3+#12) = #5
	}
L_0008c:
	{
			p0 = tstbit(r5,#1)
			r5 = and(r5,#2)
			memb(r3+#16) = #10
	}
L_00098:
	{
			if (p0) r6 = ##5220
			r5 = cmp.eq(r5,#0)
			memw(r3+#8) = r5.new
	}
L_000a8:
	{
			if (!p0) r6 = ##2412
	}
L_000b0:
	{
			memh(r3+#0) = r6
			memh(r3+#2) = r6
	}
L_000b4:
	{
			memb(r2+#4) = #1
			memw(r2+#8) = r3
	}
L_000bc:
	{
			memw(r2+#12) = r4
	}
L_000c0:
	{
			r5 = #0
			r4 = add(r2,#16)
			r3 = memb(r0+#1)
			if (cmp.eq(r3.new,#0)) jump:nt 0x48
	}
L_000d0:
	{
			r3 = add(r0,#4168)
	}
L_000d4:
	{
			r5 = add(r5,#1)
			r3 = add(r3,#16)
			memw(r4++#24) = r3
	}
L_000e0:
	{
			r6 = memub(r0+#1)
			if (cmp.gtu(r6.new,r5)) jump:t -0xc
	}
L_000e8:
	{
			r3 = add(r0,#4608)
			p0 = cmp.eq(r6,#0); if (p0.new) jump:nt 0x20
	}
L_000f0:
	{
			r2 = add(r2,#20)
			r4 = #0
	}
L_000f4:
	{
			r4 = add(r4,#1)
			r3 = add(r3,#16)
			memw(r2++#24) = r3
	}
