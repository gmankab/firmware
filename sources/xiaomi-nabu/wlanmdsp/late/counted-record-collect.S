// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x10059c,0x100698),
// runtime [0xb00c059c,0xb00c0698).
// Walk length-prefixed records and retain the last type-zero record pointer.
// Collect at most fifteen bytes from type-one and type-0x32 records, masking
// bit seven of each value. A matching seven-byte type-zero payload sets flag0.
// The destination count is reset, but the existing destination flags are not.
// The final result follows the separate signature parser at 0xb00c0698.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_counted_record_collect
wlan_late_counted_record_collect:
{ call 0x1e51dc; allocframe(#0x10) }
{ r17:16 = combine(r2,#-0x1); r19:18 = combine(r0,r1) }
{ p0 = cmp.eq(r17,#0x0); if (p0.new) jump:nt restore_result }
{ r2 = #0x0; p0 = cmp.gt(r18,#0x1); if (!p0.new) jump:t parse_signatures; memb(r17+#0x198) = r2.new }
{ r5 = add(r17,#0x198); r3 = add(r19,r18); r8 = add(r19,#0x1); r4 = #0x0 }
{ r6 = r19; r5 = add(r5,#1) }
record_loop:
{ r7 = memub(r8+#0x0) }
{ r9 = r7 }
{ r9 += add(r6,#0x1); if (!cmp.gtu(r3,r9.new)) jump:nt check_saved_record }
{ r9 = memub(r6+#0x0); if (!cmp.eq(r9.new,#0x0)) jump:t check_value_record }
{ r2 = r6 ; jump advance_record }
check_value_record:
{ p0 = cmp.eq(r9,#0x32); if (p0.new) jump:nt copy_values }
{ p0 = cmp.eq(r9,#0x1); if (!p0.new) jump:t advance_record }
copy_values:
{ r12 = #0x0; r9 = add(r6,#0x2); p0 = cmp.eq(r7,#0x0); if (p0.new) jump:nt empty_value_record }
value_loop:
{ r13 = and(r4,#0xff); if (cmp.gtu(r13.new,#0xe)) jump:nt advance_record }
{ r4 = add(r4,#0x1); r12 = add(r12,#0x1); r7 = memub(r9++#0x1); memb(r17+#0x198) = r4.new }
{ r7 = and(r7,#0x7f); memb(r5+r13<<#0x0) = r7.new }
{ r7 = memub(r8+#0x0); if (cmp.gtu(r7.new,r12)) jump:t value_loop }
{ jump advance_record }
empty_value_record:
{ r7 = #0x0 }
advance_record:
// Advance using parallel reads: r9 receives the old length, while r7
// becomes the next record length-byte address; the next packet forms
// the next record base from that saved length.
{ r7 += add(r6,#0x3); r9 = r7 }
{ r9 += add(r6,#0x2); r8 = r7; p0 = cmp.gtu(r3,r7) }
{ r6 = r9; if (p0) jump:nt record_loop }
check_saved_record:
{ if (!p0.new) r0 = add(r2,#0x2); p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt parse_signatures }
{ memw(r17+#0x190) = r2 }
{ r3 = memb(r2+#0x1); if (!cmp.eq(r3.new,#0x7)) jump:t parse_signatures }
{ r1 = ##0xb02b6f0e; r2 = #0x7 }
{ call -0xbccf4 }
{ r2 = add(r17,#0x194); p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:t parse_signatures }
{ memw(r2+#0x0) = setbit(#0x0) }
parse_signatures:
{ call 0x18; r1:0 = combine(r18,r19); r2 = r17 }
{ p0 = cmp.eq(r0,#0x0); if (p0.new) r16 = #0 }
restore_result:
{ r0 = r16 ; jump ##0x1e51c4 }
