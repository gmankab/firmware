// SPDX-License-Identifier: GPL-3.0-only
// Agent14 WLAN runtime instruction reconstruction: file [0x1045e8,0x1050d8).
// Named Hexagon V66 mnemonics and packet boundaries only; no OEM opcode words.
.text
.globl agent14_late_interface_record_dispatch
agent14_late_interface_record_dispatch:
// offset 0x0000; 2 encoded words
{ call 1970560; allocframe(#0x30) }
// offset 0x0008; 3 encoded words
{ call -541552; r17:16 = combine(r5,r3); r19:18 = combine(r2,r4) }
// offset 0x0014; 1 encoded words
{ p0 = cmp.eq(r16,#0x0); if (p0.new) jump:nt label_00d4 }
// offset 0x0018; 1 encoded words
{ p0 = cmp.eq(r19,#0x0); if (p0.new) jump:nt label_00d4 }
// offset 0x001c; 2 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_00d4; if (!p0.new) r2 = memub(r16+#0x4) }
// offset 0x0024; 3 encoded words
{ p0 = !tstbit(r2,#0x0); if (p0.new) jump:t label_0038; if (!p0.new) r3 = memh(r16+#0x8) }
// offset 0x0030; 2 encoded words
{ r4 = #-0x7923; if (!cmp.eq(r4.new,r3)) jump:t label_00f4 }
label_0038:
// offset 0x0038; 1 encoded words
{ r3 = memub(r16+#0x2a) }
// offset 0x003c; 2 encoded words
{ p0 = tstbit(r3,#0); if (!p0.new) jump:t label_004c; if (p0.new) r4 = memb(r16+#0x2c) }
// offset 0x0044; 2 encoded words
{ p0 = cmp.eq(r4,#0x3a); if (!p0.new) jump:t label_00f8 }
label_004c:
// offset 0x004c; 2 encoded words
{ r4 = memw(r16+#0x50); if (cmp.eq(r4.new,#0x0)) jump:nt label_0060 }
// offset 0x0054; 1 encoded words
{ r4 = memh(r16+#0x5a) }
// offset 0x0058; 2 encoded words
{ p0 = cmp.eq(r4,#0x88); if (!p0.new) jump:t label_00f8 }
label_0060:
// offset 0x0060; 2 encoded words
{ r3 = #0x40; r2 = memub(r16+#0x2b) }
// offset 0x0068; 3 encoded words
{ r18 = sub(r18,r2); r20 = add(r19,r2); r4 = memub(r19+r2<<#0x0) }
// offset 0x0074; 2 encoded words
{ p0 = cmp.gtu(r3,r18); r2 = and(r4,#-0x10) }
// offset 0x007c; 1 encoded words
{ if (p0) jump:nt label_0164 }
// offset 0x0080; 4 encoded words
{ p0 = cmpb.eq(r2,#0x60); if (p0.new) r21 = add(r20,#0x28); if (!p0.new) jump:nt label_0164; if (p0.new) r2 = memub(r20+#0x2a) }
// offset 0x0090; 2 encoded words
{ r19 = #0x0; r3 = memb(r20+#0x2b) }
// offset 0x0098; 2 encoded words
{ r2 |= asl(r3,#0x8); if (cmp.eq(r2.new,#0x0)) jump:nt label_0190 }
// offset 0x00a0; 2 encoded words
{ call -512; r1:0 = combine(r21,r20) }
// offset 0x00a8; 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0190 }
// offset 0x00ac; 3 encoded words
{ call -796396; r18 = add(r18,#-0x40); r2 = #0x6; r1:0 = combine(#0,r17) }
// offset 0x00b8; 3 encoded words
{ r0 = r17; r2 = #0x8; if (cmp.gtu(r2.new,r18)) jump:t label_0198 }
// offset 0x00c4; 3 encoded words
{ call -444; r0 = add(r21,#0x18); r1 = r18;  r2 = r17 }
// offset 0x00d0; 1 encoded words
{ jump label_01a8 }
label_00d4:
// offset 0x00d4; 4 encoded words
{ call 1403640; r0 = ##0x66b0; r1 = #0x2450 }
// offset 0x00e4; 3 encoded words
{ call -795572; r0 = ##-0x4fd48b64 }
// offset 0x00f0; 1 encoded words
{ jump label_018c }
label_00f4:
// offset 0x00f4; 1 encoded words
{ r3 = memub(r16+#0x2a) }
label_00f8:
// offset 0x00f8; 3 encoded words
{ r1 = #0x3e56; r0 = ##0x66b8;  r4 = memuh(r16+#0x8) }
// offset 0x0104; 4 encoded words
{ r2 = and(r2,#0x1); r3 = and(r3,#0x1); r5 = memub(r16+#0x2c); r6 = memw(r16+#0x50) }
// offset 0x0114; 2 encoded words
{ r7 = memuh(r16+#0x5a); memw(r29+#0x14) = r7.new }
// offset 0x011c; 1 encoded words
{ memw(r29+#0x10) = r6;  memw(r29+#0xc) = r5 }
// offset 0x0120; 1 encoded words
{ memw(r29+#0x8) = r3;  memw(r29+#0x4) = r4 }
// offset 0x0124; 2 encoded words
{ call 1403560; memw(r29+#0x0) = r2 }
// offset 0x012c; 3 encoded words
{ r1:0 = combine(#0x6,##0xb02b74a8); r2 = memub(r16+#0x4);  r3 = memuh(r16+#0x8) }
// offset 0x0138; 3 encoded words
{ r2 = and(r2,#0x1); r4 = memub(r16+#0x2a); r6 = memw(r16+#0x50) }
// offset 0x0144; 3 encoded words
{ r4 = and(r4,#0x1); r5 = memub(r16+#0x2c); r7 = memuh(r16+#0x5a) }
// offset 0x0150; 1 encoded words
{ memw(r29+#0x14) = r7;  memw(r29+#0x10) = r6 }
// offset 0x0154; 1 encoded words
{ memw(r29+#0xc) = r5;  memw(r29+#0x8) = r4 }
// offset 0x0158; 2 encoded words
{ call -795696; memw(r29+#0x4) = r3;  memw(r29+#0x0) = r2 }
// offset 0x0160; 1 encoded words
{ jump label_018c }
label_0164:
// offset 0x0164; 3 encoded words
{ r1 = #0x3e52; r0 = ##0x66c0;  memw(r29+#0x0) = r18 }
// offset 0x0170; 2 encoded words
{ call 1403484; memw(r29+#0x4) = r2 }
// offset 0x0178; 3 encoded words
{ r1:0 = combine(r18,##-0x4fd48b4c); r2 = memub(r20+#0x0) }
// offset 0x0184; 2 encoded words
{ call -795724; r2 = and(r2,#0xf0) }
label_018c:
// offset 0x018c; 1 encoded words
{ r19 = #0x0 }
label_0190:
// offset 0x0190; 2 encoded words
{ r0 = r19 ; jump ##1970380 }
label_0198:
// offset 0x0198; 1 encoded words
{ r2 = add(r16,#0x4) }
// offset 0x019c; 3 encoded words
{ call -796244; r1 = add(r2,#0x6); r2 = #0x6 }
label_01a8:
// offset 0x01a8; 1 encoded words
{ r19 = #0x1 ; jump label_0190 }
label_01ac:
// offset 0x01ac; 2 encoded words
{ call 1970112; allocframe(#0xa8) }
// offset 0x01b4; 2 encoded words
{ r2 = add(r29,#0x68); r5:4 = combine(#0x0,#0x0); r16 = r0 }
// offset 0x01bc; 2 encoded words
{ r17 = memw(r16+#0x1b4); memd(r29+#0x70) = r5:4 }
// offset 0x01c4; 2 encoded words
{ p0 = cmp.eq(r17,#0x0); if (p0.new) jump:nt label_02d8; memd(r29+#0x68) = r5:4;  memw(r2+#0x10) = #0 }
// offset 0x01cc; 1 encoded words
{ p0 = cmp.eq(r1,#0x0); if (p0.new) jump:nt label_02b0 }
// offset 0x01d0; 2 encoded words
{ r2 = memb(r17+#0x2); if (cmp.eq(r2.new,#0x0)) jump:nt label_02d4 }
// offset 0x01d8; 1 encoded words
{ r3 = #0x3;  r2 = and(r2,#255) }
// offset 0x01dc; 3 encoded words
{ r2 = add(r2,#0x1); p0 = cmp.gtu(r3,r2); if (!p0.new) jump:nt label_02e0; memw(r29+#0x70) = r2.new }
// offset 0x01e8; 3 encoded words
{ r2 = asl(r2,#0x5); r0 = add(r29,#0x8); r1 = #0x0; r22 = add(r29,#0x8) }
// offset 0x01f4; 1 encoded words
{ call -796724 }
// offset 0x01f8; 3 encoded words
{ r2 = add(r29,#0x68); r18 = add(r17,#0x1e4); r3 = memub(r16+#0x3);  memw(r29+#0x78) = r22 }
// offset 0x0204; 2 encoded words
{ memw(r29+#0x8) = #0x3; memw(r2+#0x4) = #0x0 }
// offset 0x020c; 1 encoded words
{ memw(r2+#0xc) = #0;  memw(r29+#0x68) = r3 }
// offset 0x0210; 2 encoded words
{ memw(r22+#0x4) = #0x1; memw(r22+#0x8) = #0x2 }
// offset 0x0218; 2 encoded words
{ memw(r22+#0xc) = #0x6; memb(r22+#0x10) = #0x8 }
// offset 0x0220; 1 encoded words
{ memb(r22+#0x11) = #0x6 }
// offset 0x0224; 3 encoded words
{ r19 = #0x1; r2 = memb(r17+#0x2); if (cmp.eq(r2.new,#0x0)) jump:nt label_02a4 }
// offset 0x0230; 2 encoded words
{ r21:20 = combine(#0x2,#0x20); jump label_0244 }
label_0238:
// offset 0x0238; 3 encoded words
{ r19 = add(r19,#1); r20 = add(r20,#0x20); r18 = add(r18,#0x18); r22 = memw(r29+#0x78) }
label_0244:
// offset 0x0244; 4 encoded words
{ r0 = ##0x66c8; r1 = #0x3e11; memw(r22+r20<<#0x0) = r21 }
// offset 0x0254; 1 encoded words
{ r2 = memw(r29+#0x78) }
// offset 0x0258; 1 encoded words
{ r2 = add(r2,r20) }
// offset 0x025c; 2 encoded words
{ memw(r2+#0x4) = #0x3; r2 = memw(r29+#0x78) }:mem_noshuf
// offset 0x0264; 1 encoded words
{ r2 = add(r2,r20) }
// offset 0x0268; 2 encoded words
{ memw(r2+#0x8) = #0x4; r2 = memw(r29+#0x78) }:mem_noshuf
// offset 0x0270; 1 encoded words
{ r2 = add(r2,r20) }
// offset 0x0274; 2 encoded words
{ memw(r2+#0xc) = #0x18; r2 = memw(r29+#0x78) }:mem_noshuf
// offset 0x027c; 1 encoded words
{ r2 = add(r2,r20);  r3 = memw(r18+#0x0) }
// offset 0x0280; 2 encoded words
{ memw(r2+#0x10) = r3; r2 = memw(r18+#0x0) }:mem_noshuf
// offset 0x0288; 2 encoded words
{ call 1403204; memw(r29+#0x0) = r2 }
// offset 0x0290; 3 encoded words
{ call -793648; r0 = ##0xb02b74c0;  r1 = memw(r18+#0x0) }
// offset 0x029c; 2 encoded words
{ r2 = memub(r17+#0x2); if (cmp.gtu(r2.new,r19)) jump:t label_0238 }
label_02a4:
// offset 0x02a4; 2 encoded words
{ call 645204; r1 = add(r29,#0x68);  r0 = memw(r16+#0x2c) }
// offset 0x02ac; 1 encoded words
{ jump 1970076 }
label_02b0:
// offset 0x02b0; 4 encoded words
{ call 1403164; r0 = ##0x66d0; r1 = #0x3e10 }
// offset 0x02c0; 3 encoded words
{ call -796048; r0 = ##-0x4fd48b34 }
// offset 0x02cc; 2 encoded words
{ call 645868; r1 = #0x0;  r0 = memw(r16+#0x2c) }
label_02d4:
// offset 0x02d4; 1 encoded words
{ jump 1970036 }
label_02d8:
// offset 0x02d8; 2 encoded words
{ r1 = #0x636; jump label_02e4 }
label_02e0:
// offset 0x02e0; 1 encoded words
{ r1 = #0x640 }
label_02e4:
// offset 0x02e4; 3 encoded words
{ call -7180; r0 = ##-0x4fd48d2a }
label_02f0:
// offset 0x02f0; 2 encoded words
{ r1 = #0x0 ; jump ##label_02f8 }
label_02f8:
// offset 0x02f8; 2 encoded words
{ call 1969780; allocframe(#0x160) }
// offset 0x0300; 2 encoded words
{ r2 = add(r29,#0x128); r16 = r0;  r5:4 = combine(#0,#0x0) }
// offset 0x0308; 2 encoded words
{ r17 = memw(r16+#0x1b4); memd(r29+#0x130) = r5:4 }
// offset 0x0310; 3 encoded words
{ p0 = cmp.eq(r17,#0x0); if (p0.new) jump:nt label_04a8; memd(r29+#0x128) = r5:4; memw(r2+#0x10) = #0x0 }
// offset 0x031c; 1 encoded words
{ p0 = cmp.eq(r1,#0x0); if (p0.new) jump:nt label_0480 }
// offset 0x0320; 2 encoded words
{ r2 = memb(r17+#0x1); if (cmp.eq(r2.new,#0x0)) jump:nt label_04a4 }
// offset 0x0328; 3 encoded words
{ r1 = #0x67e; r2 = and(r2,#0xff); if (cmp.gtu(r2.new,#0x6)) jump:nt label_04ac }
// offset 0x0334; 3 encoded words
{ r2 = add(r2,#0x3); r1 = #0x682; memw(r29+#0x130) = r2.new }
// offset 0x0340; 2 encoded words
{ if (!p0.new) r19 = #0x0; p0 = cmp.gtu(r2,#0x9); if (p0.new) jump:nt label_04ac }
// offset 0x0348; 3 encoded words
{ r2 = asl(r2,#0x5); r0 = add(r29,#0x8); r1 = #0x0; r20 = add(r29,#0x8) }
// offset 0x0354; 1 encoded words
{ call -797076 }
// offset 0x0358; 4 encoded words
{ r2 = add(r29,#0x128); r18 = #0x3; r3 = memub(r16+#0x3); memw(r29+#0x128) = r3.new }
// offset 0x0368; 1 encoded words
{ memw(r2+#0x4) = #1;  memw(r2+#0xc) = #0 }
// offset 0x036c; 2 encoded words
{ r3:2 = memd(gp+#0xef8); r1:0 = memd(gp+#0xf00) }
// offset 0x0374; 2 encoded words
{ r3:2 = memd(gp+#0xf08); memd(r29+#0x30) = r3:2 }
// offset 0x037c; 3 encoded words
{ r3:2 = combine(#0x0,#0x1); r0 = #0x87; memd(r29+#0x48) = r3:2;  memd(r29+#0x28) = r1:0 }
// offset 0x0388; 2 encoded words
{ memw(r29+#0x138) = r20; memw(r29+#0x8) = r18 }
// offset 0x0390; 2 encoded words
{ memw(r20+#0x4) = #0x1; memw(r20+#0x8) = #0x2 }
// offset 0x0398; 2 encoded words
{ memw(r20+#0xc) = #0x6; memb(r20+#0x10) = #-0x7a }
// offset 0x03a0; 3 encoded words
{ r20 = add(r17,#0x1c); memb(r20+#0x11) = #-0x23; memb(r20+#0x30) = #0x3a }
// offset 0x03ac; 2 encoded words
{ memd(r29+#0x50) = r3:2; memb(r29+#0x58) = r0 }
// offset 0x03b4; 2 encoded words
{ r2 = memb(r17+#0x1); if (cmp.eq(r2.new,#0x0)) jump:nt label_046c }
// offset 0x03bc; 1 encoded words
{ r18 = #0x3;  r21 = #0x8 }
label_03c0:
// offset 0x03c0; 3 encoded words
{ r22 = +mpyi(r19,#0x1c); r23 = add(r20,mpyi(#0x1c,r19)); r2 = #0x0 }
// offset 0x03cc; 2 encoded words
{ r22 += add(r20,#0x10); jump label_044c }
label_03d4:
// offset 0x03d4; 3 encoded words
{ r3 = asl(r18,#0x5); r2 = #0x1; p0 = cmp.eq(r2,#0x0); if (p0.new) jump:t label_044c }
// offset 0x03e0; 4 encoded words
{ r0 = ##0x66d8; r1 = #0x3e52; r2 = memw(r29+#0x138) }
// offset 0x03f0; 2 encoded words
{ memw(r2+r3<<#0x0) = r21; r2 = memw(r29+#0x138) }:mem_noshuf
// offset 0x03f8; 1 encoded words
{ r2 = addasl(r2,r18,#0x5) }
// offset 0x03fc; 2 encoded words
{ memw(r2+#0x4) = #0x3; r2 = memw(r29+#0x138) }:mem_noshuf
// offset 0x0404; 1 encoded words
{ r2 = addasl(r2,r18,#0x5) }
// offset 0x0408; 2 encoded words
{ memw(r2+#0x8) = #0x4; r2 = memw(r29+#0x138) }:mem_noshuf
// offset 0x0410; 1 encoded words
{ r2 = addasl(r2,r18,#0x5) }
// offset 0x0414; 2 encoded words
{ memw(r2+#0xc) = #0x14; r2 = memw(r29+#0x138) }:mem_noshuf
// offset 0x041c; 2 encoded words
{ r2 = addasl(r2,r18,#0x5); r3 = memw(r22+#0x0) }
// offset 0x0424; 2 encoded words
{ memw(r2+#0x10) = r3; r2 = memw(r22+#0x0) }:mem_noshuf
// offset 0x042c; 3 encoded words
{ call 1402784; memw(r29+#0x4) = r2; memw(r29+#0x0) = #0x0 }
// offset 0x0438; 4 encoded words
{ call -796416; r1:0 = combine(#0x0,##0xb02b74d8); r2 = memw(r22+#0x0) }
// offset 0x0448; 1 encoded words
{ r2 = #0x1;  r18 = add(r18,#1) }
label_044c:
// offset 0x044c; 3 encoded words
{ if (!p0.new) r19 = add(r19,#0x1); p0 = cmp.eq(r2,#0x0); if (p0.new) jump:t label_03d4; if (p0.new) r2 = memw(r23+#0x10) }
// offset 0x0458; 2 encoded words
{ r2 = memub(r17+#0x1); if (cmp.gtu(r2.new,r19)) jump:t label_03c0 }
// offset 0x0460; 3 encoded words
{ r1 = #0x6b7; r2 = memw(r29+#0x130); if (cmp.gtu(r18,r2.new)) jump:nt label_04ac }
label_046c:
// offset 0x046c; 4 encoded words
{ call 644748; r1 = add(r29,#0x128); r0 = memw(r16+#0x2c); memw(r29+#0x130) = r18 }
// offset 0x047c; 1 encoded words
{ jump 1969612 }
label_0480:
// offset 0x0480; 4 encoded words
{ call 1402700; r0 = ##0x66e0; r1 = #0x3e10 }
// offset 0x0490; 3 encoded words
{ call -796512; r0 = ##-0x4fd48b1c }
// offset 0x049c; 2 encoded words
{ call 645404; r1 = #0x1;  r0 = memw(r16+#0x2c) }
label_04a4:
// offset 0x04a4; 1 encoded words
{ jump 1969572 }
label_04a8:
// offset 0x04a8; 1 encoded words
{ r1 = #0x676 }
label_04ac:
// offset 0x04ac; 3 encoded words
{ call -7636; r0 = ##-0x4fd48d13 }
// offset 0x04b8; 2 encoded words
{ call 1969368; allocframe(#0x18) }
// offset 0x04c0; 1 encoded words
{ r16 = r0 }
// offset 0x04c4; 2 encoded words
{ r18 = memw(r16+#0x1b4); if (cmp.eq(r18.new,#0x0)) jump:nt label_05e8 }
// offset 0x04cc; 2 encoded words
{ r2 = memb(r18+#0x2); if (!cmp.eq(r2.new,#0x0)) jump:t label_04dc }
// offset 0x04d4; 2 encoded words
{ r2 = memb(r18+#0x1); if (cmp.eq(r2.new,#0x0)) jump:nt label_05e8 }
label_04dc:
// offset 0x04dc; 3 encoded words
{ r1 = #0x2; r2 = memw(r16+#0x28); r3 = memub(r16+#0x250) }
// offset 0x04e8; 3 encoded words
{ r2 = addasl(r2,r3,#0x2); r5:4 = combine(#0x1,##0x6d6) }
// offset 0x04f4; 1 encoded words
{ r2 = memw(r2+#0x328) }
// offset 0x04f8; 4 encoded words
{ call 1343504; r3:2 = combine(##-0x4fd48cfd,#0x4); r0 = memw(r2+#0x24) }
// offset 0x0508; 2 encoded words
{ call 1073776; r0 = #0x1 }
// offset 0x0510; 2 encoded words
{ call 1411984; r17 = r0 }
// offset 0x0518; 4 encoded words
{ r2 = r0; r1:0 = combine(#0x43,r17); r5:4 = combine(#0x0,#0x0); memw(r29+#0x4) = #0x0 }
// offset 0x0528; 2 encoded words
{ call 1415236; memw(r29+#0x0) = #0x0 }
// offset 0x0530; 1 encoded words
{ r2 = memw(gp+#0x78c) }
// offset 0x0534; 2 encoded words
{ call -731220; r0 = add(r2,#0x1020) }
// offset 0x053c; 2 encoded words
{ r2 = memb(r18+#0x2); if (cmp.eq(r2.new,#0x0)) jump:nt label_0564 }
// offset 0x0544; 2 encoded words
{ call label_01ac; r1:0 = combine(#0x0,r16) }
// offset 0x054c; 3 encoded words
{ call -724616; r1 = #0x7; r2 = #0x2; r0 = memub(r16+#0x3) }
// offset 0x0558; 2 encoded words
{ call -728332; r0 = #0x3;  r1 = memub(r16+#0x3) }
// offset 0x0560; 1 encoded words
{ memb(r18+#0x2) = #0x0 }
label_0564:
// offset 0x0564; 2 encoded words
{ r2 = memb(r18+#0x1); if (cmp.eq(r2.new,#0x0)) jump:nt label_058c }
// offset 0x056c; 2 encoded words
{ call label_02f0; r0 = r16 }
// offset 0x0574; 3 encoded words
{ call -724656; r1 = #0xc; r2 = #0x1; r0 = memub(r16+#0x3) }
// offset 0x0580; 2 encoded words
{ call -728372; r0 = #0x4;  r1 = memub(r16+#0x3) }
// offset 0x0588; 1 encoded words
{ memb(r18+#0x1) = #0x0 }
label_058c:
// offset 0x058c; 4 encoded words
{ r1:0 = combine(#0x0,r16); r2 = #0x0; r3 = memw(##0xb034573c) }
// offset 0x059c; 1 encoded words
{ callr r3 }
// offset 0x05a0; 2 encoded words
{ r1 = #0x1; r2 = memw(gp+#0x78c) }
// offset 0x05a8; 2 encoded words
{ call -731344; r0 = add(r2,#0x1030) }
// offset 0x05b0; 3 encoded words
{ r1 = #0x1; r2 = memw(r16+#0x28); r3 = memub(r16+#0x250) }
// offset 0x05bc; 3 encoded words
{ r2 = addasl(r2,r3,#0x2); r5:4 = combine(#0x1,##0x6f1) }
// offset 0x05c8; 1 encoded words
{ r2 = memw(r2+#0x328) }
// offset 0x05cc; 4 encoded words
{ call 1343292; r3:2 = combine(##-0x4fd48cfd,#0x4); r0 = memw(r2+#0x24) }
// offset 0x05dc; 2 encoded words
{ call 1969192; r0 = r16 }
// offset 0x05e4; 1 encoded words
{ jump -9336 }
label_05e8:
// offset 0x05e8; 1 encoded words
{ jump 1969284 }
// offset 0x05ec; 3 encoded words
{ r1:0 = bitsplit(r1,#0x8); call 1968972; r5:4 = combine(#0,#0x0);  allocframe(#0x68) }
// offset 0x05f8; 3 encoded words
{ r19 = r2; p0 = cmp.eq(r1,#0x170); memd(r29+#0x28) = r5:4;  memd(r29+#0x20) = r5:4 }
// offset 0x0604; 1 encoded words
{ if (!p0) jump:nt label_0aec }
// offset 0x0608; 3 encoded words
{ r22 = ##-0x4fcba8c4; p0 = cmp.eq(r0,#0x3); if (!p0.new) jump:t label_061c }
// offset 0x0614; 2 encoded words
{ r2 = add(r22,#0x8); jump label_0a00 }
label_061c:
// offset 0x061c; 2 encoded words
{ r2 = add(r22,#0x4); p0 = cmp.eq(r0,#0x2); if (p0.new) jump:nt label_0a00 }
// offset 0x0624; 3 encoded words
{ if (p0.new) r3 = add(r29,#0x20); p0 = cmp.eq(r0,#0x1); if (!p0.new) jump:t ##label_0aec }
// offset 0x0630; 2 encoded words
{ r24 = memw(r19+#0x0); r17 = memw(gp+#0x78c) }
// offset 0x0638; 2 encoded words
{ r0 = r17; memw(r3+#0x4) = #0x3 }
// offset 0x0640; 2 encoded words
{ r2 = memw(r24+#0x8); memb(r29+#0x2e) = r2.new }
// offset 0x0648; 3 encoded words
{ call -543152; memw(r3+#0x8) = #0x1; r1 = memw(r24+#0x8) }:mem_noshuf
// offset 0x0654; 3 encoded words
{ r5:4 = combine(#0x1,##0x717); r1 = #0x2;  r16 = r0 }
// offset 0x0660; 2 encoded words
{ r3 = memub(r16+#0x250); r2 = memw(r16+#0x28) }
// offset 0x0668; 2 encoded words
{ r2 = addasl(r2,r3,#0x2); r23 = memw(r16+#0x1b4) }
// offset 0x0670; 1 encoded words
{ r2 = memw(r2+#0x328) }
// offset 0x0674; 4 encoded words
{ call 1343124; r3:2 = combine(##-0x4fd48ce3,#0x4); r0 = memw(r2+#0x24) }
// offset 0x0684; 2 encoded words
{ call 1073396; r0 = #0x1 }
// offset 0x068c; 2 encoded words
{ call 1411604; r18 = r0 }
// offset 0x0694; 4 encoded words
{ r2 = r0; r1:0 = combine(#0x43,r18); r5:4 = combine(#0x0,#0x0); memw(r29+#0x4) = #0x0 }
// offset 0x06a4; 2 encoded words
{ call 1414856; memw(r29+#0x0) = #0x0 }
// offset 0x06ac; 1 encoded words
{ r2 = memw(gp+#0x78c) }
// offset 0x06b0; 2 encoded words
{ call -731600; r0 = add(r2,#0x1020) }
// offset 0x06b8; 4 encoded words
{ r25 = add(r23,#0x1c); r20 = add(r23,#0x20); p0 = cmp.eq(r23,#0x0); if (p0.new) jump:nt ##1080 }
// offset 0x06c8; 2 encoded words
{ r26 = #0x48; r18 = #0x0;  r2 = #0x0 }
// offset 0x06d0; 2 encoded words
{ r21 = r20; memb(r23+#0x2) = #0;  memb(r23+#0x1) = #0 }
label_06d8:
// offset 0x06d8; 1 encoded words
{ r3 = memw(r19+#0xc) }
// offset 0x06dc; 1 encoded words
{ r3 = add(r3,r26) }
// offset 0x06e0; 1 encoded words
{ r4 = memw(r3+#-0x44) }
// offset 0x06e4; 1 encoded words
{ p0 = tstbit(r4,#0); if (!p0.new) jump:t label_0790 }
// offset 0x06e8; 3 encoded words
{ r1 = #0x3e15; r0 = ##0x66e8;  r2 = and(r2,#255) }
// offset 0x06f4; 2 encoded words
{ r5 = add(r25,mpyi(#0x1c,r2)); r2 = +mpyi(r2,#0x1c) }
// offset 0x06fc; 2 encoded words
{ memw(r25+r2<<#0x0) = r4; r2 = memw(r3+#-0x40) }:mem_noshuf
// offset 0x0704; 2 encoded words
{ r6 = memw(r3+#-0x38); r4 = memw(r3+#-0x3c) }
// offset 0x070c; 2 encoded words
{ r7 = memw(r3+#-0x34); memw(r5+#0x10) = r7.new }
// offset 0x0714; 1 encoded words
{ memw(r5+#0xc) = r6;  memw(r5+#0x8) = r4 }
// offset 0x0718; 2 encoded words
{ memw(r5+#0x4) = r2; r8 = memw(r3+#0x0) }:mem_noshuf
// offset 0x0720; 2 encoded words
{ r3 = memw(r3+#0x4); memw(r5+#0x18) = r3.new }
// offset 0x0728; 2 encoded words
{ memw(r5+#0x14) = r8; r3 = memub(r16+#0x3) }:mem_noshuf
// offset 0x0730; 1 encoded words
{ memw(r29+#0x10) = r7;  memw(r29+#0xc) = r6 }
// offset 0x0734; 1 encoded words
{ memw(r29+#0x8) = r4;  memw(r29+#0x4) = r2 }
// offset 0x0738; 2 encoded words
{ call 1402004; memw(r29+#0x0) = r3 }
// offset 0x0740; 3 encoded words
{ r1:0 = combine(#0x5,##0xb02b74f0); r3 = memub(r23+#0x1);  r2 = memub(r16+#0x3) }
// offset 0x074c; 1 encoded words
{ r3 = add(r25,mpyi(#0x1c,r3)) }
// offset 0x0750; 1 encoded words
{ r5 = memw(r3+#0x8);  r4 = memw(r3+#0x4) }
// offset 0x0754; 1 encoded words
{ r6 = memw(r3+#0xc);  r3 = memw(r3+#0x10) }
// offset 0x0758; 1 encoded words
{ memw(r29+#0x10) = r3;  memw(r29+#0xc) = r6 }
// offset 0x075c; 1 encoded words
{ memw(r29+#0x8) = r5;  memw(r29+#0x4) = r4 }
// offset 0x0760; 2 encoded words
{ call -797240; memw(r29+#0x0) = r2 }
// offset 0x0768; 1 encoded words
{ r3 = memub(r21+#0x0);  r2 = memb(r21+#0x1) }
// offset 0x076c; 1 encoded words
{ r3 |= asl(r2,#0x8) }
// offset 0x0770; 3 encoded words
{ p0 = cmp.eq(r3,##-0x7f02); if (p0.new) r1:0 = combine(r21,r16) }
// offset 0x077c; 2 encoded words
{ if (p0) call ##-542440 }
// offset 0x0784; 1 encoded words
{ r18 = #0x1;  r2 = memub(r23+#0x1) }
// offset 0x0788; 2 encoded words
{ r2 = add(r2,#0x1); memb(r23+#0x1) = r2.new }
label_0790:
// offset 0x0790; 2 encoded words
{ r26 = add(r26,#0x50); r21 = add(r21,#0x1c) }
// offset 0x0798; 3 encoded words
{ p0 = cmp.eq(r26,#0xe8); if (!p0.new) jump:t label_06d8; if (p0.new) r3 = memw(r24+#0xc) }
// offset 0x07a4; 2 encoded words
{ r4 = add(r3,#-0x1); if (cmp.gtu(r4.new,#0xd)) jump:nt label_0888 }
// offset 0x07ac; 3 encoded words
{ r26 = #0x48; r21 = #0x0; p0 = cmp.eq(r3,#0x0); if (p0.new) jump:nt label_0888 }
label_07b8:
// offset 0x07b8; 1 encoded words
{ r4 = memw(r19+#0x24) }
// offset 0x07bc; 1 encoded words
{ r4 = add(r4,r26) }
// offset 0x07c0; 1 encoded words
{ r5 = memw(r4+#-0x44) }
// offset 0x07c4; 1 encoded words
{ p0 = tstbit(r5,#0); if (!p0.new) jump:t label_0878 }
// offset 0x07c8; 2 encoded words
{ r2 = and(r2,#0xff); r1 = #0x3e15 }
// offset 0x07d0; 2 encoded words
{ r3 = add(r25,mpyi(#0x1c,r2)); r2 = +mpyi(r2,#0x1c) }
// offset 0x07d8; 2 encoded words
{ memw(r25+r2<<#0x0) = r5; r2 = memw(r4+#-0x40) }:mem_noshuf
// offset 0x07e0; 2 encoded words
{ r6 = memw(r4+#-0x38); r5 = memw(r4+#-0x3c) }
// offset 0x07e8; 2 encoded words
{ r7 = memw(r4+#-0x34); memw(r3+#0x10) = r7.new }
// offset 0x07f0; 1 encoded words
{ memw(r3+#0xc) = r6;  memw(r3+#0x8) = r5 }
// offset 0x07f4; 2 encoded words
{ memw(r3+#0x4) = r2; r8 = memw(r4+#0x0) }:mem_noshuf
// offset 0x07fc; 2 encoded words
{ r0 = memw(r4+#0x4); memw(r3+#0x18) = r0.new }
// offset 0x0804; 4 encoded words
{ r0 = ##0x66f0; memw(r3+#0x14) = r8; r3 = memub(r16+#0x3) }:mem_noshuf
// offset 0x0814; 1 encoded words
{ memw(r29+#0x10) = r7;  memw(r29+#0xc) = r6 }
// offset 0x0818; 1 encoded words
{ memw(r29+#0x8) = r5;  memw(r29+#0x4) = r2 }
// offset 0x081c; 2 encoded words
{ call 1401776; memw(r29+#0x0) = r3 }
// offset 0x0824; 3 encoded words
{ r1:0 = combine(#0x5,##0xb02b74fc); r3 = memub(r23+#0x1);  r2 = memub(r16+#0x3) }
// offset 0x0830; 1 encoded words
{ r3 = add(r25,mpyi(#0x1c,r3)) }
// offset 0x0834; 1 encoded words
{ r5 = memw(r3+#0x8);  r4 = memw(r3+#0x4) }
// offset 0x0838; 1 encoded words
{ r6 = memw(r3+#0xc);  r3 = memw(r3+#0x10) }
// offset 0x083c; 1 encoded words
{ memw(r29+#0x10) = r3;  memw(r29+#0xc) = r6 }
// offset 0x0840; 1 encoded words
{ memw(r29+#0x8) = r5;  memw(r29+#0x4) = r4 }
// offset 0x0844; 2 encoded words
{ call -797468; memw(r29+#0x0) = r2 }
// offset 0x084c; 1 encoded words
{ r3 = memub(r20+#0x0);  r2 = memb(r20+#0x1) }
// offset 0x0850; 1 encoded words
{ r3 |= asl(r2,#0x8) }
// offset 0x0854; 3 encoded words
{ p0 = cmp.eq(r3,##-0x7f02); if (p0.new) r1:0 = combine(r20,r16) }
// offset 0x0860; 2 encoded words
{ if (p0) call ##-542668 }
// offset 0x0868; 1 encoded words
{ r2 = memub(r23+#0x1) }
// offset 0x086c; 2 encoded words
{ r2 = add(r2,#0x1); memb(r23+#0x1) = r2.new }
// offset 0x0874; 1 encoded words
{ r3 = memw(r24+#0xc) }
label_0878:
// offset 0x0878; 4 encoded words
{ r26 = add(r26,#0x50); r20 = add(r20,#0x1c); r21 = add(r21,#0x1); if (cmp.gtu(r3,r21.new)) jump:t label_07b8 }
label_0888:
// offset 0x0888; 2 encoded words
{ r20 = #0x0 ; jump label_08a8; r24 = #0x0 }
label_0890:
// offset 0x0890; 4 encoded words
{ r21 += add(r23,##0x1e4); r24 = #0x1; r20 = #0x1 }
// offset 0x08a0; 2 encoded words
{ call -542924; r1:0 = combine(r21,r16) }
label_08a8:
// offset 0x08a8; 1 encoded words
{ r2 = memw(r19+#0x18) }
// offset 0x08ac; 1 encoded words
{ r2 = add(r2,mpyi(#0x18,r24)) }
// offset 0x08b0; 1 encoded words
{ r2 = memw(r2+#0x4) }
// offset 0x08b4; 1 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_0918 }
// offset 0x08b8; 3 encoded words
{ r21 = +mpyi(r24,#0x18); r20 = add(r23,mpyi(#0x18,r24)); memb(r23+#0x2) += #0x1 }
// offset 0x08c4; 1 encoded words
{ r0 = r21;  r2 = memw(r19+#0x18) }
// offset 0x08c8; 4 encoded words
{ r1 = add(r2,mpyi(#0x18,r24)); r0 += add(r23,##0x1dc); r2 = #0x18 }
// offset 0x08d8; 1 encoded words
{ call -798096 }
// offset 0x08dc; 3 encoded words
{ r1 = #0x3e12; r0 = ##0x66f8;  r2 = memub(r16+#0x3) }
// offset 0x08e8; 2 encoded words
{ r3 = memw(r20+#0x1e4); memw(r29+#0x4) = r3.new }
// offset 0x08f0; 2 encoded words
{ call 1401564; memw(r29+#0x0) = r2 }
// offset 0x08f8; 4 encoded words
{ r0 = ##-0x4fd48af8; r1 = memub(r16+#0x3); r2 = memw(r20+#0x1e4) }
// offset 0x0908; 1 encoded words
{ call -797648 }
// offset 0x090c; 3 encoded words
{ p0 = cmp.eq(r24,#0x0); r20 = #0x1; if (p0.new) jump:nt label_0890 }
label_0918:
// offset 0x0918; 2 encoded words
{ r24 = add(r24,#0x1); if (!cmp.eq(r24.new,#0x2)) jump:t label_08a8 }
// offset 0x0920; 2 encoded words
{ r2 = or(r20,r18); if (cmp.eq(r2.new,#0x0)) jump:nt label_0a14 }
// offset 0x0928; 3 encoded words
{ p0 = cmp.eq(r20,#0x0); r2 = memub(r16+#0x3); memw(r29+#0x0) = r2.new }
// offset 0x0934; 4 encoded words
{ r2 = p0; r0 = ##0x6700; r1 = #0x2453 }
// offset 0x0944; 3 encoded words
{ call 1401480; r19 = r2; memw(r29+#0x8) = r18;  memw(r29+#0x4) = r20 }
// offset 0x0950; 3 encoded words
{ r3:2 = combine(r18,r20); r0 = ##0xb02b7514;  r1 = memub(r16+#0x3) }
// offset 0x095c; 1 encoded words
{ call -797232 }
// offset 0x0960; 1 encoded words
{ r2 = r19 }
// offset 0x0964; 3 encoded words
{ p0 = r2; if (p0.new) jump:nt label_09ac; if (!p0.new) memw(r29+#0x20) = #0x4 }
// offset 0x0970; 3 encoded words
{ r3:2 = combine(r17,##-0x4ff3c9a0); r6 = add(r29,#0x20);  r1:0 = combine(#3,#0x1) }
// offset 0x097c; 3 encoded words
{ r4 = #0x0; r5 = add(r29,#0x20); memb(r6+#0xf) = #0x5 }
// offset 0x0988; 1 encoded words
{ call -730100 }
// offset 0x098c; 3 encoded words
{ r1 = #0x7; r3:2 = combine(#0x2,#0x2); r5:4 = combine(#0x0,#0x3); r0 = memub(r16+#0x3) }
// offset 0x0998; 3 encoded words
{ call -725860; memw(r29+#0x4) = #0x0; memw(r29+#0x0) = #0x0 }
// offset 0x09a4; 2 encoded words
{ call label_01ac; r1:0 = combine(#0x1,r16) }
label_09ac:
// offset 0x09ac; 2 encoded words
{ r1:0 = combine(#0x4,#0x1); p0 = cmp.eq(r18,#0x0); if (p0.new) jump:nt label_0a9c }
// offset 0x09b4; 3 encoded words
{ r2 = #0x80; r6 = add(r29,#0x20); memw(r29+#0x20) = r2.new }
// offset 0x09c0; 3 encoded words
{ r3:2 = combine(r17,##-0x4ff3c4b4); r4 = #0x0;  r5 = add(r29,#0x20) }
// offset 0x09cc; 2 encoded words
{ call -730168; memb(r6+#0xf) = #0x3 }
// offset 0x09d4; 3 encoded words
{ r3:2 = combine(#0x1,#0x7); r1 = #0xc; r5:4 = combine(#0x0,#0x3); r0 = memub(r16+#0x3) }
// offset 0x09e0; 3 encoded words
{ call -725932; memw(r29+#0x4) = #0x0; memw(r29+#0x0) = #0x0 }
// offset 0x09ec; 2 encoded words
{ r2 = memub(r23+#0x1); if (cmp.gtu(r2.new,#0x6)) jump:t label_0a70 }
// offset 0x09f4; 2 encoded words
{ call label_02f8; r1:0 = combine(#0x1,r16) }
// offset 0x09fc; 1 encoded words
{ jump label_0a9c }
label_0a00:
// offset 0x0a00; 3 encoded words
{ r1 = r19; r2 = memw(r2+#0x0); r0 = memw(gp+#0x78c) }
// offset 0x0a0c; 1 encoded words
{ callr r2 }
// offset 0x0a10; 1 encoded words
{ jump 1968132 }
label_0a14:
// offset 0x0a14; 2 encoded words
{ call label_01ac; r1:0 = combine(#0x0,r16) }
// offset 0x0a1c; 3 encoded words
{ call -725848; r1 = #0x7; r2 = #0x2; r0 = memub(r16+#0x3) }
// offset 0x0a28; 2 encoded words
{ call -729564; r0 = #0x3;  r1 = memub(r16+#0x3) }
// offset 0x0a30; 2 encoded words
{ call label_02f0; r0 = r16 }
// offset 0x0a38; 3 encoded words
{ call -725876; r1 = #0xc; r2 = #0x1; r0 = memub(r16+#0x3) }
// offset 0x0a44; 2 encoded words
{ call -729592; r0 = #0x4;  r1 = memub(r16+#0x3) }
// offset 0x0a4c; 3 encoded words
{ r1 = #0x3e51; r0 = ##0x6710;  r2 = memub(r16+#0x3) }
// offset 0x0a58; 2 encoded words
{ call 1401204; memw(r29+#0x0) = r2 }
// offset 0x0a60; 3 encoded words
{ call -795648; r0 = ##0xb02b752c;  r1 = memub(r16+#0x3) }
// offset 0x0a6c; 1 encoded words
{ jump label_0a9c }
label_0a70:
// offset 0x0a70; 3 encoded words
{ r1 = #0x3e42; r0 = ##0x6708;  r3 = memub(r16+#0x3) }
// offset 0x0a7c; 2 encoded words
{ call 1401168; memw(r29+#0x4) = r2;  memw(r29+#0x0) = r3 }
// offset 0x0a84; 3 encoded words
{ r0 = ##-0x4fd48ae0; r1 = memub(r16+#0x3);  r2 = memub(r23+#0x1) }
// offset 0x0a90; 1 encoded words
{ call -798040 }
// offset 0x0a94; 2 encoded words
{ call label_02f0; r0 = r16 }
label_0a9c:
// offset 0x0a9c; 2 encoded words
{ r1:0 = combine(r20,r16); r2 = r18;  r3 = memw(r22+#0x0) }
// offset 0x0aa4; 1 encoded words
{ callr r3 }
// offset 0x0aa8; 2 encoded words
{ r1 = #0x1; r2 = memw(gp+#0x78c) }
// offset 0x0ab0; 2 encoded words
{ call -732632; r0 = add(r2,#0x1030) }
// offset 0x0ab8; 3 encoded words
{ r1 = #0x1; r2 = memw(r16+#0x28); r3 = memub(r16+#0x250) }
// offset 0x0ac4; 3 encoded words
{ r2 = addasl(r2,r3,#0x2); r5:4 = combine(#0x1,##0x7d8) }
// offset 0x0ad0; 1 encoded words
{ r2 = memw(r2+#0x328) }
// offset 0x0ad4; 4 encoded words
{ call 1342004; r3:2 = combine(##-0x4fd48ce3,#0x4); r0 = memw(r2+#0x24) }
// offset 0x0ae4; 2 encoded words
{ call -10616; r0 = r16 }
label_0aec:
// offset 0x0aec; 1 encoded words
{ jump 1967912 }
