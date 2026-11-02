// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107d30,0x107f38), runtime [0xb00c7d30,0xb00c7f38).
// Serialize an interface command and update the extended session record, with error-path diagnostics.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_record_command
agent14_late_session_record_command:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1956388; allocframe(#0x48) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ r19:18 = combine(r3,r0); r1 = #0x0; r16 = r1; r20 = r2 }
pkt_0014:
// +0x14: 2 encoded word(s)
{ r2 = #0x5c; r4 = memw(r18+#0x30); 	r22 = memw(r18+#0x8) }
pkt_001c:
// +0x1c: 1 encoded word(s)
{ r21 = memw(r4+#0x10) }
pkt_0020:
// +0x20: 2 encoded word(s)
{ r4 = memub(r21+#0x1f); memb(r21+#0x1b) = #0x0 }
pkt_0028:
// +0x28: 2 encoded word(s)
{ r5 = memub(r21+#0x1e); memb(r21+#0x1a) = #0x0 }
pkt_0030:
// +0x30: 3 encoded word(s)
{ r5 |= asl(r4,#0x8); r6 = memub(r21+#0x1d); r17 = memub(r21+#0x1c) }
pkt_003c:
// +0x3c: 1 encoded word(s)
{ r17 |= asl(r6,#0x8) }
pkt_0040:
// +0x40: 1 encoded word(s)
{ r17 |= asl(r5,#0x10) }
pkt_0044:
// +0x44: 2 encoded word(s)
{ call -810444; r0 = add(r17,#0x7) }
pkt_004c:
// +0x4c: 4 encoded word(s)
{ call -810060; r1:0 = combine(##-0x4fd485a4,r17); r2 = #0x5 }
pkt_005c:
// +0x5c: 4 encoded word(s)
{ r2 = and(r19,##0x1100); r3 = #0x1000; r0 = add(r17,#0x9) }
pkt_006c:
// +0x6c: 4 encoded word(s)
{ r1 = add(r20,#0x5); p1 = cmp.eq(r2,##0x1000); p0 = cmp.eq(r2,r3) }
pkt_007c:
// +0x7c: 3 encoded word(s)
{ r2 = p1; r19 = and(r19,#0x7); memb(r17+#0x6) = r19.new }
pkt_0088:
// +0x88: 4 encoded word(s)
{ r23 = r2; r2 = #0x8; r3 = mux(p0,#0x12,#0x3); memb(r17+#0x5) = r3.new }
pkt_0098:
// +0x98: 1 encoded word(s)
{ call -810136 }
pkt_009c:
// +0x9c: 4 encoded word(s)
{ r0 = r23; r20 = add(r17,#0x51); r2 = memub(r20+#0x0); memb(r17+#0x4) = r2.new }
pkt_00ac:
// +0xac: 3 encoded word(s)
{ p0 = r0; r2 = memub(r21+#0x1b); r3 = memub(r21+#0x1a) }
pkt_00b8:
// +0xb8: 3 encoded word(s)
{ r3 |= asl(r2,#0x8); if (!p0) r0 = add(r18,##0x108) }
pkt_00c4:
// +0xc4: 4 encoded word(s)
{ r2 = add(r3,#0x63); if (p0) r0 = add(r18,##0xc4); memb(r21+#0x1a) = r2.new }
pkt_00d4:
// +0xd4: 3 encoded word(s)
{ r2 = lsr(r2,#0x8); if (!p0) jump:nt pkt_0114; memb(r21+#0x1b) = r2.new }
pkt_00e0:
// +0xe0: 3 encoded word(s)
{ r2 = add(r17,#0x53); memb(r20+#0x1) = #0x10; memb(r20+#0x0) = #0x0 }
pkt_00ec:
// +0xec: 4 encoded word(s)
{ r2 = add(r29,#0xc); r4 = #0x1; r1 = memw(r18+#0x104); memw(r29+#0x4) = r2 }
pkt_00fc:
// +0xfc: 3 encoded word(s)
{ r3:2 = combine(#0x0,#0x0); r5 = add(r29,#0x10); memw(r29+#0x0) = r2; 	memw(r29+#0x10) = r17 }
pkt_0108:
// +0x108: 2 encoded word(s)
{ call 242020; memw(r29+#0xc) = #0x53 }
pkt_0110:
// +0x110: 1 encoded word(s)
{ jump pkt_012c }
pkt_0114:
// +0x114: 2 encoded word(s)
{ r3:2 = combine(#0x63,r17); r4 = add(r29,#0x10); 	r1 = r19 }
pkt_011c:
// +0x11c: 1 encoded word(s)
{ call -448 }
pkt_0120:
// +0x120: 3 encoded word(s)
{ call -810272; r2 = #0x10; r1 = add(r29,#0x10); r0 = r20 }
pkt_012c:
// +0x12c: 3 encoded word(s)
{ r2 = memw(##0xb035bdb0); memw(r21+#0x8) = #0x8 }
pkt_0138:
// +0x138: 1 encoded word(s)
{ memb(r21+#0x4) = #0; 	memb(r21+#0x1) = #0 }
pkt_013c:
// +0x13c: 2 encoded word(s)
{ r3 = memw(r22+#0xc); memw(r21+#0xc) = r3.new }
pkt_0144:
// +0x144: 2 encoded word(s)
{ r3 = memw(r22+#0xc); if (cmp.eq(r3.new,#0x0)) jump:nt pkt_015c }
pkt_014c:
// +0x14c: 1 encoded word(s)
{ r3 = memub(r3+#0xc) }
pkt_0150:
// +0x150: 3 encoded word(s)
{ p0 = tstbit(r3,#0x1); if (!p0.new) r2 = #0x0; jump pkt_0160 }
pkt_015c:
// +0x15c: 1 encoded word(s)
{ r2 = #0x0 }
pkt_0160:
// +0x160: 2 encoded word(s)
{ r3 = memub(r21+#0x1e); memb(r21+#0x6) = r2 }
pkt_0168:
// +0x168: 2 encoded word(s)
{ r4 = memub(r21+#0x1f); r5 = memub(r21+#0x1d) }
pkt_0170:
// +0x170: 2 encoded word(s)
{ r3 |= asl(r4,#0x8); r17 = memub(r21+#0x1c) }
pkt_0178:
// +0x178: 1 encoded word(s)
{ r17 |= asl(r5,#0x8) }
pkt_017c:
// +0x17c: 2 encoded word(s)
{ r17 |= asl(r3,#0x10); r3 = add(r21,#0x20) }
pkt_0184:
// +0x184: 1 encoded word(s)
{ r0 = add(r17,#-0xe) }
pkt_0188:
// +0x188: 4 encoded word(s)
{ p0 = !cmp.gtu(r3,r0); if (!p0.new) r0 = ##-0x4fd48602; if (!p0.new) r1 = #0x199 }
pkt_0198:
// +0x198: 2 encoded word(s)
{ if (!p0) call ##-888 }
pkt_01a0:
// +0x1a0: 4 encoded word(s)
{ r1 = lsr(r0,#0x10); r2 = lsr(r0,#0x18); r3 = memub(r21+#0x1a); memb(r21+#0x1e) = r1.new }
pkt_01b0:
// +0x1b0: 4 encoded word(s)
{ r4 = lsr(r0,#0x8); r2 = #0x6; r1 = memub(r21+#0x1b); memb(r21+#0x1f) = r2 }
pkt_01c0:
// +0x1c0: 2 encoded word(s)
{ memb(r21+#0x1c) = r0; memb(r21+#0x1d) = r4 }
pkt_01c8:
// +0x1c8: 2 encoded word(s)
{ r3 |= asl(r1,#0x8); r1 = r16 }
pkt_01d0:
// +0x1d0: 2 encoded word(s)
{ r3 = add(r3,#0xe); memb(r21+#0x1a) = r3.new }
pkt_01d8:
// +0x1d8: 3 encoded word(s)
{ call -810456; r3 = lsr(r3,#0x8); memb(r21+#0x1b) = r3.new }
pkt_01e4:
// +0x1e4: 4 encoded word(s)
{ call -810468; r0 = add(r17,#-0x8); r1 = add(r22,#0x4); r2 = #0x6 }
pkt_01f4:
// +0x1f4: 3 encoded word(s)
{ r2 = #0x8e; r0 = #0x88; memb(r17+#-0x1) = r2.new }
pkt_0200:
// +0x200: 2 encoded word(s)
{ jump 1956096; memb(r17+#-0x2) = r0 }
