// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14 file [0x102820,0x1029c4), runtime [0xb00c2820,0xb00c29c4).
// Inspect a WLAN record's 16/32-bit type and length fields, validate controller
// callback responses, and commit an active descriptor or return to the
// retry/error loop. Branches and memory packets preserve their original
// predicate/data dependencies and an explicit far-jump constant extender.
.text
.globl agent14_late_frame_status_parse
agent14_late_frame_status_parse:
// +0x0000: 2 encoded words
{ call 1978164; allocframe(#0x30) }
// +0x0008: 3 encoded words
{ r17 = r3; r19:18 = combine(r4,r0); r0 = memw(gp+#0x78c) }
// +0x0014: 2 encoded words
{ call -533940; r1 = memub(r17+#0x0) }
// +0x001c: 1 encoded words
{ r21 = r0; 	r2 = memub(r19+#0x4) }
// +0x0020: 1 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_0034 }
// +0x0024: 1 encoded words
{ r16 = #0x0; 	r2 = memh(r19+#0x8) }
// +0x0028: 3 encoded words
{ p0 = cmp.eq(r2,##0x806); if (!p0.new) jump:t label_0068 }
label_0034:
// +0x0034: 2 encoded words
{ r16 = #0x0; r2 = memub(r19+#0x2a) }
// +0x003c: 2 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_0068; if (p0.new) r2 = memub(r19+#0x2b) }
// +0x0044: 2 encoded words
{ r18 = add(r18,r2); if (cmp.eq(r18.new,#0x0)) jump:nt label_0068 }
// +0x004c: 4 encoded words
{ call -782064; r1:0 = combine(##-0x4fd48de4,r18); r2 = #0x6 }
// +0x005c: 3 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0070; if (p0.new) r23 = memub(r18+#0x18); if (p0.new) r20 = memub(r18+#0xe) }
label_0068:
// +0x0068: 2 encoded words
{ r0 = r16 ; jump ##1978280 }
label_0070:
// +0x0070: 3 encoded words
{ r19 = r23; r1 = memub(r18+#0xf); r2 = memub(r18+#0x19) }
// +0x007c: 4 encoded words
{ r20 = insert(r1,#0x18,#0x8); r19 = insert(r2,#0x18,#0x8); r0 = memub(r18+#0x10); r22 = memub(r18+#0x6) }
// +0x008c: 3 encoded words
{ r20 |= asl(r0,#0x10); r3 = memub(r18+#0x1a); r1 = memub(r18+#0x11) }
// +0x0098: 4 encoded words
{ r20 |= asl(r1,#0x18); r19 |= asl(r3,#0x10); r0 = memub(r18+#0x7); r4 = memub(r18+#0x1b) }
// +0x00a8: 4 encoded words
{ r22 = insert(r0,#0x18,#0x8); r0 = ##0x65f0; r1 = #0x3e13 }
// +0x00b8: 2 encoded words
{ r19 |= asl(r4,#0x18); memw(r29+#0x8) = r22; 	memw(r29+#0x4) = r20 }
// +0x00c0: 2 encoded words
{ call 1411284; memw(r29+#0x0) = r19 }
// +0x00c8: 4 encoded words
{ call -787412; r1:0 = combine(r19,##-0x4fd48ddc); r3:2 = combine(r22,r20) }
// +0x00d8: 2 encoded words
{ r2 = #0x200; if (cmp.eq(r2.new,r22)) jump:nt label_0130 }
// +0x00e0: 4 encoded words
{ r2 = #0x6; p0 = cmp.eq(r22,#0x100); if (p0.new) r22 = add(r18,#0x8); if (!p0.new) jump:nt label_0068 }
// +0x00f0: 3 encoded words
{ call -782228; r1:0 = combine(##-0x4fd4fea2,r22) }
// +0x00fc: 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0068 }
// +0x0100: 4 encoded words
{ call -782244; r1:0 = combine(##-0x4fd4fea8,r22); r2 = #0x6 }
// +0x0110: 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0068 }
// +0x0114: 3 encoded words
{ p0 = cmp.gtu(r23,#0xdf); if (p0.new) jump:nt label_0068; if (!p0.new) r2 = memub(r22+#0x0) }
// +0x0120: 1 encoded words
{ r2 = and(r2,#0x1) }
// +0x0124: 2 encoded words
{ r2 = #0x1; p0 = cmp.eq(r2,#0x0); if (p0.new) jump:t label_017c }
// +0x012c: 1 encoded words
{ jump label_0068 }
label_0130:
// +0x0130: 2 encoded words
{ r22 = add(r18,#0x8); r2 = #0x6 }
// +0x0138: 3 encoded words
{ call -782300; r1:0 = combine(##-0x4fd4fea2,r22) }
// +0x0144: 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0068 }
// +0x0148: 4 encoded words
{ call -782316; r1:0 = combine(##-0x4fd4fea8,r22); r2 = #0x6 }
// +0x0158: 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0068 }
// +0x015c: 2 encoded words
{ p0 = cmp.eq(r19,#0x0); if (p0.new) jump:nt label_0068; if (!p0.new) r2 = memub(r22+#0x0) }
// +0x0164: 2 encoded words
{ r2 = and(r2,#0x1); if (!cmp.eq(r2.new,#0x0)) jump:nt label_0068 }
// +0x016c: 1 encoded words
{ r2 = memw(r21+#0x60c) }
// +0x0170: 3 encoded words
{ p0 = !cmp.eq(r19,r2); r2 = #0x2; if (p0.new) jump:t label_0068 }
label_017c:
// +0x017c: 3 encoded words
{ r0 = add(r17,#0x18); r1 = r22; r2 = #0x6; memw(r17+#0x24) = r2 }
// +0x0188: 2 encoded words
{ call -788600; memw(r17+#0x4) = r20; 	memw(r17+#0x8) = r19 }
// +0x0190: 4 encoded words
{ call -788608; r1 = add(r18,#0x12); r0 = add(r17,#0x1e); r2 = #0x6 }
// +0x01a0: 1 encoded words
{ r16 = #0x1 ; jump label_0068 }
