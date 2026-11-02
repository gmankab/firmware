// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r3 = memw(r4+#0)
			memh(r16+#10) = r3.new
	}
L_00008:
	{
			r2 = memw(r2+#0)
			memh(r16+#8) = r2.new
	}
L_00010:
	{
			r2 = memw(r4+#0)
			memw(r16+#12) = r2.new
	}
L_00018:
	{
			memw(r16+#16) = #0
			memw(r17+#12) = #5
	}
L_00020:
	{
			memb(r17+#16) = #10
			memw(r16+#0) = r17
	}
L_00028:
	{
			jump 0x29ef2c
			memw(r16+#4) = #1
	}
L_00030:
	{
			call -0xde8
			r0 = ##-1339397308
			r1 = #1461
	}
L_00040:
	{
			call 0x29ee38
			allocframe(#16)
	}
L_00048:
	{
			r17:16 = combine(r1,r0)
			r1 = #0
			r19:18 = combine(r2,r3)
	}
L_00054:
	{
			r4 = add(r16,mpyi(r17,##488))
	}
L_0005c:
	{
			call 0x215da0
			r0 = memw(r4+#972)
	}
L_00064:
	{
			p0 = cmp.eq(r19,#0); if (p0.new) jump:nt 0x104
	}
L_00068:
	{
			if (!p0.new) r3 = add(r16,##808)
			p0 = cmp.eq(r18,#0); if (p0.new) jump:nt 0x100
	}
L_00074:
	{
			r2 = add(r18,#12)
			p0 = cmp.gt(r0,#0); if (!p0.new) jump:nt 0x1d4
	}
L_0007c:
	{
			r6 = +mpyi(r17,##488)
	}
L_00084:
	{
			r6 += add(r16,##968)
			r4 = #375
			r5 = #4901
	}
L_00094:
	{
			loop0(0x8,r0)
			r6 = add(r6,#8)
	}
L_0009c:
	{
			r7 = memw(r2+#-8)
			if (cmp.gtu(r7.new,#2)) jump:nt 0x1b0
	}
L_000a4:
	{
			r7 = add(r7,#-1)
			r0 = ##-1339397287
			r8 = memw(r2+#-4)
	}
L_000b4:
	{
			r9 = memw(r3+r7<<#2)
	}
L_000b8:
	{
			r9 = add(r6,mpyi(#240,r7))
			memh(r9+#12) = r8
			r8 = memw(r3+r7<<#2)
	}:mem_noshuf
L_000c4:
	{
			r8 = addasl(r3,r7,#2)
			r12 = memw(r2+#0)
			memh(r8+#14) = r12.new
	}
L_000d0:
	{
			r12 = memw(r9+#20)
	}
L_000d4:
	{
			p0 = !tstbit(r12,#1)
			if (!p0.new) jump:t 0x40
			if (!p0.new) r12 = memw(r8+#0)
	}
L_000e0:
	{
			p0 = tstbit(r12,#0)
			if (!p0.new) jump:t 0x7c
	}
L_000e8:
	{
			r1 = #1503
			memb(r16+#3) = r7
			r7 = memw(r8+#0)
	}:mem_noshuf
L_000f4:
	{
			r8 = memuh(r7+#12)
	}
L_000f8:
	{
			r8 = lsr(r8,#3)
	}
L_000fc:
	{
			p0 = cmp.gtu(r8,#374)
			if (p0.new) jump:nt 0x15c
			if (!p0.new) r7 = memuh(r7+#14)
	}
