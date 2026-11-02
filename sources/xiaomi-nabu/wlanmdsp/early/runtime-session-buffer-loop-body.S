// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			call 0xc7c98
			r4 = #0
	}
L_00008:
	{
			r18 = r0
			r2 = memub(r16+#1)
	}
L_0000c:
	{
			r3 = #255
	}
L_00010:
	{
			p0 = bitsclr(r2,r3)
			if (!p0.new) r20 = add(r29,#62)
			if (p0.new) jump:nt -0xe4
	}
L_0001c:
	{
			r19 = #0
			r2 = #0
			r21 = add(r29,#40)
	}
L_00024:
	{
			r23:22 = combine(#0,#0)
	}
L_00028:
	{
			r25 = addasl(r21,r2,#3)
			r26 = #0
			r24 = memb(r20+r2<<#0)
			if (cmp.eq(r24.new,#0)) jump:nt 0x54
	}
L_00038:
	{
			r2 = and(r26,#255)
	}
L_0003c:
	{
			r2 = memw(r25+r2<<#2)
			if (cmp.eq(r2.new,#0)) jump:nt 0x30
	}
L_00044:
	{
			r1 = add(r29,#16)
			memd(r29+#16) = r23:22
			memd(r29+#24) = r23:22
	}
L_0004c:
	{
			r2 = memw(r29+#4)
			memd(r29+#32) = r23:22
	}
L_00050:
	{
			memw(r29+#20) = r18
			memw(r29+#24) = r2
	}
L_00054:
	{
			r2 = memuh(r17+#0)
			memh(r29+#36) = r2.new
	}
L_0005c:
	{
			r0 = memw(r16+#688)
			r2 = memuh(r17+#4)
	}
L_00064:
	{
			call 0x1a020
			memh(r29+#38) = r2
	}
L_0006c:
	{
			r26 = add(r26,#1)
			r2 = and(r24,#255)
	}
L_00074:
	{
			r3 = and(r26,#255)
			if (cmp.gtu(r2,r3.new)) jump:t -0x3c
	}
L_0007c:
	{
			r19 = add(r19,#1)
			r3 = memub(r16+#1)
	}
L_00080:
	{
			r2 = and(r19,#255)
			if (cmp.gtu(r3,r2.new)) jump:t -0x58
	}
L_00088:
	{
			jump -0x15c
	}
L_0008c:
	{
			if (p0.new) r1 = #1192
			p0 = cmp.eq(r1,#0); if (p0.new) jump:nt 0x88
			allocframe(#0)
	}
L_00098:
	{
			r2 = memw(r1+#0)
	}
L_0009c:
	{
			r3 = ##2097151
			if (cmp.gt(r2,r3.new)) jump:nt 0x2c
	}
L_000a8:
	{
			r3 = #512
			if (!cmp.eq(r3.new,r2)) jump:nt 0x38
	}
L_000b0:
	{
			r2 = memw(r1+#8)
			if (cmp.eq(r2.new,#1)) jump:nt 0x4c
	}
L_000b8:
	{
			p0 = cmp.eq(r2,#3); if (p0.new) jump:nt 0x44
	}
L_000bc:
	{
			r0 = #2
			p0 = cmp.eq(r2,#2); if (!p0.new) jump:t 0x3c
	}
L_000c4:
	{
			jump 0x3c
	}
L_000c8:
	{
			r3 = ##2097152
			if (!cmp.eq(r3.new,r2)) jump:nt 0x30
	}
L_000d4:
	{
			jump -0x1d4
			r0 = memw(gp+#1932)
			deallocframe
	}
L_000e0:
	{
			r1 = #1198
			p0 = cmp.eq(r2,#32)
			if (p0.new) jump:nt 0x34
	}
L_000ec:
	{
			r1 = #1201
			p0 = cmp.eq(r2,#64)
			if (p0.new) jump:nt 0x28
	}
L_000f8:
	{
			dealloc_return
	}
L_000fc:
	{
			r0 = #1
	}
