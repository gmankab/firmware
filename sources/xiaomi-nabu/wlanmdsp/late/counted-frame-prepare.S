// SPDX-License-Identifier: GPL-3.0-only
// Agent 14 WLAN file [0x102aa8,0x102cd0), runtime
// [0xb00c2aa8,0xb00c2cd0); preserves Hexagon V66 packet boundaries.
// Interpret a counted frame descriptor, traverse optional message fields,
// normalize byte lengths and dispatch frame status. Linked functions and
// request types are inferred from call sites, not proven original names.
// An explicit ## jump extender and the r21/r2 packet order are required for
// exact firmware encoding. No raw proprietary instruction words are emitted.
.text
.globl agent14_late_counted_frame_prepare
agent14_late_counted_frame_prepare:
// +0x0000: 2 encoded words
{ call 1977492; allocframe(#0x50) }
// +0x0008: 4 encoded words
{ r23:22 = combine(r3,r1); r17:16 = combine(r0,r5); r24 = r4; r21 = r2 }
// +0x0018: 2 encoded words
{ r0 = #-0x1; p0 = cmp.eq(r23,#0x0); if (p0.new) jump:nt label_01f8 }
// +0x0020: 2 encoded words
{ r18 = memw(r17+#0xc); if (cmp.eq(r18.new,#0x0)) jump:nt label_01f8 }
// +0x0028: 2 encoded words
{ r0 = memw(r17+#0x28); r1 = memub(r18+#0x69) }
// +0x0030: 3 encoded words
{ call label_020c; r19 = memub(##0xb035bdb0) }
// +0x003c: 2 encoded words
{ r20 = r0; if (cmp.eq(r20.new,#0x0)) jump:nt label_0088 }
// +0x0044: 3 encoded words
{ r2 = or(r24,#0x88); r25 = memw(r20+#0x10); r24 = memw(r29+#0x58) }
// +0x0050: 2 encoded words
{ memb(r25+#0x1b) = #0x0; memb(r25+#0x1a) = #0x0 }
// +0x0058: 2 encoded words
{ r3 = memw(r17+#0xc); memw(r25+#0xc) = r3.new }
// +0x0060: 2 encoded words
{ memb(r25+#0x1) = #0x0; memb(r25+#0x4) = #0x0 }
// +0x0068: 1 encoded words
{ memw(r25+#0x8) = r2 }
// +0x006c: 2 encoded words
{ r2 = memw(r17+#0xc); if (cmp.eq(r2.new,#0x0)) jump:nt label_0090 }
// +0x0074: 1 encoded words
{ r2 = memub(r2+#0xc) }
// +0x0078: 2 encoded words
{ p0 = tstbit(r2,#0x1); r2 = r19 }
// +0x0080: 2 encoded words
{ if (!p0) r2 = #0x0; jump label_0094 }
label_0088:
// +0x0088: 2 encoded words
{ r0 = #0x2 ; jump ##1977576 }
label_0090:
// +0x0090: 1 encoded words
{ r2 = #0x0 }
label_0094:
// +0x0094: 4 encoded words
{ r3 = add(r29,#0x0); r5:4 = combine(#0x0,#0x0); r1 = r23; memb(r25+#0x6) = r2 }
// +0x00a4: 3 encoded words
{ r2 = #0x6; r0 = add(r3,#0x18); memd(r29+#0x0) = r5:4; 	memd(r29+#0x8) = r5:4 }
// +0x00b0: 1 encoded words
{ memd(r29+#0x10) = r5:4; 	memw(r3+#0x24) = #1 }
// +0x00b4: 2 encoded words
{ call -789036; memw(r29+#0x4) = r21; 	memw(r29+#0x8) = r22 }
// +0x00bc: 3 encoded words
{ r0 = add(r29,#0x0); r2 = memuh(r17+#0x8); memh(r29+#0x22) = r2.new }
// +0x00c8: 2 encoded words
{ r2 = memuh(r17+#0x6); memh(r29+#0x20) = r2.new }
// +0x00d0: 2 encoded words
{ r2 = memuh(r17+#0x4); memh(r29+#0x1e) = r2.new }
// +0x00d8: 2 encoded words
{ r2 = memub(r25+#0x1f); r3 = memub(r25+#0x1e) }
// +0x00e0: 3 encoded words
{ r3 |= asl(r2,#0x8); r4 = memub(r25+#0x1d); r1 = memub(r25+#0x1c) }
// +0x00ec: 3 encoded words
{ r1 |= asl(r4,#0x8); r4 = memw(##0xb034572c) }
// +0x00f8: 2 encoded words
{ r1 |= asl(r3,#0x10); r3:2 = combine(r24,#0x0) }
// +0x0100: 1 encoded words
{ callr r4 }
// +0x0104: 3 encoded words
{ r5 = add(r25,#0x20); r2 = memub(r25+#0x1f); r3 = memub(r25+#0x1e) }
// +0x0110: 3 encoded words
{ r3 |= asl(r2,#0x8); r4 = memub(r25+#0x1d); r22 = memub(r25+#0x1c) }
// +0x011c: 3 encoded words
{ r22 |= asl(r4,#0x8); r2 = memub(r25+#0x1b); r1 = memub(r25+#0x1a) }
// +0x0128: 2 encoded words
{ r22 |= asl(r3,#0x10); r1 |= asl(r2,#0x8) }
// +0x0130: 2 encoded words
{ r2 = add(r22,#-0xe); r3 = add(r1,r0) }
// +0x0138: 4 encoded words
{ r4 = lsr(r3,#0x8); p0 = !cmp.gtu(r5,r2); if (p0.new) r21 = add(r17,#0x4); memb(r25+#0x1a) = r3 }
// +0x0148: 2 encoded words
{ if (!p0) jump:nt label_01fc; memb(r25+#0x1b) = r4 }
// +0x0150: 3 encoded words
{ r5 = lsr(r2,#0x18); r6 = lsr(r2,#0x10); r3 = add(r3,#0xe); 	r4 = memw(r29+#0x5c) }
// +0x015c: 4 encoded words
{ r0 = lsr(r2,#0x8); p0 = cmp.eq(r4,#0x0); memb(r25+#0x1a) = r3; memb(r25+#0x1c) = r2 }
// +0x016c: 3 encoded words
{ r3 = lsr(r3,#0x8); memb(r25+#0x1f) = r5; memb(r25+#0x1e) = r6 }
// +0x0178: 2 encoded words
{ memb(r25+#0x1d) = r0; memb(r25+#0x1b) = r3 }
// +0x0180: 1 encoded words
{ if (p0) jump:nt label_0198 }
// +0x0184: 4 encoded words
{ call -789636; r1:0 = combine(##0xff,r2); r2 = #0x6 }
// +0x0194: 1 encoded words
{ jump label_01d0 }
label_0198:
// +0x0198: 2 encoded words
{ r2 = memw(r18+#0x0); memb(r22+#-0xe) = r2.new }
// +0x01a0: 1 encoded words
{ r2 = memw(r18+#0x0) }
// +0x01a4: 2 encoded words
{ r2 = lsr(r2,#0x8); memb(r22+#-0xd) = r2.new }
// +0x01ac: 2 encoded words
{ r2 = memuh(r18+#0x2); memb(r22+#-0xc) = r2.new }
// +0x01b4: 2 encoded words
{ r2 = memub(r18+#0x3); memb(r22+#-0xb) = r2.new }
// +0x01bc: 2 encoded words
{ r2 = memw(r18+#0x4); memb(r22+#-0xa) = r2.new }
// +0x01c4: 1 encoded words
{ r2 = memw(r18+#0x4) }
// +0x01c8: 2 encoded words
{ r2 = lsr(r2,#0x8); memb(r22+#-0x9) = r2.new }
label_01d0:
// +0x01d0: 3 encoded words
{ r0 = add(r22,#-0x8); r21 = #0x6; r2 = #0x6; r1 = r21 }
// +0x01dc: 1 encoded words
{ call -789332 }
// +0x01e0: 3 encoded words
{ r1:0 = combine(r18,r17); r3:2 = combine(r20,r19); r5 = #0x8; 	r4 = r16 }
// +0x01ec: 3 encoded words
{ call 60; memb(r22+#-0x1) = r21; memb(r22+#-0x2) = r5 }
label_01f8:
// +0x01f8: 1 encoded words
{ jump 1977208 }
label_01fc:
// +0x01fc: 4 encoded words
{ call label_0218; r0 = ##-0x4fd48db2; r1 = #0x18c }
label_020c:
// +0x020c: 3 encoded words
{ r3:2 = combine(r1,#0x0); r1 = #0x2000; jump 1403276 }
label_0218:
// +0x0218: 1 encoded words
{ r3:2 = combine(r0,r1) }
// +0x021c: 3 encoded words
{ r1:0 = combine(r3,##-0x4fd48dc8); jump 1417984 }
