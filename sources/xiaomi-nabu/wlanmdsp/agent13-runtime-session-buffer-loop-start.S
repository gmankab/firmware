// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			call 0x29f220
			r3:2 = combine(#0,#0)
			allocframe(#112)
	}
L_00008:
	{
			r16 = r0
			r5:4 = memd(##2955570464)
			memh(r29+#62) = #0
	}
L_00018:
	{
			memd(r29+#48) = r3:2
			memd(r29+#40) = r3:2
	}
L_0001c:
	{
			call 0x1de46c
			memd(r29+#8) = r5:4
			memw(r29+#4) = #0
	}
L_00028:
	{
			p0 = cmp.eq(r0,#0); if (p0.new) jump:nt 0x8
	}
L_0002c:
	{
			jump 0x29f2d0
	}
L_00030:
	{
			r2 = memb(r16+#1)
			if (cmp.eq(r2.new,#0)) jump:nt -0x4
	}
L_00038:
	{
			r22 = add(r16,#808)
			r23 = #0
			r17 = #0
			r19 = #0
	}
L_00044:
	{
			r24 = add(r29,#40)
			r25 = add(r29,#62)
			r21 = #0
	}
L_00050:
	{
			call 0x1614c
			r2 = add(r29,#8)
			r0 = memw(r16+#688)
			r1 = memw(r22+r19<<#2)
	}
L_00060:
	{
			r18 = r0
			memb(r25+r19<<#0) = r18.new
	}
L_00068:
	{
			p0 = cmp.eq(r18,#0); if (p0.new) jump:nt 0x44
	}
L_0006c:
	{
			r20 = add(r29,#8)
			r19 = and(r23,#255)
	}
L_00070:
	{
			r26 = addasl(r24,r19,#3)
			nop
			nop
			nop
	}
L_00080:
	{
			call 0x162ac
			r1 = r20
			r2 = r19
			r0 = memw(r16+#688)
	}
L_00090:
	{
			call 0x144758
			r17 = r0
			r18 = add(r18,#-1)
	}
L_00098:
	{
			r21 = add(r0,r21)
			r20 = add(r20,#4)
			p0 = cmp.eq(r18,#0)
			memw(r26++#4) = r0
	}
L_000a8:
	{
			if (!p0) jump:nt -0x28
	}
L_000ac:
	{
			r23 = add(r23,#1)
			r2 = memub(r16+#1)
	}
L_000b0:
	{
			r19 = and(r23,#255)
			if (cmp.gtu(r2,r19.new)) jump:t -0x60
	}
L_000b8:
	{
			if (!p0.new) r4 = add(r16,#84)
			p0 = cmp.eq(r21,#0); if (p0.new) jump:nt -0x8c
	}
L_000c0:
	{
			r3 = memw(r16+#320)
			if (cmp.eq(r3.new,#0)) jump:nt 0x30
	}
L_000c8:
	{
			r5 = #0
			r6 = #0
	}
L_000cc:
	{
			r0 = memw(r4+r6<<#2)
	}
L_000d0:
	{
			r6 = memuh(r0+#28)
	}
L_000d4:
	{
			r6 = and(r6,#17)
			if (!cmp.eq(r6.new,#1)) jump:t 0x10
	}
L_000dc:
	{
			r6 = memb(r0+#1)
			if (cmp.eq(r6.new,#3)) jump:nt 0x18
	}
L_000e4:
	{
			r5 = add(r5,#1)
	}
L_000e8:
	{
			r6 = and(r5,#255)
			if (cmp.gtu(r3,r6.new)) jump:t -0x1c
	}
L_000f0:
	{
			r18 = #0 ; jump 0x1c
	}
L_000f4:
	{
			r1 = add(r21,##500000)
			r3 = #0
			r2 = add(r29,#4)
	}
