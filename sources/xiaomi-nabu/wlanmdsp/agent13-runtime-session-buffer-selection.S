// SPDX-License-Identifier: GPL-3.0-only
// Agent 13 hand-verified V66 controller stream continuation; Hexagon v65-compatible subset.
.text
.globl agent13_controller_initialize_stage3_cont
agent13_controller_initialize_stage3_cont:
L_00000:
	{
			r5 = memub(r0+#1)
			if (cmp.gtu(r5.new,r4)) jump:t -0xc
	}
L_00008:
	{
			dealloc_return
	}
L_0000c:
	{
			call -0xcc4
			r0 = ##-1339397329
			r1 = #1322
	}
L_0001c:
	{
			call 0x29ef5c
			r4 = r0
			allocframe(#16)
	}
L_00024:
	{
			r18 = +mpyi(r1,##488)
			r17:16 = combine(r3,r2)
	}
L_00030:
	{
			r5 = add(r4,mpyi(r1,##488))
			r1 = #0
	}
L_0003c:
	{
			r18 += add(r4,##968)
	}
L_00044:
	{
			call 0x215eb8
			r0 = memw(r5+#972)
	}
L_0004c:
	{
			if (!p0.new) r3 = add(r17,#20)
			p0 = cmp.eq(r0,#1); if (p0.new) jump:nt 0x7c
	}
L_00054:
	{
			p0 = cmp.eq(r0,#2); if (!p0.new) jump:nt 0xdc
	}
L_00058:
	{
			r2 = memw(r18+#268)
	}
L_0005c:
	{
			p0 = tstbit(r2,#0)
			r2 = and(r2,#1)
			memw(r17+#28) = r2.new
	}
L_00068:
	{
			if (p0) r2 = ##2412
			if (p0) r4 = add(r18,##300)
	}
L_00078:
	{
			if (p0) r5 = add(r18,##304)
			if (!p0) r2 = ##5220
	}
L_00088:
	{
			if (!p0) r4 = add(r18,##332)
			memh(r17+#20) = r2
	}
L_00094:
	{
			if (!p0) r5 = add(r18,##336)
			memh(r17+#22) = r2
			r2 = memw(r4+#0)
	}:mem_noshuf
L_000a4:
	{
			memh(r16+#30) = r2
			r2 = memw(r5+#0)
	}:mem_noshuf
L_000ac:
	{
			memh(r16+#28) = r2
			r2 = memw(r4+#0)
	}:mem_noshuf
L_000b4:
	{
			memw(r16+#32) = r2
			memw(r16+#36) = #0
	}
L_000b8:
	{
			memw(r17+#32) = #5
			memb(r17+#36) = #10
	}
L_000c0:
	{
			memw(r16+#24) = #1
			memw(r16+#20) = r3
	}
L_000c8:
	{
			r2 = memw(r18+#28)
	}
L_000cc:
	{
			p0 = tstbit(r2,#1)
			r2 = and(r2,#2)
			if (p0.new) r3 = ##5220
	}
L_000dc:
	{
			if (p0) r4 = add(r18,#92)
			r2 = cmp.eq(r2,#0)
			memw(r17+#8) = r2.new
	}
L_000e8:
	{
			if (!p0) r4 = add(r18,#60)
			if (!p0) r3 = ##2412
			if (p0) r2 = add(r18,#96)
	}
L_000f8:
	{
			if (!p0) r2 = add(r18,#64)
			memh(r17+#0) = r3
			memh(r17+#2) = r3
	}
