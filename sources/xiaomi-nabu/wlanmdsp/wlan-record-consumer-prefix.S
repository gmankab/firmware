// SPDX-License-Identifier: GPL-3.0-only
// Hexagon WLAN packed-record consumer, verified INITIAL PATH ONLY.
// The true control flow continues after this prefix; the last instruction
// in this source is not a function return. Extended compared range is 300 bytes. The caller passes a configuration
// array and desired record index. This portion checks prerequisites, reads
// runtime global-pointer slots, validates record count and initializes local
// state for the subsequent loop.
//
// Calls/jumps reference existing functions in the same WLAN executable image
// and use original PC-relative displacements, normalized because this source
// assembles alone with its first instruction mapped to runtime 0xb0001c80.
// They are not copied machine words, and are not proof of a complete function.

.text
.globl wlan_consume_records
wlan_consume_records:
{ call 0x2a3aa0; allocframe(#112) }
{ r26 = r3; r19:18 = combine(r0,r1); r20 = r2; r2 = #-1 }
{ r1:0 = combine(r26,r20); r24 = #-1; r3 = add(r29,#32); r16 = r4 }
{ call -0xbc }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt next_check }
fail_exit:
{ r0 = r24; jump 0x2a3b54 }
next_check:
{ p0 = cmp.eq(r20,#0); if (p0.new) jump:nt init_record; r27 = memw(r29+#52) }
{ call 0x21b9d0 }
{ if (!p0.new) r2 = #405; p0 = cmp.eq(r0,#0); if (!p0.new) jump:nt 0x2d8 }
init_record:
{ r2 = memw(##2959748528); memw(r29+#24) = r2.new }
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:nt -0x28; memw(r16+#0) = r2 }
{ r2 = memw(##2956331872); if (cmp.gtu(r27,r2.new)) jump:nt -0x30 }
{ r2 = +mpyi(r27,#12); r24 = #0; r1 = #0; r0 = memw(r29+#24) }
{ call 0x2b8 }
{ p0 = cmp.eq(r27,#0); if (p0.new) jump:nt fail_exit }
{ r21 = #4; memw(r29+#8) = r16 }
{ p0 = cmp.gtu(r21,r18); if (p0.new) jump:nt fail_exit }
{ r17 = #0; r22 = #0; memw(r29+#12) = r26; memw(r29+#20) = r20 }
// Carry decoded records and local state forward to the caller's output area.
{ r1:0 = combine(r26,r20); r2 = r22; r3 = add(r29,#32); memw(r29+#28) = r27 }
{ r5:4 = combine(#0,#0); r16 = memw(r19+#0) }
{ memd(r29+#48) = r5:4; memd(r29+#40) = r5:4 }
{ call -0x148; memd(r29+#32) = r5:4 }
{ r25:24 = bitsplit(r16,#16); p0 = cmp.eq(r0,#0); if (!p0.new) jump:nt 0x250 }
{ r2 = memw(r29+#36); if (!cmp.eq(r2.new,r25)) jump:nt 0x248 }
{ r2 = lsr(r16,#20) }
{ p0 = cmp.eq(r2,#1); if (p0.new) r23 = add(r24,#0); if (!p0.new) r21 = add(r17,#0); if (!p0.new) r23 = add(r24,#4) }
{ r27 = add(r21,r23); if (cmp.gtu(r27.new,r18)) jump:nt 0x22c }
{ if (p0) r19 = add(r19,#4); r2 = memw(r29+#44) }
{ if (!p1.new) r2 = add(r25,#-16); p1 = cmp.eq(r2,#0); if (p1.new) jump:nt 0x70 }
{ r3 = #2; if (cmp.gtu(r3.new,r2)) jump:nt 0x48 }
{ p0 = cmp.eq(r25,#19); if (p0.new) jump:nt 0x40 }
{ p0 = cmp.eq(r25,#18); if (!p0.new) jump:nt 0x204 }
{ r16 = #0; p0 = cmp.eq(r23,#0); if (p0.new) jump:nt 0x1a0 }
{ r0 = r23; r2 = memuh(r19+#0); r17 = memw(r29+#40) }
{ r20 = add(r2,sub(#4,r17)); r2 = p1; r1 = add(r2,#4); memw(r29+#16) = r2.new }
{ call 0x208 }

// Later handler/consumer code is deliberately unimplemented.
