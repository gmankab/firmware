// SPDX-License-Identifier: GPL-3.0-only
// Agent 13: summarize caller record list into a compact status class.
// File [0x42960,0x42a14), VMA [0xb0002960,0xb0002a14).
// The list walk preserves bounded count, four predicate paths and a shared
// far tail branch. Packet-local .new value checks are semantically required.
.text
.globl agent13_runtime_record_summary
agent13_runtime_record_summary:
{ call 0x2a2df4; allocframe(#32) }
{ r16 = r0 }
{ r21 = memw(r16+#40) }
{ r22 = add(r21,#84); r2 = memw(r21+#320); if (cmp.eq(r2.new,#0)) jump:nt empty_records }
{ r20 = #0; r17 = #0; r23 = #0; r19:18 = combine(#0,#0) }

record_walk:
{ r0 = memw(r22+#0); if (cmp.eq(r0.new,#0)) jump:nt next_record }
{ r2 = memb(r0+#592) }
{ r3 = memb(r16+#0); if (!cmp.eq(r3.new,r2)) jump:t next_record }
{ call -0x1fc }
{ if (p0.new) r20 = #1; p0 = cmp.eq(r0,#5); if (p0.new) jump:nt next_record }
{ if (p0.new) r19 = #1; p0 = cmp.eq(r0,#6); if (p0.new) jump:nt next_record }
{ p0 = cmp.eq(r0,#255); if (!p0.new) r18 = #1; if (p0.new) jump:t next_record }
{ r17 = maxu(r17,r0) }
next_record:
{ r22 = add(r22,#4); r23 = add(r23,#1) }
{ r2 = memw(r21+#320); if (cmp.gtu(r2.new,r23)) jump:t record_walk }
{ jump combine_flags }
empty_records:
{ r17 = #0; r20 = #0; r19:18 = combine(#0,#0) }
combine_flags:
{ r2 = r19; r0 = #255 }
{ r2 |= or(r20,r18) }
{ p0 = bitsclr(r2,r0); if (p0.new) jump:nt finish }
{ p0 = cmp.eq(r20,#1); if (!p0.new) jump:nt use_largest }
{ r2 = #255; r0 = #5 }
{ p0 = bitsclr(r18,r2); if (p0.new) jump:t finish }
use_largest:
{ r0 = r17; p0 = cmp.eq(r19,#1); if (!p0.new) jump:nt finish }
{ p0 = cmp.eq(r18,#0); if (p0.new) r0 = #6 }
finish:
{ jump 0x2a2e20 }
