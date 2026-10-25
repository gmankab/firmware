// SPDX-License-Identifier: GPL-3.0-only
// WLAN Hexagon file [0x103110,0x1033c4), runtime
// [0xb00c3110,0xb00c33c4), a complete 692-byte packet routine.
// Parse counted controller frame, inspect record signature and encode status
// response paths; length checks, byte fields and branch outcomes are
// reconstructed from instruction dataflow, not an original API contract.
//
// Two explicitly named 3-word source packet fixtures at offsets 0x4c and
// 0x14c preserve the original Hexagon duplex instruction grouping. LLVM
// shuffles equivalent operands into a different packet even when lengths
// match. The builder independently assembles the typed miniature packets,
// checks the 11/11/00 parse pattern and joins only packet-boundary bits.
// The branch at +0x2ac uses an explicit constant extender (jump ##1975360)
// to preserve the full 692-byte function. No OEM opcode arrays are embedded.
.text
.globl agent14_late_controller_frame_command
agent14_late_controller_frame_command:
// +0x0000: 4 encoded words
{ call -1104; r0 = ##-0x4fd48d93; r1 = #0x22a }
// +0x0010: 2 encoded words
{ call 1975808; allocframe(#0x80) }
// +0x0018: 3 encoded words
{ r5 = #0x40; r17:16 = combine(r3,#0x0); r2 = memub(r4+#0x2b) }
// +0x0024: 1 encoded words
{ r21 = sub(r1,r2) }
// +0x0028: 3 encoded words
{ r6 = and(r21,##0xffc0); if (cmp.gtu(r5,r6.new)) jump:t label_02ac }
// +0x0034: 1 encoded words
{ r3 = memub(r0+r2<<#0x0) }
// +0x0038: 1 encoded words
{ r3 = and(r3,#0xf0) }
// +0x003c: 4 encoded words
{ p0 = cmp.eq(r3,#0x60); if (p0.new) r19 = add(r21,##0xffd8); if (!p0.new) jump:t label_02ac }
// +0x004c: 3 encoded words
{ r20 = add(r0,r2); r22 = add(r29,#0x10); r2 = #0x10; 	r5:4 = combine(#0,#0x0) }
// +0x0058: 4 encoded words
{ r18 = add(r20,#0x18); r0 = add(r22,#0x10); r3 = memub(r20+#0x13); memd(r29+#0x48) = r5:4 }
// +0x0068: 2 encoded words
{ r1 = r18; memd(r29+#0x38) = r5:4; 	memd(r29+#0x40) = r5:4 }
// +0x0070: 2 encoded words
{ r5 = memub(r20+#0x12); memb(r22+#0x24) = #0x0 }
// +0x0078: 3 encoded words
{ r5 |= asl(r3,#0x8); r6 = memub(r20+#0x11); r8 = memub(r20+#0xb) }
// +0x0084: 2 encoded words
{ r4 = memub(r20+#0x10); memb(r22+#0x23) = #0x0 }
// +0x008c: 3 encoded words
{ r4 |= asl(r6,#0x8); r3 = memub(r20+#0xa); r7 = memub(r20+#0x17) }
// +0x0098: 4 encoded words
{ r3 |= asl(r8,#0x8); r4 |= asl(r5,#0x10); r6 = memub(r20+#0x9); r9 = memub(r20+#0x16) }
// +0x00a8: 3 encoded words
{ r9 |= asl(r7,#0x8); r12 = memub(r20+#0x8); r27 = memub(r20+#0x15) }
// +0x00b4: 3 encoded words
{ r12 |= asl(r6,#0x8); r5 = memub(r20+#0x14); memb(r22+#0x22) = #0x0 }
// +0x00c0: 3 encoded words
{ r12 |= asl(r3,#0x10); r5 |= asl(r27,#0x8); r3 = memub(r20+#0xf); 	r6 = memub(r20+#0xe) }
// +0x00cc: 3 encoded words
{ r5 |= asl(r9,#0x10); r6 |= asl(r3,#0x8); r7 = memub(r20+#0xd) }
// +0x00d8: 2 encoded words
{ r13 = memub(r20+#0xc); memd(r29+#0x18) = r5:4 }
// +0x00e0: 1 encoded words
{ r13 |= asl(r7,#0x8) }
// +0x00e4: 1 encoded words
{ r13 |= asl(r6,#0x10) }
// +0x00e8: 2 encoded words
{ call -790728; memd(r29+#0x10) = r13:12 }
// +0x00f0: 3 encoded words
{ r19 = zxth(r19); r2 = memub(r20+#0x5); memb(r22+#0x25) = #0x3a }
// +0x00fc: 1 encoded words
{ r3 = memub(r20+#0x4) }
// +0x0100: 2 encoded words
{ r3 |= asl(r2,#0x8); memh(r29+#0x30) = r3.new }
// +0x0108: 1 encoded words
{ r2 = swiz(r3) }
// +0x010c: 2 encoded words
{ r24 = lsr(r2,#0x10); if (!cmp.gtu(r24.new,r19)) jump:t label_014c }
// +0x0114: 4 encoded words
{ r0 = ##0x65f8; r1 = #0x3e52; memw(r29+#0x4) = r24 }
// +0x0124: 2 encoded words
{ call 1408896; memw(r29+#0x0) = r19 }
// +0x012c: 3 encoded words
{ r1:0 = combine(r19,##-0x4fd48c78); r2 = memub(r20+#0x4); 	r3 = memub(r20+#0x5) }
// +0x0138: 1 encoded words
{ r2 |= asl(r3,#0x8) }
// +0x013c: 1 encoded words
{ r2 = swiz(r2) }
// +0x0140: 2 encoded words
{ call -790320; r2 = lsr(r2,#0x10) }
// +0x0148: 1 encoded words
{ jump label_02ac }
label_014c:
// +0x014c: 3 encoded words
{ r23 = add(r20,#0x28); r0 = add(r29,#0x38); r2 = add(r29,#0x38); 	memw(r29+#0x44) = r22 }
// +0x0158: 3 encoded words
{ r3 = add(r2,#0xc); r25 = memub(r23+#0x2); memw(r29+#0x38) = r23 }
// +0x0164: 3 encoded words
{ r22 = r25; r26 = memb(r23+#0x3); memw(r29+#0x3c) = r24 }
// +0x0170: 3 encoded words
{ r22 |= asl(r26,#0x8); memw(r29+#0x40) = r3; memw(r2+#0x10) = #0x26 }
// +0x017c: 2 encoded words
{ call -1032; memb(r23+#0x3) = #0; 	memb(r23+#0x2) = #0 }
// +0x0184: 3 encoded words
{ r19 = r0; memb(r23+#0x3) = r26; memb(r23+#0x2) = r25 }
// +0x0190: 2 encoded words
{ r2 = sxth(r19); if (!cmp.eq(r2.new,r22)) jump:t label_0214 }
// +0x0198: 3 encoded words
{ r1 = add(r23,#0x8); r19 = add(r20,#0x8); r2 = #0x10; 	r0 = r17 }
// +0x01a4: 1 encoded words
{ call -790916 }
// +0x01a8: 3 encoded words
{ call -790920; r0 = add(r17,#0x10); r2 = #0x10; 	r1 = r18 }
// +0x01b4: 3 encoded words
{ call -790932; r0 = add(r17,#0x20); r2 = #0x10; 	r1 = r19 }
// +0x01c0: 4 encoded words
{ r2 = add(r21,##0x1ffc0); r3 = ##0xffff }
// +0x01d0: 3 encoded words
{ p0 = bitsclr(r2,r3); if (!p0.new) r2 = sxth(r2); if (p0.new) jump:nt label_0270 }
// +0x01dc: 2 encoded words
{ r3 = #0x8; if (cmp.gtu(r3.new,r2)) jump:t label_02ac }
// +0x01e4: 1 encoded words
{ r3 = add(r23,#0x4) }
// +0x01e8: 1 encoded words
{ r3 = add(r3,#0x14) }
label_01ec:
// +0x01ec: 2 encoded words
{ r4 = memb(r3+#0x0); if (cmp.eq(r4.new,#0x1)) jump:nt label_0260 }
// +0x01f4: 2 encoded words
{ r4 = memub(r3+#0x1); if (cmp.eq(r4.new,#0x0)) jump:nt label_0274 }
// +0x01fc: 2 encoded words
{ r4 = asl(r4,#0x3); if (cmp.gt(r4.new,r2)) jump:nt label_0290 }
// +0x0204: 2 encoded words
{ r2 = sub(r2.l,r4.l); r3 = add(r3,r4) }
// +0x020c: 1 encoded words
{ p0 = cmp.gtu(r2,#0x7); if (p0.new) jump:t label_01ec }
// +0x0210: 1 encoded words
{ jump label_02ac }
label_0214:
// +0x0214: 3 encoded words
{ r1 = #0x3e13; r0 = ##0x6600; 	r2 = zxth(r22) }
// +0x0220: 2 encoded words
{ memw(r29+#0x8) = r24; memw(r29+#0x4) = r2 }
// +0x0228: 2 encoded words
{ call 1408636; memw(r29+#0x0) = r19 }
// +0x0230: 4 encoded words
{ r1:0 = combine(r19,##-0x4fd48c6c); r2 = memub(r20+#0x5); r4 = memub(r20+#0x2b) }
// +0x0240: 1 encoded words
{ r3 = memub(r20+#0x4) }
// +0x0244: 2 encoded words
{ r3 |= asl(r2,#0x8); r2 = memub(r20+#0x2a) }
// +0x024c: 2 encoded words
{ r2 |= asl(r4,#0x8); r3 = swiz(r3) }
// +0x0254: 2 encoded words
{ call -790096; r3 = lsr(r3,#0x10) }
// +0x025c: 1 encoded words
{ jump label_02ac }
label_0260:
// +0x0260: 4 encoded words
{ call -791104; r1 = add(r3,#0x2); r0 = add(r17,#0x36); r2 = #0x6 }
label_0270:
// +0x0270: 1 encoded words
{ r16 = #0x1 ; jump label_02ac }
label_0274:
// +0x0274: 4 encoded words
{ call 1408560; r0 = ##0x6608; r1 = #0x3e40 }
// +0x0284: 3 encoded words
{ r0 = ##-0x4fd48c60; jump label_02a8 }
label_0290:
// +0x0290: 4 encoded words
{ call 1408532; r0 = ##0x6610; r1 = #0x3e50 }
// +0x02a0: 2 encoded words
{ r0 = ##-0x4fd48c54 }
label_02a8:
// +0x02a8: 1 encoded words
{ call -790688 }
label_02ac:
// +0x02ac: 2 encoded words
{ r0 = r16 ; jump ##1975360 }
