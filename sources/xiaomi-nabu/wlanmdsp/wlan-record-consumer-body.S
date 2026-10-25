// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 implementation of the remaining 508 bytes of the WLAN packed
// record-consumer routine at [0xb0001dac,0xb0001fa8). This follows the
// verified 300-byte entry [0xb0001c80,0xb0001dac); the 0x1dac body contains
// arithmetic validation, index-dependent moves and several copy/merge loops.
// The semantic multiply/subtract is encoded by LLVM22 using CPU hexagonv66.
// Source is assembled at offset zero; calls to existing alias thunks at the
// end of this function are relative to 0xb0001dac.
.text
.globl wlan_record_consumer_body
wlan_record_consumer_body:
{ r16 = r0; r3 = memw(r29+#16) }
{ p1 = r3; jump mode_entry }
retry_lookup:
{ r2 = p1; r0 = r23; r1 = memw(r29+#40); memw(r29+#16) = r2.new }
{ call 0x1e8 }
{ r16 = r0; r2 = memw(r29+#16) }
{ p1 = r2; jump choose_source_after_load }
normalize_input:
{ r20 = r23; r16 = memw(r29+#48); r17 = memw(r29+#40) }
{ r20 -= mpyi(r17,r16); p0 = cmp.eq(r16,#510); if (!p0.new) jump:t mode_entry }
{ p0 = cmp.gtu(r23,#4); r20 = sub(r23,r17) }
{ r16 = mux(p0,#1,#0) }
mode_entry:
{ p0 = cmp.eq(r25,#18); r24 = add(r19,r23); if (!p0.new) jump:nt is_secondary }
{ p0 = cmp.eq(r20,#0); if (p0.new) jump:nt is_secondary; if (p1) jump:nt negative_count }
{ p0 = cmp.gt(r20,#-1); r25 = sub(#0,r20); r2 = memuh(r19+#0) }
{ r26 = r2; if (p0) memw(r29+#16) = r16 }
{ r26 += add(r19,#4); if (!p0) jump:nt skip_compare_loop }
{ r20 = add(r23,sub(#-4,r2)); r24 = add(r16,#-1); memw(r29+#4) = r24 }
compare_loop:
{ r16 = sub(#-4,r2); p0 = cmp.eq(r24,#0); if (p0.new) jump:nt compare_done }
compare_body:
{ call 0x178; r0 = add(r26,r25); r1 = r26; r2 = r20 }
{ r20 = add(r20,r16); r26 = add(r26,r17); r24 = add(r24,#-1); if (!cmp.eq(r24.new,#0)) jump:t compare_body }
compare_done:
{ p0 = !cmp.gtu(r18,r27); if (p0.new) jump:t finish_metadata; r16 = memw(r29+#16); if (!p0.new) r1 = memw(r29+#4) }
{ r0 = mpyi(r16,r25); r2 = sub(r18,r27) }
{ r0 += add(r19,r23); call 0x140 }
{ jump finish_metadata }
is_secondary:
{ if (!p0.new) r0 = sub(r24,r20); if (!p0.new) r2 = sub(r18,r27); p0 = cmp.gt(r20,#-1); if (p0.new) jump:t choose_source_after_load }
{ r1 = r24; r25 = sub(#0,r20) }
{ call 0x120 }
{ call 0x10c; r1:0 = combine(#0,r24); r2 = r25 }
{ r3:2 = combine(#1,r19); jump write_block }
skip_compare_loop:
{ p0 = !cmp.gtu(r18,r27); if (!p0.new) r1 = add(r24,#0); if (p0.new) jump:t prepare_count }
{ r0 = mpyi(r16,r25); r20 = r2; r2 = sub(r18,r27) }
{ r0 += add(r19,r23); call 0xf0 }
{ r2 = r20 }
prepare_count:
{ r27 = add(r23,sub(#-4,r2)); r20 = sub(#-4,r2); p0 = cmp.eq(r16,#0); if (p0.new) jump:nt finish_metadata }
{ r24 = r16 }
active_compare_loop:
{ p0 = cmp.eq(r24,#1); if (!p0.new) r0 = add(r26,r25); if (!p0.new) r1 = add(r26,#0); if (p0.new) jump:t no_copy }
{ call 0xc4; r2 = r27 }
no_copy:
{ call 0xac; r1:0 = combine(#0,r26); r2 = r25; r27 = add(r27,r20) }
{ r26 = add(r26,r17); r24 = add(r24,#-1); if (!cmp.eq(r24.new,#0)) jump:t active_compare_loop }
finish_metadata:
{ r25 = mpyi(r16,r25); r3:2 = combine(#1,r19); jump write_block; r26 = memw(r29+#12) }
choose_source_after_load:
{ r25 = #0; r2 = mux(p1,r19,#0); r3 = #0; p0 = cmp.eq(r23,#0) }
{ if (!p0) r2 = add(r19,#0) }
write_block:
{ r4 = +mpyi(r22,#12); r24 = #0; r27 = memw(r29+#28); r6 = memw(r29+#24) }
{ r5 = add(r6,mpyi(#12,r22)); r27 = add(r27,#-1); memw(r6+r4<<#0) = r2 }
{ p0 = cmp.eq(r27,#0); if (!p0.new) r2 = add(r25,r23) }
{ if (p0) jump:nt -692; memw(r5+#4) = r16; memw(r5+#8) = r3 }
{ r18 = add(r25,r18); r17 = r21; r22 = add(r22,#1) }
{ r17 += add(r25,r23); r19 += add(r25,r23); r20 = memw(r29+#20) }
{ r21 += add(r2,#4); if (!cmp.gtu(r21.new,r18)) jump:t -612 }
{ jump -728 }
negative_count:
{ r24 = #-1; r2 = memw(r29+#8) }
{ jump -740; memw(r2+#0) = #0 }
{ r0 = ##-1339399874; r1 = ##-1339399857 }
{ call 0x21b020 }
