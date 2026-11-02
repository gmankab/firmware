// SPDX-License-Identifier: GPL-3.0-only
// Agent 14 WLAN file [0x102f0c,0x103110), runtime
// [0xb00c2f0c,0xb00c3110).
// Build a counted interface status message using packed protocol fields,
// validate data lengths, call an external checksum helper and dispatch
// either a normal response or a tagged controller error. Inferred semantics
// follow observable register writes; external helper names are not proven.
// An explicit Hexagon constant extender preserves the original far error jump.
// No OEM bytes or non-instruction padding are embedded here.
.text
.globl agent14_late_interface_status_construct
agent14_late_interface_status_construct:
// +0x0000: 2 encoded words
{ call 1976368; allocframe(#0x68) }
// +0x0008: 4 encoded words
{ r17:16 = combine(r0,r5); r24 = r4; r23:22 = combine(r1,r2); r0 = #-0x1 }
// +0x0018: 2 encoded words
{ r21 = r3; if (cmp.eq(r21.new,#0x0)) jump:nt label_0200 }
// +0x0020: 2 encoded words
{ r18 = memw(r17+#0xc); if (cmp.eq(r18.new,#0x0)) jump:nt label_0200 }
// +0x0028: 2 encoded words
{ r0 = memw(r17+#0x28); r1 = memub(r18+#0x69) }
// +0x0030: 3 encoded words
{ call -648; r19 = memub(##0xb035bdb0) }
// +0x003c: 2 encoded words
{ r20 = r0; if (cmp.eq(r20.new,#0x0)) jump:nt label_0084 }
// +0x0044: 2 encoded words
{ r2 = or(r24,#0x88); r25 = memw(r20+#0x10) }
// +0x004c: 2 encoded words
{ memb(r25+#0x1b) = #0x0; memb(r25+#0x1a) = #0x0 }
// +0x0054: 2 encoded words
{ r3 = memw(r17+#0xc); memw(r25+#0xc) = r3.new }
// +0x005c: 2 encoded words
{ memb(r25+#0x1) = #0x0; memb(r25+#0x4) = #0x0 }
// +0x0064: 1 encoded words
{ memw(r25+#0x8) = r2 }
// +0x0068: 3 encoded words
{ r3 = #0x0; r2 = memw(r17+#0xc); if (cmp.eq(r2.new,#0x0)) jump:nt label_008c }
// +0x0074: 1 encoded words
{ r3 = r19; 	r2 = memub(r2+#0xc) }
// +0x0078: 3 encoded words
{ p0 = tstbit(r2,#0x1); if (!p0.new) r3 = #0x0; jump label_008c }
label_0084:
// +0x0084: 2 encoded words
{ r0 = #0x2 ; jump ##1976456 }
label_008c:
// +0x008c: 4 encoded words
{ r24 = add(r29,#0x0); r1 = #0x0; r2 = #0x10; memb(r25+#0x6) = r3 }
// +0x009c: 2 encoded words
{ call -790528; r0 = add(r24,#0x10) }
// +0x00a4: 1 encoded words
{ r2 = memub(r23+#0xb); 	r6 = memub(r23+#0x3) }
// +0x00a8: 1 encoded words
{ r3 = memub(r23+#0xa); 	r7 = memub(r23+#0x2) }
// +0x00ac: 3 encoded words
{ r3 |= asl(r2,#0x8); r7 |= asl(r6,#0x8); r5 = memub(r23+#0x9); 	r4 = memub(r23+#0x8) }
// +0x00b8: 2 encoded words
{ r4 |= asl(r5,#0x8); r5 = memub(r23+#0x1); 	r2 = memub(r23+#0xf) }
// +0x00c0: 3 encoded words
{ r4 |= asl(r3,#0x10); r8 = memub(r23+#0x0); r3 = memub(r23+#0xe) }
// +0x00cc: 3 encoded words
{ r8 |= asl(r5,#0x8); r3 |= asl(r2,#0x8); r0 = memub(r23+#0x7); 	r1 = memub(r23+#0xd) }
// +0x00d8: 3 encoded words
{ r8 |= asl(r7,#0x10); r12 = memub(r23+#0x6); r5 = memub(r23+#0xc) }
// +0x00e4: 4 encoded words
{ r5 |= asl(r1,#0x8); r12 |= asl(r0,#0x8); r0 = add(r24,#0x20); r9 = memub(r23+#0x4) }
// +0x00f4: 2 encoded words
{ r5 |= asl(r3,#0x10); r1 = r22; 	r2 = memub(r23+#0x5) }
// +0x00fc: 2 encoded words
{ r9 |= asl(r2,#0x8); r2 = #0x10; 	memd(r29+#0x8) = r5:4 }
// +0x0104: 1 encoded words
{ r9 |= asl(r12,#0x10) }
// +0x0108: 3 encoded words
{ call -790244; r22 = add(r17,#0x4); memd(r29+#0x0) = r9:8 }
// +0x0114: 3 encoded words
{ call -790256; r0 = add(r24,#0x36); r2 = #0x6; 	r1 = r22 }
// +0x0120: 3 encoded words
{ call -790268; r0 = add(r24,#0x30); r2 = #0x6; 	r1 = r21 }
// +0x012c: 3 encoded words
{ r0 = add(r29,#0x0); r2 = memub(r25+#0x1f); r3 = memub(r25+#0x1e) }
// +0x0138: 4 encoded words
{ r3 |= asl(r2,#0x8); r2 = #0x0; r4 = memub(r25+#0x1d); r1 = memub(r25+#0x1c) }
// +0x0148: 1 encoded words
{ r1 |= asl(r4,#0x8) }
// +0x014c: 2 encoded words
{ r1 |= asl(r3,#0x10); call -884 }
// +0x0154: 3 encoded words
{ r5 = add(r25,#0x20); r2 = memub(r25+#0x1f); r3 = memub(r25+#0x1e) }
// +0x0160: 3 encoded words
{ r3 |= asl(r2,#0x8); r4 = memub(r25+#0x1d); r23 = memub(r25+#0x1c) }
// +0x016c: 3 encoded words
{ r23 |= asl(r4,#0x8); r2 = memub(r25+#0x1b); r1 = memub(r25+#0x1a) }
// +0x0178: 2 encoded words
{ r23 |= asl(r3,#0x10); r1 |= asl(r2,#0x8) }
// +0x0180: 2 encoded words
{ r2 = add(r23,#-0xe); r3 = add(r1,r0) }
// +0x0188: 4 encoded words
{ r4 = lsr(r3,#0x8); p0 = !cmp.gtu(r5,r2); memb(r25+#0x1a) = r3; if (p0.new) memb(r25+#0x1c) = r2 }
// +0x0198: 2 encoded words
{ if (!p0) jump:nt 108; memb(r25+#0x1b) = r4 }
// +0x01a0: 3 encoded words
{ r4 = lsr(r2,#0x18); r7 = lsr(r2,#0x10); r3 = add(r3,#0xe) }
// +0x01ac: 4 encoded words
{ r6 = lsr(r2,#0x8); r1:0 = combine(r21,r2); r2 = #0x6; memb(r25+#0x1a) = r3 }
// +0x01bc: 3 encoded words
{ r3 = lsr(r3,#0x8); memb(r25+#0x1f) = r4; memb(r25+#0x1e) = r7 }
// +0x01c8: 3 encoded words
{ call -790436; memb(r25+#0x1d) = r6; memb(r25+#0x1b) = r3 }
// +0x01d4: 3 encoded words
{ call -790448; r0 = add(r23,#-0x8); r2 = #0x6; 	r1 = r22 }
// +0x01e0: 4 encoded words
{ r1:0 = combine(r18,r17); r3:2 = combine(r20,r19); r4 = r16; r5 = #0xdd }
// +0x01f0: 2 encoded words
{ r6 = #0x86; memb(r23+#-0x1) = r5 }
// +0x01f8: 2 encoded words
{ call -1076; memb(r23+#-0x2) = r6 }
label_0200:
// +0x0200: 1 encoded words
{ jump 1976076 }
