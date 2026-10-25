// SPDX-License-Identifier: GPL-3.0-only
// Hexagon WLAN packed-record consumer, verified function ENTRY ONLY.
// The true control flow continues after this prefix; the last instruction
// in this source is not a function return. The caller passes a configuration
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
