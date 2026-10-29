// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x1077e8,0x1078d0), runtime [0xb00c77e8,0xb00c78d0).
// Update window bounds and optional channel pointers with logged validation failures.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_window_update
agent14_late_session_window_update:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1957760; allocframe(#0x28) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ call -4728; r17:16 = combine(r3,r4); r20 = r1; 	r18 = r2 }
pkt_0014:
// +0x14: 2 encoded word(s)
{ r19 = r0; if (cmp.eq(r19.new,#0x0)) jump:nt pkt_002c }
pkt_001c:
// +0x1c: 4 encoded word(s)
{ if (p0.new) jump:t pkt_004c; jump pkt_0068; p0 = cmp.gtu(r18,#0x64); if (!p0.new) memw(r19+#0x58) = r18 }
pkt_002c:
// +0x2c: 3 encoded word(s)
{ r1 = #0x3e11; r0 = ##0x68b0; 	memw(r29+#0x0) = r20 }
pkt_0038:
// +0x38: 1 encoded word(s)
{ call 1390996 }
pkt_003c:
// +0x3c: 3 encoded word(s)
{ call 1957812; r1:0 = combine(r20,##-0x4fd486ac) }
pkt_0048:
// +0x48: 1 encoded word(s)
{ jump -805864 }
pkt_004c:
// +0x4c: 3 encoded word(s)
{ r1 = #0x3e51; r0 = ##0x68b8; 	memw(r29+#0x0) = r18 }
pkt_0058:
// +0x58: 1 encoded word(s)
{ call 1390964 }
pkt_005c:
// +0x5c: 3 encoded word(s)
{ call -805884; r1:0 = combine(r18,##-0x4fd486a0) }
pkt_0068:
// +0x68: 1 encoded word(s)
{ p0 = cmp.eq(r17,#0x0); if (p0.new) jump:nt pkt_0074 }
pkt_006c:
// +0x6c: 2 encoded word(s)
{ jump pkt_0094; memw(r19+#0x5c) = r17 }
pkt_0074:
// +0x74: 4 encoded word(s)
{ r0 = ##0x68c0; r1 = #0x3e51; memw(r29+#0x0) = #0x0 }
pkt_0084:
// +0x84: 1 encoded word(s)
{ call 1390920 }
pkt_0088:
// +0x88: 3 encoded word(s)
{ call -805928; r1:0 = combine(#0x0,##0xb02b796c) }
pkt_0094:
// +0x94: 1 encoded word(s)
{ p0 = cmp.eq(r16,#0x0); if (p0.new) jump:nt pkt_00a0 }
pkt_0098:
// +0x98: 2 encoded word(s)
{ jump pkt_00c0; memw(r19+#0x60) = r16 }
pkt_00a0:
// +0xa0: 4 encoded word(s)
{ r0 = ##0x68c8; r1 = #0x3e51; memw(r29+#0x0) = #0x0 }
pkt_00b0:
// +0xb0: 1 encoded word(s)
{ call 1390876 }
pkt_00b4:
// +0xb4: 3 encoded word(s)
{ call -805972; r1:0 = combine(#0x0,##0xb02b7978) }
pkt_00c0:
// +0xc0: 3 encoded word(s)
{ r1 = #0x3e53; r0 = ##0x68d0; 	memw(r29+#0x8) = r16 }
pkt_00cc:
// +0xcc: 2 encoded word(s)
{ call 1390848; memw(r29+#0x4) = r17; 	memw(r29+#0x0) = r18 }
pkt_00d4:
// +0xd4: 4 encoded word(s)
{ call 1957660; r1:0 = combine(r18,##-0x4fd4867c); r3:2 = combine(r16,r17) }
pkt_00e4:
// +0xe4: 1 encoded word(s)
{ jump -807864 }
