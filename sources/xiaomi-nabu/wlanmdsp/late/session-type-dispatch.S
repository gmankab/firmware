// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x1078d0,0x107970), runtime [0xb00c78d0,0xb00c7970).
// Dispatch a bounded record kind through an indexed handler table and forward normalized argument pairs.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_type_dispatch
agent14_late_session_type_dispatch:
pkt_0000:
// +0x0: 3 encoded word(s)
{ r5 = r2; r3 = memw(r1+#0x0); if (cmp.gtu(r3.new,#0x12)) jump:nt pkt_0044 }
pkt_000c:
// +0xc: 3 encoded word(s)
{ r2 = memw(gp+#0x78c); r3 = memw(r3<<#0x2+##0xb02b78b4) }
pkt_0018:
// +0x18: 2 encoded word(s)
{ jumpr r3; r6 = memw(r2+#0x3a8) }
pkt_0020:
// +0x20: 2 encoded word(s)
{ r1:0 = combine(r5,r6); jump -4780 }
pkt_0028:
// +0x28: 3 encoded word(s)
{ r0 = r5 ; jump ##-912; r1 = memw(r1+#0x4) }
pkt_0034:
// +0x34: 2 encoded word(s)
{ r1:0 = combine(r5,r6); jump -500 }
pkt_003c:
// +0x3c: 2 encoded word(s)
{ r1:0 = combine(r5,r6); jump -1304 }
pkt_0044:
// +0x44: 1 encoded word(s)
{ jumpr r31 }
pkt_0048:
// +0x48: 3 encoded word(s)
{ r1:0 = combine(r5,r6); jump -1248; r2 = memw(r1+#0x4) }
pkt_0054:
// +0x54: 2 encoded word(s)
{ r1:0 = combine(r6,r5); r3 = memub(r1+#0x8); 	r2 = memub(r1+#0x4) }
pkt_005c:
// +0x5c: 1 encoded word(s)
{ jump -916 }
pkt_0060:
// +0x60: 1 encoded word(s)
{ r3 = memw(r1+#0x8); 	r2 = memw(r1+#0x4) }
pkt_0064:
// +0x64: 3 encoded word(s)
{ r1:0 = combine(r5,r6); jump -332; r4 = memw(r1+#0xc) }
pkt_0070:
// +0x70: 2 encoded word(s)
{ r1:0 = combine(r6,r5); r2 = memw(r1+#0x4); 	r3 = memw(r1+#0x8) }
pkt_0078:
// +0x78: 1 encoded word(s)
{ jump -900 }
pkt_007c:
// +0x7c: 1 encoded word(s)
{ r3 = memw(r1+#0x8); 	r2 = memw(r1+#0x4) }
pkt_0080:
// +0x80: 3 encoded word(s)
{ r1:0 = combine(r6,r5); jump -796; r4 = memw(r1+#0xc) }
pkt_008c:
// +0x8c: 3 encoded word(s)
{ r1:0 = combine(r6,r5); jump -632; r2 = memw(r1+#0x4) }
pkt_0098:
// +0x98: 1 encoded word(s)
{ jumpr r31 }
pkt_009c:
// +0x9c: 1 encoded word(s)
{ r0 = #0x0; 	jumpr r31 }
