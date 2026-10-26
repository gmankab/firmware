// SPDX-License-Identifier: GPL-3.0-only
// Agent14 WLAN runtime instruction reconstruction: file [0x105140,0x1055f8).
// Named Hexagon V66 mnemonics and packet boundaries only; no OEM opcode words.
.text
.globl agent14_late_interface_status_query
agent14_late_interface_status_query:
// offset 0x0000; 4 encoded words
{ call -9344; r0 = ##-0x4fd48cc6; r1 = #0x7f9 }
label_0010:
// offset 0x0010; 2 encoded words
{ call 1967620; allocframe(#0x40) }
// offset 0x0018; 2 encoded words
{ r17 = r0; r0 = memw(gp+#0x78c) }
// offset 0x0020; 2 encoded words
{ call -544480; r1 = memub(r17+#0x0) }
// offset 0x0028; 2 encoded words
{ r16 = r0; r0 = add(r29,#0x10); memw(r29+#0x10) = r17 }
// offset 0x0030; 3 encoded words
{ r2 = memuh(r16+#0x1c); memw(r0+#0x4) = ##-0x4ff3af18 }
// offset 0x003c; 2 encoded words
{ if (p0.new) r20 = add(r17,#0x30); p0 = tstbit(r2,#0); if (!p0.new) jump:t label_019c }
// offset 0x0044; 2 encoded words
{ r18 = memw(##0xb035bdb0) }
// offset 0x004c; 2 encoded words
{ r2 = memb(r16+#0x0); if (!cmp.eq(r2.new,#0x2)) jump:t label_00e0 }
// offset 0x0054; 3 encoded words
{ r2 = add(r29,#0x18); r21 = memub(r16+#0xa); memw(r29+#0x18) = #-0x1 }
// offset 0x0060; 2 encoded words
{ p0 = cmp.eq(r21,#0x0); if (p0.new) jump:nt label_00e0; memh(r2+#0x4) = #-0x1 }
// offset 0x0068; 1 encoded words
{ r22 = #0x0;  r19 = memw(r16+#0x10) }
label_006c:
// offset 0x006c; 1 encoded words
{ p0 = cmp.eq(r19,#0x0); if (p0.new) jump:nt label_0078 }
// offset 0x0070; 2 encoded words
{ r2 = memw(r19+#0x18); if (cmp.eq(r2.new,r16)) jump:nt label_0088 }
label_0078:
// offset 0x0078; 3 encoded words
{ r19 = add(r19,#0x98); r22 = add(r22,#0x1); if (cmp.gtu(r21,r22.new)) jump:t label_006c }
// offset 0x0084; 1 encoded words
{ jump label_00e0 }
label_0088:
// offset 0x0088; 2 encoded words
{ r2 = memw(r16+#0xc); if (cmp.eq(r2.new,r19)) jump:nt label_0078 }
// offset 0x0090; 1 encoded words
{ r2 = memb(r19+#0x3d) }
// offset 0x0094; 2 encoded words
{ p0 = cmp.eq(r2,#-0x7f); if (p0.new) jump:t label_0078 }
// offset 0x009c; 4 encoded words
{ r0 = add(r29,#0x18); r1 = r20; r2 = memw(r19+#0x0); memb(r29+#0x18) = r2.new }
// offset 0x00ac; 2 encoded words
{ r3 = lsr(r2,#0x8); memb(r29+#0x19) = r3.new }
// offset 0x00b4; 3 encoded words
{ r3 = lsr(r2,#0x10); r2 = lsr(r2,#0x18); memb(r29+#0x1a) = r3.new }
// offset 0x00c0; 2 encoded words
{ memb(r29+#0x1b) = r2; r2 = memw(r19+#0x4) }:mem_noshuf
// offset 0x00c8; 2 encoded words
{ r2 = lsr(r2,#0x8); memb(r29+#0x1c) = r2 }
// offset 0x00d0; 2 encoded words
{ call -10948; memb(r29+#0x1d) = r2 }
// offset 0x00d8; 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:t label_0078 }
// offset 0x00dc; 1 encoded words
{ jump label_00e8 }
label_00e0:
// offset 0x00e0; 2 encoded words
{ r19 = memw(r16+#0xc); if (cmp.eq(r19.new,#0x0)) jump:nt label_019c }
label_00e8:
// offset 0x00e8; 3 encoded words
{ call -9588; r0 = memw(gp+#0x78c); r1 = memub(r19+#0x69) }
// offset 0x00f4; 2 encoded words
{ r21 = r0; if (cmp.eq(r21.new,#0x0)) jump:nt label_019c }
// offset 0x00fc; 3 encoded words
{ r2 = ##0x1000088; r1 = r20; r22 = memw(r21+#0x10) }
// offset 0x0108; 2 encoded words
{ r2 = memub(r22+#0x1f); memw(r22+#0x8) = r2 }
// offset 0x0110; 2 encoded words
{ r3 = memub(r22+#0x1e); memw(r22+#0xc) = r19 }
// offset 0x0118; 3 encoded words
{ r3 |= asl(r2,#0x8); r4 = memub(r22+#0x1d); memb(r22+#0x6) = #0x0 }
// offset 0x0124; 2 encoded words
{ r0 = memub(r22+#0x1c); memb(r22+#0x4) = #0x0 }
// offset 0x012c; 2 encoded words
{ r0 |= asl(r4,#0x8); r2 = memw(r17+#0x8) }
// offset 0x0134; 2 encoded words
{ r0 |= asl(r3,#0x10); call -799044 }
// offset 0x013c; 2 encoded words
{ r3 = memub(r22+#0x1b); r2 = memw(r17+#0x8) }
// offset 0x0144; 1 encoded words
{ r4 = memub(r22+#0x1a) }
// offset 0x0148; 1 encoded words
{ r4 |= asl(r3,#0x8) }
// offset 0x014c; 2 encoded words
{ r2 = add(r2,r4); memb(r22+#0x1a) = r2.new }
// offset 0x0154; 2 encoded words
{ r2 = lsr(r2,#0x8); memb(r22+#0x1b) = r2.new }
// offset 0x015c; 1 encoded words
{ r0 = memw(r17+#0x8);  r1 = memub(r17+#0x2) }
// offset 0x0160; 3 encoded words
{ r0 = ##0x6718; r2 = memub(r16+#0x3);  memw(r29+#0x8) = r0 }
// offset 0x016c; 3 encoded words
{ call 1400584; r1 = #0x2413; memw(r29+#0x4) = r1;  memw(r29+#0x0) = r2 }
// offset 0x0178; 3 encoded words
{ r0 = ##-0x4fd48ac8; r1 = memub(r16+#0x3);  r2 = memub(r17+#0x2) }
// offset 0x0184; 2 encoded words
{ call -798128; r3 = memw(r17+#0x8) }
// offset 0x018c; 4 encoded words
{ call -9724; r1:0 = combine(r19,r16); r3:2 = combine(r21,r18); r4 = add(r29,#0x10) }
label_019c:
// offset 0x019c; 1 encoded words
{ jump 1967444 }
// offset 0x01a0; 2 encoded words
{ call 1967256; allocframe(#0x10) }
// offset 0x01a8; 2 encoded words
{ r2 = memw(gp+#0x78c); r3 = memuh(r1+#0x0) }
// offset 0x01b0; 3 encoded words
{ r17 = +mpyi(r3,##0x130); r18 = memw(r2+#0x2d8) }
// offset 0x01bc; 2 encoded words
{ r16 = add(r18,mpyi(r3,##0x130)) }
// offset 0x01c4; 3 encoded words
{ call label_0010; r17 += add(r18,#0x10); r0 = r16 }
// offset 0x01d0; 2 encoded words
{ call 1402444; r0 = r17 }
// offset 0x01d8; 2 encoded words
{ r3 = #0x3e8; r0 = r17;  r2 = memw(r16+#0x4) }
// offset 0x01e0; 2 encoded words
{ r3:2 = mpyu(r2,r3); call 1967308 }
// offset 0x01e8; 1 encoded words
{ jump label_01ec }
label_01ec:
// offset 0x01ec; 2 encoded words
{ r4 = #0x0 ; jump ##1402292 }
// offset 0x01f4; 2 encoded words
{ call 1967156; allocframe(#0x28) }
// offset 0x01fc; 4 encoded words
{ r17:16 = combine(r0,r1); r0 = ##0x6720; r20 = memw(r2+#0x2d8) }
// offset 0x020c; 3 encoded words
{ r1 = #0x1003; r21 = ##0xb036010e;  r3 = memub(r17+#0x3) }
// offset 0x0218; 2 encoded words
{ r19 = memuh(gp+#0x10e); r18 = memub(r2+#0x1dc) }
// offset 0x0220; 3 encoded words
{ r6 = combine(r3.l,r19.l); r2 = ##0x840f800d;  r4 = memw(r16+#0x0) }
// offset 0x022c; 2 encoded words
{ r19 |= asl(r3,#0x10); memw(r29+#0x8) = r4;  memw(r29+#0x4) = r6 }
// offset 0x0234; 2 encoded words
{ call 1400384; memw(r29+#0x0) = r2 }
// offset 0x023c; 4 encoded words
{ r0 = ##-0x4fd48abc; r2 = memub(r17+#0x3); r3 = memuh(gp+#0x10e) }
// offset 0x024c; 3 encoded words
{ r2 = combine(r2.l,r3.l); r1 = ##0x840f800d;  r3 = memw(r16+#0x0) }
// offset 0x0258; 1 encoded words
{ call -798340 }
// offset 0x025c; 1 encoded words
{ memh(r21+#0x0) += #0x1 }
// offset 0x0260; 2 encoded words
{ r16 = add(r20,#0x10); r2 = memw(r16+#0x0) }
// offset 0x0268; 1 encoded words
{ p0 = cmp.eq(r2,#0x1); if (p0.new) jump:nt label_02ac }
// offset 0x026c; 1 encoded words
{ p0 = cmp.eq(r2,#0x2); if (!p0.new) jump:nt label_02f4 }
// offset 0x0270; 3 encoded words
{ r19 = extract(r19,#0x8,#0x10); r17 = #0x0; p0 = cmp.eq(r18,#0x0); if (p0.new) jump:nt label_02f4 }
label_027c:
// offset 0x027c; 2 encoded words
{ r2 = memb(r16+#-0xf); if (!cmp.eq(r2.new,#0x0)) jump:t label_0298 }
// offset 0x0284; 1 encoded words
{ r2 = memb(r16+#-0x10) }
// offset 0x0288; 2 encoded words
{ p0 = !cmp.eq(r2,r19); if (!p0.new) r0 = add(r16,#0x0) }
// offset 0x0290; 2 encoded words
{ if (!p0) call ##1402252 }
label_0298:
// offset 0x0298; 2 encoded words
{ r17 = add(r17,#0x1); r16 = add(r16,#0x130) }
// offset 0x02a0; 2 encoded words
{ r2 = and(r17,#0xff); if (cmp.gtu(r18,r2.new)) jump:t label_027c }
// offset 0x02a8; 1 encoded words
{ jump label_02f4 }
label_02ac:
// offset 0x02ac; 2 encoded words
{ r19 = extract(r19,#0x8,#0x10); p0 = cmp.eq(r18,#0x0); if (p0.new) jump:nt label_02f4 }
// offset 0x02b4; 2 encoded words
{ r17 = #0x0; r20 = #0x3e8 }
label_02bc:
// offset 0x02bc; 2 encoded words
{ r2 = memb(r16+#-0xf); if (!cmp.eq(r2.new,#0x0)) jump:t label_02e4 }
// offset 0x02c4; 2 encoded words
{ r2 = memb(r16+#-0x10); if (!cmp.eq(r2.new,r19)) jump:t label_02e4 }
// offset 0x02cc; 2 encoded words
{ call 1402192; r0 = r16 }
// offset 0x02d4; 2 encoded words
{ r0 = r16; r2 = memw(r16+#-0xc) }
// offset 0x02dc; 2 encoded words
{ r3:2 = mpyu(r2,r20); call label_01ec }
label_02e4:
// offset 0x02e4; 2 encoded words
{ r17 = add(r17,#0x1); r16 = add(r16,#0x130) }
// offset 0x02ec; 2 encoded words
{ r2 = and(r17,#0xff); if (cmp.gtu(r18,r2.new)) jump:t label_02bc }
label_02f4:
// offset 0x02f4; 1 encoded words
{ jump 1967120 }
// offset 0x02f8; 2 encoded words
{ call 1966896; allocframe(#0x20) }
// offset 0x0300; 1 encoded words
{ r16 = r0;  r19 = memw(r1+#0x0) }
// offset 0x0304; 1 encoded words
{ r20 = memw(r16+#0x1dc) }
// offset 0x0308; 2 encoded words
{ r2 = memw(r19+#0x8); if (!cmp.gtu(r20,r2.new)) jump:t label_0394 }
// offset 0x0310; 2 encoded words
{ call -545232; r0 = r16;  r1 = memw(r19+#0x4) }
// offset 0x0318; 3 encoded words
{ r17 = r0; r2 = memw(r19+#0x8); r21 = memw(r16+#0x2d8) }
// offset 0x0324; 4 encoded words
{ r18 = +mpyi(r2,##0x130); r3 = add(r21,mpyi(r2,##0x130)) }
// offset 0x0334; 1 encoded words
{ r18 += add(r21,#0x10) }
// offset 0x0338; 2 encoded words
{ call 1402084; r0 = r18;  memb(r3+#0x1) = #1 }
// offset 0x0340; 2 encoded words
{ call 1402328; r0 = r18 }
// offset 0x0348; 1 encoded words
{ r2 = #0xff }
// offset 0x034c; 2 encoded words
{ p0 = bitsclr(r20,r2); if (p0.new) jump:nt label_0380 }
// offset 0x0354; 2 encoded words
{ r4 = #0x0; r3 = add(r21,#0x1); r2 = and(r20,#255) }
label_035c:
// offset 0x035c; 2 encoded words
{ r5 = memb(r3+#0x0); if (!cmp.eq(r5.new,#0x0)) jump:t label_0370 }
// offset 0x0364; 1 encoded words
{ r5 = memub(r3+#-0x1) }
// offset 0x0368; 2 encoded words
{ r6 = memw(r19+#0x4); if (cmp.eq(r6.new,r5)) jump:nt label_03b4 }
label_0370:
// offset 0x0370; 2 encoded words
{ r4 = add(r4,#0x1); r3 = add(r3,#0x130) }
// offset 0x0378; 2 encoded words
{ r5 = and(r4,#0xff); if (cmp.gtu(r2,r5.new)) jump:t label_035c }
label_0380:
// offset 0x0380; 4 encoded words
{ call 1966872; r1:0 = combine(##-0x4ff3accc,r17); r2 = r16 }
// offset 0x0390; 1 encoded words
{ jump -575240 }
label_0394:
// offset 0x0394; 3 encoded words
{ r1 = #0x2451; r0 = ##0x6728;  memw(r29+#0x0) = r2 }
// offset 0x03a0; 1 encoded words
{ call 1400020 }
// offset 0x03a4; 3 encoded words
{ call 1966836; r0 = ##0xb02b7550;  r1 = memw(r19+#0x8) }
// offset 0x03b0; 1 encoded words
{ jump -796840 }
label_03b4:
// offset 0x03b4; 1 encoded words
{ jump 1966928 }
// offset 0x03b8; 2 encoded words
{ call 1966704; allocframe(#0x20) }
// offset 0x03c0; 1 encoded words
{ r16 = r0;  r19 = r1 }
// offset 0x03c4; 2 encoded words
{ r2 = memw(r16+#0x2d8); if (cmp.eq(r2.new,#0x0)) jump:nt 244 }
// offset 0x03cc; 2 encoded words
{ r20 = memw(r19+#0x0); r3 = memub(r16+#0x1dc) }
// offset 0x03d4; 1 encoded words
{ r4 = add(r3,#-0x1) }
// offset 0x03d8; 2 encoded words
{ r3 = memw(r20+#0x8); if (cmp.gtu(r3.new,r4)) jump:nt 224 }
// offset 0x03e0; 1 encoded words
{ r4 = memw(r20+#0x10) }
// offset 0x03e4; 2 encoded words
{ r5 = #0x101; if (cmp.gtu(r5.new,r4)) jump:t label_040c }
// offset 0x03ec; 3 encoded words
{ r1 = #0x3e51; r0 = ##0x6730;  memw(r29+#0x0) = r4 }
// offset 0x03f8; 1 encoded words
{ call 1399932 }
// offset 0x03fc; 3 encoded words
{ call 1966748; r0 = ##0xb02b755c;  r1 = memw(r20+#0x10) }
// offset 0x0408; 1 encoded words
{ jump -796928 }
label_040c:
// offset 0x040c; 3 encoded words
{ r21 = add(r2,mpyi(r3,##0x130)); r4 = memw(r20+#0x4) }
// offset 0x0418; 3 encoded words
{ r18 = +mpyi(r3,##0x130); r5 = addasl(r16,r4,#0x2) }
// offset 0x0424; 2 encoded words
{ r18 += add(r2,#0x10); r17 = memw(r5+#0x54) }
// offset 0x042c; 2 encoded words
{ r6 = memb(r21+#0x1); if (!cmp.eq(r6.new,#0x0)) jump:t label_0448 }
// offset 0x0434; 2 encoded words
{ call 1401832; r0 = r18 }
// offset 0x043c; 2 encoded words
{ call 1402076; r0 = r18 }
// offset 0x0444; 1 encoded words
{ r4 = memw(r20+#0x4) }
label_0448:
// offset 0x0448; 3 encoded words
{ r0 = add(r21,#0x30); memb(r21+#0x0) = r4; r2 = memw(r20+#0x10) }:mem_noshuf
// offset 0x0454; 2 encoded words
{ memw(r21+#0x8) = r2; r2 = memw(r20+#0xc) }:mem_noshuf
// offset 0x045c; 2 encoded words
{ memw(r21+#0x4) = r2; r2 = memw(r20+#0x8) }:mem_noshuf
// offset 0x0464; 3 encoded words
{ r19 = add(r21,#0x2); memb(r21+#0x2) = r2; r1 = memw(r19+#0xc) }:mem_noshuf
// offset 0x0470; 2 encoded words
{ call -799872; r2 = memw(r20+#0x10) }
// offset 0x0478; 4 encoded words
{ call 1401156; r1:0 = combine(##-0x4ff3ad20,r18); r2 = r19 }
// offset 0x0488; 2 encoded words
{ memb(r21+#0x1) = #0x0; r2 = memuh(r17+#0x1c) }:mem_noshuf
// offset 0x0490; 1 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_04a4 }
// offset 0x0494; 2 encoded words
{ r3 = #0x3e8; r0 = r18;  r2 = memw(r21+#0x4) }
// offset 0x049c; 2 encoded words
{ r3:2 = mpyu(r2,r3); call label_01ec }
label_04a4:
// offset 0x04a4; 4 encoded words
{ call 1966580; r1:0 = combine(##-0x4ff3accc,r17); r2 = r16 }
// offset 0x04b4; 1 encoded words
{ jump -577824 }
