// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100528,0x10059c),
// runtime [0xb00c0528,0xb00c059c).
// Search context-associated patterns against data in the caller-owned stack frame.
// After the 0x18-byte allocation, r29+0x20 addresses the caller stack pointer;
// the count word is at caller_sp+0x18c and comparison bytes at caller_sp+0x164.
// The helper at 0xb00c0e74 searches counted eight-byte candidate patterns.
// Its detailed pattern rules are recovered in later functions. Return 1 when
// either search matches; return 0 if both searches miss. A zero full caller
// count word is a distinct early path that returns 1 before either search.
// After its second zero result the counted loop only increments registers;
// it does not perform a fresh comparison on each iteration.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_caller_frame_search
wlan_late_caller_frame_search:
{ call 0x1e5240; allocframe(#0x18) }
{ r20 = r0; r16 = #0x1; r2 = memw(r29+#0x1ac); if (cmp.eq(r2.new,#0x0)) jump:nt restore_result }
{ r19 = add(r20,#0x230); r3 = add(r29,#0x20); r18 = and(r2,#255) }
{ r17 = add(r3,#0x164); r2 = r18 }
{ call 0x924; r1:0 = combine(r17,r19) }
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:t restore_result }
{ r2 = memw(r20+#0x22c) }
{ r2 = add(r2,#0x7f8) }
{ r20 = extractu(r2,#0x8,#0x3); if (cmp.eq(r20.new,#0x0)) jump:nt searches_missed }
{ r0 = asl(r20,#0x3); r1 = r17; r2 = r18 }
{ r0 += add(r19,#0x8); call 0x900 }
{ r2 = #0x0; p0 = cmp.eq(r0,#0x0) }
counted_success_loop:
{ if (p0) r2 = add(r2,#0x1); if (!p0) jump:nt restore_result }
{ r3 = and(r2,#0xff); if (cmp.gtu(r20,r3.new)) jump:t counted_success_loop }
searches_missed:
{ r16 = #0x0 }
restore_result:
{ r0 = r16 ; jump ##0x1e52b0 }
