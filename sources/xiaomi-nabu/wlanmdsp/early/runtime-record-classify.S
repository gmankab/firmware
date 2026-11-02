// SPDX-License-Identifier: GPL-3.0-only
// Agent13: classify an input WLAN record into format/type diagnostic classes.
// File [0x427a0,0x42960), VMA [0xb00027a0,0xb0002960).
// Packet order is deliberate: QuRT logging uses extended constants and
// new-value comparisons shared with packet-local conditional branches.
.text
.globl agent13_runtime_record_classify
agent13_runtime_record_classify:
{ call 0x2a2fc8; allocframe(#40) }
{ r16 = r0; r18 = #255 }
{ r2 = memub(r16+#28) }
{ p0 = !tstbit(r2,#1); if (p0.new) jump:t packet_report; if (!p0.new) r2 = memw(r16+#40); if (!p0.new) r0 = memw(r16+#44) }
{ r3 = memub(r16+#592) }
{ r2 = addasl(r2,r3,#2) }
{ call 0x1609b4; r18 = memw(r2+#808) }
{ r17 = r0; r20 = memw(r18+#36) }
{ call 0x1d6f94; r0 = r20 }
{ call 0x1fcfe8; r18 = r0; r0 = r20 }
{ r1 = #0x413; r0 = ##0x50; r19 = r0 }
{ r2 = memw(r18+#8); r3 = memub(r16+#0) }
{ r4 = memw(r20+##0x14ec); memw(r29+#8) = r4.new }
{ call 0x2189b0; memw(r29+#0) = r2; memw(r29+#4) = r3 }
{ r0 = ##0xb02a6270; r1 = memw(r18+#8); r2 = memub(r16+#0) }
{ call -0x304; r3 = memw(r20+##0x14ec) }
{ r2 = memb(r16+#0); if (!cmp.eq(r2.new,#2)) jump:t test_buffer_type }
{ r2 = memb(r16+#1); if (cmp.eq(r2.new,#3)) jump:t test_buffer_type }
{ r2 = memb(r16+##0xc38); if (cmp.eq(r2.new,#0)) jump:nt type_six }

test_buffer_type:
{ r2 = memw(r18+#8); if (cmp.gtu(r2.new,#3)) jump:t fallback_type }
{ r3 = memw(r2<<#2+##0xb02a6318) }
{ r3 = lsr(r19,r3) }
{ r3 = and(r3,#3); if (cmp.eq(r3.new,#1)) jump:nt type_one }
{ p0 = cmp.eq(r3,#0); if (!p0.new) jump:nt fallback_type }
{ r1 = #0x412; r0 = ##0x58; memw(r29+#4) = r2 }
{ call 0x218948; memw(r29+#0) = r17 }
{ call -0x554; r1:0 = combine(r17,##0xb02a627c); r2 = memw(r18+#8) }
fallback_type:
{ r18 = r17; r2 = #4; if (cmp.gtu(r2.new,r17)) jump:t packet_report }
{ r1 = #0x442; r0 = ##0x68; r2 = memub(r16+#3) }
{ call 0x218918; memw(r29+#4) = r17; memw(r29+#0) = r2 }
{ r0 = ##0xb02a6294; r2 = r17; r1 = memub(r16+#3) }
{ call -0x590; r18 = #0 }
packet_report:
{ r0 = ##0x70; r1 = #0x404; r3 = memuh(r16+#28) }
{ r4 = extractu(r3,#1,#1); r3 = extractu(r3,#1,#2); r2 = memub(r16+#3); memw(r29+#4) = r18 }
{ memw(r29+#12) = r3; memw(r29+#8) = r4 }
{ call 0x2188dc; memw(r29+#0) = r2 }
{ r1:0 = combine(#4,##0xb02a62a0); r3 = memuh(r16+#28); r2 = memub(r16+#3) }
{ r4 = extractu(r3,#1,#1); r3 = extractu(r3,#1,#2); memw(r29+#12) = r3.new }
{ memw(r29+#8) = r4; memw(r29+#4) = r18 }
{ call -0x5f0; memw(r29+#0) = r2 }
// The external target is VMA 0xb02a5844. The Hexagon assembler measures
// this extended jump relative to its packet start, so +4 beyond the
// instruction-address subtraction is required for exact original encoding.
{ r0 = r18; jump ##0x2a2f3c }

type_one:
{ r1 = #0x412; r0 = ##0x60; memw(r29+#4) = r2 }
{ call 0x218898; memw(r29+#0) = r17 }
{ call -0x604; r1:0 = combine(r17,##0xb02a6288); r2 = memw(r18+#8) }
{ r18 = #5; r2 = memub(r16+#0) }
{ r2 = setbit(r2,#1); if (cmp.eq(r2.new,#2)) jump:t packet_report }
{ r2 = memw(r16+#12); if (cmp.eq(r2.new,#0)) jump:nt fallback_type }
{ r3 = ##0x2001000; r2 = memw(r2+#12) }
{ p0 = bitsclr(r2,r3); if (!p0.new) jump:t fallback_type }
{ jump packet_report }
type_six:
{ r18 = #6; jump packet_report }
