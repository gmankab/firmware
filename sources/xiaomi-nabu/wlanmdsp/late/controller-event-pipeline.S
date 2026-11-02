// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x108bd4,0x10904c), runtime [0xb00c8bd4,0xb00c904c).
// Bounded controller event processing, ring-buffer copy, diagnostic fall-through
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_controller_event_pipeline
agent14_late_controller_event_pipeline:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1952616; allocframe(#0x78) }
pkt_0008:
// +0x8: 2 encoded word(s)
{ r19 = memw(r2+#0xc); if (cmp.eq(r19.new,#0x0)) jump:nt pkt_00d4 }
pkt_0010:
// +0x10: 3 encoded word(s)
{ r1 = #0x3e53; r0 = ##0x69b8; 	r17 = memw(r19+#0x8) }
pkt_001c:
// +0x1c: 1 encoded word(s)
{ r16 = memw(r19+#0x4); 	r20 = memw(r19+#0xc) }
pkt_0020:
// +0x20: 1 encoded word(s)
{ memw(r29+#0x8) = r20; 	memw(r29+#0x4) = r17 }
pkt_0024:
// +0x24: 2 encoded word(s)
{ call 1385916; memw(r29+#0x0) = r16 }
pkt_002c:
// +0x2c: 4 encoded word(s)
{ call -812780; r1:0 = combine(r16,##-0x4fd48488); r3:2 = combine(r20,r17) }
pkt_003c:
// +0x3c: 4 encoded word(s)
{ r0 = ##0x69c0; r1 = #0x2458; r2 = memub(r19+#0x60) }
pkt_004c:
// +0x4c: 2 encoded word(s)
{ r3 = memub(r19+#0x61); r4 = memub(r19+#0x62) }
pkt_0054:
// +0x54: 2 encoded word(s)
{ r5 = memub(r19+#0x63); r6 = memub(r19+#0x64) }
pkt_005c:
// +0x5c: 2 encoded word(s)
{ r7 = memub(r19+#0x65); r8 = memub(r19+#0x66) }
pkt_0064:
// +0x64: 2 encoded word(s)
{ r9 = memub(r19+#0x67); memw(r29+#0x1c) = r9.new }
pkt_006c:
// +0x6c: 2 encoded word(s)
{ memw(r29+#0x18) = r8; memw(r29+#0x14) = r7 }
pkt_0074:
// +0x74: 1 encoded word(s)
{ memw(r29+#0x10) = r6; 	memw(r29+#0xc) = r5 }
pkt_0078:
// +0x78: 1 encoded word(s)
{ memw(r29+#0x8) = r4; 	memw(r29+#0x4) = r3 }
pkt_007c:
// +0x7c: 2 encoded word(s)
{ call 1385828; memw(r29+#0x0) = r2 }
pkt_0084:
// +0x84: 4 encoded word(s)
{ r1:0 = combine(#0x8,##0xb02b7b84); r2 = memub(r19+#0x60); r3 = memub(r19+#0x61) }
pkt_0094:
// +0x94: 3 encoded word(s)
{ r18 = add(r19,#0x60); r4 = memub(r19+#0x62); r5 = memub(r19+#0x63) }
pkt_00a0:
// +0xa0: 2 encoded word(s)
{ r6 = memub(r19+#0x64); r7 = memub(r19+#0x65) }
pkt_00a8:
// +0xa8: 2 encoded word(s)
{ r8 = memub(r19+#0x66); r9 = memub(r19+#0x67) }
pkt_00b0:
// +0xb0: 2 encoded word(s)
{ memw(r29+#0x1c) = r9; memw(r29+#0x18) = r8 }
pkt_00b8:
// +0xb8: 1 encoded word(s)
{ memw(r29+#0x14) = r7; 	memw(r29+#0x10) = r6 }
pkt_00bc:
// +0xbc: 1 encoded word(s)
{ memw(r29+#0xc) = r5; 	memw(r29+#0x8) = r4 }
pkt_00c0:
// +0xc0: 2 encoded word(s)
{ call -813444; memw(r29+#0x4) = r3; 	memw(r29+#0x0) = r2 }
pkt_00c8:
// +0xc8: 3 encoded word(s)
{ r24 = add(r19,#0x10); r23 = add(r19,#0x50); jump pkt_0194 }
pkt_00d4:
// +0xd4: 3 encoded word(s)
{ r1 = #0x3e53; r0 = ##0x69c8; 	r19 = memw(r2+#0x0) }
pkt_00e0:
// +0xe0: 1 encoded word(s)
{ r16 = memw(r19+#0x4); 	r17 = memw(r19+#0x8) }
pkt_00e4:
// +0xe4: 1 encoded word(s)
{ memw(r29+#0x4) = r17; 	memw(r29+#0x0) = r16 }
pkt_00e8:
// +0xe8: 2 encoded word(s)
{ call 1385720; memw(r29+#0x8) = #0x10 }
pkt_00f0:
// +0xf0: 4 encoded word(s)
{ r1:0 = combine(r16,##-0x4fd48470); r3:2 = combine(#0x10,r17); r20 = #0x10 }
pkt_0100:
// +0x100: 1 encoded word(s)
{ call -812992 }
pkt_0104:
// +0x104: 4 encoded word(s)
{ r0 = ##0x69d0; r2 = memub(r19+#0x2c); r6 = memub(r19+#0x30) }
pkt_0114:
// +0x114: 3 encoded word(s)
{ r1 = #0x2458; r3 = memub(r19+#0x2d); r7 = memub(r19+#0x31) }
pkt_0120:
// +0x120: 2 encoded word(s)
{ r4 = memub(r19+#0x2e); r8 = memub(r19+#0x32) }
pkt_0128:
// +0x128: 2 encoded word(s)
{ r5 = memub(r19+#0x2f); r9 = memub(r19+#0x33) }
pkt_0130:
// +0x130: 2 encoded word(s)
{ memw(r29+#0x1c) = r9; memw(r29+#0x18) = r8 }
pkt_0138:
// +0x138: 1 encoded word(s)
{ memw(r29+#0x14) = r7; 	memw(r29+#0x10) = r6 }
pkt_013c:
// +0x13c: 1 encoded word(s)
{ memw(r29+#0xc) = r5; 	memw(r29+#0x8) = r4 }
pkt_0140:
// +0x140: 2 encoded word(s)
{ call 1385632; memw(r29+#0x4) = r3; 	memw(r29+#0x0) = r2 }
pkt_0148:
// +0x148: 4 encoded word(s)
{ r1:0 = combine(#0x8,##0xb02b7b9c); r2 = memub(r19+#0x2c); r6 = memub(r19+#0x30) }
pkt_0158:
// +0x158: 3 encoded word(s)
{ r18 = add(r19,#0x2c); r3 = memub(r19+#0x2d); r7 = memub(r19+#0x31) }
pkt_0164:
// +0x164: 2 encoded word(s)
{ r4 = memub(r19+#0x2e); r8 = memub(r19+#0x32) }
pkt_016c:
// +0x16c: 2 encoded word(s)
{ r5 = memub(r19+#0x2f); r9 = memub(r19+#0x33) }
pkt_0174:
// +0x174: 2 encoded word(s)
{ memw(r29+#0x1c) = r9; memw(r29+#0x18) = r8 }
pkt_017c:
// +0x17c: 1 encoded word(s)
{ memw(r29+#0x14) = r7; 	memw(r29+#0x10) = r6 }
pkt_0180:
// +0x180: 1 encoded word(s)
{ memw(r29+#0xc) = r5; 	memw(r29+#0x8) = r4 }
pkt_0184:
// +0x184: 2 encoded word(s)
{ call -813640; memw(r29+#0x4) = r3; 	memw(r29+#0x0) = r2 }
pkt_018c:
// +0x18c: 2 encoded word(s)
{ r24 = add(r19,#0xc); r23 = add(r19,#0x1c) }
pkt_0194:
// +0x194: 3 encoded word(s)
{ r2 = add(r29,#0x20); r22 = memw(gp+#0x78c); memw(r29+#0x20) = #0x8 }
pkt_01a0:
// +0x1a0: 2 encoded word(s)
{ memw(r2+#0xc) = #0x0; memb(r29+#0x2e) = r16 }
pkt_01a8:
// +0x1a8: 2 encoded word(s)
{ memb(r2+#0xf) = #0x1; memw(r2+#0x4) = #0x3 }
pkt_01b0:
// +0x1b0: 3 encoded word(s)
{ call 1390700; r19 = memw(r22+#0x2a8); memw(r2+#0x8) = #0x0 }
pkt_01bc:
// +0x1bc: 3 encoded word(s)
{ r2 = and(r17,##-0x1000000); r3 = memub(gp+#0x2a) }
pkt_01c8:
// +0x1c8: 4 encoded word(s)
{ r4 = ##0x4000000; memw(r3<<#0x3+##0xb06e6d3c) = r17 }
pkt_01d8:
// +0x1d8: 3 encoded word(s)
{ p0 = cmp.eq(r2,r4); r4 = add(r3,#1); r5 = r3; if (p0.new) r21 = #0 }
pkt_01e4:
// +0x1e4: 3 encoded word(s)
{ r5 = add(##0xb06e6d38,asl(r5,#0x3)); r6 = and(r4,#0xff) }
pkt_01f0:
// +0x1f0: 3 encoded word(s)
{ p1 = cmp.eq(r6,#0xa); if (p0) r1:0 = combine(r16,r22); memw(r5+#0x0) = r0 }
pkt_01fc:
// +0x1fc: 1 encoded word(s)
{ r6 = mux(p1,#0x0,r4) }
pkt_0200:
// +0x200: 2 encoded word(s)
{ if (p0) jump:nt pkt_0314; memb(gp+#0x2a) = r6 }
pkt_0208:
// +0x208: 3 encoded word(s)
{ r3 = ##0x2000000; if (cmp.eq(r3.new,r2)) jump:nt pkt_0264 }
pkt_0214:
// +0x214: 4 encoded word(s)
{ r1 = #0x593; p0 = cmp.eq(r2,##0x1000000); if (!p0.new) jump:nt pkt_046c }
pkt_0224:
// +0x224: 3 encoded word(s)
{ call -996; r1:0 = combine(r16,r19); r2 = #0x1 }
pkt_0230:
// +0x230: 3 encoded word(s)
{ r1 = #0x523; r21 = r0; if (cmp.eq(r21.new,#0x0)) jump:nt pkt_046c }
pkt_023c:
// +0x23c: 2 encoded word(s)
{ r2 = #0x41; if (cmp.gtu(r2.new,r20)) jump:t pkt_035c }
pkt_0244:
// +0x244: 3 encoded word(s)
{ r1 = #0x3e51; r0 = ##0x69d8; 	memw(r29+#0x0) = r20 }
pkt_0250:
// +0x250: 1 encoded word(s)
{ call 1385360 }
pkt_0254:
// +0x254: 3 encoded word(s)
{ call -811488; r1:0 = combine(r20,##-0x4fd48458) }
pkt_0260:
// +0x260: 1 encoded word(s)
{ jump 1952228 }
pkt_0264:
// +0x264: 4 encoded word(s)
{ call 1385340; r0 = ##0x69e8; r1 = #0x3e10 }
pkt_0274:
// +0x274: 3 encoded word(s)
{ call -813872; r0 = ##-0x4fd48440 }
pkt_0280:
// +0x280: 2 encoded word(s)
{ call -560084; r1:0 = combine(r16,r22) }
pkt_0288:
// +0x288: 1 encoded word(s)
{ r21 = #0x0; 	r22 = r0 }
pkt_028c:
// +0x28c: 2 encoded word(s)
{ r20 = memw(r22+#0xec); if (cmp.eq(r20.new,#0x0)) jump:nt pkt_0404 }
pkt_0294:
// +0x294: 2 encoded word(s)
{ r2 = memw(r19+#0x8); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0404 }
pkt_029c:
// +0x29c: 2 encoded word(s)
{ r2 = add(r2,#-0x1); memw(r19+#0x8) = r2.new }
pkt_02a4:
// +0x2a4: 2 encoded word(s)
{ if (p0.new) r1 = zxtb(r16); p0 = cmp.eq(r2,#0x0); 	if (p0.new) r0 = #0 }
pkt_02ac:
// +0x2ac: 2 encoded word(s)
{ if (p0) call ##-745548 }
pkt_02b4:
// +0x2b4: 1 encoded word(s)
{ r2 = memub(r20+#0xb8) }
pkt_02b8:
// +0x2b8: 3 encoded word(s)
{ p0 = !tstbit(r2,#0x1); if (p0.new) r1 = #0x565; if (p0.new) jump:nt pkt_046c }
pkt_02c4:
// +0x2c4: 2 encoded word(s)
{ r2 = memw(r20+#0x30); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_02e0 }
pkt_02cc:
// +0x2cc: 2 encoded word(s)
{ call -4928; r0 = r20 }
pkt_02d4:
// +0x2d4: 2 encoded word(s)
{ call 1377492; r1 = #-0x1; 	r0 = memw(r20+#0x30) }
pkt_02dc:
// +0x2dc: 1 encoded word(s)
{ memw(r20+#0x30) = #0x0 }
pkt_02e0:
// +0x2e0: 1 encoded word(s)
{ r21 = add(r20,#0x10) }
pkt_02e4:
// +0x2e4: 2 encoded word(s)
{ call 1387172; r0 = r21 }
pkt_02ec:
// +0x2ec: 2 encoded word(s)
{ call 1387416; r0 = r21 }
pkt_02f4:
// +0x2f4: 3 encoded word(s)
{ call -814880; r1:0 = combine(#0x0,r20); r2 = #0x218 }
pkt_0300:
// +0x300: 2 encoded word(s)
{ r2 = memw(r19+#0xc); memw(r20+#0xb4) = r2.new }
pkt_0308:
// +0x308: 3 encoded word(s)
{ jump pkt_0358; memw(r19+#0xc) = r20; memw(r22+#0xec) = #0x0 }
pkt_0314:
// +0x314: 1 encoded word(s)
{ call -560232 }
pkt_0318:
// +0x318: 2 encoded word(s)
{ r20 = memw(r0+#0xec); if (cmp.eq(r20.new,#0x0)) jump:nt pkt_0404 }
pkt_0320:
// +0x320: 2 encoded word(s)
{ r2 = memw(r19+#0x8); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0404 }
pkt_0328:
// +0x328: 1 encoded word(s)
{ r1:0 = memd(r20+#0x0) }
pkt_032c:
// +0x32c: 3 encoded word(s)
{ r0 = ##0x69f0; memw(r29+#0x4) = r0; 	memw(r29+#0x0) = r1 }
pkt_0338:
// +0x338: 2 encoded word(s)
{ call 1385128; r1 = #0x3e12 }
pkt_0340:
// +0x340: 1 encoded word(s)
{ r3:2 = memd(r20+#0x0) }
pkt_0344:
// +0x344: 3 encoded word(s)
{ call -814072; r1:0 = combine(r3,##-0x4fd48434) }
pkt_0350:
// +0x350: 2 encoded word(s)
{ call -1008; r1:0 = combine(r16,r20) }
pkt_0358:
// +0x358: 1 encoded word(s)
{ r21 = r20 ; jump pkt_0404 }
pkt_035c:
// +0x35c: 3 encoded word(s)
{ call -814592; r1:0 = combine(r18,r21); r2 = #0x8 }
pkt_0368:
// +0x368: 4 encoded word(s)
{ call -814604; r0 = add(r21,#0xc4); r1 = r24; r2 = r20 }
pkt_0378:
// +0x378: 4 encoded word(s)
{ r0 = add(r21,#0x108); r1 = r23; r2 = #0x10; memw(r21+#0x104) = r20 }
pkt_0388:
// +0x388: 1 encoded word(s)
{ call -814636 }
pkt_038c:
// +0x38c: 3 encoded word(s)
{ r2 = #0x0; r3 = memw(r21+#0xb8); memw(r21+#0xc0) = #0x0 }
pkt_0398:
// +0x398: 3 encoded word(s)
{ r2 = clrbit(r3,#0x2); memw(r21+#0x20c) = r2; memb(r21+#0x210) = r2 }
pkt_03a4:
// +0x3a4: 1 encoded word(s)
{ memw(r21+#0xb8) = r2 }
pkt_03a8:
// +0x3a8: 2 encoded word(s)
{ r3 = memw(r19+#0x8); if (!cmp.eq(r3.new,#0x0)) jump:t pkt_03c8 }
pkt_03b0:
// +0x3b0: 3 encoded word(s)
{ r3:2 = combine(r22,##-0x4ff378dc); r4 = #0x0; 	r1:0 = combine(#0,#0x1) }
pkt_03bc:
// +0x3bc: 2 encoded word(s)
{ call -746516; r5 = add(r29,#0x20) }
pkt_03c4:
// +0x3c4: 1 encoded word(s)
{ r2 = memw(r21+#0xb8) }
pkt_03c8:
// +0x3c8: 2 encoded word(s)
{ p0 = tstbit(r2,#0x1); if (p0.new) jump:t pkt_03dc }
pkt_03d0:
// +0x3d0: 2 encoded word(s)
{ r2 = setbit(r2,#0x1); memw(r21+#0xb8) = r2.new }
pkt_03d8:
// +0x3d8: 1 encoded word(s)
{ memw(r19+#0x8) += #0x1 }
pkt_03dc:
// +0x3dc: 1 encoded word(s)
{ r1:0 = memd(r21+#0x0) }
pkt_03e0:
// +0x3e0: 3 encoded word(s)
{ r0 = ##0x69e0; memw(r29+#0x4) = r0; 	memw(r29+#0x0) = r1 }
pkt_03ec:
// +0x3ec: 2 encoded word(s)
{ call 1384948; r1 = #0x3e12 }
pkt_03f4:
// +0x3f4: 1 encoded word(s)
{ r3:2 = memd(r21+#0x0) }
pkt_03f8:
// +0x3f8: 3 encoded word(s)
{ call -814252; r1:0 = combine(r3,##-0x4fd4844c) }
pkt_0404:
// +0x404: 3 encoded word(s)
{ call -815152; r0 = add(r29,#0x33); r1 = #0x0; 	r2 = #0x15 }
pkt_0410:
// +0x410: 4 encoded word(s)
{ r2 = lsr(r17,#0x18); p0 = cmp.eq(r21,#0x0); r0 = memw(r19+#0x8); memb(r29+#0x30) = r16 }
pkt_0420:
// +0x420: 3 encoded word(s)
{ if (p0) jump:nt pkt_0440; memb(r29+#0x31) = r2; memb(r29+#0x32) = r0 }
pkt_042c:
// +0x42c: 1 encoded word(s)
{ r2 = add(r29,#0x30); 	r1 = r21 }
pkt_0430:
// +0x430: 3 encoded word(s)
{ call -814804; r0 = add(r2,#0x8); r2 = #0x8 }
pkt_043c:
// +0x43c: 1 encoded word(s)
{ jump pkt_0448 }
pkt_0440:
// +0x440: 1 encoded word(s)
{ r1:0 = combine(#0x0,#0x0) }
pkt_0444:
// +0x444: 1 encoded word(s)
{ memd(r29+#0x38) = r1:0 }
pkt_0448:
// +0x448: 1 encoded word(s)
{ r2 = add(r29,#0x30); 	r1 = r18 }
pkt_044c:
// +0x44c: 3 encoded word(s)
{ call -814832; r0 = add(r2,#0x10); r2 = #0x8 }
pkt_0458:
// +0x458: 4 encoded word(s)
{ call 1382716; r1:0 = combine(#0x18,##0xae7); r2 = add(r29,#0x30) }
pkt_0468:
// +0x468: 1 encoded word(s)
{ jump 1951708 }
pkt_046c:
// +0x46c: 3 encoded word(s)
{ call -5360; r0 = ##-0x4fd485e6 }
