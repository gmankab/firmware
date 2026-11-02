// SPDX-License-Identifier: GPL-3.0-only
// Agent13: update packed WLAN record bitmap and field table using 2-slot
// Hexagon hardware loops, preserving dual predicate/store semantics.
// File [0x42e20,0x42ed4), VMA [0xb0002e20,0xb0002ed4).
.text
.globl agent13_runtime_record_update
agent13_runtime_record_update:
{ call 0x2a2948; allocframe(#40) }
{ r16 = r0; r21:20 = combine(r1,r4); r19:18 = combine(r2,r3) }
{ r17 = add(r16,#44); r5 = memw(r16+#0); memw(r16+#52) = #7 }
{ memw(r16+#44) += #1 }
{ r2 = add(r16,#60); r5 = memb(r5+#1); if (cmp.eq(r5.new,#0)) jump:nt report_update }
{ r3 = #0; r7 = #0; r4 = #1; p0 = cmp.eq(r20,#0) }
outer_loop:
{ loop0(slot_compare,#2); r5 = r19; r6 = and(r3,#255) }
{ r5 &= asl(r4,r7); r6 = addasl(r2,r6,#2); r7 = #0 }
slot_compare:
{ p1 = cmp.eq(r5,#0); if (p1.new) jump:nt slot_advance }
{ p1 = !tstbit(r18,r7); if (p1.new) jump:nt slot_advance }
{ if (!p0) memb(r6+#0) = r21; if (p0) memb(r6+#1) = r21 }
slot_advance:
{ nop; r6 = add(r6,#2); r7 = add(r7,#1) }:endloop0
{ r3 = add(r3,#1); r5 = memw(r16+#0) }
{ r7 = and(r3,#255); r5 = memub(r5+#1) }
{ p1 = cmp.gtu(r5,r7); if (p1.new) jump:t outer_loop }
report_update:
{ r1 = #0x7614; r0 = ##0xc8; memw(r29+#12) = r20 }
{ memw(r29+#8) = r21; memw(r29+#4) = r19 }
{ call 0x21830c; memw(r29+#0) = r18 }
{ r1:0 = combine(#4,##0xb02a6334); memw(r29+#12) = r20; memw(r29+#8) = r21 }
{ call -0xbac; memw(r29+#4) = r19; memw(r29+#0) = r18 }
{ r1 = r17; r2 = memw(r16+#0) }
{ call 0x2a2910; r0 = memw(r2+#688) }
{ jump -0xbc }
