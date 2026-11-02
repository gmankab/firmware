// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100698,0x1007c4),
// runtime [0xb00c0698,0xb00c07c4).
// Walk type-0xdd length-prefixed records and match two four-byte signatures:
// 00:50:f2:04 and 50:6f:9a:09. Pass the bytes after each signature to its
// external parser. Each parser-result slot starts at -1; return their sum.
// The code does not prove that a zero sum means both parsers succeeded.
// A truncated record takes the diagnostic path and returns -1.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_signature_dispatch
wlan_late_signature_dispatch:
{ call 0x1e50bc; allocframe(#0x28) }
{ r17:16 = combine(r2,r0); r0 = #-0x1 }
{ p0 = cmp.eq(r16,#0x0); if (p0.new) jump:nt restore_result }
{ r2 = #0x2; if (cmp.gtu(r2.new,r1)) jump:nt restore_result }
{ r22 = add(r16,r1); r19:18 = combine(#-0x1,#-0x1); p0 = cmp.gt(r1,#0x1); if (!p0.new) jump:t sum_parser_results }
{ r19:18 = combine(#-0x1,#-0x1); r23 = #0x4; r21 = add(r16,#1) }
record_loop:
{ r2 = memub(r21+#0x0) }
{ r3 = r2 }
{ r3 += add(r16,#0x2) }
// The predicate tests the previously computed record end. The parallel
// load overwrites r3 with the record type only after that predicate reads it.
{ p0 = !cmp.gtu(r3,r22); if (p0.new) r20 = add(r16,#0x6); if (!p0.new) jump:nt truncated_record; r3 = memub(r16+#0x0) }
{ p0 = cmp.gtu(r23,r2); if (p0.new) jump:nt check_second_record_type }
{ p0 = cmp.eq(r3,#0xdd); if (!p0.new) jump:nt check_second_record_type }
{ r3 = memb(r16+#0x2); if (!cmp.eq(r3.new,#0x0)) jump:t check_second_signature }
{ r3 = memb(r16+#0x3) }
{ p0 = cmp.eq(r3,#0x50); if (!p0.new) jump:t check_second_signature; if (p0.new) r3 = memb(r16+#0x4) }
{ p0 = cmp.eq(r3,#-0xe); if (!p0.new) jump:t check_second_signature }
{ r3 = memb(r16+#0x5); if (!cmp.eq(r3.new,#0x4)) jump:t check_second_signature }
{ call 0x2ec; r1 = add(r2,#-0x4); r0 = r20; r2 = r17 }
{ r19 = r0; r3 = memub(r16+#0x0) }
check_second_record_type:
{ p0 = cmpb.eq(r3,#0xdd); if (!p0.new) jump:t advance_record }
check_second_signature:
{ r2 = memub(r21+#0x0); if (cmp.gtu(r23,r2.new)) jump:t advance_record }
{ r3 = memb(r16+#0x2) }
{ p0 = cmp.eq(r3,#0x50); if (!p0.new) jump:t advance_record; if (p0.new) r3 = memb(r16+#0x3) }
{ p0 = cmp.eq(r3,#0x6f); if (!p0.new) jump:t advance_record; if (p0.new) r3 = memb(r16+#0x4) }
{ p0 = cmp.eq(r3,#-0x66); if (!p0.new) jump:t advance_record }
{ r3 = memb(r16+#0x5); if (!cmp.eq(r3.new,#0x9)) jump:t advance_record }
{ call 0x364; r1 = add(r2,#-0x4); r0 = r20; r2 = r17 }
{ r18 = r0 }
advance_record:
{ r21 = memub(r21+#0x0) }
// r2 reads the old record length while r21 becomes next_length_ptr.
// The following packet forms next_record_ptr from the saved old length.
{ r21 += add(r16,#0x3); r2 = r21 }
{ r2 += add(r16,#0x2); p0 = cmp.gtu(r22,r21) }
{ r16 = r2; if (p0) jump:nt record_loop }
sum_parser_results:
{ r0 = add(r18,r19) }
restore_result:
{ jump 0x1e509c }
truncated_record:
{ r1 = #0x2412; r0 = ##0x64c0; memw(r29+#0x4) = r2 }
{ call 0x15aa10; memw(r29+#0x0) = r3 }
{ r0 = ##-0x4fd490b8; r1 = memub(r16+#0x0); r2 = memub(r21+#0x0) }
{ call -0xbe498 }
{ r0 = #-0x1; jump 0x1e5074 }
