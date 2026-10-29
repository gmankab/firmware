// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x108368,0x1085e4), runtime [0xb00c8368,0xb00c85e4).
// Decode tagged interface record, validate ranges and dispatch status
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_record_codec_check
agent14_late_session_record_codec_check:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1954796; allocframe(#0x40) }
pkt_0008:
// +0x8: 4 encoded word(s)
{ r17:16 = combine(r2,r0); r21:20 = combine(r3,r1); r0 = ##0x6908 }
pkt_0018:
// +0x18: 3 encoded word(s)
{ r1 = #0x2413; r23 = and(r21,#0x100); r5:4 = memd(r16+#0x0) }
pkt_0024:
// +0x24: 3 encoded word(s)
{ r22 = and(r21,##0x1000); memw(r29+#0x8) = r22.new }
pkt_0030:
// +0x30: 2 encoded word(s)
{ call 1388060; memw(r29+#0x4) = r23; 	memw(r29+#0x0) = r4 }
pkt_0038:
// +0x38: 3 encoded word(s)
{ r3:2 = combine(r22,r23); r0 = ##0xb02b7a70; 	r1 = memw(r16+#0x0) }
pkt_0044:
// +0x44: 1 encoded word(s)
{ call -810648 }
pkt_0048:
// +0x48: 2 encoded word(s)
{ r3:2 = combine(#0x0,#0x0); r4 = memub(r17+#0x8); 	r5 = memub(r17+#0x7) }
pkt_0050:
// +0x50: 2 encoded word(s)
{ r5 |= asl(r4,#0x8); r6 = memub(r17+#0xc); 	r0 = memub(r17+#0x6) }
pkt_0058:
// +0x58: 1 encoded word(s)
{ r7 = memub(r17+#0xb); 	memd(r29+#0x18) = r3:2 }
pkt_005c:
// +0x5c: 3 encoded word(s)
{ r7 |= asl(r6,#0x8); r8 = memub(r17+#0xa); r11 = memub(r17+#0x5) }
pkt_0068:
// +0x68: 3 encoded word(s)
{ r11 |= asl(r0,#0x8); r9 = memub(r17+#0x9); r1:0 = memd(r16+#0x0) }
pkt_0074:
// +0x74: 3 encoded word(s)
{ r9 |= asl(r8,#0x8); r11 |= asl(r5,#0x10); memd(r29+#0x10) = r3:2 }
pkt_0080:
// +0x80: 2 encoded word(s)
{ r9 |= asl(r7,#0x10); r19 = swiz(r11) }
pkt_0088:
// +0x88: 1 encoded word(s)
{ r18 = swiz(r9) }
pkt_008c:
// +0x8c: 3 encoded word(s)
{ p0 = cmp.gtu(r19:18,r1:0); if (!p0.new) jump:t pkt_00e0; if (!p0.new) memw(r29+#0x4) = r18 }
pkt_0098:
// +0x98: 1 encoded word(s)
{ p0 = cmp.eq(r23,#0x0); if (!p0.new) jump:t pkt_0108 }
pkt_009c:
// +0x9c: 3 encoded word(s)
{ if (!p0.new) r3 = add(r17,#0x4d); if (!p0.new) r5 = add(r17,#0x4f); p0 = cmp.eq(r22,#0x0); if (!p0.new) jump:t pkt_0188 }
pkt_00a8:
// +0xa8: 3 encoded word(s)
{ r1 = #0x2451; r0 = ##0x6918; 	r2 = memub(r17+#0x2) }
pkt_00b4:
// +0xb4: 1 encoded word(s)
{ r3 = memub(r17+#0x1) }
pkt_00b8:
// +0xb8: 3 encoded word(s)
{ r3 |= asl(r2,#0x8); call 1387924; memw(r29+#0x0) = r3.new }
pkt_00c4:
// +0xc4: 3 encoded word(s)
{ r0 = ##-0x4fd48578; r2 = memub(r17+#0x2); 	r1 = memub(r17+#0x1) }
pkt_00d0:
// +0xd0: 3 encoded word(s)
{ r1 |= asl(r2,#0x8); call -808944; r20 = #0x1; 	r17 = #0x0 }
pkt_00dc:
// +0xdc: 1 encoded word(s)
{ jump pkt_0268 }
pkt_00e0:
// +0xe0: 3 encoded word(s)
{ r1 = #0x2452; r0 = ##0x6910; 	memw(r29+#0x0) = r0 }
pkt_00ec:
// +0xec: 1 encoded word(s)
{ call 1387872 }
pkt_00f0:
// +0xf0: 3 encoded word(s)
{ r0 = ##0xb02b7a7c; r2 = r18; r1 = memw(r16+#0x0) }
pkt_00fc:
// +0xfc: 2 encoded word(s)
{ call -811332; r17 = #0x1 }
pkt_0104:
// +0x104: 1 encoded word(s)
{ jump pkt_0264 }
pkt_0108:
// +0x108: 2 encoded word(s)
{ r3 = memub(r17+#0x57); r5 = memub(r17+#0x4f) }
pkt_0110:
// +0x110: 2 encoded word(s)
{ r4 = memub(r17+#0x56); r7 = memub(r17+#0x4e) }
pkt_0118:
// +0x118: 2 encoded word(s)
{ r0 = memub(r17+#0x4d); r2 = memub(r17+#0x58) }
pkt_0120:
// +0x120: 4 encoded word(s)
{ r3 |= asl(r2,#0x8); r0 |= asl(r7,#0x8); r6 = memub(r17+#0x50); r2 = memub(r17+#0x55) }
pkt_0130:
// +0x130: 4 encoded word(s)
{ r2 |= asl(r4,#0x8); r5 |= asl(r6,#0x8); r7 = memub(r17+#0x53); r6 = memub(r17+#0x5c) }
pkt_0140:
// +0x140: 4 encoded word(s)
{ r2 |= asl(r3,#0x10); r0 |= asl(r5,#0x10); r5 = memub(r17+#0x54); r4 = memub(r17+#0x5b) }
pkt_0150:
// +0x150: 4 encoded word(s)
{ r7 |= asl(r5,#0x8); r4 |= asl(r6,#0x8); r8 = memub(r17+#0x5a); r6 = memub(r17+#0x52) }
pkt_0160:
// +0x160: 2 encoded word(s)
{ r5 = add(r17,#0x5f); r3 = memub(r17+#0x59) }
pkt_0168:
// +0x168: 2 encoded word(s)
{ r3 |= asl(r8,#0x8); r1 = memub(r17+#0x51) }
pkt_0170:
// +0x170: 2 encoded word(s)
{ r1 |= asl(r6,#0x8); r3 |= asl(r4,#0x10) }
pkt_0178:
// +0x178: 3 encoded word(s)
{ r1 |= asl(r7,#0x10); r3 = add(r17,#0x5d); memd(r29+#0x18) = r3:2 }
pkt_0184:
// +0x184: 1 encoded word(s)
{ memd(r29+#0x10) = r1:0 }
pkt_0188:
// +0x188: 3 encoded word(s)
{ r1:0 = combine(r20,r16); r4 = add(r17,#0x2d); r3 = memub(r3+#0x1); 	r2 = memub(r17+#0x0) }
pkt_0194:
// +0x194: 2 encoded word(s)
{ call -1408; r3 = r21; 	memw(r29+#0x0) = r3 }
pkt_019c:
// +0x19c: 1 encoded word(s)
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0224 }
pkt_01a0:
// +0x1a0: 2 encoded word(s)
{ r2 = memb(r17+#0x0); if (!cmp.eq(r2.new,#0x2)) jump:t pkt_01f0 }
pkt_01a8:
// +0x1a8: 1 encoded word(s)
{ r2 = memub(r17+#0x2) }
pkt_01ac:
// +0x1ac: 3 encoded word(s)
{ if (p0.new) r0 = add(r17,#0x4d); p0 = tstbit(r2,#0); if (!p0.new) jump:t pkt_01f0; if (p0.new) r2 = memub(r20+#0x2) }
pkt_01b8:
// +0x1b8: 2 encoded word(s)
{ r1 = and(r21,#0x7); r5 = add(r16,#0x108) }
pkt_01c0:
// +0x1c0: 3 encoded word(s)
{ r0 = #0x0; r4 = add(r29,#0x10); r3 = memub(r20+#0x3); 	memw(r29+#0x0) = r0 }
pkt_01cc:
// +0x1cc: 1 encoded word(s)
{ r2 |= asl(r3,#0x8) }
pkt_01d0:
// +0x1d0: 2 encoded word(s)
{ r3 = swiz(r2); r2 = r20 }
pkt_01d8:
// +0x1d8: 1 encoded word(s)
{ r3 = add(#0x4,lsr(r3,#0x10)) }
pkt_01dc:
// +0x1dc: 2 encoded word(s)
{ call -1548; r3 = zxth(r3) }
pkt_01e4:
// +0x1e4: 3 encoded word(s)
{ if (p0.new) r0 = ##0x6928; p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_024c }
pkt_01f0:
// +0x1f0: 1 encoded word(s)
{ r2 = memw(r16+#0x8) }
pkt_01f4:
// +0x1f4: 1 encoded word(s)
{ r2 = memub(r2+#0x1e) }
pkt_01f8:
// +0x1f8: 3 encoded word(s)
{ r2 = memw(r2<<#0x2+##0xb0345888); if (cmp.eq(r2.new,#0x4)) jump:nt pkt_0208 }
pkt_0204:
// +0x204: 1 encoded word(s)
{ p0 = cmp.eq(r2,#0x8); if (!p0.new) jump:nt pkt_0238 }
pkt_0208:
// +0x208: 4 encoded word(s)
{ r0 = add(r16,#0x13b); r1 = add(r17,#0x3d); r2 = #0x8; memd(r16+#0x0) = r19:18 }
pkt_0218:
// +0x218: 2 encoded word(s)
{ call -812112; r17 = #0x0; 	r20 = #0x0 }
pkt_0220:
// +0x220: 1 encoded word(s)
{ jump pkt_0268 }
pkt_0224:
// +0x224: 2 encoded word(s)
{ r20 = ##0xb02b7a94; 	r17 = #0x2 }
pkt_022c:
// +0x22c: 3 encoded word(s)
{ r0 = ##0x6920; jump pkt_0254 }
pkt_0238:
// +0x238: 2 encoded word(s)
{ r20 = ##0xb02b7aac; 	r17 = #0x0 }
pkt_0240:
// +0x240: 3 encoded word(s)
{ r0 = ##0x6930; jump pkt_0254 }
pkt_024c:
// +0x24c: 2 encoded word(s)
{ r20 = ##0xb02b7aa0; 	r17 = #0x3 }
pkt_0254:
// +0x254: 2 encoded word(s)
{ call 1387512; r1 = #0x2450 }
pkt_025c:
// +0x25c: 2 encoded word(s)
{ call -811692; r0 = r20 }
pkt_0264:
// +0x264: 1 encoded word(s)
{ r20 = #0x1 }
pkt_0268:
// +0x268: 3 encoded word(s)
{ call -2460; r1:0 = combine(r17,r16); r3:2 = combine(r19,r18) }
pkt_0274:
// +0x274: 2 encoded word(s)
{ r0 = r20 ; jump ##1954388 }
