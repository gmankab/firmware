// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x1085e4,0x108724), runtime [0xb00c85e4,0xb00c8724).
// Validate incoming session record type and check packed byte fields
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_record_type_guard
agent14_late_session_record_type_guard:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1954196; allocframe(#0x18) }
pkt_0008:
// +0x8: 4 encoded word(s)
{ r19 = r4; r17 = r1; r0 = #0x4; r6 = memub(r5+#0x2a) }
pkt_0018:
// +0x18: 3 encoded word(s)
{ p0 = tstbit(r6,#0x0); r16 = #0x0; if (p0.new) r4 = memub(r5+#0x2b) }
pkt_0024:
// +0x24: 2 encoded word(s)
{ if (p0) r2 = sub(r2,r4); if (p0) r17 = add(r17,r4) }
pkt_002c:
// +0x2c: 1 encoded word(s)
{ p0 = cmp.gtu(r0,r2); if (p0.new) jump:t pkt_0138 }
pkt_0030:
// +0x30: 1 encoded word(s)
{ memw(r3+#0x0) = r17 }
pkt_0034:
// +0x34: 2 encoded word(s)
{ r3 = memb(r17+#0x0); if (cmp.eq(r3.new,#0x0)) jump:nt pkt_0138 }
pkt_003c:
// +0x3c: 2 encoded word(s)
{ r3 = memb(r17+#0x1); if (!cmp.eq(r3.new,#0x3)) jump:t pkt_0138 }
pkt_0044:
// +0x44: 2 encoded word(s)
{ r18 = add(r2,#-0x4); if (cmp.eq(r18.new,#0x0)) jump:nt pkt_0138 }
pkt_004c:
// +0x4c: 2 encoded word(s)
{ r2 = #0x60; if (cmp.gtu(r2.new,r18)) jump:nt pkt_0138 }
pkt_0054:
// +0x54: 1 encoded word(s)
{ r2 = memub(r17+#0x4) }
pkt_0058:
// +0x58: 2 encoded word(s)
{ p0 = cmp.eq(r2,#0xfe); p1 = cmp.eq(r2,#0x2) }
pkt_0060:
// +0x60: 1 encoded word(s)
{ p0 = not(p0) }
pkt_0064:
// +0x64: 2 encoded word(s)
{ p0 = and(p0,!p1); if (p0.new) jump:nt pkt_0138 }
pkt_006c:
// +0x6c: 3 encoded word(s)
{ r1 = #0x2451; r0 = ##0x6938; 	r2 = memub(r17+#0x5) }
pkt_0078:
// +0x78: 2 encoded word(s)
{ r3 = memub(r17+#0x6); memb(r19+#0x1) = r3.new }
pkt_0080:
// +0x80: 2 encoded word(s)
{ memb(r19+#0x0) = r2; r2 = memuh(r19+#0x0) } :mem_noshuf
pkt_0088:
// +0x88: 2 encoded word(s)
{ r3 = asl(r2,#0x8); r4 = swiz(r2) }
pkt_0090:
// +0x90: 3 encoded word(s)
{ r3 |= lsr(r2,#0x8); r2 = lsr(r4,#0x10); memh(r19+#0x0) = r3.new }
pkt_009c:
// +0x9c: 2 encoded word(s)
{ call 1387316; memw(r29+#0x0) = r2 }
pkt_00a4:
// +0xa4: 3 encoded word(s)
{ call -809536; r0 = ##0xb02b7ab8; 	r1 = memuh(r19+#0x0) }
pkt_00b0:
// +0xb0: 2 encoded word(s)
{ r4 = #0x1000; r3 = memuh(r19+#0x0) }
pkt_00b8:
// +0xb8: 2 encoded word(s)
{ r2 = and(r3,##0x1100) }
pkt_00c0:
// +0xc0: 1 encoded word(s)
{ p0 = cmp.eq(r2,r4) }
pkt_00c4:
// +0xc4: 2 encoded word(s)
{ r4 = mux(p0,#0x51,#0x61); r5 = mux(p0,#0x4f,#0x5f) }
pkt_00cc:
// +0xcc: 2 encoded word(s)
{ r6 = add(r17,r4); r4 = memub(r17+r4<<#0x0) }
pkt_00d4:
// +0xd4: 1 encoded word(s)
{ r6 = memub(r6+#0x1) }
pkt_00d8:
// +0xd8: 1 encoded word(s)
{ r4 |= asl(r6,#0x8) }
pkt_00dc:
// +0xdc: 1 encoded word(s)
{ r4 = swiz(r4) }
pkt_00e0:
// +0xe0: 2 encoded word(s)
{ r5 += lsr(r4,#0x10); if (!cmp.gt(r5.new,r18)) jump:t pkt_0114 }
pkt_00e8:
// +0xe8: 4 encoded word(s)
{ r17 = lsr(r4,#0x10); r0 = ##0x6940; r1 = #0x2452 }
pkt_00f8:
// +0xf8: 2 encoded word(s)
{ call 1387224; memw(r29+#0x4) = r17; 	memw(r29+#0x0) = r18 }
pkt_0100:
// +0x100: 4 encoded word(s)
{ call -811972; r1:0 = combine(r18,##-0x4fd4853c); r2 = r17 }
pkt_0110:
// +0x110: 1 encoded word(s)
{ jump pkt_0138 }
pkt_0114:
// +0x114: 2 encoded word(s)
{ p0 = !tstbit(r3,#0x7); if (p0.new) jump:nt pkt_0138 }
pkt_011c:
// +0x11c: 2 encoded word(s)
{ r3 = and(r3,##0x208) }
pkt_0124:
// +0x124: 3 encoded word(s)
{ p0 = cmp.eq(r3,##0x200); if (!p0.new) jump:nt pkt_0138 }
pkt_0130:
// +0x130: 2 encoded word(s)
{ p0 = cmp.eq(r2,#0x0); if (!p0.new) r16 = add(r17,#0x4) }
pkt_0138:
// +0x138: 2 encoded word(s)
{ r0 = r16 ; jump ##1954104 }
