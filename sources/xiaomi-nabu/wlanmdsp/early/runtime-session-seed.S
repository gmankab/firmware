// SPDX-License-Identifier: GPL-3.0-only
// Agent 13: initialize two indexed WLAN session tables, with branch-driven
// iteration over the parent radio/QuRT connection state.
// File [0x42c8c,0x42d30), VMA [0xb0002c8c,0xb0002d30).
.text
.globl agent13_runtime_session_seed
agent13_runtime_session_seed:
{ call 0x2a2aec; allocframe(#16) }
{ call 0x219c9c; r17 = r0; r1:0 = combine(#84,#3) }
{ r16 = r0; r18 = add(r17,#1) }
{ r3 = add(r16,#32); memw(r16+#0) = r17; memw(r16+#12) = #2 }
{ memw(r16+#16) = #3 }
{ r1:0 = combine(#0,#0); r2 = memb(r17+#1); if (cmp.eq(r2.new,#0)) jump:nt first_ready }
{ r2 = add(r16,#20); r4 = add(r16,#36); r5 = r18 }
{ call 0x208 }
first_ready:
{ r1 = #2; r3:2 = combine(##0xb0002d30,#1); r0 = memw(r17+#688) }
{ call 0x1f96c; r4 = r16 }
{ r3 = add(r16,#72); memw(r16+#52) = #7; memw(r16+#56) = #3 }
{ r1:0 = combine(#0,#0); r2 = memb(r17+#1); if (cmp.eq(r2.new,#0)) jump:nt second_ready }
{ r2 = add(r16,#60); r4 = add(r16,#76); r5 = r18 }
{ call 0x1c8 }
second_ready:
{ r1 = #7; r3:2 = combine(##0xb0002d30,#1); r0 = memw(r17+#688) }
{ call 0x1f92c; r4 = r16 }
{ r0 = r16; jump ##0x2a2b2c }
