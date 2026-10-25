// SPDX-License-Identifier: GPL-3.0-only
// Agent13: copy selected WLAN record fields to in-memory packet table.
// Implements the original per-entry bit-mask selection and bounded loop.
// File [0x42d64,0x42e20), VMA [0xb0002d64,0xb0002e20).
.text
.globl agent13_runtime_record_exchange
agent13_runtime_record_exchange:
{ call 0x2a29f0; allocframe(#48) }
{ r16 = r0; r19:18 = combine(r1,r2) }
{ r17 = add(r16,#4); r3 = memw(r16+#0); memw(r16+#4) += #1 }
{ r21:20 = combine(#0,#0); r4 = memb(r3+#1); if (cmp.eq(r4.new,#0)) jump:nt finish_records }

field_loop:
{ r4 = addasl(r16,r21,#1); p0 = tstbit(r18,r21); r3 = add(r19,r21); if (!p0.new) jump:nt previous_record }
{ r2 = memub(r19+r21<<#0) }
{ memb(r4+#36) = r2; r3 = memub(r3+#2) }:mem_noshuf
{ jump report_entry; memb(r4+#37) = r3 }
previous_record:
{ r3 = addasl(r16,r21,#1) }
{ r2 = memub(r3+#36); r3 = memub(r3+#37) }
report_entry:
{ r1 = #0x7614; r0 = ##0xc0; memw(r29+#4) = r21 }
{ r22 = addasl(r16,r21,#1); memw(r29+#0) = r18; memw(r29+#12) = r3 }
{ call 0x2183e4; memw(r29+#8) = r2 }
{ r1:0 = combine(#4,##0xb02a6328); r2 = memub(r22+#36); r3 = memub(r22+#37) }
{ memw(r29+#12) = r3; memw(r29+#8) = r2 }
{ call -0xadc; memw(r29+#4) = r21; memw(r29+#0) = r18 }
{ r20 = add(r20,#1); r3 = memw(r16+#0) }
{ r21 = and(r20,#255) }
{ r2 = memub(r3+#1); if (cmp.gtu(r2.new,r21)) jump:t field_loop }
finish_records:
{ call 0x2a29bc; r1 = r17; r0 = memw(r3+#688) }
{ jump forward_record }
forward_record:
{ r2 = r1; jump ##0x1f910; r1 = #1 }
