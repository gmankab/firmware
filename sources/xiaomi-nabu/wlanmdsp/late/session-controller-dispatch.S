// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107f7c,0x108288), runtime [0xb00c7f7c,0xb00c8288).
// Allocate controller resources, process pending session record callbacks, validate stream states and commit completion.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_controller_dispatch
agent14_late_session_controller_dispatch:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1955748; allocframe(#0x68) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ r21:20 = combine(r5,r2); r24 = r4; r19 = r3; 	r17 = memw(r29+#0x70) }
pkt_0014:
// +0x14: 3 encoded word(s)
{ p0 = cmp.gtu(r17,#0x80); r23 = #0x0; r16 = r0; r6 = r1 }
pkt_0020:
// +0x20: 4 encoded word(s)
{ if (!p0) r18 = add(r16,##0x143); if (!p0) r22 = add(r16,##0xc4) }
pkt_0030:
// +0x30: 1 encoded word(s)
{ if (p0) jump:nt pkt_0304 }
pkt_0034:
// +0x34: 1 encoded word(s)
{ r2 = and(r19,#0x7) }
pkt_0038:
// +0x38: 2 encoded word(s)
{ r3 = setbit(r2,#0x0); if (!cmp.eq(r3.new,#0x3)) jump:t pkt_0068 }
pkt_0040:
// +0x40: 3 encoded word(s)
{ r0 = r22; r2 = #0x8; if (cmp.gtu(r2.new,r17)) jump:t pkt_0304 }
pkt_004c:
// +0x4c: 2 encoded word(s)
{ r2 = add(r17,#-0x8); r17 = add(r17,#0xf8) }
pkt_0054:
// +0x54: 1 encoded word(s)
{ r2 = and(r2,#0xf8) }
pkt_0058:
// +0x58: 3 encoded word(s)
{ call 515708; r1 = lsr(r2,#0x3); r3:2 = combine(r18,r21) }
pkt_0064:
// +0x64: 1 encoded word(s)
{ jump pkt_0134 }
pkt_0068:
// +0x68: 2 encoded word(s)
{ r5:4 = combine(#0x0,#0x0); p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt pkt_02a4 }
pkt_0070:
// +0x70: 2 encoded word(s)
{ r1:0 = combine(#0x0,r18); r2 = #0x80 }
pkt_0078:
// +0x78: 1 encoded word(s)
{ memd(r29+#0x28) = r5:4; 	memd(r29+#0x20) = r5:4 }
pkt_007c:
// +0x7c: 2 encoded word(s)
{ call -811088; memd(r29+#0x18) = r5:4; 	memd(r29+#0x10) = r5:4 }
pkt_0084:
// +0x84: 4 encoded word(s)
{ p0 = cmp.eq(r24,#0x0); if (p0.new) jump:nt pkt_0104; if (!p0.new) r3 = memub(r24+#0xb); if (!p0.new) r2 = memub(r24+#0xa) }
pkt_0094:
// +0x94: 3 encoded word(s)
{ r2 |= asl(r3,#0x8); r0 = memub(r24+#0x8); r3 = memub(r24+#0x2) }
pkt_00a0:
// +0xa0: 2 encoded word(s)
{ r6 = memub(r24+#0x3); r4 = memub(r24+#0x9) }
pkt_00a8:
// +0xa8: 4 encoded word(s)
{ r0 |= asl(r4,#0x8); r3 |= asl(r6,#0x8); r5 = memub(r24+#0x1); r4 = memub(r24+#0xf) }
pkt_00b8:
// +0xb8: 3 encoded word(s)
{ r0 |= asl(r2,#0x10); r2 = memub(r24+#0x0); r6 = memub(r24+#0xe) }
pkt_00c4:
// +0xc4: 4 encoded word(s)
{ r2 |= asl(r5,#0x8); r6 |= asl(r4,#0x8); r5 = memub(r24+#0x7); r7 = memub(r24+#0xd) }
pkt_00d4:
// +0xd4: 3 encoded word(s)
{ r2 |= asl(r3,#0x10); r8 = memub(r24+#0x6); r1 = memub(r24+#0xc) }
pkt_00e0:
// +0xe0: 4 encoded word(s)
{ r1 |= asl(r7,#0x8); r8 |= asl(r5,#0x8); r3 = memub(r24+#0x4); r4 = memub(r24+#0x5) }
pkt_00f0:
// +0xf0: 2 encoded word(s)
{ r1 |= asl(r6,#0x10); r3 |= asl(r4,#0x8) }
pkt_00f8:
// +0xf8: 2 encoded word(s)
{ r3 |= asl(r8,#0x10); memd(r29+#0x18) = r1:0 }
pkt_0100:
// +0x100: 1 encoded word(s)
{ memd(r29+#0x10) = r3:2 }
pkt_0104:
// +0x104: 1 encoded word(s)
{ r2 = add(r29,#0x10); 	r1 = r22 }
pkt_0108:
// +0x108: 3 encoded word(s)
{ call -810836; r0 = add(r2,#0x10); r2 = #0x10 }
pkt_0114:
// +0x114: 3 encoded word(s)
{ call -810848; r1:0 = combine(r21,r18); r2 = r17 }
pkt_0120:
// +0x120: 3 encoded word(s)
{ r3:2 = combine(r18,##0x100); r1 = #0x20; 	r0 = add(r29,#0x10) }
pkt_012c:
// +0x12c: 2 encoded word(s)
{ call 201784; r4 = r17 }
pkt_0134:
// +0x134: 4 encoded word(s)
{ r23 = extractu(r19,#0x2,#0x4); r24 = #0x0; p0 = cmp.eq(r20,#0xfe); if (!p0.new) jump:t pkt_0174 }
pkt_0144:
// +0x144: 1 encoded word(s)
{ r2 = #0x0; 	r19 = r18 }
pkt_0148:
// +0x148: 3 encoded word(s)
{ p0 = cmpb.gtu(r17,#0x20); r0 = add(r16,#0x118); if (p0.new) jump:t pkt_028c }
pkt_0154:
// +0x154: 3 encoded word(s)
{ r2 = and(r17,#0xff); r1 = r19; memb(r16+#0x138) = r23 }
pkt_0160:
// +0x160: 4 encoded word(s)
{ call -810924; r23 = #0x1; memb(r16+#0x139) = r17; memb(r16+#0x13a) = r17 }
pkt_0170:
// +0x170: 1 encoded word(s)
{ jump pkt_0304 }
pkt_0174:
// +0x174: 2 encoded word(s)
{ if (p0.new) r25 = zxtb(r17); p0 = cmp.eq(r20,#0x2); if (!p0.new) jump:nt pkt_02a0 }
pkt_017c:
// +0x17c: 2 encoded word(s)
{ r2 = #0x6; if (cmp.gtu(r2.new,r25)) jump:t pkt_02a0 }
pkt_0184:
// +0x184: 3 encoded word(s)
{ r3 = add(r18,#0x5); r24 = #0x0; r23 = #0x1; 	r2 = #0x0 }
pkt_0190:
// +0x190: 4 encoded word(s)
{ r27:26 = combine(r18,#0x15); r21 = add(r16,#0x205); r6 = add(r16,#0x1e5); r19 = #0x0 }
pkt_01a0:
// +0x1a0: 1 encoded word(s)
{ r17 = #0x0 }
pkt_01a4:
// +0x1a4: 1 encoded word(s)
{ r4 = memb(r27+#0x0) }
pkt_01a8:
// +0x1a8: 2 encoded word(s)
{ p0 = cmp.eq(r4,#-0x23); if (!p0.new) jump:t pkt_0268 }
pkt_01b0:
// +0x1b0: 2 encoded word(s)
{ r4 = memub(r27+#0x1); if (cmp.gtu(r26,r4.new)) jump:t pkt_0268 }
pkt_01b8:
// +0x1b8: 2 encoded word(s)
{ r5 = memb(r27+#0x2); if (!cmp.eq(r5.new,#0x0)) jump:t pkt_0268 }
pkt_01c0:
// +0x1c0: 2 encoded word(s)
{ r5 = memb(r27+#0x3); if (!cmp.eq(r5.new,#0xf)) jump:t pkt_0268 }
pkt_01c8:
// +0x1c8: 1 encoded word(s)
{ r5 = memb(r27+#0x4) }
pkt_01cc:
// +0x1cc: 3 encoded word(s)
{ p0 = cmp.eq(r5,#-0x54); if (p0.new) r22 = add(r27,#0x8); if (!p0.new) jump:t pkt_0268 }
pkt_01d8:
// +0x1d8: 2 encoded word(s)
{ r3 = memub(r3+#0x0); if (cmp.eq(r3.new,#0x9)) jump:nt pkt_01fc }
pkt_01e0:
// +0x1e0: 3 encoded word(s)
{ if (p0.new) r19 = add(r22,#0x0); p0 = cmp.eq(r3,#0x1); if (!p0.new) jump:nt pkt_0268; if (p0.new) r3 = memub(r27+#0x6) }
pkt_01ec:
// +0x1ec: 3 encoded word(s)
{ r17 = add(r4,#0xfa); r23 = and(r3,#0x3); memw(r16+#0xbc) = setbit(#0x0) }
pkt_01f8:
// +0x1f8: 1 encoded word(s)
{ jump pkt_0268 }
pkt_01fc:
// +0x1fc: 4 encoded word(s)
{ r1 = r22; p0 = cmp.gtu(r4,#0x2c); if (p0.new) jump:t pkt_0268; if (!p0.new) r2 = memub(r27+#0x6) }
pkt_020c:
// +0x20c: 2 encoded word(s)
{ r2 = and(r2,#0x3); memb(r16+#0x1e3) = r2.new }
pkt_0214:
// +0x214: 1 encoded word(s)
{ r20 = memub(r27+#0x1) }
pkt_0218:
// +0x218: 2 encoded word(s)
{ r2 = add(r20,#0xfa); memb(r16+#0x1e4) = r2.new }
pkt_0220:
// +0x220: 4 encoded word(s)
{ r2 = #0x6; r21 = r6; r3 = memub(r27+#0x1); memw(r29+#0xc) = r21 }
pkt_0230:
// +0x230: 3 encoded word(s)
{ call -811132; r24 = add(r3,#0xfa); r0 = memw(r29+#0xc) }
pkt_023c:
// +0x23c: 4 encoded word(s)
{ r2 = add(r20,#-0xc); r1 = add(r27,#0xe); r0 = r21; r3 = add(r20,#0xf4) }
pkt_024c:
// +0x24c: 3 encoded word(s)
{ call -811160; r2 = and(r2,#0xff); memb(r16+#0x1e4) = r3 }
pkt_0258:
// +0x258: 4 encoded word(s)
{ r6 = r21; r2 = r22; r21 = memw(r29+#0xc); memw(r16+#0xbc) = setbit(#0x0) }
pkt_0268:
// +0x268: 1 encoded word(s)
{ r3 = memub(r27+#0x1) }
pkt_026c:
// +0x26c: 3 encoded word(s)
{ r3 += add(r27,#0x7); r4 = add(r27,r3); r5 = r3 }
pkt_0278:
// +0x278: 1 encoded word(s)
{ r5 += add(r27,#0x2) }
pkt_027c:
// +0x27c: 3 encoded word(s)
{ r4 = add(r4,sub(#0x7,r18)); r27 = r5; if (cmp.gt(r25,r4.new)) jump:t pkt_01a4 }
pkt_0288:
// +0x288: 1 encoded word(s)
{ p0 = cmp.eq(r19,#0x0); if (!p0.new) jump:t pkt_0148 }
pkt_028c:
// +0x28c: 2 encoded word(s)
{ p0 = cmpb.gtu(r24,#0x20); if (p0.new) jump:nt pkt_02a0 }
pkt_0294:
// +0x294: 1 encoded word(s)
{ r23 = #0x1; 	p0 = cmp.eq(r2,#0x0) }
pkt_0298:
// +0x298: 2 encoded word(s)
{ if (p0) r23 = #0x0; jump pkt_0304 }
pkt_02a0:
// +0x2a0: 1 encoded word(s)
{ r23 = #0x0 ; jump pkt_0304 }
pkt_02a4:
// +0x2a4: 2 encoded word(s)
{ r2 = and(r19,##0x1100) }
pkt_02ac:
// +0x2ac: 3 encoded word(s)
{ p0 = cmp.eq(r2,##0x1000); if (!p0.new) jump:t pkt_0304 }
pkt_02b8:
// +0x2b8: 4 encoded word(s)
{ r2 = add(r29,#0x34); r0 = r22; r1 = memw(r16+#0x104); memw(r29+#0x0) = r2.new }
pkt_02c8:
// +0x2c8: 3 encoded word(s)
{ r3:2 = combine(r17,r21); r4 = #0x1; r5 = add(r29,#0x10); memw(r29+#0x10) = r6 }
pkt_02d4:
// +0x2d4: 2 encoded word(s)
{ r6 = sub(r21,r6); memw(r29+#0x4) = r18 }
pkt_02dc:
// +0x2dc: 2 encoded word(s)
{ call 241616; memw(r29+#0x34) = r6 }
pkt_02e4:
// +0x2e4: 1 encoded word(s)
{ p0 = cmp.gt(r0,#-0x1); if (p0.new) jump:t pkt_0134 }
pkt_02e8:
// +0x2e8: 3 encoded word(s)
{ r1 = #0x2451; r0 = ##0x6900; 	memw(r29+#0x0) = r17 }
pkt_02f4:
// +0x2f4: 1 encoded word(s)
{ call 1388356 }
pkt_02f8:
// +0x2f8: 3 encoded word(s)
{ call -808492; r1:0 = combine(r17,##-0x4fd4859c) }
pkt_0304:
// +0x304: 2 encoded word(s)
{ r0 = r23 ; jump ##1955196 }
