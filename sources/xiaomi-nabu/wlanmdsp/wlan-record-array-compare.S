// SPDX-License-Identifier: GPL-3.0-only
// Hexagon record-array comparator including hardware loops and a shared
// reporting epilogue, runtime [0xb0002010,0xb0002138). Calls are PC-relative
// to this independently assembled fragment at 0xb0002010.
.text
.globl wlan_compare_array
wlan_compare_array:
{ call 0x2a3768; allocframe(#16) }
{ r16 = r4; r5 = memw(r2+#8) }
{ r6 = memw(r3+#8); if (!cmp.eq(r6.new,r5)) jump:t mismatch }
{ r4 = memw(r2+#12) }
{ r6 = memw(r3+#12); if (!cmp.eq(r6.new,r4)) jump:t mismatch }
{ r6 = memw(r2+#16) }
{ r7 = memw(r3+#16); if (!cmp.eq(r7.new,r6)) jump:t mismatch }
{ r17 = memw(r2+#0); r7 = memw(r3+#0) }
{ r9:8 = bitsplit(r17,#24); r19:18 = bitsplit(r7,#24) }
{ p0 = cmp.gtu(r18,r8); if (p0.new) jump:nt mismatch }
{ p0 = !cmp.eq(r9,r19); if (p0.new) jump:nt mismatch; if (!p0.new) r7 = memw(r2+#20) }
{ r3 = memw(r3+#20); if (!cmp.eq(r3.new,r7)) jump:nt mismatch }
{ r19 = r8; r3 = add(r1,#12); p0 = !cmp.gtu(r8,r18); if (p0.new) jump:t unequal_count }
{ p0 = cmp.gt(r0,#0); r12 = sub(r8,r18) }
{ loop1(inner_first,r12) }
inner_first:
{ if (p0) r12 = add(r3,#0); if (!p0) r19 = add(r8,#0); if (!p0) jump:nt complete_compare }
{ loop0(inner_iteration,r0) }
inner_iteration:
{ r13 = memw(r12+#-12); if (!cmp.eq(r13.new,r9)) jump:t inner_next }
{ r13 = memw(r12+#-4); if (!cmp.eq(r13.new,r5)) jump:t inner_next }
{ r13 = memw(r12+#0); if (!cmp.eq(r13.new,r4)) jump:t inner_next }
{ r13 = memw(r12+#4); if (!cmp.eq(r13.new,r6)) jump:t inner_next }
{ r13 = memw(r12+#8); if (!cmp.eq(r13.new,r7)) jump:t inner_next }
{ r13 = memw(r12+#-8); if (cmp.eq(r13.new,r19)) jump:nt inner_match }
inner_next:
{ r12 = add(r12,#24); nop }:endloop0
{ jump complete_compare }
inner_match:
{ r19 = add(r19,#-1); nop; nop }:endloop1
{ jump complete_compare }
mismatch:
{ call consumer_exit; r1:0 = combine(r2,r16); r2 = #24 }
{ r0 = #0; jump ##0x2a376c }
unequal_count:
{ call consumer_exit; r1:0 = combine(r2,r16); r2 = #24 }
{ r0 = #1; jump ##0x2a3758 }
complete_compare:
{ call consumer_exit; r1:0 = combine(r2,r16); r2 = #24 }
{ r2 = extractu(r19,#24,#0); p0 = !cmp.eq(r19,r18); r0 = #0 }
{ r17 = or(r2,and(r17,##-16777216)); if (!p0) r0 = #1; memw(r16+#0) = r17.new }
{ jump 0x2a3728 }
consumer_exit:
{ jump ##-4208 }
