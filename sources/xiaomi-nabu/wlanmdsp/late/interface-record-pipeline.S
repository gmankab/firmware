// SPDX-License-Identifier: GPL-3.0-only
// WLAN runtime file [0x103660,0x103b4c), VMA [0xb00c3660,0xb00c3b4c).
// Process active interface record, check bounds, copy fields and commit or reject state changes
// Typed Hexagon instruction mnemonics, packet layout and local labels;
// external helper names are inferred from observable call sites, not
// independent C-level source or hardware execution evidence.
// No OEM opcode arrays or vendor firmware payload are embedded.
.text
.globl agent14_late_interface_record_pipeline
agent14_late_interface_record_pipeline:
// +0x0000: 2 encoded words
{ call 1974492; allocframe(#0x68) }
// +0x0008: 3 encoded words
{ call -537576; r21:20 = combine(r5,r4); r18 = r1; 	r17 = r0 }
// +0x0014: 4 encoded words
{ r2 = add(r29,#0x10); r16 = r0; r19 = #0x0; memw(r29+#0x10) = r16.new }
// +0x0024: 2 encoded words
{ memw(r2+#0x4) = ##-0x4ff3ca08 }
// +0x002c: 3 encoded words
{ r2 = memb(r16+#0x0); if (cmp.eq(r2.new,#0x2)) jump:t label_04c8 }
// +0x0038: 4 encoded words
{ r1 = #0x39a; r22 = memw(r16+#0x1b4); if (cmp.eq(r22.new,#0x0)) jump:nt label_04d0 }
// +0x0048: 3 encoded words
{ r19 = #0x0; r1 = #0x0; r2 = #0x28; r0 = add(r29,#0x18) }
// +0x0054: 1 encoded words
{ call -792332 }
// +0x0058: 1 encoded words
{ r2 = memub(r22+#0x3) }
// +0x005c: 2 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_04c8 }
// +0x0064: 3 encoded words
{ r2 = memb(r22+#0x2); if (cmp.eq(r2.new,#0x0)) jump:nt label_04c8 }
// +0x0070: 4 encoded words
{ r1 = zxth(r21); r4 = memw(r29+#0x70); r5 = memw(r23=##0xb0345728) }
// +0x0080: 3 encoded words
{ r19 = #0x0; r2 = #0x0; r3 = add(r29,#0x18); r0 = r20 }
// +0x008c: 1 encoded words
{ callr r5 }
// +0x0090: 2 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_04c8 }
// +0x0098: 1 encoded words
{ r0 = add(r29,#0x18); 	r2 = memw(r23+#0x2c) }
// +0x009c: 1 encoded words
{ callr r2 }
// +0x00a0: 2 encoded words
{ r2 = add(r16,#0x7bc); memw(r22+#0x10) += #0x1 }
// +0x00a8: 1 encoded words
{ memw(r2+#0x0) += #0x1 }
// +0x00ac: 3 encoded words
{ r2 = memw(r29+#0x3c); if (!cmp.eq(r2.new,#0x1)) jump:t label_04c8 }
// +0x00b8: 1 encoded words
{ r2 = memub(r22+#0x2) }
// +0x00bc: 3 encoded words
{ if (!p0.new) r24 = #0x0; if (!p0.new) r3 = add(r29,#0x18); p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt label_0118 }
// +0x00c8: 2 encoded words
{ loop0(0xc,r2); r19 = add(r22,#0x1f0) }
// +0x00d0: 1 encoded words
{ r2 = add(r3,#0x8) }
// +0x00d4: 2 encoded words
{ r3 = memub(r2+#0x2); r0 = memub(r19+#-0xa) }
// +0x00dc: 2 encoded words
{ r4 = memub(r2+#0x3); r7 = memub(r19+#-0x9) }
// +0x00e4: 4 encoded words
{ r3 |= asl(r4,#0x8); r0 |= asl(r7,#0x8); r5 = memub(r2+#0x0); r8 = memub(r19+#-0xb) }
// +0x00f4: 2 encoded words
{ r9 = memub(r19+#-0xc); r6 = memub(r2+#0x1) }
// +0x00fc: 2 encoded words
{ r5 |= asl(r6,#0x8); r9 |= asl(r8,#0x8) }
// +0x0104: 1 encoded words
{ r5 |= asl(r3,#0x10) }
// +0x0108: 2 encoded words
{ r9 |= asl(r0,#0x10); if (cmp.eq(r9.new,r5)) jump:nt label_0128 }
// +0x0110: 2 encoded words
{ r24 = add(r24,#0x1); r19 = add(r19,#0x18)  }:endloop0
label_0118:
// +0x0118: 2 encoded words
{ r2 = add(r16,#0x7c4); r19 = #0x1 }
// +0x0120: 2 encoded words
{ jump label_04c8; memw(r2+#0x0) += #0x1 }
label_0128:
// +0x0128: 4 encoded words
{ r2 = add(r16,#0x7c0); r0 = ##0x6628; memw(r22+#0x14) += #0x1 }
// +0x0138: 4 encoded words
{ r1 = #0x2412; r20 = ##-0x4fc9fef2; memw(r2+#0x0) += #0x1 }
// +0x0148: 2 encoded words
{ r2 = memub(r16+#0x3); memw(r29+#0x0) = r2.new }
// +0x0150: 3 encoded words
{ call 1407492; r3 = memw(r29+#0x1c); memw(r29+#0x4) = r3.new }
// +0x015c: 3 encoded words
{ r0 = ##-0x4fd48c30; r1 = memub(r16+#0x3); 	r2 = memw(r29+#0x1c) }
// +0x0168: 1 encoded words
{ call -791720 }
// +0x016c: 4 encoded words
{ r0 = ##0x6630; r1 = #0x2403; r2 = memuh(gp+#0x10e) }
// +0x017c: 3 encoded words
{ r2 = combine(r18.l,r2.l); r4 = ##0x890f800d; 	r3 = memw(r29+#0x1c) }
// +0x0188: 1 encoded words
{ memw(r29+#0x8) = r3; 	memw(r29+#0x4) = r2 }
// +0x018c: 2 encoded words
{ call 1407432; memw(r29+#0x0) = r4 }
// +0x0194: 4 encoded words
{ r0 = ##-0x4fd48c24; r2 = memuh(gp+#0x10e); r3 = memw(r29+#0x1c) }
// +0x01a4: 4 encoded words
{ call -791280; r1 = ##-0x76f07ff3; r2 = combine(r18.l,r2.l) }
// +0x01b4: 1 encoded words
{ memh(r20+#0x0) += #0x1 }
// +0x01b8: 3 encoded words
{ r2 = memb(r16+##0x786); if (cmp.eq(r2.new,#0x0)) jump:nt label_0234 }
// +0x01c4: 4 encoded words
{ call 1407376; r0 = ##0x6638; r1 = #0x2410 }
// +0x01d4: 3 encoded words
{ call -791836; r0 = ##-0x4fd48c18 }
// +0x01e0: 4 encoded words
{ r0 = ##0x6640; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
// +0x01f0: 3 encoded words
{ r1 = #0x2403; r4 = ##0x890f800e; 	r5 = memw(r29+#0x1c) }
// +0x01fc: 2 encoded words
{ r2 = combine(r2.l,r3.l); memw(r29+#0x8) = r5 }
// +0x0204: 2 encoded words
{ call 1407312; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r4 }
// +0x020c: 4 encoded words
{ r0 = ##-0x4fd48c0c; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
// +0x021c: 3 encoded words
{ r2 = combine(r2.l,r3.l); r1 = ##0x890f800e; 	r3 = memw(r29+#0x1c) }
// +0x0228: 1 encoded words
{ call -791412 }
// +0x022c: 2 encoded words
{ r19 = #0x1 ; jump label_04c8; memh(r20+#0x0) += #0x1 }
label_0234:
// +0x0234: 3 encoded words
{ call -3040; r0 = r17; r1 = memub(r16+#0x250) }
// +0x0240: 2 encoded words
{ r17 = r0; if (cmp.eq(r17.new,#0x0)) jump:nt label_02ac }
// +0x0248: 3 encoded words
{ p0 = cmp.gt(r24,#-0x1); if (p0.new) r18 = add(r16,#0x4); r21 = memw(r17+#0x10); 	r2 = memw(r16+#0xc) }
// +0x0254: 4 encoded words
{ r3 = ##0x1000088; if (p0) r4 = memub(r21+#0x1d); if (p0) r1 = memub(r21+#0x1c) }
// +0x0264: 2 encoded words
{ memb(r21+#0x6) = #0x0; memw(r21+#0xc) = r2 }
// +0x026c: 3 encoded words
{ if (!p0) jump:nt label_04e4; if (p0) r2 = memub(r21+#0x1f); memw(r21+#0x8) = r3 }
// +0x0278: 2 encoded words
{ r5 = memub(r19+#-0x10); r3 = memub(r21+#0x1e) }
// +0x0280: 2 encoded words
{ r3 |= asl(r2,#0x8); p0 = !tstbit(r5,#0x1) }
// +0x0288: 1 encoded words
{ r1 |= asl(r4,#0x8) }
// +0x028c: 2 encoded words
{ r1 |= asl(r3,#0x10); if (!p0) jump:nt label_02c4 }
// +0x0294: 2 encoded words
{ r2 = add(r29,#0x18); r3 = memuh(r18+#0x0); 	r4 = memuh(r18+#0x2) }
// +0x029c: 1 encoded words
{ r2 = add(r2,#0x1e); 	r5 = memuh(r18+#0x4) }
// +0x02a0: 1 encoded words
{ memh(r2+#0x4) = r5; 	memh(r2+#0x2) = r4 }
// +0x02a4: 2 encoded words
{ jump label_02f4; memh(r2+#0x0) = r3 }
label_02ac:
// +0x02ac: 1 encoded words
{ call 304980 }
// +0x02b0: 4 encoded words
{ call -4456; r19 = !cmp.eq(r0,#0x0); r1:0 = combine(#0x1,r16); r3:2 = combine(#0x1,#0x1) }
// +0x02c0: 1 encoded words
{ jump label_04c8 }
label_02c4:
// +0x02c4: 2 encoded words
{ r2 = memw(r19+#-0x4); memb(r29+#0x36) = r2.new }
// +0x02cc: 2 encoded words
{ r3 = lsr(r2,#0x8); memb(r29+#0x37) = r3.new }
// +0x02d4: 3 encoded words
{ r3 = lsr(r2,#0x10); r2 = lsr(r2,#0x18); memb(r29+#0x38) = r3.new }
// +0x02e0: 2 encoded words
{ memb(r29+#0x39) = r2; r2 = memw(r19+#0x0) }:mem_noshuf
// +0x02e8: 2 encoded words
{ r2 = lsr(r2,#0x8); memb(r29+#0x3a) = r2 }
// +0x02f0: 1 encoded words
{ memb(r29+#0x3b) = r2 }
label_02f4:
// +0x02f4: 3 encoded words
{ r0 = add(r29,#0x18); r3:2 = combine(#0x0,#0x0); r23 = add(r29,#0x18); r4 = memw(r23+#0x4) }
// +0x0300: 1 encoded words
{ callr r4 }
// +0x0304: 3 encoded words
{ r5 = add(r21,#0x20); r2 = memub(r21+#0x1f); r3 = memub(r21+#0x1e) }
// +0x0310: 3 encoded words
{ r3 |= asl(r2,#0x8); r4 = memub(r21+#0x1d); r19 = memub(r21+#0x1c) }
// +0x031c: 3 encoded words
{ r19 |= asl(r4,#0x8); r2 = memub(r21+#0x1b); r1 = memub(r21+#0x1a) }
// +0x0328: 2 encoded words
{ r19 |= asl(r3,#0x10); r1 |= asl(r2,#0x8) }
// +0x0330: 2 encoded words
{ r2 = add(r19,#-0xe); r3 = add(r1,r0) }
// +0x0338: 4 encoded words
{ r4 = lsr(r3,#0x8); p0 = !cmp.gtu(r5,r2); if (p0.new) r1 = add(r23,#0x18); memb(r21+#0x1a) = r3 }
// +0x0348: 3 encoded words
{ if (p0) r23 = #0x6; if (!p0) jump:nt label_04dc; memb(r21+#0x1b) = r4 }
// +0x0354: 3 encoded words
{ r4 = lsr(r2,#0x18); r7 = lsr(r2,#0x10); r3 = add(r3,#0xe) }
// +0x0360: 4 encoded words
{ r6 = lsr(r2,#0x8); r0 = r2; r2 = #0x6; memb(r21+#0x1c) = r2 }
// +0x0370: 3 encoded words
{ r3 = lsr(r3,#0x8); memb(r21+#0x1a) = r3; memb(r21+#0x1f) = r4 }
// +0x037c: 2 encoded words
{ memb(r21+#0x1e) = r7; memb(r21+#0x1d) = r6 }
// +0x0384: 2 encoded words
{ call -792756; memb(r21+#0x1b) = r3 }
// +0x038c: 3 encoded words
{ call -792764; r0 = add(r19,#-0x8); r2 = #0x6; 	r1 = r18 }
// +0x0398: 3 encoded words
{ r2 = #0x8; r4 = add(r29,#0x10); memb(r19+#-0x1) = r23 }
// +0x03a4: 4 encoded words
{ r0 = r16; r3 = r17; memb(r19+#-0x2) = r2; r1 = memw(r16+#0xc) }:mem_noshuf
// +0x03b4: 1 encoded words
{ r2 = memub(r1+#0xc) }
// +0x03b8: 3 encoded words
{ p0 = !tstbit(r2,#0x1); r2 = #0x10; if (!p0.new) memb(r21+#0x4) = #0x0 }
// +0x03c4: 4 encoded words
{ call -3412; if (!p0) r2 = memub(##0xb035bdb0); if (!p0) r1 = memw(r16+#0xc) }
// +0x03d4: 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0438 }
// +0x03d8: 3 encoded words
{ call -4752; r1:0 = combine(#0x1,r16); r3:2 = combine(#0x2,#0x1) }
// +0x03e4: 4 encoded words
{ r0 = ##0x6658; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
// +0x03f4: 3 encoded words
{ r1 = #0x2403; r4 = ##0x890f8010; 	r5 = memw(r29+#0x1c) }
// +0x0400: 2 encoded words
{ r2 = combine(r2.l,r3.l); memw(r29+#0x8) = r5 }
// +0x0408: 2 encoded words
{ call 1406796; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r4 }
// +0x0410: 4 encoded words
{ r0 = ##-0x4fd48be8; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
// +0x0420: 3 encoded words
{ r2 = combine(r2.l,r3.l); r1 = ##0x890f8010; 	r3 = memw(r29+#0x1c) }
// +0x042c: 2 encoded words
{ call -791928; r19 = #0x0 }
// +0x0434: 1 encoded words
{ jump label_04c4 }
label_0438:
// +0x0438: 3 encoded words
{ r19 = #0x1; memb(r16+##0x786) = r19.new }
// +0x0444: 4 encoded words
{ r0 = ##0x6648; r1 = #0x3e12; memw(r22+#0x18) += #0x1 }
// +0x0454: 2 encoded words
{ r2 = memub(r16+#0x3); memw(r29+#0x0) = r2.new }
// +0x045c: 3 encoded words
{ call 1406712; r3 = memw(r29+#0x1c); memw(r29+#0x4) = r3.new }
// +0x0468: 3 encoded words
{ r0 = ##-0x4fd48c00; r1 = memub(r16+#0x3); 	r2 = memw(r29+#0x1c) }
// +0x0474: 1 encoded words
{ call -792500 }
// +0x0478: 4 encoded words
{ r0 = ##0x6650; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
// +0x0488: 3 encoded words
{ r1 = #0x2403; r4 = ##0x890f800f; 	r5 = memw(r29+#0x1c) }
// +0x0494: 2 encoded words
{ r2 = combine(r2.l,r3.l); memw(r29+#0x8) = r5 }
// +0x049c: 2 encoded words
{ call 1406648; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r4 }
// +0x04a4: 4 encoded words
{ r0 = ##-0x4fd48bf4; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
// +0x04b4: 3 encoded words
{ r2 = combine(r2.l,r3.l); r1 = ##0x890f800f; 	r3 = memw(r29+#0x1c) }
// +0x04c0: 1 encoded words
{ call -792076 }
label_04c4:
// +0x04c4: 1 encoded words
{ memh(r20+#0x0) += #0x1 }
label_04c8:
// +0x04c8: 2 encoded words
{ r0 = r19 ; jump ##1973488 }
label_04d0:
// +0x04d0: 3 encoded words
{ call -3696; r0 = ##-0x4fd48d59 }
label_04dc:
// +0x04dc: 2 encoded words
{ r1 = #0x406; jump label_04d0 }
label_04e4:
// +0x04e4: 2 encoded words
{ r1 = #0x403; jump label_04d0 }
